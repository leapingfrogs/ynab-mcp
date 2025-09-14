//! MCP server implementation layer.
//!
//! This module contains the Model Context Protocol server implementation,
//! including request handlers and server setup.

pub mod handler;
pub mod jsonrpc;
pub mod mcp_protocol;
pub mod transport;

pub use handler::*;
pub use jsonrpc::*;
pub use mcp_protocol::*;
pub use transport::*;

use crate::adapters::YnabClient;
use crate::domain::{TransactionService, YnabResult};
use std::io::{Read, Write};

/// Runs the complete MCP server session, processing messages from stdin and writing to stdout.
///
/// This is the main server runtime that ties together all components:
/// - Transport layer (Content-Length framed stdio)
/// - JSON-RPC message parsing
/// - MCP protocol handling
/// - Tool execution via Handler
///
/// # Arguments
/// * `reader` - Input stream (usually stdin)
/// * `writer` - Output stream (usually stdout)
/// * `api_token` - YNAB API token for client integration
pub fn run_mcp_server<R: Read, W: Write>(
    mut reader: R,
    mut writer: W,
    api_token: &str,
) -> YnabResult<()> {
    // Set up the complete MCP server stack
    let transaction_service = TransactionService::new();
    let ynab_client = YnabClient::new(api_token.to_string());
    let handler = Handler::with_full_integration(transaction_service, ynab_client);
    let mcp_server = McpServer::new(handler);

    // Server loop: read messages, process them, write responses
    loop {
        // Read incoming message with Content-Length framing
        let message = match read_message(&mut reader) {
            Ok(msg) => msg,
            Err(_) => break, // EOF or error, exit gracefully
        };

        // Parse JSON-RPC request
        let request = match JsonRpcRequest::from_json(&message) {
            Ok(req) => req,
            Err(e) => {
                // Send error response for malformed JSON-RPC
                let error_response = JsonRpcResponse::error(
                    serde_json::Value::Null,
                    -32700,
                    format!("Parse error: {}", e),
                    None,
                );
                let response_json = error_response.to_json();
                write_message(&mut writer, &response_json)?;
                continue;
            }
        };

        // Process request through MCP protocol layer
        let response = match mcp_server.handle_request(request) {
            Ok(resp) => resp,
            Err(e) => {
                // Send error response for MCP handling failure
                let error_response = JsonRpcResponse::error(
                    serde_json::Value::Null,
                    -32000,
                    format!("Server error: {}", e),
                    None,
                );
                error_response
            }
        };

        // Write response back with Content-Length framing
        let response_json = response.to_json();
        write_message(&mut writer, &response_json)?;
    }

    Ok(())
}
