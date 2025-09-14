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
