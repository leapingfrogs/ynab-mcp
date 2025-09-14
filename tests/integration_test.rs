//! Integration tests for the complete MCP server runtime.

use std::io::Cursor;
use ynab_mcp::server::run_mcp_server;

#[test]
fn should_run_complete_mcp_server_session() {
    // Test 1: Initialize request
    let init_message = r#"{"jsonrpc":"2.0","method":"initialize","id":1,"params":{"protocolVersion":"2024-11-05"}}"#;
    let input = format!(
        "Content-Length: {}\r\n\r\n{}",
        init_message.len(),
        init_message
    );

    let mut stdin = Cursor::new(input);
    let mut stdout = Vec::new();

    run_mcp_server(&mut stdin, &mut stdout, "test-api-token").unwrap();

    let output = String::from_utf8(stdout).unwrap();
    println!("Initialize response: {}", output);

    // Should contain initialize response
    assert!(output.contains("Content-Length:"));
    assert!(output.contains("protocolVersion"));
    assert!(output.contains("2024-11-05"));
    assert!(output.contains("capabilities"));
    assert!(output.contains("serverInfo"));
}

#[test]
fn should_handle_tools_list_request() {
    // Test 2: Tools list request
    let tools_message = r#"{"jsonrpc":"2.0","method":"tools/list","id":2}"#;
    let input = format!(
        "Content-Length: {}\r\n\r\n{}",
        tools_message.len(),
        tools_message
    );

    let mut stdin = Cursor::new(input);
    let mut stdout = Vec::new();

    run_mcp_server(&mut stdin, &mut stdout, "test-api-token").unwrap();

    let output = String::from_utf8(stdout).unwrap();
    println!("Tools list response: {}", output);

    // Should contain tools list with our 5 analytical tools
    assert!(output.contains("Content-Length:"));
    assert!(output.contains("tools"));
    assert!(output.contains("analyze_category_spending"));
    assert!(output.contains("get_budget_overview"));
    assert!(output.contains("search_transactions"));
    assert!(output.contains("analyze_spending_trends"));
    assert!(output.contains("budget_health_check"));
}

#[test]
fn should_handle_tools_call_request() {
    // Test 3: Tools call request
    let call_message = r#"{"jsonrpc":"2.0","method":"tools/call","id":3,"params":{"name":"analyze_category_spending","arguments":{"budget_id":"test-budget","category_name":"Groceries"}}}"#;
    let input = format!(
        "Content-Length: {}\r\n\r\n{}",
        call_message.len(),
        call_message
    );

    let mut stdin = Cursor::new(input);
    let mut stdout = Vec::new();

    run_mcp_server(&mut stdin, &mut stdout, "test-api-token").unwrap();

    let output = String::from_utf8(stdout).unwrap();
    println!("Tools call response: {}", output);

    // Should contain tools call response with content
    assert!(output.contains("Content-Length:"));
    assert!(output.contains("content"));
    assert!(output.contains("type"));
    assert!(output.contains("text"));
}
