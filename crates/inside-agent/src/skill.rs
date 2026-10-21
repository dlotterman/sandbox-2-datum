//! The shared shape of a skill's result.
use serde_json::Value;

/// What a skill returns when it succeeds: a line of text and, usually, structured data.
#[derive(Debug)]
pub struct SkillOutput {
    pub text: String,
    pub data: Option<Value>,
}

impl SkillOutput {
    pub fn new(text: impl Into<String>, data: Value) -> Self {
        Self { text: text.into(), data: Some(data) }
    }
}

/// Why a skill did not produce output. The variants map to different A2A task states and audit decisions.
#[derive(Debug)]
pub enum SkillError {
    /// A deterministic rule refused the request: bad input, over quota, something not on an allowlist.
    RuleDenied(String),
    /// The LLM reviewer refused the request, or could not be asked at all. The review fails closed.
    ReviewDenied(String),
    /// The request was acceptable but something it depends on (Postgres, rclone, the LLM) failed.
    UpstreamFailed(String),
}

pub type SkillResult = Result<SkillOutput, SkillError>;
