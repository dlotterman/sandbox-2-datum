//! Looking at ourselves: what this process can see about where it runs. Local files and environment only,
//! no network access.
use std::{collections::BTreeMap, fs};

use serde_json::{json, Value};

/// Environment variable name prefixes that tend to identify a hosting platform. Their values are shared
/// (unless the name looks secret); every other variable is reported by name only.
const PLATFORM_PREFIXES: &[&str] = &[
    "FLY_",
    "RAILWAY_",
    "RUNPOD_",
    "DAYTONA_",
    "KUBERNETES_",
    "AWS_",
    "GOOGLE_",
    "GCP_",
    "AZURE_",
    "RENDER",
    "VERCEL_",
    "HEROKU_",
    "EXE_",
    "DATUM_",
    "CONTAINER",
    "HOSTNAME",
    "NOMAD_",
    "ECS_",
    "K_",
    "WEBSITE_",
];

/// A variable whose name contains any of these is never reported with its value, whatever its prefix.
const SECRETISH_WORDS: &[&str] = &["KEY", "SECRET", "TOKEN", "PASSWORD", "PASS", "CREDENTIAL", "AUTH"];

/// DMI fields (from `/sys/class/dmi/id`) that reveal the hardware or hypervisor vendor.
const DMI_FIELDS: &[&str] = &["sys_vendor", "product_name", "bios_vendor", "board_vendor", "chassis_vendor"];

/// The facts used in prompts: enough to identify a platform, without the kernel build string, uptime,
/// timezone or the names of unrelated environment variables.
const IDENTIFYING_FIELDS: &[&str] = &[
    "hostname",
    "kernel",
    "os",
    "arch",
    "cpus",
    "mem_total_kb",
    "uid_line",
    "cgroup",
    "containerenv_present",
    "dockerenv_present",
    "dmi",
    "nameservers",
    "interfaces",
    "platform_env",
];

/// Gather everything this process can learn locally. The dashboard shows all of it.
pub fn collect() -> Value {
    let (platform_env, other_env_names) = split_environment();

    json!({
        "hostname": read_trimmed("/proc/sys/kernel/hostname"),
        "kernel": read_trimmed("/proc/sys/kernel/osrelease"),
        "kernel_version": read_trimmed("/proc/sys/kernel/version"),
        "os": os_pretty_name(),
        "arch": std::env::consts::ARCH,
        "cpus": std::thread::available_parallelism().map(|n| n.get()).ok(),
        "mem_total_kb": memory_total_kb(),
        "uptime_secs": read_trimmed("/proc/uptime").and_then(|text| text.split_whitespace().next().map(str::to_string)),
        "uid_line": uid_line(),
        "cgroup": read_trimmed("/proc/self/cgroup").map(|text| text.lines().take(3).collect::<Vec<_>>().join(" | ")),
        "containerenv_present": std::path::Path::new("/run/.containerenv").exists(),
        "dockerenv_present": std::path::Path::new("/.dockerenv").exists(),
        "dmi": dmi_fields(),
        "timezone": read_trimmed("/etc/timezone"),
        "nameservers": nameservers(),
        "interfaces": network_interfaces(),
        "platform_env": platform_env,
        "other_env_names": other_env_names,
    })
}

/// The subset of `facts` (as returned by [`collect`]) that goes into prompts.
pub fn identifying_signals(facts: &Value) -> Value {
    match facts.as_object() {
        Some(all) => Value::Object(
            all.iter()
                .filter(|(name, _)| IDENTIFYING_FIELDS.contains(&name.as_str()))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        ),
        None => Value::Null,
    }
}

/// The file's contents with surrounding whitespace removed, or `None` if it is missing, unreadable or empty.
fn read_trimmed(path: &str) -> Option<String> {
    let text = fs::read_to_string(path).ok()?;
    let text = text.trim();
    (!text.is_empty()).then(|| text.to_string())
}

/// Split the environment into platform-hint variables (with values) and everything else (names only).
/// Names that look secret always land in the second group, so their values never leave the process.
fn split_environment() -> (BTreeMap<String, String>, Vec<String>) {
    let mut platform = BTreeMap::new();
    let mut other_names = Vec::new();
    for (name, value) in std::env::vars() {
        let looks_secret = SECRETISH_WORDS.iter().any(|word| name.to_uppercase().contains(word));
        let is_platform_hint = PLATFORM_PREFIXES.iter().any(|prefix| name.starts_with(prefix));
        if is_platform_hint && !looks_secret {
            platform.insert(name, value);
        } else {
            other_names.push(name);
        }
    }
    other_names.sort();
    (platform, other_names)
}

fn memory_total_kb() -> Option<u64> {
    let meminfo = read_trimmed("/proc/meminfo")?;
    let line = meminfo.lines().find(|line| line.starts_with("MemTotal:"))?;
    line.split_whitespace().nth(1)?.parse().ok()
}

/// The `Uid:` line of `/proc/self/status`: real, effective, saved and filesystem user ids. Shows whether we run as root.
fn uid_line() -> Option<String> {
    let status = read_trimmed("/proc/self/status")?;
    status.lines().find(|line| line.starts_with("Uid:")).map(String::from)
}

fn nameservers() -> Vec<String> {
    let resolv_conf = read_trimmed("/etc/resolv.conf").unwrap_or_default();
    resolv_conf
        .lines()
        .filter_map(|line| line.strip_prefix("nameserver "))
        .map(|address| address.trim().to_string())
        .collect()
}

fn network_interfaces() -> Vec<Value> {
    if_addrs::get_if_addrs()
        .unwrap_or_default()
        .iter()
        .map(|interface| json!({"name": interface.name, "ip": interface.ip().to_string()}))
        .collect()
}

fn dmi_fields() -> BTreeMap<&'static str, String> {
    DMI_FIELDS
        .iter()
        .filter_map(|field| read_trimmed(&format!("/sys/class/dmi/id/{field}")).map(|value| (*field, value)))
        .collect()
}

/// The `PRETTY_NAME` from `/etc/os-release`, e.g. "Debian GNU/Linux 12 (bookworm)".
fn os_pretty_name() -> Option<String> {
    let os_release = read_trimmed("/etc/os-release")?;
    os_release.lines().find_map(|line| line.strip_prefix("PRETTY_NAME=")).map(|name| name.trim_matches('"').to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variables_that_look_secret_are_never_reported_but_platform_hints_are() {
        // Names are unique to this test; the environment is process-wide.
        std::env::set_var("FLY_REGION", "lax");
        std::env::set_var("FLY_API_TOKEN", "value-that-must-not-leak-1");
        std::env::set_var("RAILWAY_SECRET_KEY", "value-that-must-not-leak-2");
        std::env::set_var("SOME_UNRELATED_PASSWORD", "value-that-must-not-leak-3");

        let facts = collect();
        let everything = facts.to_string();
        assert_eq!(facts["platform_env"]["FLY_REGION"], "lax", "platform hints are shared with their values");
        for leaked in ["leak-1", "leak-2", "leak-3"] {
            assert!(!everything.contains(leaked), "a secret-looking value ({leaked}) was included in the facts");
        }
        let names = facts["other_env_names"].as_array().unwrap();
        assert!(
            names.iter().any(|name| name == "SOME_UNRELATED_PASSWORD"),
            "unrelated variables are reported by name only"
        );
    }

    #[test]
    fn prompts_get_only_the_identifying_signals() {
        let facts = json!({"hostname": "h", "kernel_version": "#1 SMP", "uptime_secs": "99", "other_env_names": ["X"], "dmi": {}});
        let signals = identifying_signals(&facts);
        assert_eq!(signals, json!({"hostname": "h", "dmi": {}}));
    }
}
