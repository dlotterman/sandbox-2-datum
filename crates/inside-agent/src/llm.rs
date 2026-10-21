//! The one place that talks to the LLM provider.
use std::time::Duration;

use serde_json::Value;

use crate::app::App;

/// Give up after this many attempts at one completion.
const MAX_ATTEMPTS: u32 = 5;
/// Never wait longer than this between attempts, even if the provider asks for more.
const MAX_WAIT_SECS: u64 = 20;

/// POST a chat completion request and return the reply text.
///
/// At most `App::llm_slots` requests run at once, and HTTP 429 or 5xx answers are retried with
/// backoff (honoring `Retry-After`). Without this, several agents running their cycles at the same time
/// triggered rate limits, and because the policy review fails closed, those turned into wrongful denials.
pub async fn chat_completion(app: &App, request: &Value) -> Result<String, String> {
    if app.llm.api_key.is_empty() {
        return Err("gateway has no LLM credentials".into());
    }
    let _slot = app.llm_slots.acquire().await.map_err(|e| e.to_string())?;

    let mut last_failure = String::new();
    for attempt in 1..=MAX_ATTEMPTS {
        let response = app
            .http
            .post(format!("{}/chat/completions", app.llm.base_url))
            .bearer_auth(&app.llm.api_key)
            .json(request)
            .send()
            .await
            .map_err(|e| format!("request: {e}"))?;
        let status = response.status();

        if status.as_u16() == 429 || status.is_server_error() {
            let wait = seconds_to_wait(&response, attempt);
            last_failure = format!("HTTP {status}");
            eprintln!("llm {status}; retry {attempt}/{MAX_ATTEMPTS} in {wait}s");
            tokio::time::sleep(Duration::from_secs(wait)).await;
            continue;
        }

        let body: Value = response.json().await.map_err(|e| format!("decode (HTTP {status}): {e}"))?;
        if !status.is_success() {
            return Err(format!("HTTP {status}"));
        }
        return body["choices"][0]["message"]["content"]
            .as_str()
            .map(|text| text.trim().to_string())
            .ok_or_else(|| "no content in completion".to_string());
    }
    Err(format!("still failing after {MAX_ATTEMPTS} attempts ({last_failure})"))
}

/// The provider's `Retry-After` if it sent one, otherwise 2, 4, 8... seconds.
fn seconds_to_wait(response: &reqwest::Response, attempt: u32) -> u64 {
    let requested = response
        .headers()
        .get("retry-after")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok());
    requested.unwrap_or(2u64 << (attempt - 1)).min(MAX_WAIT_SECS)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::testutil::{app, completion, mock_llm};

    #[tokio::test]
    async fn rate_limits_are_retried_so_concurrent_agents_do_not_cause_spurious_denials() {
        let (url, script) = mock_llm(vec![(429, "{}".into()), (429, "{}".into()), (200, completion("ok"))]).await;
        assert_eq!(chat_completion(&app(&url, "k"), &json!({})).await.unwrap(), "ok");
        assert!(script.lock().unwrap().is_empty(), "all three scripted replies were used");
    }

    #[tokio::test]
    async fn without_an_api_key_no_request_is_made() {
        let (url, script) = mock_llm(vec![(200, completion("never sent"))]).await;
        assert!(chat_completion(&app(&url, ""), &json!({})).await.is_err());
        assert_eq!(script.lock().unwrap().len(), 1, "the mock was never contacted");
    }
}
