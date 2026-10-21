//! The registry of agents seen recently, and the `agents.sync` skill.
//!
//! Every authenticated request updates the caller's row (first seen, last seen, request count). An agent
//! can also report facts about itself with `agents.sync`; those reports are rebuilt field by field from an
//! allowlist before they are stored, because they are self-declared and shown to other agents.
use serde_json::{json, Map, Value};

use super::postgres;
use crate::{
    app::App,
    skill::{SkillError, SkillOutput, SkillResult},
};

/// How far back `agents.sync` lists agents.
const WINDOW_SECS: i64 = 6 * 3600;
/// An agent counts as live if it was seen this recently. A little over two 30-second sync intervals.
const LIVE_SECS: i64 = 90;
/// Rows older than this are deleted.
const KEEP_HOURS: i64 = 24;

/// Text fields a report may carry, and the most characters kept of each. Anything else is dropped.
const TEXT_FIELDS: [(&str, usize); 12] = [
    ("label", 60),
    ("platform_hint", 80),
    ("provider_guess", 200),
    ("hosting_guess", 1500),
    ("ip_org", 100),
    ("ip_city", 80),
    ("ip_region", 80),
    ("ip_country", 8),
    ("hostname", 80),
    ("kernel", 80),
    ("os", 80),
    ("arch", 16),
];
/// Whole-number fields a report may carry.
const NUMBER_FIELDS: [&str; 5] = ["cpus", "mem_mb", "started", "cycles", "last_cycle"];
/// A sanitized report larger than this is refused outright.
const MAX_REPORT_BYTES: usize = 6000;

/// Note that `peer` just made a request. Best effort: a database problem must not affect the request.
pub async fn touch(app: &App, peer: &str) {
    if !app.postgres.is_configured() {
        return;
    }
    if let Ok(client) = postgres::connect(app).await {
        let _ = client
            .execute(
                "INSERT INTO agents_seen (endpoint_id, requests) VALUES ($1, 1)
                 ON CONFLICT (endpoint_id) DO UPDATE SET last_seen = now(), requests = agents_seen.requests + 1",
                &[&peer],
            )
            .await;
    }
}

/// `agents.sync`: store the caller's report if it sent one, then list every agent seen in the last 6 hours.
pub async fn sync(app: &App, peer: &str, args: &Value) -> SkillResult {
    let client = postgres::connect(app).await.map_err(SkillError::UpstreamFailed)?;

    if !args["report"].is_null() {
        let report = sanitize_report(&args["report"])?.to_string();
        client
            .execute(
                "INSERT INTO agents_seen (endpoint_id, report, reported_at) VALUES ($1, $2, now())
                 ON CONFLICT (endpoint_id) DO UPDATE SET report = $2, reported_at = now(), last_seen = now()",
                &[&peer, &report],
            )
            .await
            .map_err(|e| SkillError::UpstreamFailed(e.to_string()))?;
    }

    // Tidy up opportunistically; failing to is harmless.
    let _ = client
        .execute("DELETE FROM agents_seen WHERE last_seen < now() - ($1::bigint * interval '1 hour')", &[&KEEP_HOURS])
        .await;

    let rows = client
        .query(
            "SELECT endpoint_id,
                    extract(epoch from first_seen)::bigint,
                    extract(epoch from last_seen)::bigint,
                    requests,
                    report,
                    extract(epoch from now())::bigint
             FROM agents_seen
             WHERE last_seen > now() - ($1::bigint * interval '1 second')
             ORDER BY last_seen DESC",
            &[&WINDOW_SECS],
        )
        .await
        .map_err(|e| SkillError::UpstreamFailed(e.to_string()))?;

    let now: i64 = rows.first().map(|row| row.get(5)).unwrap_or(0);
    let agents: Vec<Value> = rows
        .iter()
        .map(|row| {
            let last_seen: i64 = row.get(2);
            let report: Option<String> = row.get(4);
            json!({
                "endpoint_id": row.get::<_, String>(0),
                "first_seen": row.get::<_, i64>(1),
                "last_seen": last_seen,
                "requests": row.get::<_, i64>(3),
                "active": now - last_seen <= LIVE_SECS,
                "report": report.and_then(|text| serde_json::from_str::<Value>(&text).ok()).unwrap_or(Value::Null),
            })
        })
        .collect();

    Ok(SkillOutput::new(
        format!("{} agent(s) seen in the last 6h", agents.len()),
        json!({"window_secs": WINDOW_SECS, "now": now, "agents": agents}),
    ))
}

/// Rebuild a report from the allowed fields only, each with the right type and a length cap.
/// A `public_ip` is kept only if it really is a public address.
fn sanitize_report(report: &Value) -> Result<Value, SkillError> {
    let sent = report.as_object().ok_or_else(|| SkillError::RuleDenied("report must be an object".into()))?;
    let mut clean = Map::new();

    for (field, max_chars) in TEXT_FIELDS {
        if let Some(text) = sent.get(field).and_then(|value| shortened(value, max_chars)) {
            clean.insert(field.into(), text);
        }
    }
    for field in NUMBER_FIELDS {
        if let Some(number) = sent.get(field).and_then(|value| value.as_u64()) {
            clean.insert(field.into(), json!(number));
        }
    }
    let public_ip = sent.get("public_ip").and_then(|v| v.as_str()).and_then(|s| s.parse::<std::net::IpAddr>().ok());
    if let Some(ip) = public_ip.filter(a2a_iroh::is_global_ip) {
        clean.insert("public_ip".into(), json!(ip.to_string()));
    }

    let clean = Value::Object(clean);
    if clean.to_string().len() > MAX_REPORT_BYTES {
        return Err(SkillError::RuleDenied("report too large".into()));
    }
    Ok(clean)
}

/// The value as a string cut to `max_chars` characters, or nothing if it is not a non-empty string.
fn shortened(value: &Value, max_chars: usize) -> Option<Value> {
    let text = value.as_str()?;
    (!text.is_empty()).then(|| json!(text.chars().take(max_chars).collect::<String>()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_reports_are_rebuilt_from_an_allowlist_not_stored_as_sent() {
        let report = json!({
            "platform_hint": "Fly.io lax",
            "hostname": "x".repeat(5000),
            "public_ip": "8.8.8.8",
            "cycles": 3,
            "evil": {"nested": "payload"},
            "BASETEN_API_KEY": "sk-should-never-appear",
        });
        let clean = sanitize_report(&report).unwrap();
        assert_eq!(clean["platform_hint"], "Fly.io lax");
        assert_eq!(clean["public_ip"], "8.8.8.8");
        assert_eq!(clean["cycles"], 3);
        assert_eq!(clean["hostname"].as_str().unwrap().len(), 80, "long strings are truncated");
        assert!(clean.get("evil").is_none() && clean.get("BASETEN_API_KEY").is_none(), "unknown fields are dropped");

        let private = sanitize_report(&json!({"public_ip": "10.0.0.5"})).unwrap();
        assert!(private.get("public_ip").is_none(), "a private address is not recorded as public");
        assert!(matches!(sanitize_report(&json!("just a string")), Err(SkillError::RuleDenied(_))));
        assert!(matches!(sanitize_report(&json!([1, 2])), Err(SkillError::RuleDenied(_))));
    }
}
