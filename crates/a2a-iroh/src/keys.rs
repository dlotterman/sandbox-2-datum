//! Endpoint secret keys: read one from the environment or a file, or make a fresh one.
use std::{env, path::Path};

use anyhow::{anyhow, Result};
use iroh::SecretKey;

/// Find this process's secret key, trying in order:
///
/// 1. the environment variable `hex_env`, holding the key as 64 hex characters;
/// 2. the file named by the environment variable `file_env`, holding the same hex. If that file does not
///    exist yet, a new key is generated and written there so the identity survives restarts;
/// 3. a fresh random key. The endpoint id then changes on every start, so nothing can be allowlisted.
pub fn load_secret_key(hex_env: &str, file_env: &str) -> Result<SecretKey> {
    if let Ok(hex) = env::var(hex_env) {
        return parse_secret_key(&hex);
    }
    if let Ok(path) = env::var(file_env) {
        let path = Path::new(&path);
        if path.exists() {
            return parse_secret_key(&std::fs::read_to_string(path)?);
        }
        let key = SecretKey::generate();
        std::fs::write(path, hex_encode(&key.to_bytes()))?;
        return Ok(key);
    }
    Ok(SecretKey::generate())
}

/// Parse a secret key from hex. Surrounding whitespace is ignored, because secret stores often add a newline.
pub fn parse_secret_key(hex: &str) -> Result<SecretKey> {
    let bytes = hex_decode(hex.trim())?;
    let key_bytes: [u8; 32] = bytes.try_into().map_err(|_| anyhow!("secret key must be 32 bytes"))?;
    Ok(SecretKey::from_bytes(&key_bytes))
}

/// Print a new identity as `SECRET=<hex>` and `ENDPOINT_ID=<hex>`, for the `keygen` command of both agents.
/// The secret goes in a secret store; the endpoint id is public and goes on the other side's allowlist.
pub fn print_new_identity() {
    let key = SecretKey::generate();
    println!("SECRET={}", hex_encode(&key.to_bytes()));
    println!("ENDPOINT_ID={}", key.public());
}

// Hex is hand-rolled here, rather than adding a dependency, because it is all this crate needs.

pub fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn hex_decode(text: &str) -> Result<Vec<u8>> {
    if !text.len().is_multiple_of(2) {
        return Err(anyhow!("odd-length hex"));
    }
    (0..text.len())
        .step_by(2)
        .map(|start| u8::from_str_radix(&text[start..start + 2], 16).map_err(|e| anyhow!("bad hex: {e}")))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_key_survives_a_trip_through_hex_even_with_a_trailing_newline() {
        let key = SecretKey::generate();
        let hex = hex_encode(&key.to_bytes());
        assert_eq!(hex.len(), 64);
        assert_eq!(parse_secret_key(&format!("{hex}\n")).unwrap().public(), key.public());
    }

    #[test]
    fn malformed_keys_are_rejected_rather_than_replaced() {
        for bad in ["", "abc", "zz".repeat(32).as_str(), "00".repeat(31).as_str()] {
            assert!(parse_secret_key(bad).is_err(), "{bad:?} should not parse");
        }
    }
}
