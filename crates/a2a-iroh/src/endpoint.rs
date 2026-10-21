//! Creating Iroh endpoints that use Datum's relays, and working out where to dial.
use std::env;

use anyhow::{anyhow, Result};
use iroh::{endpoint::presets, Endpoint, EndpointAddr, EndpointId, RelayMode, RelayUrl, SecretKey};
use iroh_tickets::endpoint::EndpointTicket;

/// Datum's Iroh relays, used unless the `RELAYS` environment variable (comma-separated URLs) says otherwise.
const DATUM_RELAYS: &str = "https://iroh-relay.us-west-1.datumconnect.net,\
https://iroh-relay.us-central-1.datumconnect.net,\
https://iroh-relay.us-east-1.datumconnect.net";

pub fn relay_urls() -> Result<Vec<RelayUrl>> {
    let configured = env::var("RELAYS").unwrap_or_else(|_| DATUM_RELAYS.to_string());
    configured.split(',').map(|url| url.trim().parse().map_err(|e| anyhow!("bad relay url {url}: {e}"))).collect()
}

/// Bind an endpoint that talks only to Datum's relays and uses no address lookup service, so
/// nothing contacts n0's default DNS or pkarr servers. The only way to find a peer is its endpoint id
/// (or ticket) plus relay hints; see [`target_from`].
pub async fn bind(key: SecretKey, alpns: Vec<Vec<u8>>) -> Result<Endpoint> {
    bind_with(key, alpns, RelayMode::custom(relay_urls()?)).await
}

/// Like [`bind`] with an explicit relay mode. Tests use `RelayMode::Disabled` and dial loopback addresses.
pub async fn bind_with(key: SecretKey, alpns: Vec<Vec<u8>>, relays: RelayMode) -> Result<Endpoint> {
    let endpoint = Endpoint::builder(presets::Minimal).secret_key(key).relay_mode(relays).alpns(alpns).bind().await?;
    Ok(endpoint)
}

/// Work out where to dial, from either a full ticket or a bare endpoint id.
///
/// Both forms end up as "this endpoint id, reachable via any of our relays". A ticket's own direct
/// addresses are dropped: they were only true when the ticket was made and go stale or private quickly.
pub fn target_from(ticket: Option<&str>, endpoint_id: Option<&str>) -> Result<EndpointAddr> {
    let addr = match (ticket, endpoint_id) {
        (Some(ticket), _) => {
            let ticket: EndpointTicket = ticket.trim().parse().map_err(|e| anyhow!("bad ticket: {e}"))?;
            let mut addr = ticket.endpoint_addr().clone();
            addr.addrs.retain(|a| a.is_relay());
            addr
        }
        (None, Some(id)) => {
            let id: EndpointId = id.trim().parse().map_err(|e| anyhow!("bad endpoint id: {e}"))?;
            EndpointAddr::new(id)
        }
        (None, None) => return Err(anyhow!("need an inside ticket or endpoint id")),
    };
    add_relay_hints(addr)
}

fn add_relay_hints(mut addr: EndpointAddr) -> Result<EndpointAddr> {
    for relay in relay_urls()? {
        addr = addr.with_relay_url(relay);
    }
    Ok(addr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_ticket_keeps_only_relay_hints_because_direct_addresses_go_stale() {
        let id = SecretKey::generate().public();
        let relay: RelayUrl = "https://relay.example./".parse().unwrap();
        let addr = EndpointAddr::new(id).with_relay_url(relay).with_ip_addr("10.1.2.3:4000".parse().unwrap());
        let ticket = EndpointTicket::new(addr).to_string();

        let target = target_from(Some(&ticket), None).unwrap();
        assert_eq!(target.id, id);
        assert!(target.addrs.iter().all(|a| a.is_relay()), "no stale direct addresses survive");
        assert!(target.addrs.len() >= relay_urls().unwrap().len());

        let bare = target_from(None, Some(&id.to_string())).unwrap();
        assert_eq!(bare.id, id);
        assert_eq!(bare.addrs.len(), relay_urls().unwrap().len(), "a bare id still gets every relay as a hint");

        assert!(target_from(None, None).is_err(), "with neither a ticket nor an id there is nothing to dial");
    }
}
