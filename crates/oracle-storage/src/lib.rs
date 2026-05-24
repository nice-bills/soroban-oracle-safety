//! Shared instance storage helpers for oracle adapter contracts.

#![no_std]

mod admin;
mod error;
mod keys;
mod source;
mod ttl;

pub use admin::{get_admin, has_admin, require_admin, set_admin};
pub use error::AdapterError;
pub use keys::{admin_key, source_key};
pub use source::{get_source, set_source};
pub use ttl::extend_instance;
