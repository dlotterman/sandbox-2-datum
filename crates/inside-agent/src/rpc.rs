//! The A2A surface of the inside agent: the Agent Card, and `message/send` turned into skill calls.
//!
//! A request moves through three steps: [`SkillCall::from_request`] reads which skill is wanted,
//! [`run_skill`] applies the quota and runs it, and [`reply_and_audit`] turns the outcome into an A2A
//! Task (`completed`, `rejected` or `failed`) and records the decision.
use std::sync::Arc;

use a2a_iroh::{iroh::EndpointId, rpc_error, task_result, Peer};
use axum::{
    extract::{DefaultBodyLimit, State},
    routing::{get, post},
    Extension, Json, Router,
};
use serde_json::{json, Value};

use crate::{
    app::App,
    data,
    skill::{SkillError, SkillOutput, SkillResult},
    skills,
};

/// JSON-RPC's "method not found" error code.
const METHOD_NOT_FOUND: i64 = -32601;

pub fn router(app: Arc<App>) -> Router {
    Router::new()
        .route("/.well-known/agent-card.json", get(agent_card))
        .route("/.well-known/agent.json", get(agent_card))
        .route("/", post(message_send))
        .layer(DefaultBodyLimit::max(a2a_iroh::MAX_REQUEST_BYTES))
        .with_state(app)
}

// ---- the Agent Card

/// What an outside agent can ask for. This list is only advertised; the dispatch in [`run_skill`] is what is enforced.
struct SkillInfo {
    id: &'static str,
    name: &'static str,
    description: &'static str,
    tags: &'static [&'static str],
}

const SKILLS: [SkillInfo; 6] = [
    SkillInfo {
        id: "llm.complete",
        name: "LLM completion",
        description: "Chat completion on a fixed model. args: {messages:[{role,content}], purpose, max_tokens?}",
        tags: &["llm"],
    },
    SkillInfo {
        id: "ip.lookup",
        name: "IP lookup",
        description: "Geo/ASN/rDNS for one public IP. args: {ip}",
        tags: &["network"],
    },
    SkillInfo {
        id: "data.catalog",
        name: "Data catalog",
        description: "Lists the datasets (Postgres) and files (rclone) this gateway can serve. args: {}",
        tags: &["data"],
    },
    SkillInfo {
        id: "data.query",
        name: "Query a dataset",
        description: "Read rows from an allowlisted dataset. args: {dataset, columns?, filters?, limit?, purpose}",
        tags: &["data"],
    },
    SkillInfo {
        id: "agents.sync",
        name: "Agent registry",
        description:
            "Optionally report your own metadata and get every agent seen in the last 6 hours. args: {report?}",
        tags: &["registry"],
    },
    SkillInfo {
        id: "files.get",
        name: "Fetch a file",
        description: "Fetch one small text file from the file store. args: {name, purpose}",
        tags: &["data", "files"],
    },
];

async fn agent_card(State(app): State<Arc<App>>) -> Json<Value> {
    let skills: Vec<Value> = SKILLS
        .iter()
        .map(|s| json!({"id": s.id, "name": s.name, "description": s.description, "tags": s.tags}))
        .collect();
    Json(json!({
        "protocolVersion": "0.3.0",
        "name": "Datum Inside Agent",
        "description": "Brokers LLM access and internet lookups for enrolled outside agents, after a policy check.",
        "url": format!("iroh://{}", app.endpoint_id),
        "preferredTransport": "JSONRPC",
        "version": "0.1.0",
        "capabilities": {"streaming": false, "pushNotifications": false},
        "defaultInputModes": ["application/json", "text/plain"],
        "defaultOutputModes": ["application/json", "text/plain"],
        "skills": skills,
    }))
}

// ---- message/send

/// The skill an outside agent asked for, read from the data part of its message.
struct SkillCall {
    skill: String,
    args: Value,
}

impl SkillCall {
    /// The first data part of `params.message`, which should look like `{"skill": "...", "args": {...}}`.
    fn from_request(request: &Value) -> Option<Self> {
        let parts = request["params"]["message"]["parts"].as_array()?;
        let data = &parts.iter().find(|part| part["kind"] == "data")?["data"];
        Some(Self { skill: data["skill"].as_str().unwrap_or("").to_string(), args: data["args"].clone() })
    }
}

async fn message_send(
    State(app): State<Arc<App>>,
    Extension(Peer(peer)): Extension<Peer>,
    Json(request): Json<Value>,
) -> Json<Value> {
    let request_id = request.get("id").cloned().unwrap_or(Value::Null);
    if request["method"] != "message/send" {
        return Json(rpc_error(&request_id, METHOD_NOT_FOUND, "only message/send is supported"));
    }
    let Some(call) = SkillCall::from_request(&request) else {
        return Json(task_result(&request_id, "input-required", "send a DataPart {skill, args}", None));
    };

    // Count the request in the registry in the background; it must not slow the answer down.
    let (background_app, caller) = (app.clone(), peer.to_string());
    tokio::spawn(async move { data::touch(&background_app, &caller).await });

    let outcome = run_skill(&app, &peer, &call).await;
    Json(reply_and_audit(&app, &peer, &call.skill, &request_id, outcome))
}

/// Check the caller's quota, then run the skill. Registry syncs have their own, larger quota so that
/// polling every 30 seconds cannot use up the budget for LLM and data calls.
async fn run_skill(app: &App, peer: &EndpointId, call: &SkillCall) -> SkillResult {
    if call.skill == "agents.sync" {
        if !app.sync_quota.check(peer) {
            return Err(SkillError::RuleDenied("sync quota exceeded".into()));
        }
        return data::sync(app, &peer.to_string(), &call.args).await;
    }
    if !app.quota.check(peer) {
        return Err(SkillError::RuleDenied("hourly call quota exceeded".into()));
    }
    match call.skill.as_str() {
        "llm.complete" => skills::llm_complete(app, &call.args).await,
        "ip.lookup" => skills::ip_lookup(app, &call.args).await,
        "data.catalog" => data::catalog(app).await,
        "data.query" => data::query(app, &call.args).await,
        "files.get" => data::file_get(app, &call.args).await,
        other => Err(SkillError::RuleDenied(format!("unknown skill {other:?}"))),
    }
}

/// Turn a skill's outcome into the A2A reply, and record the decision in the audit log.
fn reply_and_audit(app: &Arc<App>, peer: &EndpointId, skill: &str, request_id: &Value, outcome: SkillResult) -> Value {
    match outcome {
        Ok(SkillOutput { text, data }) => {
            audit(app, peer, skill, "allow", "");
            task_result(request_id, "completed", &text, data)
        }
        Err(SkillError::RuleDenied(reason)) => {
            audit(app, peer, skill, "deny-rule", &reason);
            task_result(request_id, "rejected", &format!("denied by rule: {reason}"), None)
        }
        Err(SkillError::ReviewDenied(reason)) => {
            audit(app, peer, skill, "deny-llm", &reason);
            task_result(request_id, "rejected", &format!("denied by review: {reason}"), None)
        }
        Err(SkillError::UpstreamFailed(reason)) => {
            audit(app, peer, skill, "error", &reason);
            task_result(request_id, "failed", &format!("upstream error: {reason}"), None)
        }
    }
}

/// Log one decision as a JSON line and store it in Postgres. Allowed registry syncs are skipped: polling
/// every 30 seconds would drown everything else in the audit log.
fn audit(app: &Arc<App>, peer: &EndpointId, skill: &str, decision: &str, reason: &str) {
    if skill == "agents.sync" && decision == "allow" {
        return;
    }
    println!(
        "{}",
        json!({"audit": {"peer": peer.to_string(), "skill": skill, "decision": decision, "reason": reason}})
    );

    let app = app.clone();
    let (peer, skill, decision, reason) =
        (peer.to_string(), skill.to_string(), decision.to_string(), reason.to_string());
    tokio::spawn(async move { data::record_audit(&app, &peer, &skill, &decision, &reason).await });
}
