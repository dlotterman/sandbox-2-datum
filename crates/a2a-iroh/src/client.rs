//! The dialing side: send A2A requests to one peer over Iroh, reconnecting when needed.
use anyhow::{anyhow, Context, Result};
use bytes::Bytes;
use http_body_util::{BodyExt, Full, Limited};
use hyper::{Method, Request, StatusCode};
use hyper_util::rt::TokioIo;
use iroh::{endpoint::Connection, Endpoint, EndpointAddr};
use serde_json::Value;
use tokio::sync::Mutex;

use crate::{a2a, ALPN};

/// Refuse to read more than this from a peer's response.
const MAX_RESPONSE_BYTES: usize = 1 << 20;

/// A client for one peer. It keeps a single Iroh connection and opens a fresh stream per request.
pub struct Client {
    endpoint: Endpoint,
    target: EndpointAddr,
    connection: Mutex<Option<Connection>>,
}

impl Client {
    pub fn new(endpoint: Endpoint, target: EndpointAddr) -> Self {
        Self { endpoint, target, connection: Mutex::new(None) }
    }

    /// Fetch the peer's A2A Agent Card.
    pub async fn agent_card(&self) -> Result<Value> {
        let (status, body) = self.request(Method::GET, "/.well-known/agent-card.json", None).await?;
        if !status.is_success() {
            return Err(anyhow!("agent card: HTTP {status}"));
        }
        Ok(serde_json::from_slice(&body)?)
    }

    /// Ask the peer to run `skill`: an A2A `message/send` whose data part names the skill and its arguments.
    pub async fn call_skill(&self, skill: &str, args: Value, text: &str) -> Result<a2a::TaskOutcome> {
        let request = a2a::message_send(skill, args, text);
        let (_, body) = self.request(Method::POST, "/", Some(request)).await?;
        a2a::parse_task(&serde_json::from_slice(&body)?)
    }

    /// Send one HTTP request over Iroh. If the first attempt fails (for example the connection went stale),
    /// dial again and retry once.
    pub async fn request(&self, method: Method, path: &str, body: Option<Value>) -> Result<(StatusCode, Bytes)> {
        match self.send_once(method.clone(), path, body.as_ref()).await {
            Ok(response) => Ok(response),
            Err(first_error) => {
                eprintln!("request failed ({first_error:#}); redialing");
                *self.connection.lock().await = None;
                self.send_once(method, path, body.as_ref()).await
            }
        }
    }

    /// The existing connection if it is still open, otherwise a newly dialed one.
    async fn connection(&self) -> Result<Connection> {
        let mut slot = self.connection.lock().await;
        if let Some(existing) = slot.as_ref() {
            if existing.close_reason().is_none() {
                return Ok(existing.clone());
            }
        }
        let fresh = self.endpoint.connect(self.target.clone(), ALPN).await.context("dial inside agent")?;
        *slot = Some(fresh.clone());
        Ok(fresh)
    }

    async fn send_once(&self, method: Method, path: &str, body: Option<&Value>) -> Result<(StatusCode, Bytes)> {
        let connection = self.connection().await?;
        let (send, receive) = connection.open_bi().await?;
        let io = TokioIo::new(tokio::io::join(receive, send));
        let (mut sender, connection_driver) = hyper::client::conn::http1::handshake::<_, Full<Bytes>>(io).await?;
        tokio::spawn(async move {
            let _ = connection_driver.await;
        });

        let mut request = Request::builder().method(method).uri(path).header("host", "inside.iroh");
        let payload = match body {
            Some(json) => {
                request = request.header("content-type", "application/json");
                Bytes::from(serde_json::to_vec(json)?)
            }
            None => Bytes::new(),
        };
        let response = sender.send_request(request.body(Full::new(payload))?).await?;

        let status = response.status();
        let body = Limited::new(response.into_body(), MAX_RESPONSE_BYTES)
            .collect()
            .await
            .map_err(|e| anyhow!("read response: {e}"))?
            .to_bytes();
        Ok((status, body))
    }
}
