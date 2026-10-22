//! The slice of A2A (Agent2Agent) JSON-RPC that this project speaks.
//!
//! A caller sends `message/send` with a text part and a data part naming a skill. The callee answers with
//! a Task whose artifacts carry the result as text and/or data. These helpers build and read those messages.
use anyhow::{anyhow, Result};
use iroh::SecretKey;
use serde_json::{json, Value};

/// Build a `message/send` request that asks for `skill` with `args`; `text` is a human-readable note.
pub fn message_send(skill: &str, args: Value, text: &str) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": new_id(),
        "method": "message/send",
        "params": {
            "message": {
                "kind": "message",
                "role": "user",
                "messageId": new_id(),
                "parts": [
                    {"kind": "text", "text": text},
                    {"kind": "data", "data": {"skill": skill, "args": args}},
                ],
            }
        }
    })
}

/// The skill a peer calls to exchange an enrollment token for access, `{"token": "..."}`.
pub const ENROLL_SKILL: &str = "agents.enroll";

/// What the inside agent's refusal says when the caller is not enrolled. Clients look for it to know they should enroll.
pub const NOT_ENROLLED: &str = "not enrolled";

/// A finished A2A Task, flattened to what callers here need.
#[derive(Debug, Clone)]
pub struct TaskOutcome {
    /// `completed`, `rejected` (a policy denial), `failed`, or `input-required`.
    pub state: String,
    pub text: String,
    pub data: Option<Value>,
}

impl TaskOutcome {
    /// True if the peer refused because we have not enrolled (or our enrollment expired).
    pub fn is_not_enrolled(&self) -> bool {
        self.state == "rejected" && self.text.contains(NOT_ENROLLED)
    }
}

/// Build the JSON-RPC response for `request_id`: a Task in `state` whose single artifact holds `text` and, if given, `data`.
pub fn task_result(request_id: &Value, state: &str, text: &str, data: Option<Value>) -> Value {
    let mut parts = vec![json!({"kind": "text", "text": text})];
    if let Some(data) = data {
        parts.push(json!({"kind": "data", "data": data}));
    }
    json!({
        "jsonrpc": "2.0",
        "id": request_id,
        "result": {
            "kind": "task",
            "id": new_id(),
            "contextId": new_id(),
            "status": {"state": state},
            "artifacts": [{"artifactId": new_id(), "name": "result", "parts": parts}],
        }
    })
}

/// Build a JSON-RPC protocol error (as opposed to a Task in a failed state).
pub fn rpc_error(request_id: &Value, code: i64, message: &str) -> Value {
    json!({"jsonrpc": "2.0", "id": request_id, "error": {"code": code, "message": message}})
}

/// Read a response produced by [`task_result`]. A JSON-RPC error becomes an `Err`.
pub fn parse_task(response: &Value) -> Result<TaskOutcome> {
    if let Some(error) = response.get("error") {
        return Err(anyhow!("rpc error: {error}"));
    }
    let task = response.get("result").ok_or_else(|| anyhow!("no result in response"))?;
    let state = task["status"]["state"].as_str().unwrap_or("unknown").to_string();

    let mut text = String::new();
    let mut data = None;
    for artifact in task["artifacts"].as_array().into_iter().flatten() {
        for part in artifact["parts"].as_array().into_iter().flatten() {
            match part["kind"].as_str() {
                Some("text") => text.push_str(part["text"].as_str().unwrap_or_default()),
                Some("data") => data = Some(part["data"].clone()),
                _ => {}
            }
        }
    }
    Ok(TaskOutcome { state, text, data })
}

/// A unique-enough id for JSON-RPC requests, tasks and messages: the clock plus 32 random bits.
/// They need to be distinct, not secret; borrowing Iroh's key generator for the randomness avoids another dependency.
pub fn new_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
    let random = SecretKey::generate().to_bytes();
    let random = u32::from_le_bytes([random[0], random[1], random[2], random[3]]);
    format!("{nanos:x}-{random:x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_skill_call_is_a_message_send_with_a_data_part() {
        let request = message_send("ip.lookup", json!({"ip": "8.8.8.8"}), "where is this?");
        assert_eq!(request["method"], "message/send");
        assert_eq!(
            request["params"]["message"]["parts"][1]["data"],
            json!({"skill": "ip.lookup", "args": {"ip": "8.8.8.8"}})
        );
    }

    #[test]
    fn only_a_rejection_that_says_not_enrolled_counts_as_not_enrolled() {
        let outcome = |state: &str, text: &str| TaskOutcome { state: state.into(), text: text.into(), data: None };
        assert!(outcome("rejected", &format!("denied by rule: {NOT_ENROLLED}: call agents.enroll")).is_not_enrolled());
        assert!(!outcome("rejected", "denied by rule: file too large").is_not_enrolled());
        assert!(!outcome("completed", NOT_ENROLLED).is_not_enrolled(), "only a rejection counts");
    }

    #[test]
    fn task_results_round_trip_and_rpc_errors_surface_as_errors() {
        let ok = parse_task(&task_result(&json!(1), "completed", "done", Some(json!({"k": 1})))).unwrap();
        assert_eq!((ok.state.as_str(), ok.text.as_str(), ok.data), ("completed", "done", Some(json!({"k": 1}))));
        assert_eq!(parse_task(&task_result(&json!(1), "rejected", "no", None)).unwrap().state, "rejected");
        assert!(parse_task(&rpc_error(&json!(1), -32601, "nope")).is_err());
    }
}
