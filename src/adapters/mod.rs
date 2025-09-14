//! Adapters layer for external integrations.
//!
//! This module contains adapters for external services and APIs,
//! including the YNAB API client.

pub mod response_mapper;
pub mod ynab_client;

pub use response_mapper::*;
pub use ynab_client::*;
