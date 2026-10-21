//! Is an IP address one that could be reached on the public internet?
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// True for publicly routable addresses; false for private, loopback, link-local, carrier-grade NAT,
/// unique-local, documentation, multicast and similar reserved ranges.
///
/// Used both to pick this agent's public IP out of its Iroh addresses and to keep the inside agent from
/// being asked to look up (or record) an address that only means something on a private network.
pub fn is_global_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => is_global_v4(v4),
        IpAddr::V6(v6) => match v6.to_ipv4_mapped() {
            Some(mapped) => is_global_v4(&mapped),
            None => is_global_v6(v6),
        },
    }
}

fn is_global_v4(ip: &Ipv4Addr) -> bool {
    let [a, b, c, _] = ip.octets();
    let is_carrier_grade_nat = a == 100 && (64..=127).contains(&b); // 100.64.0.0/10
    let is_ietf_protocol_block = a == 192 && b == 0 && c == 0; // 192.0.0.0/24
    let is_reserved = a >= 240; // 240.0.0.0/4, includes broadcast

    !(ip.is_private()
        || ip.is_loopback()
        || ip.is_link_local()
        || ip.is_broadcast()
        || ip.is_documentation()
        || ip.is_unspecified()
        || ip.is_multicast()
        || is_carrier_grade_nat
        || is_ietf_protocol_block
        || is_reserved)
}

fn is_global_v6(ip: &Ipv6Addr) -> bool {
    let first = ip.segments()[0];
    let second = ip.segments()[1];
    let is_unique_local = (first & 0xfe00) == 0xfc00; // fc00::/7
    let is_link_local = (first & 0xffc0) == 0xfe80; // fe80::/10
    let is_documentation = first == 0x2001 && second == 0x0db8; // 2001:db8::/32

    !(ip.is_loopback()
        || ip.is_unspecified()
        || ip.is_multicast()
        || is_unique_local
        || is_link_local
        || is_documentation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_publicly_routable_addresses_count_as_global() {
        for ip in ["8.8.8.8", "2606:4700:4700::1111", "1.1.1.1"] {
            assert!(is_global_ip(&ip.parse().unwrap()), "{ip} should be global");
        }
        for ip in [
            "10.0.0.1",
            "192.168.1.1",
            "172.16.0.9",
            "127.0.0.1",
            "169.254.1.1",
            "100.64.0.1",
            "0.0.0.0",
            "224.0.0.1",
            "::1",
            "fd20:0:3f::1",
            "fe80::1",
            "2001:db8::1",
            "::ffff:10.0.0.1",
        ] {
            assert!(!is_global_ip(&ip.parse().unwrap()), "{ip} must not count as global");
        }
    }
}
