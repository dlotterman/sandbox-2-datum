//! End-to-end checks of the tunnel itself: real Iroh endpoints on loopback, no relays and no network.
use std::{
    collections::HashSet,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};

use a2a_iroh::{
    iroh::{Endpoint, EndpointAddr, EndpointId, RelayMode, SecretKey},
    serve, task_result, Client, Peer, ALPN, ENROLL_SKILL, NOT_ENROLLED,
};
use axum::{routing::post, Extension, Json, Router};
use serde_json::{json, Value};

/// An endpoint with no relays, reachable only on loopback.
async fn local_endpoint(alpns: Vec<Vec<u8>>) -> Endpoint {
    a2a_iroh::bind_with(SecretKey::generate(), alpns, RelayMode::Disabled).await.unwrap()
}

fn loopback_address_of(endpoint: &Endpoint) -> EndpointAddr {
    let port = endpoint.bound_sockets().iter().find(|s| s.is_ipv4()).expect("an IPv4 socket").port();
    EndpointAddr::new(endpoint.id()).with_ip_addr(SocketAddr::from(([127, 0, 0, 1], port)))
}

/// A tiny A2A server whose only skill reports which authenticated peer called it.
fn whoami_app() -> Router {
    Router::new().route(
        "/",
        post(|Extension(Peer(peer)): Extension<Peer>, Json(request): Json<Value>| async move {
            Json(task_result(
                &request["id"],
                "completed",
                &format!("hello {peer}"),
                Some(json!({"echo": request["params"]})),
            ))
        }),
    )
}

#[tokio::test]
async fn an_a2a_request_crosses_an_iroh_stream_and_the_server_sees_the_callers_key() {
    let server = local_endpoint(vec![ALPN.to_vec()]).await;
    let caller = local_endpoint(vec![]).await;
    let caller_id = caller.id();
    tokio::spawn(serve(server.clone(), Arc::new(move |peer| *peer == caller_id), whoami_app()));

    let client = Client::new(caller, loopback_address_of(&server));
    let outcome = tokio::time::timeout(Duration::from_secs(20), client.call_skill("echo", json!({"n": 1}), "hi"))
        .await
        .expect("no timeout")
        .expect("request succeeds");

    assert_eq!(outcome.state, "completed");
    assert_eq!(outcome.text, format!("hello {caller_id}"), "identity comes from the connection, not the request body");
    assert_eq!(outcome.data.unwrap()["echo"]["message"]["parts"][1]["data"]["skill"], "echo");
}

#[tokio::test]
async fn a_peer_that_is_not_on_the_allowlist_gets_no_answer() {
    let server = local_endpoint(vec![ALPN.to_vec()]).await;
    let stranger = local_endpoint(vec![]).await;
    tokio::spawn(serve(server.clone(), Arc::new(|_| false), whoami_app()));

    let client = Client::new(stranger, loopback_address_of(&server));
    let result = tokio::time::timeout(Duration::from_secs(30), client.call_skill("echo", json!({}), "hi")).await;
    assert!(result.expect("the server must close the connection, not hang").is_err());
}

/// A server that, like the inside agent, only answers enrolled peers: a peer enrolls with the right token and is
/// refused ("not enrolled") until then. Any connection is accepted, so strangers can reach the enroll skill.
fn enrolling_app(token: &'static str) -> Router {
    let enrolled: Arc<Mutex<HashSet<EndpointId>>> = Arc::default();
    Router::new().route(
        "/",
        post(move |Extension(Peer(peer)): Extension<Peer>, Json(request): Json<Value>| {
            let enrolled = enrolled.clone();
            async move {
                let id = &request["id"];
                let call = &request["params"]["message"]["parts"][1]["data"];
                if call["skill"] == ENROLL_SKILL {
                    if call["args"]["token"] != token {
                        return Json(task_result(id, "rejected", "denied by rule: invalid enrollment token", None));
                    }
                    enrolled.lock().unwrap().insert(peer);
                    return Json(task_result(id, "completed", "enrolled", None));
                }
                if !enrolled.lock().unwrap().contains(&peer) {
                    return Json(task_result(id, "rejected", &format!("denied by rule: {NOT_ENROLLED}"), None));
                }
                Json(task_result(id, "completed", "secret answer", None))
            }
        }),
    )
}

#[tokio::test]
async fn a_client_with_the_enrollment_token_enrolls_itself_and_retries_without_being_asked() {
    let server = local_endpoint(vec![ALPN.to_vec()]).await;
    tokio::spawn(serve(server.clone(), Arc::new(|_| true), enrolling_app("open-sesame")));
    let newcomer = local_endpoint(vec![]).await;

    let client = Client::new(newcomer, loopback_address_of(&server)).with_enrollment_token("open-sesame");
    let outcome = tokio::time::timeout(Duration::from_secs(20), client.call_skill("anything", json!({}), "hi"))
        .await
        .expect("no timeout")
        .expect("the call succeeds after enrolling");
    assert_eq!((outcome.state.as_str(), outcome.text.as_str()), ("completed", "secret answer"));
}

#[tokio::test]
async fn without_the_right_token_a_newcomer_stays_locked_out() {
    let server = local_endpoint(vec![ALPN.to_vec()]).await;
    tokio::spawn(serve(server.clone(), Arc::new(|_| true), enrolling_app("open-sesame")));
    let address = loopback_address_of(&server);

    let no_token = Client::new(local_endpoint(vec![]).await, address.clone());
    let outcome = no_token.call_skill("anything", json!({}), "hi").await.unwrap();
    assert!(outcome.is_not_enrolled(), "got {outcome:?}");

    let wrong_token = Client::new(local_endpoint(vec![]).await, address).with_enrollment_token("guess");
    let error = wrong_token.call_skill("anything", json!({}), "hi").await.unwrap_err();
    assert!(format!("{error:#}").contains("enrollment refused"), "got {error:#}");
}
