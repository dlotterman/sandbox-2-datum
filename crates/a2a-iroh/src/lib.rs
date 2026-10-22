//! A2A (JSON-RPC over HTTP/1.1) carried on Iroh streams, brokered by Datum's Iroh relays.
//!
//! One Iroh connection links a pair of peers; every HTTP request/response exchange gets its own bi-directional
//! stream on it, under the ALPN [`ALPN`]. Peers are identified by their Iroh endpoint id (a public key), so the
//! server always knows who is calling without any credentials in the request.
//!
//! - [`keys`]: loading or generating the secret key that gives an endpoint its identity.
//! - [`endpoint`]: binding an endpoint to Datum's relays, and turning an id or ticket into a dial target.
//! - [`server`]: accepting connections from allowlisted peers and serving HTTP over their streams.
//! - [`client`]: dialing a peer and sending requests, with one reconnect-and-retry.
//! - [`a2a`]: building and reading the A2A messages.
//! - [`ip`]: the public-address check shared by both agents.
mod a2a;
mod client;
mod endpoint;
mod ip;
mod keys;
mod server;

pub use a2a::{message_send, new_id, parse_task, rpc_error, task_result, TaskOutcome, ENROLL_SKILL, NOT_ENROLLED};
pub use client::Client;
pub use endpoint::{bind, bind_with, relay_urls, target_from};
pub use ip::is_global_ip;
pub use keys::{hex_encode, load_secret_key, parse_secret_key, print_new_identity};
pub use server::{serve, AllowFn, Peer};

/// Re-exported so the agents use exactly the Iroh version this crate was built against.
pub use iroh;
pub use iroh_tickets::endpoint::EndpointTicket;

/// The application protocol name negotiated when two peers connect.
pub const ALPN: &[u8] = b"a2a-iroh/http1/0";

/// The largest request body the servers should accept.
pub const MAX_REQUEST_BYTES: usize = 64 * 1024;
