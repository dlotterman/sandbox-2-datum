//! Configuration, read from environment variables in one place.
//!
//! | Variable | Meaning |
//! | --- | --- |
//! | `INSIDE_ENDPOINT_ID` or `INSIDE_TICKET` | who to dial: the inside agent's endpoint id (preferred) or a ticket |
//! | `OUTSIDE_SECRET_KEY_HEX` / `OUTSIDE_KEY_FILE` | this agent's identity (read by `a2a_iroh::load_secret_key`); with neither set, a fresh identity every start |
//! | `ENROLLMENT_TOKEN` | the inside agent's enrollment token: lets this agent enroll itself, so its key need not be pre-registered |
//! | `REFRESH_SECS` | seconds between introspection cycles (default 600) |
//! | `SYNC_SECS` | seconds between registry syncs (default 30) |
//! | `PORT` or `HTTP_BIND` | where the dashboard listens (default `0.0.0.0:8080`); `HTTP_BIND=off` disables it |
//! | `DASHBOARD_TOKEN` | if set, the dashboard requires it |
//! | `RELAYS` | override Datum's relays (read by `a2a_iroh`) |
use std::{env, time::Duration};

use a2a_iroh::iroh::EndpointAddr;
use anyhow::Result;

pub struct Config {
    pub inside: EndpointAddr,
    pub cycle_interval: Duration,
    pub sync_interval: Duration,
    /// `None` when the dashboard is switched off.
    pub dashboard_bind: Option<String>,
    pub dashboard_token: Option<String>,
    /// If set, the client enrolls with the inside agent by itself whenever it is told it is not enrolled.
    pub enrollment_token: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let ticket = env::var("INSIDE_TICKET").ok();
        let endpoint_id = env::var("INSIDE_ENDPOINT_ID").ok();
        Ok(Self {
            inside: a2a_iroh::target_from(ticket.as_deref(), endpoint_id.as_deref())?,
            cycle_interval: seconds_or("REFRESH_SECS", 600),
            sync_interval: seconds_or("SYNC_SECS", 30),
            dashboard_bind: dashboard_bind(),
            dashboard_token: dashboard_token(),
            enrollment_token: non_empty_env("ENROLLMENT_TOKEN"),
        })
    }
}

fn seconds_or(name: &str, default: u64) -> Duration {
    Duration::from_secs(env::var(name).ok().and_then(|value| value.parse().ok()).unwrap_or(default))
}

/// `HTTP_BIND` wins if set. Otherwise listen on every interface at `PORT`, which platforms like Railway
/// and Cloud Run inject, or 8080.
fn dashboard_bind() -> Option<String> {
    let bind = env::var("HTTP_BIND").unwrap_or_else(|_| {
        let port = env::var("PORT").unwrap_or_else(|_| "8080".into());
        format!("0.0.0.0:{port}")
    });
    (bind != "off").then_some(bind)
}

/// The token, trimmed: secret stores sometimes keep a trailing newline, which would otherwise never match.
fn dashboard_token() -> Option<String> {
    non_empty_env("DASHBOARD_TOKEN")
}

/// The variable's value with whitespace trimmed, or `None` if it is unset or empty.
fn non_empty_env(name: &str) -> Option<String> {
    env::var(name).ok().map(|value| value.trim().to_string()).filter(|value| !value.is_empty())
}
