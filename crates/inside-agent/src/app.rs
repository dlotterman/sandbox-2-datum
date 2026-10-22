//! The inside agent's shared state: configuration plus the things that live as long as the process.
use std::time::Duration;

use anyhow::Result;

use crate::{config::Config, enrollment::Enrollments, policy::Limiter};

/// Postgres connection settings. An empty `host` means "no database": the data skills are switched off.
pub struct PostgresConfig {
    pub host: String,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl PostgresConfig {
    pub fn is_configured(&self) -> bool {
        !self.host.is_empty()
    }
}

/// Where the rclone WebDAV file store lives. An empty `url` means "no file store".
pub struct RcloneConfig {
    pub url: String,
    pub user: String,
    pub password: String,
}

impl RcloneConfig {
    pub fn is_configured(&self) -> bool {
        !self.url.is_empty()
    }
}

/// The LLM provider (an OpenAI-compatible chat completions API). This is the only place the key exists.
pub struct LlmConfig {
    pub api_key: String,
    pub model: String,
    pub base_url: String,
}

/// Everything a skill needs. Built once at startup and shared by reference.
pub struct App {
    pub postgres: PostgresConfig,
    pub rclone: RcloneConfig,
    pub llm: LlmConfig,
    /// Who may use the skills: the fixed allowlist plus peers that enrolled with the shared token.
    pub enrollments: Enrollments,
    pub http: reqwest::Client,
    /// This agent's Iroh endpoint id, advertised in the Agent Card.
    pub endpoint_id: String,
    /// Per-peer hourly budget for the LLM and data skills.
    pub quota: Limiter,
    /// Per-peer budget for `agents.enroll` attempts, so a stranger cannot hammer the token.
    pub enroll_quota: Limiter,
    /// A separate, larger per-peer budget for registry syncs, so 30-second polling cannot use up `quota`.
    pub sync_quota: Limiter,
    /// Caps how many LLM requests run at once; the provider rate-limits (HTTP 429) beyond a few.
    pub llm_slots: tokio::sync::Semaphore,
}

/// Registry syncs allowed per peer per hour (a poll every 30 seconds is 120).
const SYNC_CALLS_PER_HOUR: u32 = 600;
/// Enrollment attempts allowed per peer per hour. A legitimate agent needs one every few hours.
const ENROLL_ATTEMPTS_PER_HOUR: u32 = 30;
/// How long to wait for the LLM provider or the IP lookup service before giving up.
const HTTP_TIMEOUT: Duration = Duration::from_secs(90);

impl App {
    pub fn new(config: Config, endpoint_id: String) -> Result<Self> {
        Ok(Self {
            postgres: config.postgres,
            rclone: config.rclone,
            llm: config.llm,
            enrollments: Enrollments::new(config.enrollment_token, config.enrollment_lifetime, config.allowed_peers),
            http: reqwest::Client::builder().timeout(HTTP_TIMEOUT).build()?,
            endpoint_id,
            quota: Limiter::new(config.llm_calls_per_hour),
            enroll_quota: Limiter::new(ENROLL_ATTEMPTS_PER_HOUR),
            sync_quota: Limiter::new(SYNC_CALLS_PER_HOUR),
            llm_slots: tokio::sync::Semaphore::new(config.llm_concurrency),
        })
    }
}
