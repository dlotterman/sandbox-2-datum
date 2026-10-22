//! Test helpers: a stub `App` with no Postgres or rclone, and a mock OpenAI-compatible server whose replies
//! are scripted, so the policy gate and the LLM paths can be exercised without a real model.
use std::{collections::VecDeque, sync::Arc, sync::Mutex};

use axum::{extract::State, http::StatusCode, routing::post, Router};
use serde_json::json;

use crate::{
    app::{App, LlmConfig, PostgresConfig, RcloneConfig},
    enrollment::Enrollments,
    policy::Limiter,
};

/// The scripted replies still to be served, as `(HTTP status, body)`.
pub type Script = Arc<Mutex<VecDeque<(u16, String)>>>;

/// An `App` whose LLM calls go to `llm_url` with `api_key`, and which has no database or file store.
pub fn app(llm_url: &str, api_key: &str) -> App {
    App {
        postgres: PostgresConfig {
            host: String::new(),
            user: String::new(),
            password: String::new(),
            database: String::new(),
        },
        rclone: RcloneConfig { url: String::new(), user: String::new(), password: String::new() },
        llm: LlmConfig { api_key: api_key.into(), model: "test-model".into(), base_url: llm_url.into() },
        enrollments: Enrollments::new(None, std::time::Duration::from_secs(3600), std::collections::HashSet::new()),
        http: reqwest::Client::new(),
        endpoint_id: "test".into(),
        quota: Limiter::new(1000),
        enroll_quota: Limiter::new(1000),
        sync_quota: Limiter::new(1000),
        llm_slots: tokio::sync::Semaphore::new(2),
    }
}

/// The body of a successful chat completion whose reply is `text`.
pub fn completion(text: &str) -> String {
    json!({"choices": [{"message": {"content": text}}]}).to_string()
}

/// Start a server for `/chat/completions` that answers each request with the next scripted `(status, body)`.
/// An empty script answers 500. Every reply carries `Retry-After: 0` so retry tests stay fast.
/// Returns the server's base URL and the script, which tests can inspect to see what was used.
pub async fn mock_llm(script: Vec<(u16, String)>) -> (String, Script) {
    let script: Script = Arc::new(Mutex::new(script.into()));
    let router = Router::new()
        .route(
            "/chat/completions",
            post(|State(script): State<Script>| async move {
                let (status, body) = script.lock().unwrap().pop_front().unwrap_or((500, "{}".into()));
                (
                    StatusCode::from_u16(status).unwrap(),
                    [("content-type", "application/json"), ("retry-after", "0")],
                    body,
                )
            }),
        )
        .with_state(script.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
    (url, script)
}
