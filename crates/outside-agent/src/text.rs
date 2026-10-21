//! Small text helpers for building prompts and reading model answers.
use serde_json::Value;

/// `text` cut to at most `max_bytes` on a character boundary, with a marker if anything was dropped.
pub fn truncate_for_prompt(text: &str, max_bytes: usize) -> String {
    if text.len() <= max_bytes {
        return text.to_string();
    }
    let mut end = max_bytes;
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}…[truncated]", &text[..end])
}

/// The first `{` to the last `}` of `text`, parsed as JSON. Models often wrap their JSON in chatter.
pub fn find_json_object(text: &str) -> Option<Value> {
    let start = text.find('{')?;
    let end = text.rfind('}')?;
    serde_json::from_str(text.get(start..=end)?).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncation_respects_character_boundaries_and_says_so() {
        assert_eq!(truncate_for_prompt("short", 10), "short");
        assert_eq!(truncate_for_prompt("abcdef", 3), "abc…[truncated]");
        assert_eq!(truncate_for_prompt("caf\u{e9}", 4), "caf…[truncated]", "never splits the two-byte é");
    }

    #[test]
    fn json_is_found_inside_model_chatter() {
        assert_eq!(find_json_object(r#"Sure: {"a": 1} hope that helps"#), Some(serde_json::json!({"a": 1})));
        assert_eq!(find_json_object("no json here"), None);
    }
}
