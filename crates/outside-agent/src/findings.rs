//! Calling the inside agent and keeping a visible record of each exchange.
use std::time::Instant;

use a2a_iroh::{Client, TaskOutcome};
use anyhow::Result;
use serde_json::Value;

use crate::state::{now, Finding, Shared, MAX_FINDINGS};

/// Call `skill` on the inside agent, time it, and add the exchange to the findings shown on the dashboard.
///
/// `title` is the question in plain words for the dashboard; `note` is the human-readable text that
/// travels with the A2A message. The outcome is returned so the caller can use the answer.
pub async fn ask_inside(
    shared: &Shared,
    client: &Client,
    title: &str,
    skill: &str,
    args: Value,
    note: &str,
) -> Result<TaskOutcome> {
    let started = Instant::now();
    let outcome = client.call_skill(skill, args, note).await;
    record(shared, title, skill, started, &outcome).await;
    outcome
}

async fn record(shared: &Shared, title: &str, skill: &str, started: Instant, outcome: &Result<TaskOutcome>) {
    let (state, answer, detail) = match outcome {
        Ok(task) => (task.state.clone(), task.text.clone(), task.data.clone().unwrap_or(Value::Null)),
        Err(error) => ("error".to_string(), format!("{error:#}"), Value::Null),
    };
    let finding = Finding {
        id: shared.next_finding_id(),
        at: now(),
        title: title.to_string(),
        skill: skill.to_string(),
        state,
        answer,
        detail,
        ms: started.elapsed().as_millis() as u64,
    };

    let mut state = shared.state.write().await;
    state.findings.push(finding);
    let excess = state.findings.len().saturating_sub(MAX_FINDINGS);
    state.findings.drain(..excess);
}
