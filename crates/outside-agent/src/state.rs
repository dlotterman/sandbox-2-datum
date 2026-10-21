//! What the outside agent knows, shared between its background loops and the web server.
use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::Serialize;
use serde_json::Value;
use tokio::sync::{Notify, RwLock};

/// Everything shown on the dashboard. Updated by the cycle and registry loops, read by the web handlers.
#[derive(Default)]
pub struct AgentState {
    /// When this process started (Unix seconds), and its own and the inside agent's endpoint ids.
    pub started: u64,
    pub endpoint_id: String,
    pub inside_id: String,

    // --- the link to the inside agent
    pub connected: bool,
    pub connect_ms: Option<u128>,
    pub inside_card: Option<Value>,

    // --- what this agent has learned about itself
    pub facts: Option<Value>,
    pub public_ips: Vec<String>,
    pub ip_info: Option<Value>,
    /// The exchanges with the inside agent, oldest first, capped at [`MAX_FINDINGS`].
    pub findings: Vec<Finding>,
    pub cycles: u64,
    pub last_cycle: Option<u64>,
    pub last_error: Option<String>,

    // --- the registry of other agents, refreshed by the sync loop
    pub agents: Vec<Value>,
    pub agents_synced: Option<u64>,
    pub agents_error: Option<String>,
    pub agents_window_secs: Option<u64>,
}

/// How many findings to keep; older ones are dropped.
pub const MAX_FINDINGS: usize = 100;

/// One exchange with the inside agent, as shown on the dashboard.
#[derive(Clone, Serialize)]
pub struct Finding {
    pub id: u64,
    /// Unix seconds.
    pub at: u64,
    /// The question in plain words, for people.
    pub title: String,
    /// The skill that was called, e.g. `llm.complete`.
    pub skill: String,
    /// The task state (`completed`, `rejected`, `failed`) or `error` if the call itself failed.
    pub state: String,
    pub answer: String,
    /// Structured data the skill returned, if any.
    pub detail: Value,
    pub ms: u64,
}

/// The state plus the signal used to wake the cycle loop early ("refresh now").
pub struct Shared {
    pub state: RwLock<AgentState>,
    pub refresh: Notify,
    next_finding_id: AtomicU64,
}

impl Shared {
    pub fn new(state: AgentState) -> Self {
        Self { state: RwLock::new(state), refresh: Notify::new(), next_finding_id: AtomicU64::new(1) }
    }

    pub fn next_finding_id(&self) -> u64 {
        self.next_finding_id.fetch_add(1, Ordering::Relaxed)
    }
}

/// The current time as Unix seconds.
pub fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|since_epoch| since_epoch.as_secs()).unwrap_or(0)
}
