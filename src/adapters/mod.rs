//! Adapters layer for external integrations.
//!
//! This module contains adapters for external services and APIs,
//! including the YNAB API client and caching mechanisms.

pub mod cache;
pub mod response_mapper;
pub mod ynab_client;

pub use cache::*;
pub use response_mapper::*;
pub use ynab_client::*;
