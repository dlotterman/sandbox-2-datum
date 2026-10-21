//! The dashboard's web server: pages, a small JSON API, and an optional shared-secret gate.
use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{header, HeaderMap, Request, StatusCode},
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde_json::{json, Value};

use crate::state::{now, Shared};

/// The name of the cookie that remembers the token after `?t=` has been used once.
const TOKEN_COOKIE: &str = "dt";

pub fn router(shared: Arc<Shared>, token: Option<String>) -> Router {
    Router::new()
        // The pages and the files they load. They are compiled into the binary from `assets/`.
        .route("/", get(|| async { Html(include_str!("../assets/dashboard.html")) }))
        .route("/about", get(|| async { Html(include_str!("../assets/about.html")) }))
        .route("/style.css", get(|| async { asset(CSS, include_str!("../assets/style.css")) }))
        .route("/common.js", get(|| async { asset(JAVASCRIPT, include_str!("../assets/common.js")) }))
        .route("/dashboard.js", get(|| async { asset(JAVASCRIPT, include_str!("../assets/dashboard.js")) }))
        .route("/about.js", get(|| async { asset(JAVASCRIPT, include_str!("../assets/about.js")) }))
        .route("/healthz", get(|| async { "ok" }))
        // The JSON API the dashboard reads.
        .route("/api/state", get(api_state))
        .route("/api/facts", get(api_facts))
        .route("/api/findings", get(api_findings))
        .route("/api/agents", get(api_agents))
        .route("/api/refresh", get(api_refresh))
        .with_state(shared)
        .layer(middleware::from_fn_with_state(token.map(Arc::new), require_token))
}

const CSS: &str = "text/css; charset=utf-8";
const JAVASCRIPT: &str = "text/javascript; charset=utf-8";

/// A static file with its content type.
fn asset(content_type: &'static str, body: &'static str) -> impl IntoResponse {
    ([(header::CONTENT_TYPE, content_type)], body)
}

// ---- the JSON API

/// Everything the dashboard page needs, in one response.
async fn api_state(State(shared): State<Arc<Shared>>) -> Json<Value> {
    let state = shared.state.read().await;
    Json(json!({
        "started": state.started, "now": now(),
        "endpoint_id": state.endpoint_id, "inside_id": state.inside_id,
        "connected": state.connected, "connect_ms": state.connect_ms,
        "inside_card": state.inside_card, "facts": state.facts,
        "public_ips": state.public_ips, "ip_info": state.ip_info,
        "findings": state.findings,
        "cycles": state.cycles, "last_cycle": state.last_cycle, "last_error": state.last_error,
        "agents": state.agents, "agents_synced": state.agents_synced,
        "agents_error": state.agents_error, "agents_window_secs": state.agents_window_secs,
    }))
}

async fn api_facts(State(shared): State<Arc<Shared>>) -> Json<Value> {
    Json(json!(shared.state.read().await.facts))
}

async fn api_findings(State(shared): State<Arc<Shared>>) -> Json<Value> {
    Json(json!(shared.state.read().await.findings))
}

async fn api_agents(State(shared): State<Arc<Shared>>) -> Json<Value> {
    let state = shared.state.read().await;
    Json(json!({
        "synced": state.agents_synced,
        "error": state.agents_error,
        "window_secs": state.agents_window_secs,
        "agents": state.agents,
    }))
}

/// Wake the cycle loop so it runs again now instead of waiting for its interval.
async fn api_refresh(State(shared): State<Arc<Shared>>) -> &'static str {
    shared.refresh.notify_one();
    "refresh queued"
}

// ---- the token gate

/// An optional shared-secret gate for public deployments. With a token configured, a request must present it as
/// `?t=<token>` (which also sets a cookie so later requests need nothing), as that cookie, or as a bearer header.
/// `/healthz` stays open so platform health checks work. With no token configured, everything is open.
async fn require_token(State(token): State<Option<Arc<String>>>, request: Request<Body>, next: Next) -> Response {
    let Some(token) = token else {
        return next.run(request).await;
    };
    if request.uri().path() == "/healthz" {
        return next.run(request).await;
    }

    let query_token = token_in_query(request.uri().query(), &token);
    let presented =
        query_token || token_in_cookie(request.headers(), &token) || token_in_bearer(request.headers(), &token);
    if !presented {
        return (StatusCode::UNAUTHORIZED, "dashboard token required: open /?t=<token>").into_response();
    }

    let over_https = request.headers().get("x-forwarded-proto").and_then(|value| value.to_str().ok()) == Some("https");
    let mut response = next.run(request).await;
    if query_token {
        let secure = if over_https { "; Secure" } else { "" };
        let cookie = format!("{TOKEN_COOKIE}={token}; Path=/; HttpOnly; SameSite=Lax{secure}");
        response.headers_mut().append(header::SET_COOKIE, cookie.parse().expect("a valid cookie header"));
    }
    response
}

fn token_in_query(query: Option<&str>, token: &str) -> bool {
    query.is_some_and(|query| query.split('&').any(|pair| pair.strip_prefix("t=") == Some(token)))
}

fn token_in_cookie(headers: &HeaderMap, token: &str) -> bool {
    let cookies = headers.get(header::COOKIE).and_then(|value| value.to_str().ok());
    let prefix = format!("{TOKEN_COOKIE}=");
    cookies.is_some_and(|cookies| {
        cookies.split(';').any(|cookie| cookie.trim().strip_prefix(prefix.as_str()) == Some(token))
    })
}

fn token_in_bearer(headers: &HeaderMap, token: &str) -> bool {
    let authorization = headers.get(header::AUTHORIZATION).and_then(|value| value.to_str().ok());
    authorization.is_some_and(|value| value.strip_prefix("Bearer ") == Some(token))
}

#[cfg(test)]
mod tests {
    use axum::http::header::{AUTHORIZATION, COOKIE, SET_COOKIE};
    use tower::ServiceExt;

    use super::*;

    fn guarded(token: Option<&str>) -> Router {
        Router::new()
            .route("/", get(|| async { "dashboard" }))
            .route("/healthz", get(|| async { "ok" }))
            .layer(middleware::from_fn_with_state(token.map(|t| Arc::new(t.to_string())), require_token))
    }

    /// The response status, and whether it set a cookie.
    async fn fetch(app: &Router, uri: &str, header: Option<(&str, &str)>) -> (StatusCode, bool) {
        let mut request = Request::builder().uri(uri);
        if let Some((name, value)) = header {
            request = request.header(name, value);
        }
        let response = app.clone().oneshot(request.body(Body::empty()).unwrap()).await.unwrap();
        (response.status(), response.headers().contains_key(SET_COOKIE))
    }

    #[tokio::test]
    async fn a_public_dashboard_needs_the_token_but_health_checks_do_not() {
        let app = guarded(Some("s3cret-token"));
        assert_eq!(fetch(&app, "/", None).await.0, StatusCode::UNAUTHORIZED);
        assert_eq!(fetch(&app, "/?t=wrong", None).await.0, StatusCode::UNAUTHORIZED);
        assert_eq!(fetch(&app, "/", Some((COOKIE.as_str(), "dt=wrong"))).await.0, StatusCode::UNAUTHORIZED);
        assert_eq!(fetch(&app, "/", Some((AUTHORIZATION.as_str(), "Bearer wrong"))).await.0, StatusCode::UNAUTHORIZED);
        assert_eq!(fetch(&app, "/healthz", None).await.0, StatusCode::OK, "platform health checks stay open");

        assert_eq!(fetch(&app, "/?t=s3cret-token", None).await, (StatusCode::OK, true), "query token sets a cookie");
        assert_eq!(fetch(&app, "/", Some((COOKIE.as_str(), "other=1; dt=s3cret-token"))).await.0, StatusCode::OK);
        assert_eq!(fetch(&app, "/", Some((AUTHORIZATION.as_str(), "Bearer s3cret-token"))).await.0, StatusCode::OK);
    }

    #[tokio::test]
    async fn pages_and_their_scripts_and_styles_are_all_served_with_the_right_content_type() {
        use crate::state::AgentState;

        let app = router(Arc::new(Shared::new(AgentState::default())), None);
        for (path, content_type) in [
            ("/", "text/html"),
            ("/about", "text/html"),
            ("/style.css", "text/css"),
            ("/common.js", "text/javascript"),
            ("/dashboard.js", "text/javascript"),
            ("/about.js", "text/javascript"),
            ("/api/state", "application/json"),
        ] {
            let response =
                app.clone().oneshot(Request::builder().uri(path).body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK, "{path}");
            let actual = response.headers()[header::CONTENT_TYPE].to_str().unwrap().to_string();
            assert!(actual.starts_with(content_type), "{path} was served as {actual}");
        }
    }

    #[tokio::test]
    async fn without_a_configured_token_the_dashboard_is_open() {
        assert_eq!(fetch(&guarded(None), "/", None).await.0, StatusCode::OK);
    }
}
