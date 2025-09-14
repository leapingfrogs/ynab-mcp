# YNAB MCP Server

A Model Context Protocol (MCP) server that provides Claude with read-only access to your [You Need A Budget (YNAB)](https://ynab.com) data for budget analysis and insights.

## Overview

This MCP server allows Claude to help you analyze your YNAB budget data, including:

- **Category spending analysis** - Track spending patterns across categories
- **Budget overview** - Get comprehensive budget summaries
- **Transaction search** - Find and filter transactions with advanced criteria
- **Spending trends** - Analyze spending patterns over time
- **Budget health checks** - Get insights on budget performance

All data access is **read-only** - the server cannot modify your YNAB budget.

## Prerequisites

- Rust 1.70+ installed
- A YNAB account with budget data
- YNAB Personal Access Token (see setup instructions below)

## Getting Your YNAB API Token

1. Log into your YNAB account at [app.ynab.com](https://app.ynab.com)
2. Go to **Account Settings** → **Developer Settings**
3. Click **New Token**
4. Give it a name (e.g., "Claude MCP Server")
5. Copy the generated token (keep it secure!)

## Installation & Setup

### 1. Clone and Build

```bash
git clone https://github.com/your-username/ynab-mcp.git
cd ynab-mcp
cargo build --release
```

### 2. Test the Server

```bash
# Set your YNAB API token
export YNAB_API_TOKEN="your_ynab_token_here"

# Test the server
cargo run
```

The server should start and wait for MCP protocol messages on stdin/stdout.

## Configuring with Claude Desktop

### Method 1: Using the Built Binary

Add this configuration to your Claude Desktop settings:

```json
{
  "mcpServers": {
    "ynab": {
      "command": "/path/to/ynab-mcp/target/release/ynab-mcp",
      "env": {
        "YNAB_API_TOKEN": "your_ynab_token_here"
      }
    }
  }
}
```

### Method 2: Using Cargo Run

If you prefer to run via Cargo:

```json
{
  "mcpServers": {
    "ynab": {
      "command": "cargo",
      "args": ["run", "--release", "--manifest-path", "/path/to/ynab-mcp/Cargo.toml"],
      "cwd": "/path/to/ynab-mcp",
      "env": {
        "YNAB_API_TOKEN": "your_ynab_token_here"
      }
    }
  }
}
```

**Important:** Replace `/path/to/ynab-mcp` with the actual path to your cloned repository and `your_ynab_token_here` with your actual YNAB API token.

## Available Tools

Once configured, Claude will have access to these YNAB analysis tools:

### `analyze_category_spending`
Analyze spending for specific categories with optional date filtering.

**Example:** "How much did I spend on groceries last month?"

### `get_budget_overview`
Get a comprehensive overview of your budget including categories, balances, and activity.

**Example:** "Give me an overview of my current budget status."

### `search_transactions`
Search and filter transactions with advanced criteria including amount ranges, categories, and text search.

**Example:** "Show me all transactions over $100 from last week."

### `analyze_spending_trends`
Analyze spending patterns and trends over time for better budget insights.

**Example:** "What are my spending trends for dining out over the past 3 months?"

### `budget_health_check`
Get insights and recommendations about your budget performance and areas for improvement.

**Example:** "How is my budget performing this month?"

## Usage Examples

After setup, you can ask Claude questions like:

- "What did I spend on groceries this month?"
- "Show me my largest transactions from last week"
- "How is my dining out budget performing?"
- "Give me a budget health check"
- "What are my spending trends for entertainment?"

## Security & Privacy

- **Read-only access:** The server can only read your YNAB data, never modify it
- **Local processing:** All analysis happens locally on your machine
- **Token security:** Your YNAB API token is stored locally in your Claude configuration
- **No data storage:** The server doesn't store or cache your financial data persistently

## Development

### Running Tests

```bash
cargo test
```

### Code Coverage

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
open tarpaulin-report.html
```

Current test coverage: **93.71%** (target: 95%)

### Code Quality

```bash
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt
```

## Project Structure

```
ynab-mcp/
├── src/
│   ├── main.rs              # Server entry point
│   ├── lib.rs               # Public API exports
│   ├── domain/              # Business logic & models
│   │   ├── budget.rs        # Budget entities
│   │   ├── category.rs      # Category & spending logic
│   │   ├── transaction.rs   # Transaction models
│   │   └── ...
│   ├── server/              # MCP server implementation
│   │   ├── handler.rs       # Tool handlers
│   │   ├── transport.rs     # MCP transport layer
│   │   └── ...
│   └── adapters/            # External integrations
│       └── ynab_client.rs   # YNAB API client
├── tests/                   # Integration tests
└── CLAUDE.md               # Development guidelines
```

## Contributing

This project follows strict Test-Driven Development (TDD) practices. Please see [CLAUDE.md](./CLAUDE.md) for detailed development guidelines including:

- TDD workflow (Red-Green-Refactor)
- Code coverage requirements (95% minimum)
- Code style and quality standards
- Testing best practices

## License

[MIT License](LICENSE)

## Troubleshooting

### "YNAB_API_TOKEN environment variable is required"
Make sure you've set your YNAB API token in the Claude configuration under the `env` section.

### "API request failed"
- Verify your YNAB API token is correct and hasn't expired
- Check your internet connection
- Ensure you have budgets created in your YNAB account

### Server not responding
- Try restarting Claude Desktop
- Check that the binary path in your configuration is correct
- Verify the server builds successfully with `cargo build --release`

For more issues, please check the [GitHub Issues](https://github.com/your-username/ynab-mcp/issues) page.