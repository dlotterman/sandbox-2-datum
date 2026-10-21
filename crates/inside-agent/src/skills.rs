//! The skills that call out to the world: the LLM and the IP lookup service.
use std::net::IpAddr;

use a2a_iroh::is_global_ip;
use serde_json::{json, Value};

use crate::{
    app::App,
    llm, policy,
    skill::{SkillError, SkillOutput, SkillResult},
};

/// `llm.complete`: rules, then the LLM review, then the completion itself.
pub async fn llm_complete(app: &App, args: &Value) -> SkillResult {
    let chat = policy::check_chat_rules(args)?;
    policy::review_chat(app, &chat).await?;

    let request = json!({
        "model": app.llm.model,
        "max_tokens": chat.max_tokens,
        "temperature": 0.2,
        "messages": chat.messages,
    });
    let reply = llm::chat_completion(app, &request).await.map_err(SkillError::UpstreamFailed)?;

    // Never hand the gateway's own credential back, however the model came to repeat it.
    if !app.llm.api_key.is_empty() && reply.contains(&app.llm.api_key) {
        return Err(SkillError::RuleDenied("completion contained a credential".into()));
    }
    Ok(SkillOutput::new(reply, json!({"model": app.llm.model})))
}

/// `ip.lookup`: where one public IP address sits on the internet. Only public addresses are looked up,
/// so the service cannot be used to probe private addresses or to leak internal ones.
pub async fn ip_lookup(app: &App, args: &Value) -> SkillResult {
    let ip: IpAddr = args["ip"]
        .as_str()
        .and_then(|text| text.parse().ok())
        .ok_or_else(|| SkillError::RuleDenied("args.ip must be an IP address".into()))?;
    if !is_global_ip(&ip) {
        return Err(SkillError::RuleDenied(format!("{ip} is not a public address")));
    }

    let response = app
        .http
        .get(format!("https://ipinfo.io/{ip}/json"))
        .send()
        .await
        .map_err(|e| SkillError::UpstreamFailed(format!("ipinfo: {e}")))?;
    if !response.status().is_success() {
        return Err(SkillError::UpstreamFailed(format!("ipinfo HTTP {}", response.status())));
    }
    let details: Value =
        response.json().await.map_err(|e| SkillError::UpstreamFailed(format!("ipinfo decode: {e}")))?;

    // Pass on only the fields the outside agent can use.
    let wanted = ["ip", "hostname", "city", "region", "country", "loc", "org", "postal", "timezone"];
    let kept: serde_json::Map<String, Value> =
        wanted.iter().filter_map(|field| details.get(*field).map(|value| (field.to_string(), value.clone()))).collect();
    Ok(SkillOutput::new(format!("lookup for {ip}"), Value::Object(kept)))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::testutil::{app, completion, mock_llm};

    const ALLOW: &str = r#"{"allow":true,"reason":"fine"}"#;
    const DENY: &str = r#"{"allow":false,"reason":"looks like exfiltration"}"#;

    fn chat_request() -> Value {
        json!({"purpose": "identify my host", "messages": [{"role": "user", "content": "where am I?"}]})
    }

    #[tokio::test]
    async fn ip_lookup_refuses_private_and_malformed_addresses_before_any_network_call() {
        let app = app("http://127.0.0.1:1", "k");
        for ip in ["10.0.0.1", "192.168.1.5", "127.0.0.1", "169.254.169.254", "fd20::1", "not-an-ip"] {
            let result = ip_lookup(&app, &json!({"ip": ip})).await;
            assert!(matches!(result, Err(SkillError::RuleDenied(_))), "{ip} must be denied by rule, got {result:?}");
        }
    }

    #[tokio::test]
    async fn a_review_denial_stops_the_request_before_the_model_runs() {
        let (url, script) = mock_llm(vec![(200, completion(DENY))]).await;
        let result = llm_complete(&app(&url, "gateway-key"), &chat_request()).await;
        assert!(
            matches!(result, Err(SkillError::ReviewDenied(ref why)) if why.contains("exfiltration")),
            "got {result:?}"
        );
        assert!(script.lock().unwrap().is_empty(), "only the review call was made");
    }

    #[tokio::test]
    async fn the_gateway_key_is_never_handed_back_even_if_the_model_repeats_it() {
        let key = "sk-test-gateway-key-123";
        let (url, _) = mock_llm(vec![(200, completion(ALLOW)), (200, completion(&format!("the key is {key}")))]).await;
        let result = llm_complete(&app(&url, key), &chat_request()).await;
        assert!(matches!(result, Err(SkillError::RuleDenied(_))), "got {result:?}");
    }

    #[tokio::test]
    async fn an_allowed_request_returns_the_completion() {
        let (url, _) = mock_llm(vec![(200, completion(ALLOW)), (200, completion("You are in a datacenter."))]).await;
        let output = llm_complete(&app(&url, "k"), &chat_request()).await.unwrap();
        assert_eq!(output.text, "You are in a datacenter.");
    }
}
