//! The `files.get` skill, over the rclone WebDAV file store.
//!
//! The store is a flat directory, read-only, behind basic auth. This agent is the only client. Names are
//! validated so a caller can never reach outside that directory, and only small UTF-8 text files are served.
use serde_json::json;

use crate::{
    app::App,
    policy,
    skill::{SkillError, SkillOutput, SkillResult},
};

/// Files larger than this are refused.
const MAX_FILE_BYTES: usize = 64 * 1024;
const MAX_NAME_CHARS: usize = 64;

/// One file in the store, as listed by the WebDAV server.
pub struct RemoteFile {
    pub name: String,
    pub size: usize,
}

/// `files.get`: rules, then the LLM review, then the download.
pub async fn file_get(app: &App, args: &serde_json::Value) -> SkillResult {
    let name = args["name"].as_str().unwrap_or("");
    if !valid_name(name) {
        return Err(SkillError::RuleDenied("file name must be a flat name of letters, digits, '.', '_' or '-'".into()));
    }
    let purpose = policy::require_purpose(args)?;

    let listing = list_files(app).await.map_err(SkillError::UpstreamFailed)?;
    let file = listing
        .iter()
        .find(|file| file.name == name)
        .ok_or_else(|| SkillError::RuleDenied("no such file (see data.catalog)".into()))?;
    if file.size > MAX_FILE_BYTES {
        return Err(SkillError::RuleDenied(format!("file too large ({} > {MAX_FILE_BYTES} bytes)", file.size)));
    }
    policy::review(app, "files.get", &purpose, args).await?;

    let response = app
        .http
        .get(format!("{}/{name}", app.rclone.url))
        .basic_auth(&app.rclone.user, Some(&app.rclone.password))
        .send()
        .await
        .map_err(|e| SkillError::UpstreamFailed(format!("rclone: {e}")))?;
    if !response.status().is_success() {
        return Err(SkillError::UpstreamFailed(format!("rclone HTTP {}", response.status())));
    }
    let bytes = response.bytes().await.map_err(|e| SkillError::UpstreamFailed(e.to_string()))?;
    let content = String::from_utf8(bytes.to_vec())
        .map_err(|_| SkillError::RuleDenied("only UTF-8 text files are served".into()))?;

    Ok(SkillOutput::new(format!("{name} ({} bytes)", content.len()), json!({"name": name, "content": content})))
}

/// A name that can only refer to a file directly inside the store: no path separators, no leading dot
/// (so no `..`), only plain characters. A name like this cannot escape the directory.
pub fn valid_name(name: &str) -> bool {
    let plain = |c: char| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-');
    !name.is_empty() && name.len() <= MAX_NAME_CHARS && !name.starts_with('.') && name.chars().all(plain)
}

/// List the store's top-level files with a WebDAV PROPFIND at depth 1.
pub async fn list_files(app: &App) -> Result<Vec<RemoteFile>, String> {
    let propfind = reqwest::Method::from_bytes(b"PROPFIND").expect("PROPFIND is a valid HTTP method");
    let response = app
        .http
        .request(propfind, format!("{}/", app.rclone.url))
        .basic_auth(&app.rclone.user, Some(&app.rclone.password))
        .header("Depth", "1")
        .send()
        .await
        .map_err(|e| format!("rclone: {e}"))?;
    if !response.status().is_success() {
        return Err(format!("rclone HTTP {}", response.status()));
    }
    let xml = response.text().await.map_err(|e| e.to_string())?;
    Ok(parse_propfind(&xml))
}

/// Pull plain files out of a WebDAV multistatus reply. The reply is simple and fixed in shape, so this
/// reads the few tags it needs directly instead of adding an XML dependency. Directories and anything with
/// an unsafe name are skipped.
fn parse_propfind(xml: &str) -> Vec<RemoteFile> {
    let mut files = Vec::new();
    for entry in xml.split("<D:response>").skip(1) {
        let name = tag_text(entry, "displayname").unwrap_or_default();
        let is_directory = entry.contains("<D:collection");
        if is_directory || !valid_name(&name) {
            continue;
        }
        let size = tag_text(entry, "getcontentlength").and_then(|text| text.parse().ok()).unwrap_or(0);
        files.push(RemoteFile { name, size });
    }
    files
}

/// The text between `<D:tag>` and `</D:tag>` in `entry`, if both are present.
fn tag_text(entry: &str, tag: &str) -> Option<String> {
    let open = format!("<D:{tag}>");
    let close = format!("</D:{tag}>");
    let start = entry.find(&open)? + open.len();
    let end = entry.find(&close)?;
    Some(entry[start..end].to_string())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::testutil::app;

    #[test]
    fn file_names_are_flat_so_traversal_is_impossible() {
        let too_long = "x".repeat(MAX_NAME_CHARS + 1);
        for ok in ["README.txt", "platform-fingerprints.md", "regions.csv", "a_b-c.1"] {
            assert!(valid_name(ok), "{ok}");
        }
        for bad in [
            "",
            ".hidden",
            "..",
            "../etc/passwd",
            "a/b",
            "a\\b",
            "name with space",
            "semi;colon",
            "nul\0",
            too_long.as_str(),
            "caf\u{e9}.txt",
        ] {
            assert!(!valid_name(bad), "{bad:?} must be refused");
        }
    }

    #[tokio::test]
    async fn file_requests_with_unsafe_names_are_denied_by_rule() {
        let app = app("http://127.0.0.1:1", "k");
        for name in ["../secrets", ".env", "a/b", ""] {
            let result = file_get(&app, &json!({"name": name, "purpose": "read it"})).await;
            assert!(matches!(result, Err(SkillError::RuleDenied(_))), "{name:?}: got {result:?}");
        }
    }

    #[test]
    fn a_webdav_listing_yields_plain_files_only() {
        let xml = "<D:multistatus>\
            <D:response><D:href>/</D:href><D:propstat><D:prop><D:resourcetype><D:collection/></D:resourcetype>\
              <D:displayname></D:displayname></D:prop></D:propstat></D:response>\
            <D:response><D:href>/a.txt</D:href><D:propstat><D:prop><D:displayname>a.txt</D:displayname>\
              <D:getcontentlength>6</D:getcontentlength></D:prop></D:propstat></D:response>\
            <D:response><D:href>/sub</D:href><D:propstat><D:prop><D:resourcetype><D:collection/></D:resourcetype>\
              <D:displayname>sub</D:displayname></D:prop></D:propstat></D:response>\
            <D:response><D:href>/.secret</D:href><D:propstat><D:prop><D:displayname>.secret</D:displayname>\
              <D:getcontentlength>1</D:getcontentlength></D:prop></D:propstat></D:response>\
            </D:multistatus>";
        let files = parse_propfind(xml);
        assert_eq!(files.len(), 1, "the root, the directory and the dotfile are all skipped");
        assert_eq!((files[0].name.as_str(), files[0].size), ("a.txt", 6));
    }
}
