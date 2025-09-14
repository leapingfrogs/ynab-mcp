//! Domain layer containing business logic and entities.
//!
//! This module contains the core business logic of the YNAB MCP server,
//! including entities like Budget, Category, and value objects like Money.

pub mod budget;
pub mod category;
pub mod money;

pub use budget::*;
pub use category::*;
pub use money::*;
