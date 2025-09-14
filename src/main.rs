//! YNAB MCP Server binary entry point.

use ynab_mcp::server::Handler;

fn main() {
    let _handler = Handler::new();
    println!("YNAB MCP Server starting...");
    // Server implementation will be added in later iterations
}
