//! Adapters layer for external integrations.
//!
//! This module contains adapters for external services and APIs,
//! including the YNAB API client.

pub mod ynab_client;

pub use ynab_client::*;
