//! The accepting side: take Iroh connections from allowlisted peers and serve HTTP/1.1 over their streams.
use std::sync::Arc;

use axum::Router;
use hyper::{body::Incoming, Request};
use hyper_util::rt::TokioIo;
use iroh::{endpoint::Connection, Endpoint, EndpointId};
use tower::ServiceExt;

/// The authenticated identity of the remote endpoint, added to every request before it reaches a handler.
/// It comes from the connection's cryptographic handshake, never from anything the caller sent.
#[derive(Clone, Copy, Debug)]
pub struct Peer(pub EndpointId);

/// Decides whether a peer may talk to us at all.
pub type AllowFn = Arc<dyn Fn(&EndpointId) -> bool + Send + Sync>;

/// QUIC close code sent to peers that are not allowed.
const CLOSE_NOT_ENROLLED: u32 = 1;

/// Accept connections until the endpoint closes. Peers failing `allow` are disconnected straight after the
/// handshake; everyone else gets `app` served over HTTP/1.1, one request per Iroh bi-directional stream.
pub async fn serve(endpoint: Endpoint, allow: AllowFn, app: Router) {
    while let Some(incoming) = endpoint.accept().await {
        let (app, allow) = (app.clone(), allow.clone());
        tokio::spawn(async move {
            match incoming.await {
                Ok(connection) => handle_connection(connection, allow, app).await,
                Err(e) => eprintln!("handshake failed: {e:#}"),
            }
        });
    }
}

async fn handle_connection(connection: Connection, allow: AllowFn, app: Router) {
    let peer = connection.remote_id();
    if !allow(&peer) {
        eprintln!("denied peer {peer}: not enrolled");
        connection.close(CLOSE_NOT_ENROLLED.into(), b"not enrolled");
        return;
    }
    eprintln!("peer connected: {peer}");

    // Each stream the peer opens is one HTTP request/response exchange.
    while let Ok((send, receive)) = connection.accept_bi().await {
        tokio::spawn(serve_stream(send, receive, peer, app.clone()));
    }
    eprintln!("peer disconnected: {peer}");
}

async fn serve_stream(
    send: iroh::endpoint::SendStream,
    receive: iroh::endpoint::RecvStream,
    peer: EndpointId,
    app: Router,
) {
    let io = TokioIo::new(tokio::io::join(receive, send));
    let service = hyper::service::service_fn(move |mut request: Request<Incoming>| {
        request.extensions_mut().insert(Peer(peer));
        app.clone().oneshot(request)
    });
    if let Err(e) = hyper::server::conn::http1::Builder::new().serve_connection(io, service).await {
        eprintln!("stream error from {peer}: {e}");
    }
}
