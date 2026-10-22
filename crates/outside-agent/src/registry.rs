//! The registry loop: tell the inside agent about ourselves, and learn about the other agents it has seen.
//!
//! Every sync sends a report of what this agent knows about itself and receives the list of agents seen in
//! the last six hours. The list is shown on the dashboard, so an agent on a platform that blocks inbound HTTP
//! still appears on every other agent's page.
use std::time::Duration;

use a2a_iroh::{iroh::Endpoint, Client};
use serde_json::{json, Value};

use crate::{
    cycle, facts,
    investigation::ANSWER_TITLE,
    state::{now, AgentState, Shared},
    text::truncate_for_prompt,
};

/// The most characters of our own hosting guess to share with other agents.
const SHARED_GUESS_CHARS: usize = 1500;

/// Sync forever, once per `interval`. Failures are shown on the dashboard and retried next time.
pub async fn run(shared: &Shared, client: &Client, endpoint: &Endpoint, interval: Duration) {
    // Facts are cheap to gather, and having them before the first cycle finishes lets the first report say something useful.
    let early_facts = facts::collect();
    loop {
        let report = build_report(shared, endpoint, &early_facts).await;
        let outcome = client.call_skill("agents.sync", json!({"report": report}), "Sync agent registry").await;

        let mut state = shared.state.write().await;
        match outcome {
            Ok(task) if task.state == "completed" => {
                let data = task.data.unwrap_or(Value::Null);
                state.agents = data["agents"].as_array().cloned().unwrap_or_default();
                state.agents_window_secs = data["window_secs"].as_u64();
                state.agents_synced = Some(now());
                state.agents_error = None;
            }
            Ok(task) => state.agents_error = Some(format!("{}: {}", task.state, task.text)),
            Err(error) => state.agents_error = Some(format!("{error:#}")),
        }
        drop(state);

        tokio::time::sleep(interval).await;
    }
}

/// What this agent tells the inside agent about itself. The inside agent re-validates every field.
async fn build_report(shared: &Shared, endpoint: &Endpoint, early_facts: &Value) -> Value {
    let state = shared.state.read().await;
    let facts = state.facts.as_ref().unwrap_or(early_facts);

    let public_ip = state.public_ips.first().cloned().or_else(|| cycle::public_ips(endpoint).into_iter().next());
    let ip_details = state.ip_info.as_ref().and_then(|details| details.get(0)).cloned().unwrap_or(Value::Null);
    let hosting_guess = latest_hosting_guess(&state);

    json!({
        "platform_hint": platform_hint(facts),
        "provider_guess": provider_line(&hosting_guess),
        "hosting_guess": truncate_for_prompt(&hosting_guess, SHARED_GUESS_CHARS),
        "public_ip": public_ip,
        "ip_org": ip_details["org"], "ip_city": ip_details["city"],
        "ip_region": ip_details["region"], "ip_country": ip_details["country"],
        "hostname": facts["hostname"], "kernel": facts["kernel"], "os": facts["os"], "arch": facts["arch"],
        "cpus": facts["cpus"],
        "mem_mb": facts["mem_total_kb"].as_u64().map(|kilobytes| kilobytes / 1024),
        "started": state.started, "cycles": state.cycles, "last_cycle": state.last_cycle,
    })
}

/// The text of the most recent completed final answer, or an empty string if there is none yet.
fn latest_hosting_guess(state: &AgentState) -> String {
    state
        .findings
        .iter()
        .rev()
        .find(|finding| finding.title == ANSWER_TITLE && finding.state == "completed")
        .map(|finding| finding.answer.clone())
        .unwrap_or_default()
}

/// The value on the answer's "Provider: ..." line, without markdown decoration; empty if there is no such line.
fn provider_line(answer: &str) -> String {
    answer
        .lines()
        .find(|line| line.to_lowercase().contains("provider"))
        .and_then(|line| line.split_once(':'))
        .map(|(_, value)| value.replace(['*', '`'], "").trim().to_string())
        .unwrap_or_default()
}

/// A quick label from platform environment variables, available before the LLM has said anything.
fn platform_hint(facts: &Value) -> String {
    let env = &facts["platform_env"];
    let is_set = |name: &str| env.get(name).is_some();

    if is_set("FLY_APP_NAME") {
        format!("Fly.io {}", env["FLY_REGION"].as_str().unwrap_or("")).trim().to_string()
    } else if env.as_object().is_some_and(|vars| vars.keys().any(|name| name.starts_with("SPRITE_"))) {
        "Fly Sprite".to_string()
    } else if is_set("RAILWAY_ENVIRONMENT") || is_set("RAILWAY_PROJECT_ID") {
        "Railway".to_string()
    } else if is_set("K_SERVICE") && is_set("K_CONFIGURATION") {
        "Google Cloud Run".to_string()
    } else if facts["dmi"]["sys_vendor"] == "Vultr" {
        "Vultr".to_string()
    } else if is_set("RUNPOD_POD_ID") {
        "RunPod".to_string()
    } else if is_set("KUBERNETES_SERVICE_HOST") {
        "Kubernetes".to_string()
    } else if facts["containerenv_present"] == true || facts["dockerenv_present"] == true {
        "container".to_string()
    } else {
        "unknown".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_hints_come_from_environment_markers() {
        let hint = |facts: Value| platform_hint(&facts);
        assert_eq!(hint(json!({"platform_env": {"FLY_APP_NAME": "x", "FLY_REGION": "lax"}})), "Fly.io lax");
        assert_eq!(hint(json!({"platform_env": {"SPRITE_STORAGE_DRIVER": "x"}})), "Fly Sprite");
        assert_eq!(hint(json!({"platform_env": {}, "dmi": {"sys_vendor": "Vultr", "product_name": "VC2"}})), "Vultr");
        assert_eq!(hint(json!({"platform_env": {"K_SERVICE": "svc", "K_CONFIGURATION": "svc"}})), "Google Cloud Run");
        assert_eq!(hint(json!({"platform_env": {}, "containerenv_present": true})), "container");
        assert_eq!(hint(json!({"platform_env": {}})), "unknown");
    }

    #[test]
    fn the_provider_is_read_from_the_answers_provider_line() {
        assert_eq!(provider_line("**Provider:** Fly.io\n**Region:** lax"), "Fly.io");
        assert_eq!(provider_line("Confidence: 90%"), "");
    }
}
