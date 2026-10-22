//! The outside agent: long-lived, and with exactly one peer, the inside agent (A2A over Iroh).
//!
//! It looks at its own runtime, asks the inside agent to look up its public IP and to answer LLM questions
//! about where it is hosted, and serves what it learned on a small dashboard. It holds no LLM credentials and
//! makes no other network calls.
//!
//! - [`cycle`] and [`investigation`]: the periodic introspection and the hosting-guess story.
//! - [`registry`]: the 30-second sync with the inside agent's list of recently seen agents.
//! - [`web`]: the dashboard and its API. [`state`] and [`findings`]: what is shown. [`facts`]: local introspection.
mod config;
mod cycle;
mod facts;
mod findings;
mod investigation;
mod registry;
mod state;
mod text;
mod web;

use std::{env, sync::Arc};

use a2a_iroh::Client;
use anyhow::Result;

use crate::{
    config::Config,
    state::{now, AgentState, Shared},
};

#[tokio::main]
async fn main() -> Result<()> {
    if env::args().nth(1).as_deref() == Some("keygen") {
        a2a_iroh::print_new_identity();
        return Ok(());
    }

    let config = Config::from_env()?;
    let secret_key = a2a_iroh::load_secret_key("OUTSIDE_SECRET_KEY_HEX", "OUTSIDE_KEY_FILE")?;
    let endpoint = a2a_iroh::bind(secret_key, vec![]).await?;
    eprintln!("outside endpoint id: {}", endpoint.id());
    eprintln!("dialing inside agent: {}", config.inside.id);

    let shared = Arc::new(Shared::new(AgentState {
        started: now(),
        endpoint_id: endpoint.id().to_string(),
        inside_id: config.inside.id.to_string(),
        ..Default::default()
    }));
    let mut client = Client::new(endpoint.clone(), config.inside.clone());
    if let Some(token) = &config.enrollment_token {
        client = client.with_enrollment_token(token);
    }
    let client = Arc::new(client);

    spawn_cycle_loop(&shared, &client, &endpoint, config.cycle_interval);
    spawn_registry_loop(&shared, &client, &endpoint, config.sync_interval);

    match config.dashboard_bind {
        Some(bind) => serve_dashboard(shared, &bind, config.dashboard_token).await,
        // No dashboard: the background loops are all there is, so just keep running.
        None => std::future::pending().await,
    }
}

/// Run an introspection cycle now and then every `interval`, or sooner if "refresh now" is requested.
fn spawn_cycle_loop(
    shared: &Arc<Shared>,
    client: &Arc<Client>,
    endpoint: &a2a_iroh::iroh::Endpoint,
    interval: std::time::Duration,
) {
    let (shared, client, endpoint) = (shared.clone(), client.clone(), endpoint.clone());
    tokio::spawn(async move {
        loop {
            if let Err(error) = cycle::run(&shared, &client, &endpoint).await {
                eprintln!("cycle failed: {error:#}");
                shared.state.write().await.last_error = Some(format!("{error:#}"));
            }
            tokio::select! {
                _ = tokio::time::sleep(interval) => {}
                _ = shared.refresh.notified() => {}
            }
        }
    });
}

fn spawn_registry_loop(
    shared: &Arc<Shared>,
    client: &Arc<Client>,
    endpoint: &a2a_iroh::iroh::Endpoint,
    interval: std::time::Duration,
) {
    let (shared, client, endpoint) = (shared.clone(), client.clone(), endpoint.clone());
    tokio::spawn(async move { registry::run(&shared, &client, &endpoint, interval).await });
}

async fn serve_dashboard(shared: Arc<Shared>, bind: &str, token: Option<String>) -> Result<()> {
    let listener = tokio::net::TcpListener::bind(bind).await?;
    eprintln!("dashboard on http://{bind}");
    axum::serve(listener, web::router(shared, token)).await?;
    Ok(())
}
