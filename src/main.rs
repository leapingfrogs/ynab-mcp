//! YNAB MCP Server binary entry point.

use std::io::{stdin, stdout};
use std::env;
use ynab_mcp::server::run_mcp_server;

fn main() {
    // Get YNAB API token from environment variable
    let api_token = match env::var("YNAB_API_TOKEN") {
        Ok(token) if !token.trim().is_empty() => token,
        _ => {
            eprintln!("Error: YNAB_API_TOKEN environment variable is required");
            eprintln!("Please set it with: export YNAB_API_TOKEN=your_token_here");
            std::process::exit(1);
        }
    };

    // Run the complete MCP server with stdin/stdout
    if let Err(e) = run_mcp_server(stdin(), stdout(), &api_token) {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}
