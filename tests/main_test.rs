//! Tests for main.rs functionality.

use std::process::Command;

#[test]
fn should_require_ynab_api_token_environment_variable() {
    // Test that the binary exits with error when YNAB_API_TOKEN is not set
    let output = Command::new("cargo")
        .args(["run", "--bin", "ynab-mcp"])
        .env_remove("YNAB_API_TOKEN") // Ensure it's not set
        .output()
        .expect("Failed to execute command");

    // Should exit with non-zero code
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("YNAB_API_TOKEN environment variable is required"));
    assert!(stderr.contains("Please set it with: export YNAB_API_TOKEN=your_token_here"));
}

#[test]
fn should_reject_empty_ynab_api_token() {
    // Test that the binary exits with error when YNAB_API_TOKEN is empty
    let output = Command::new("cargo")
        .args(["run", "--bin", "ynab-mcp"])
        .env("YNAB_API_TOKEN", "") // Set to empty string
        .output()
        .expect("Failed to execute command");

    // Should exit with non-zero code
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("YNAB_API_TOKEN environment variable is required"));
    assert!(stderr.contains("Please set it with: export YNAB_API_TOKEN=your_token_here"));
}

#[test]
fn should_reject_whitespace_only_ynab_api_token() {
    // Test that the binary exits with error when YNAB_API_TOKEN is only whitespace
    let output = Command::new("cargo")
        .args(["run", "--bin", "ynab-mcp"])
        .env("YNAB_API_TOKEN", "   ") // Set to whitespace
        .output()
        .expect("Failed to execute command");

    // Should exit with non-zero code
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("YNAB_API_TOKEN environment variable is required"));
    assert!(stderr.contains("Please set it with: export YNAB_API_TOKEN=your_token_here"));
}

#[test]
fn should_display_comprehensive_error_messages() {
    // Test that all error message parts are displayed when API token is missing
    let output = Command::new("cargo")
        .args(["run", "--bin", "ynab-mcp"])
        .env_remove("YNAB_API_TOKEN") // Ensure it's not set
        .output()
        .expect("Failed to execute command");

    // Should exit with non-zero code
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    // Test both parts of the error message to ensure lines 13-14 are covered
    assert!(stderr.contains("Error: YNAB_API_TOKEN environment variable is required"));
    assert!(stderr.contains("Please set it with: export YNAB_API_TOKEN=your_token_here"));
}

#[test]
fn should_start_server_with_valid_token() {
    // Test that the binary starts and can handle a simple request with valid token
    use std::io::Write;
    use std::process::Stdio;
    use std::thread;
    use std::time::Duration;

    let mut child = Command::new("cargo")
        .args(["run", "--bin", "ynab-mcp"])
        .env("YNAB_API_TOKEN", "test-token-123")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start server");

    // Give it a moment to start
    thread::sleep(Duration::from_millis(100));

    // Send a simple request to trigger server error (since token is fake)
    if let Some(stdin) = child.stdin.as_mut() {
        let request = r#"{"jsonrpc":"2.0","method":"tools/list","id":1}"#;
        let message = format!("Content-Length: {}\r\n\r\n{}", request.len(), request);
        stdin.write_all(message.as_bytes()).ok();
    }

    // Give it time to process
    thread::sleep(Duration::from_millis(200));

    // Kill the process since it would run indefinitely
    child.kill().expect("Failed to kill server process");

    let output = child.wait_with_output().expect("Failed to get output");

    // Should have started successfully (even though it would fail later with fake token)
    // The important thing is it didn't exit immediately with token validation error
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should not contain the "YNAB_API_TOKEN environment variable is required" message
    // since we provided a non-empty token
    assert!(!stderr.contains("YNAB_API_TOKEN environment variable is required"));
}
