//! # YNAB MCP Server
//!
//! A high-performance Model Context Protocol (MCP) server providing sophisticated read-only
//! access to YNAB (You Need A Budget) financial data. This library enables AI agents to
//! analyze budgets, spending patterns, and financial health with advanced analytics capabilities.
//!
//! ## Features
//!
//! ### 🚀 Advanced Analytics Tools
//! - **Category Spending Analysis** - Deep insights into spending patterns by category
//! - **Budget Health Assessment** - Comprehensive scoring and optimization recommendations
//! - **Spending Trend Analysis** - Multi-month trend detection with growth indicators
//! - **Transaction Pattern Recognition** - Advanced filtering and search capabilities
//! - **Budget Overview** - Complete financial summaries with income/expense analysis
//!
//! ### ⚡ Performance Optimizations
//! - **Smart Caching** - In-memory response caching with configurable TTL
//! - **Request Batching** - Concurrent API requests for improved throughput
//! - **Connection Pooling** - Efficient HTTP client with persistent connections
//! - **Background Cache Cleanup** - Automatic expired entry management
//!
//! ### 🏗️ Architecture Excellence
//! - **Domain-Driven Design** - Clean separation of business logic and infrastructure
//! - **Test-Driven Development** - 101+ tests with >90% coverage requirement
//! - **MCP Protocol Compliance** - Full JSON-RPC 2.0 support for AI integration
//! - **Error Resilience** - Comprehensive error handling with detailed diagnostics
//!
//! ## Quick Start
//!
//! ```rust
//! use ynab_mcp::{YnabClient, server::Handler};
//! use serde_json::json;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create YNAB API client with caching
//! let client = YnabClient::new("your-api-token".to_string());
//!
//! // Set up MCP handler with full integration
//! let transaction_service = ynab_mcp::domain::TransactionService::new();
//! let handler = Handler::with_full_integration(transaction_service, client);
//!
//! // Execute advanced analytics
//! let health_check = handler.execute_tool(
//!     "budget_health_check",
//!     json!({"budget_id": "your-budget-id"})
//! )?;
//!
//! let spending_trends = handler.execute_tool(
//!     "analyze_spending_trends",
//!     json!({"budget_id": "your-budget-id", "months": 6})
//! )?;
//!
//! println!("Budget Health: {}", health_check);
//! println!("Spending Trends: {}", spending_trends);
//! # Ok(())
//! # }
//! ```
//!
//! ## Available Tools
//!
//! The server provides 5 sophisticated MCP tools:
//!
//! 1. **`analyze_category_spending`** - Category-specific spending analysis with date filtering
//! 2. **`get_budget_overview`** - Complete budget summary with income/expense breakdowns
//! 3. **`search_transactions`** - Advanced transaction search with filtering and sorting
//! 4. **`analyze_spending_trends`** - Multi-month trend analysis with category insights
//! 5. **`budget_health_check`** - Comprehensive health scoring with optimization suggestions
//!
//! ## Performance Features
//!
//! ### Caching
//! ```rust
//! use ynab_mcp::YnabClient;
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = YnabClient::new("token".to_string());
//!
//! // First request hits API
//! let data = client.get_json("/budgets").await?;
//!
//! // Second request uses cache (much faster)
//! let cached_data = client.get_json("/budgets").await?;
//!
//! // Cache management
//! client.clear_cache();
//! client.cleanup_cache(); // Remove expired entries
//! println!("Cache size: {}", client.cache_size());
//! # Ok(())
//! # }
//! ```
//!
//! ### Request Batching
//! ```rust
//! # use ynab_mcp::YnabClient;
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = YnabClient::new("token".to_string());
//!
//! // Batch multiple requests for better performance
//! let paths = vec!["/budgets", "/budgets/123/categories", "/budgets/123/transactions"];
//! let results = client.batch_requests(paths).await;
//!
//! // Convenience method for common budget data
//! let (budget, categories, transactions) = client.get_budget_batch("budget-id").await;
//! # Ok(())
//! # }
//! ```
//!
//! ## Domain Model
//!
//! The library provides rich domain entities for financial data modeling:
//!
//! - **[`Budget`]** - Budget container with metadata
//! - **[`Category`]** - Spending categories with optional grouping
//! - **[`Transaction`]** - Financial transactions with full details
//! - **[`Money`]** - Type-safe monetary amounts using milliunits
//! - **[`TransactionService`]** - Advanced querying and aggregation capabilities
//!
//! ## Development Principles
//!
//! This library follows strict software engineering practices:
//!
//! - **Test-Driven Development** - Every feature starts with a failing test
//! - **Domain-Driven Design** - Business logic separated from infrastructure concerns
//! - **Performance-First** - Caching, batching, and optimization built-in
//! - **AI-Agent Ready** - MCP protocol compliance for seamless AI integration
//! - **Production Quality** - Comprehensive error handling and diagnostics
//!
//! ## Error Handling
//!
//! All operations return [`YnabResult<T>`] for consistent error handling:
//!
//! ```rust
//! use ynab_mcp::{YnabClient, YnabError};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = YnabClient::new("invalid-token".to_string());
//!
//! match client.get_json("/budgets").await {
//!     Ok(data) => println!("Success: {}", data),
//!     Err(YnabError::HttpApiError(e)) => println!("Network error: {}", e),
//!     Err(YnabError::ApiError(msg)) => println!("API error: {}", msg),
//!     Err(e) => println!("Other error: {}", e),
//! }
//! # Ok(())
//! # }
//! ```

pub mod adapters;
pub mod domain;
pub mod server;

// Re-export key types for convenience
pub use adapters::YnabClient;
pub use domain::*;
