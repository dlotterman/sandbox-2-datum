//! The hosting-guess story, in two model calls.
//!
//! 1. Ask the inside agent what reference data it has (`data.catalog`).
//! 2. One LLM call: state a hunch about where this agent runs, and choose which of that data would help.
//! 3. Fetch the chosen data through the inside agent's policy gate (`data.query`, `files.get`).
//! 4. One LLM call: give the final answer, using what came back.
//!
//! Every step goes through [`ask_inside`], so each shows up as a finding on the dashboard.
use a2a_iroh::Client;
use serde_json::{json, Value};

use crate::{
    findings::ask_inside,
    state::Shared,
    text::{find_json_object, truncate_for_prompt},
};

// ---- how much text goes into each prompt, and how long the replies may be

/// Reference data is clipped to this many bytes each, to keep prompts small and cheap.
const REFERENCE_PROMPT_BYTES: usize = 2500;
/// Reasoning models spend tokens thinking before they answer, so these leave generous headroom.
const PLAN_MAX_TOKENS: u64 = 900;
const ANSWER_MAX_TOKENS: u64 = 1000;
/// Ask for at most this much reference data.
const MAX_DATASETS: usize = 1;
const MAX_FILES: usize = 2;

// ---- the prompts

const PLAN_PURPOSE: &str = "decide which reference data to request to identify the hosting platform";
const PLAN_INSTRUCTIONS: &str = "\
You identify the hosting platform of a sandboxed agent. State your hunch and choose ONLY catalog items that would \
help (at most 1 dataset, 2 files; none if nothing helps). Reply with ONLY compact JSON: \
{\"hunch\":\"<provider, region, one line>\",\"datasets\":[<names>],\"files\":[<names>],\"reason\":\"<short>\"}";

const ANSWER_PURPOSE: &str = "answer where this agent is hosted, using the reference data from the gateway";
const ANSWER_INSTRUCTIONS: &str = "\
You are an infrastructure forensics assistant. In under 120 words give: Provider, Region/location, Platform type, \
Confidence %, 3 short evidence bullets, and one line on what the reference data changed or confirmed.";

/// The dashboard title of the final answer. The registry report looks for findings that start with "Hosting guess".
pub const ANSWER_TITLE: &str = "Hosting guess, refined with inside data";
const PLAN_TITLE: &str = "What do I suspect, and what data do I need?";

/// What the model is told about this agent, already trimmed to prompt size.
pub struct Evidence {
    pub facts: String,
    pub ip_info: String,
}

/// Run the whole story and record each step. Failures are recorded as findings, not raised: the cycle carries on.
pub async fn run(shared: &Shared, client: &Client, evidence: &Evidence) {
    let catalog = fetch_catalog(shared, client).await;
    let plan = plan_data_needs(shared, client, evidence, &catalog).await;
    let reference_data = fetch_reference_data(shared, client, &plan).await;
    give_final_answer(shared, client, evidence, &reference_data).await;
}

// ---- step 1: what data exists

/// The inside agent's datasets and files, as returned by `data.catalog`.
struct Catalog(Value);

impl Catalog {
    fn has_dataset(&self, name: &str) -> bool {
        self.lists(&self.0["datasets"], name)
    }

    fn has_file(&self, name: &str) -> bool {
        self.lists(&self.0["files"], name)
    }

    fn lists(&self, entries: &Value, name: &str) -> bool {
        entries.as_array().into_iter().flatten().any(|entry| entry["name"].as_str() == Some(name))
    }
}

async fn fetch_catalog(shared: &Shared, client: &Client) -> Catalog {
    let outcome = ask_inside(
        shared,
        client,
        "What data does the inside agent have?",
        "data.catalog",
        json!({}),
        "What data do you have?",
    )
    .await;
    // If the catalog is unavailable, carry on as if it were empty.
    let catalog = outcome.ok().and_then(|task| task.data).unwrap_or(json!({"datasets": [], "files": []}));
    Catalog(catalog)
}

// ---- step 2: a hunch, and what data to ask for

/// What the model decided to ask for.
struct DataPlan {
    datasets: Vec<String>,
    files: Vec<String>,
    /// The model's one-line reason, passed to the inside agent as the stated purpose of each data request.
    reason: String,
}

impl DataPlan {
    /// Read the model's JSON reply. A reply we cannot understand is treated as "ask for nothing".
    fn from_reply(reply: &str) -> Self {
        let json = find_json_object(reply).unwrap_or(Value::Null);
        let names = |field: &str, limit: usize| -> Vec<String> {
            json[field]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .take(limit)
                .map(String::from)
                .collect()
        };
        let reason = json["reason"].as_str().unwrap_or("identify hosting platform").chars().take(250).collect();
        Self { datasets: names("datasets", MAX_DATASETS), files: names("files", MAX_FILES), reason }
    }
}

impl DataPlan {
    /// Keep only what the catalog really lists: the model is guessing and can name things that do not exist.
    fn limited_to(self, catalog: &Catalog) -> Self {
        Self {
            datasets: self.datasets.into_iter().filter(|name| catalog.has_dataset(name)).collect(),
            files: self.files.into_iter().filter(|name| catalog.has_file(name)).collect(),
            reason: self.reason,
        }
    }
}

async fn plan_data_needs(shared: &Shared, client: &Client, evidence: &Evidence, catalog: &Catalog) -> DataPlan {
    let content =
        format!("Runtime facts:\n{}\n\nIP lookup:\n{}\n\nCatalog:\n{}", evidence.facts, evidence.ip_info, catalog.0);
    let outcome = ask_llm(shared, client, PLAN_TITLE, PLAN_PURPOSE, PLAN_INSTRUCTIONS, content, PLAN_MAX_TOKENS).await;
    let plan = match outcome {
        Ok(task) => DataPlan::from_reply(&task.text),
        Err(_) => DataPlan::from_reply(""),
    };
    plan.limited_to(catalog)
}

// ---- step 3: fetch what it asked for

/// Request the planned data. Returns each piece as a labelled block of text for the final prompt.
async fn fetch_reference_data(shared: &Shared, client: &Client, plan: &DataPlan) -> Vec<String> {
    let mut blocks = Vec::new();

    for dataset in &plan.datasets {
        let args = json!({"dataset": dataset, "purpose": plan.reason});
        let title = format!("Asked inside for dataset: {dataset}");
        let outcome = ask_inside(shared, client, &title, "data.query", args, "Request dataset").await;
        if let Some(data) = outcome.ok().and_then(|task| task.data) {
            blocks.push(format!(
                "dataset {dataset}:\n{}",
                truncate_for_prompt(&data.to_string(), REFERENCE_PROMPT_BYTES)
            ));
        }
    }

    for file in &plan.files {
        let args = json!({"name": file, "purpose": plan.reason});
        let title = format!("Asked inside for file: {file}");
        let outcome = ask_inside(shared, client, &title, "files.get", args, "Request file").await;
        if let Some(data) = outcome.ok().and_then(|task| task.data) {
            let content = data["content"].as_str().unwrap_or("");
            blocks.push(format!("file {file}:\n{}", truncate_for_prompt(content, REFERENCE_PROMPT_BYTES)));
        }
    }
    blocks
}

// ---- step 4: the answer

async fn give_final_answer(shared: &Shared, client: &Client, evidence: &Evidence, reference_data: &[String]) {
    let reference =
        if reference_data.is_empty() { "(none available)".to_string() } else { reference_data.join("\n\n") };
    let content = format!(
        "Runtime facts:\n{}\n\nIP lookup:\n{}\n\nReference data from the gateway:\n{reference}",
        evidence.facts, evidence.ip_info
    );
    // The outcome is already recorded as a finding; nothing else needs it.
    let _ =
        ask_llm(shared, client, ANSWER_TITLE, ANSWER_PURPOSE, ANSWER_INSTRUCTIONS, content, ANSWER_MAX_TOKENS).await;
}

// ---- shared

/// Ask the inside agent's LLM a question: `instructions` as the system message, `content` as the user message.
async fn ask_llm(
    shared: &Shared,
    client: &Client,
    title: &str,
    purpose: &str,
    instructions: &str,
    content: String,
    max_tokens: u64,
) -> anyhow::Result<a2a_iroh::TaskOutcome> {
    let args = json!({
        "purpose": purpose,
        "max_tokens": max_tokens,
        "messages": [
            {"role": "system", "content": instructions},
            {"role": "user", "content": content},
        ],
    });
    ask_inside(shared, client, title, "llm.complete", args, title).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_plan_is_read_from_the_models_json_and_capped() {
        let reply = r#"Here you go: {"hunch":"Fly.io lax","datasets":["platforms","extra"],"files":["a.md","b.md","c.md"],"reason":"signals match"}"#;
        let plan = DataPlan::from_reply(reply);
        assert_eq!(plan.datasets, vec!["platforms"], "at most {MAX_DATASETS} dataset");
        assert_eq!(plan.files, vec!["a.md", "b.md"], "at most {MAX_FILES} files");
        assert_eq!(plan.reason, "signals match");
    }

    #[test]
    fn an_unreadable_reply_means_asking_for_nothing() {
        let plan = DataPlan::from_reply("I could not decide.");
        assert!(plan.datasets.is_empty() && plan.files.is_empty());
    }

    #[test]
    fn a_plan_is_trimmed_to_what_the_catalog_actually_lists() {
        let catalog = Catalog(json!({"datasets": [{"name": "platforms"}], "files": [{"name": "a.md", "bytes": 3}]}));
        let plan = DataPlan {
            datasets: vec!["platforms".into(), "secrets".into()],
            files: vec!["../etc/passwd".into(), "a.md".into()],
            reason: "r".into(),
        }
        .limited_to(&catalog);
        assert_eq!(plan.datasets, vec!["platforms"]);
        assert_eq!(plan.files, vec!["a.md"]);
    }

    #[test]
    fn the_catalog_only_vouches_for_what_it_lists() {
        let catalog = Catalog(json!({"datasets": [{"name": "platforms"}], "files": [{"name": "a.md", "bytes": 3}]}));
        assert!(catalog.has_dataset("platforms") && catalog.has_file("a.md"));
        assert!(!catalog.has_dataset("secrets") && !catalog.has_file("../etc/passwd"));
    }
}
