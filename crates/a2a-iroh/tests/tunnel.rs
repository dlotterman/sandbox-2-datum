//! End-to-end checks of the tunnel itself: real Iroh endpoints on loopback, no relays and no network.
use std::{net::SocketAddr, sync::Arc, time::Duration};

use a2a_iroh::{
    iroh::{Endpoint, EndpointAddr, RelayMode, SecretKey},
    serve, task_result, Client, Peer, ALPN,
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
