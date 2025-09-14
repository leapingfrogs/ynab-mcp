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
                JsonRpcResponse::error(
                    serde_json::Value::Null,
                    -32000,
                    format!("Server error: {}", e),
                    None,
                )
            }
        };

        // Write response back with Content-Length framing
        let response_json = response.to_json();
        write_message(&mut writer, &response_json)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn should_handle_malformed_json_in_server_loop() {
        let malformed_input = "Content-Length: 15\r\n\r\n{\"invalid\":json";
        let mut stdin = Cursor::new(malformed_input);
        let mut stdout = Vec::new();

        run_mcp_server(&mut stdin, &mut stdout, "test-token").unwrap();

        let output = String::from_utf8(stdout).unwrap();
        assert!(output.contains("Content-Length:"));
        assert!(output.contains("Parse error"));
        assert!(output.contains("-32700"));
    }

    #[test]
    fn should_handle_empty_input_gracefully() {
        let empty_input = "";
        let mut stdin = Cursor::new(empty_input);
        let mut stdout = Vec::new();

        let result = run_mcp_server(&mut stdin, &mut stdout, "test-token");
        assert!(result.is_ok());

        let output = String::from_utf8(stdout).unwrap();
        assert!(output.is_empty()); // No input, no output
    }

    #[test]
    fn should_handle_server_error_during_request_processing() {
        // Test malformed MCP request that causes server error
        let invalid_mcp_request = r#"{"jsonrpc":"2.0","method":"tools/call","id":1}"#; // Missing params
        let input = format!(
            "Content-Length: {}\r\n\r\n{}",
            invalid_mcp_request.len(),
            invalid_mcp_request
        );
        let mut stdin = Cursor::new(input);
        let mut stdout = Vec::new();

        run_mcp_server(&mut stdin, &mut stdout, "test-token").unwrap();

        let output = String::from_utf8(stdout).unwrap();
        assert!(output.contains("Content-Length:"));
        assert!(output.contains("Server error"));
        assert!(output.contains("-32000"));
    }
}
