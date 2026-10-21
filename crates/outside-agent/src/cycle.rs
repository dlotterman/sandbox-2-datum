//! One introspection cycle: connect, look at ourselves, learn our public IP, then investigate.
use std::time::{Duration, Instant};

use a2a_iroh::{iroh::Endpoint, Client};
use anyhow::Result;
use serde_json::{json, Value};

use crate::{
    facts,
    findings::ask_inside,
    investigation::{self, Evidence},
    state::{now, Shared},
    text::truncate_for_prompt,
};

/// Discovery of our public address can lag behind the endpoint coming online (Cloud Run was the slow one),
/// so wait up to this many times for it before giving up on the IP lookup.
const IP_WAIT_ATTEMPTS: u32 = 8;
const IP_WAIT_STEP: Duration = Duration::from_secs(2);
/// An agent normally has one or two public addresses (v4 and v6); look up at most this many.
const MAX_IPS_LOOKED_UP: usize = 2;

/// How much of the facts and IP details go into prompts. The dashboard shows everything.
const FACTS_PROMPT_BYTES: usize = 3500;
const IP_INFO_PROMPT_BYTES: usize = 800;

/// Run a full cycle. An error here means the inside agent could not be reached at all; the caller records it.
pub async fn run(shared: &Shared, client: &Client, endpoint: &Endpoint) -> Result<()> {
    record_connection(shared, client).await?;

    let facts = facts::collect();
    let public_ips = wait_for_public_ips(endpoint).await;
    {
        let mut state = shared.state.write().await;
        state.facts = Some(facts.clone());
        state.public_ips = public_ips.clone();
    }

    let ip_info = look_up_public_ips(shared, client, &public_ips).await;
    shared.state.write().await.ip_info = (!ip_info.is_empty()).then(|| Value::Array(ip_info.clone()));

    // The dashboard shows everything; the model only gets the signals that identify a platform.
    let evidence = Evidence {
        facts: truncate_for_prompt(&facts::identifying_signals(&facts).to_string(), FACTS_PROMPT_BYTES),
        ip_info: truncate_for_prompt(&Value::Array(ip_info).to_string(), IP_INFO_PROMPT_BYTES),
    };
    investigation::run(shared, client, &evidence).await;

    let mut state = shared.state.write().await;
    state.cycles += 1;
    state.last_cycle = Some(now());
    Ok(())
}

/// Fetch the inside agent's Agent Card. Doing so proves the link works, and how long it took is shown on the dashboard.
async fn record_connection(shared: &Shared, client: &Client) -> Result<()> {
    let started = Instant::now();
    let card = client.agent_card().await?;
    let mut state = shared.state.write().await;
    state.connected = true;
    state.connect_ms = Some(started.elapsed().as_millis());
    state.inside_card = Some(card);
    state.last_error = None;
    Ok(())
}

/// Our public IP addresses, as Iroh learned them from the relay (no outside "what is my IP" service is used).
pub fn public_ips(endpoint: &Endpoint) -> Vec<String> {
    endpoint
        .addr()
        .ip_addrs()
        .map(|socket| socket.ip())
        .filter(a2a_iroh::is_global_ip)
        .map(|ip| ip.to_string())
        .collect()
}

async fn wait_for_public_ips(endpoint: &Endpoint) -> Vec<String> {
    let mut ips = public_ips(endpoint);
    for _ in 0..IP_WAIT_ATTEMPTS {
        if !ips.is_empty() {
            break;
        }
        tokio::time::sleep(IP_WAIT_STEP).await;
        ips = public_ips(endpoint);
    }
    ips
}

/// Ask the inside agent to look each address up; returns the details of the lookups that worked.
async fn look_up_public_ips(shared: &Shared, client: &Client, ips: &[String]) -> Vec<Value> {
    let mut details = Vec::new();
    for ip in ips.iter().take(MAX_IPS_LOOKED_UP) {
        let title = format!("Where does {ip} sit on the internet?");
        let outcome = ask_inside(shared, client, &title, "ip.lookup", json!({"ip": ip}), "Look up my public IP").await;
        if let Some(data) = outcome.ok().and_then(|task| task.data) {
            details.push(data);
        }
    }
    details
}
