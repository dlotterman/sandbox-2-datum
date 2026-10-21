//! Everything the inside agent keeps or serves: the Postgres database, reference data, the rclone file
//! store, and the registry of agents that have called.
//!
//! - [`postgres`]: connecting, the schema, seed data, and the audit log.
//! - [`datasets`]: the `data.catalog` and `data.query` skills over the reference table.
//! - [`files`]: the `files.get` skill over the rclone WebDAV store.
//! - [`registry`]: who has called recently, and the `agents.sync` skill.
mod datasets;
mod files;
mod postgres;
mod registry;

pub use datasets::{catalog, query};
pub use files::file_get;
pub use postgres::{init, record_audit};
pub use registry::{sync, touch};
