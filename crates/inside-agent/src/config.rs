//! Configuration, read from environment variables in one place.
//!
//! | Variable | Meaning |
//! | --- | --- |
//! | `ALLOWED_PEERS` | comma-separated endpoint ids of the outside agents allowed to connect |
//! | `INSIDE_SECRET_KEY_HEX` / `INSIDE_KEY_FILE` | the agent's identity (read by `a2a_iroh::load_secret_key`) |
//! | `BASETEN_API_KEY`, `BASETEN_MODEL`, `BASETEN_BASE_URL` | the LLM provider |
//! | `POSTGRES_HOST`, `POSTGRES_USER`, `POSTGRES_PASSWORD`, `POSTGRES_DB` | the database |
//! | `RCLONE_URL`, or `RCLONE_HOST` (an IPv6 literal), `RC_USER`, `RC_PASS` | the file store |
//! | `LLM_CALLS_PER_HOUR`, `LLM_CONCURRENCY` | per-peer quota and simultaneous LLM requests |
//! | `RELAYS` | override Datum's relays (read by `a2a_iroh`) |
use std::{collections::HashSet, env, str::FromStr};

use a2a_iroh::iroh::EndpointId;
use anyhow::{anyhow, Result};

use crate::app::{LlmConfig, PostgresConfig, RcloneConfig};

pub struct Config {
    pub allowed_peers: HashSet<EndpointId>,
    pub postgres: PostgresConfig,
    pub rclone: RcloneConfig,
    pub llm: LlmConfig,
    pub llm_calls_per_hour: u32,
    pub llm_concurrency: usize,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            allowed_peers: parse_allowed_peers(&text("ALLOWED_PEERS"))?,
            postgres: PostgresConfig {
                host: text("POSTGRES_HOST"),
                user: text("POSTGRES_USER"),
                password: text("POSTGRES_PASSWORD"),
                database: text("POSTGRES_DB"),
            },
            rclone: RcloneConfig { url: rclone_url(), user: text("RC_USER"), password: text("RC_PASS") },
            llm: LlmConfig {
                api_key: text("BASETEN_API_KEY"),
                model: text_or("BASETEN_MODEL", "zai-org/GLM-5.3-Flash"),
                base_url: text_or("BASETEN_BASE_URL", "https://inference.baseten.co/v1"),
            },
            llm_calls_per_hour: number_or("LLM_CALLS_PER_HOUR", 120),
            llm_concurrency: number_or("LLM_CONCURRENCY", 2),
        })
    }
}

fn parse_allowed_peers(list: &str) -> Result<HashSet<EndpointId>> {
    list.split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(|entry| entry.parse().map_err(|e| anyhow!("bad ALLOWED_PEERS entry {entry}: {e}")))
        .collect()
}

/// `RCLONE_URL` if set; otherwise build one from `RCLONE_HOST`, which holds a bare IPv6 address (Datum has
/// no private DNS, so the address comes from a ConfigMap) and needs brackets in a URL.
fn rclone_url() -> String {
    let url = text("RCLONE_URL");
    let host = text("RCLONE_HOST");
    if !url.is_empty() {
        url
    } else if !host.is_empty() {
        format!("http://[{host}]:8080")
    } else {
        String::new()
    }
}

fn text(name: &str) -> String {
    env::var(name).unwrap_or_default()
}

fn text_or(name: &str, default: &str) -> String {
    env::var(name).unwrap_or_else(|_| default.to_string())
}

fn number_or<T: FromStr>(name: &str, default: T) -> T {
    env::var(name).ok().and_then(|value| value.parse().ok()).unwrap_or(default)
}
