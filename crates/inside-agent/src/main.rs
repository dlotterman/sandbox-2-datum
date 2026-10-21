//! The inside agent: the only way for outside agents to reach an LLM, the internet and the data stores.
//!
//! It receives A2A `message/send` requests over Iroh from allowlisted peers, runs each through the policy
//! gate (allowlist, rules, LLM review), and only then does the work. It has no HTTP listener of its own.
//!
//! - [`config`]: environment variables. [`app`]: the shared state built from them.
//! - [`rpc`]: the Agent Card and request handling. [`policy`]: the gate. [`skill`]: result and error types.
//! - [`skills`], [`llm`]: the LLM and IP-lookup skills and the one LLM client. [`data`]: Postgres, files, registry.
mod app;
mod config;
mod data;
mod llm;
mod policy;
mod rpc;
mod skill;
mod skills;
#[cfg(test)]
mod testutil;

use std::{env, sync::Arc};

use a2a_iroh::EndpointTicket;
use anyhow::Result;

use crate::{app::App, config::Config};

#[tokio::main]
async fn main() -> Result<()> {
    if env::args().nth(1).as_deref() == Some("keygen") {
        a2a_iroh::print_new_identity();
        return Ok(());
    }

    let config = Config::from_env()?;
    eprintln!("allowlist: {} peer(s)", config.allowed_peers.len());
    let allowed_peers = config.allowed_peers.clone();

    let secret_key = a2a_iroh::load_secret_key("INSIDE_SECRET_KEY_HEX", "INSIDE_KEY_FILE")?;
    let endpoint = a2a_iroh::bind(secret_key, vec![a2a_iroh::ALPN.to_vec()]).await?;
    endpoint.online().await;
    // Outside agents need the endpoint id (or the ticket, which also carries a relay hint).
    println!("ENDPOINT_ID={}", endpoint.id());
    println!("TICKET={}", EndpointTicket::new(endpoint.addr()));
    eprintln!("addr: {:?}", endpoint.addr());

    let app = Arc::new(App::new(config, endpoint.id().to_string())?);
    if app.llm.api_key.is_empty() {
        eprintln!("WARNING: BASETEN_API_KEY not set; llm.complete will fail");
    }

    // The database often finishes starting after this agent does, so set it up in the background.
    let seeding = app.clone();
    tokio::spawn(async move { data::init(&seeding).await });

    let allow = Arc::new(move |peer: &a2a_iroh::iroh::EndpointId| allowed_peers.contains(peer));
    a2a_iroh::serve(endpoint, allow, rpc::router(app)).await;
    Ok(())
}
