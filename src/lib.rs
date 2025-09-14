//! YNAB MCP Server
//!
//! A Model Context Protocol (MCP) server providing read-only access to YNAB budget data.
//! This library follows strict Test-Driven Development practices and emphasizes simplicity
//! and maintainability.

pub mod adapters;
pub mod domain;
pub mod server;

// Re-export key types for convenience
pub use adapters::YnabClient;
pub use domain::*;
