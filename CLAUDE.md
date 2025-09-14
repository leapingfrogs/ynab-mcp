# CLAUDE.md - Test-Driven Development Guide for YNAB MCP Server

## Project Overview

This is a Rust-based MCP (Model Context Protocol) server for YNAB (You Need A Budget) that provides read-only access to budget data. The project follows **strict Test-Driven Development (TDD)** practices and emphasizes code simplicity, maintainability, and Rust best practices.

## Core Development Principles

### 1. Test-Driven Development (TDD) is MANDATORY

Every piece of functionality MUST follow the Red-Green-Refactor cycle:

1. **RED**: Write a failing test first
2. **GREEN**: Write the minimal code to make the test pass
3. **REFACTOR**: Improve the code while keeping tests green

**NEVER write production code without a failing test first.**

### 2. Simplicity Over Cleverness

- Write code that a junior developer can understand
- Avoid complex abstractions unless absolutely necessary
- Use descriptive variable and function names
- Keep functions small (under 20 lines ideally)
- One responsibility per function/struct

### 3. Incremental Development

- Make small, atomic commits
- Each commit should have all tests passing
- Commit message format: `test: add failing test for X` or `feat: implement X to pass test`

## Development Workflow

### Standard TDD Cycle

```bash
# 1. Write a failing test
# Edit test file first
vim src/domain/category.rs  # Add test in #[cfg(test)] module

# 2. Run test to see it fail (RED)
cargo test category::tests::test_name -- --nocapture

# 3. Write minimal implementation
# Edit implementation
vim src/domain/category.rs  # Add just enough code

# 4. Run test to see it pass (GREEN)
cargo test category::tests::test_name

# 5. Run all tests to ensure nothing broke
cargo test

# 6. Refactor if needed (keep tests green)
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt

# 7. Commit when all tests pass
git add -A
git commit -m "feat: implement category spending calculation"
```

### Before Every Commit Checklist

Run this command sequence before EVERY commit:

```bash
# Format code
cargo fmt

# Check for issues
cargo clippy --all-targets --all-features -- -D warnings

# Run all tests
cargo test

# Check test coverage (MUST be ≥90%)
cargo tarpaulin --out Html
# Review tarpaulin-report.html and ensure coverage ≥90%
# If coverage is below 90%, write additional tests before committing

# Check documentation
cargo doc --no-deps --document-private-items

# If all pass AND coverage ≥90%, commit
git add -A && git commit
```

### Code Coverage Requirements

- **Minimum coverage: 90%**
- Coverage is checked using `cargo tarpaulin`
- HTML reports generated in `tarpaulin-report.html`
- **DO NOT commit if coverage falls below 90%**
- Focus on testing business logic in the `domain` module
- Server handlers and adapters should also maintain high coverage
- Main.rs and simple constructors may have lower coverage if justified

## Code Style Guidelines

### Testing Standards

```rust
// GOOD: Descriptive test names that explain the scenario
#[test]
fn should_calculate_category_spending_for_current_month() {
    // Arrange
    let category = create_test_category();
    let transactions = vec![create_test_transaction(500)];
    
    // Act
    let spending = category.calculate_spending(&transactions);
    
    // Assert
    assert_eq!(spending, Money::from_milliunits(500));
}

// BAD: Vague test names
#[test]
fn test_spending() {
    // Don't do this
}
```

### Implementation Standards

```rust
// GOOD: Simple, single-purpose function
pub fn calculate_category_spending(
    transactions: &[Transaction],
    category_id: &str,
) -> Money {
    transactions
        .iter()
        .filter(|t| t.category_id() == category_id)
        .map(|t| t.amount())
        .sum()
}

// BAD: Complex, multi-purpose function
pub fn process_budget_data(data: &str) -> Result<(Vec<Transaction>, Vec<Category>, Money)> {
    // Don't create functions that do too many things
}
```

### Error Handling

```rust
// GOOD: Use thiserror for domain errors
#[derive(Debug, thiserror::Error)]
pub enum YnabError {
    #[error("Invalid budget ID: {0}")]
    InvalidBudgetId(String),
    
    #[error("API request failed: {0}")]
    ApiError(#[from] reqwest::Error),
}

// Use Result<T, YnabError> for all fallible operations
```

## Project Structure

```
ynab-mcp-server/
├── src/
│   ├── lib.rs              # Public API exports
│   ├── main.rs             # Minimal binary entry point
│   ├── domain/             # Business logic (test first!)
│   │   ├── mod.rs
│   │   ├── budget.rs       # Budget entity
│   │   ├── category.rs     # Category entity
│   │   └── money.rs        # Money value object
│   ├── server/             # MCP server implementation
│   │   ├── mod.rs
│   │   └── handler.rs      # Request handlers
│   └── adapters/           # External integrations
│       ├── mod.rs
│       └── ynab_client.rs  # YNAB API client
└── tests/                  # Integration tests
    └── integration/
```

## Iteration-Specific Guidelines

### Iteration 0: Project Setup
- Set up GitHub Actions CI/CD
- Configure development tools
- Create initial project structure
- NO production code yet!

### Iteration 1: Category Spending (COMPLETED)
✅ Implemented category spending analysis:
- Domain model for `Category`, `Transaction`, `Money`
- Spending calculation with date filtering
- Transaction querying and filtering system
- TransactionService for aggregations
- 92.75% test coverage achieved

### Iteration 2: Domain Model Completion (CURRENT)
Focus on completing missing domain entities and error handling:

**Phase 2.1: Missing Domain Entities**
1. Add `Payee` entity with full TDD
2. Add `Account` entity with account types
3. Add `CategoryGroup` for organizing categories
4. Enhance `Transaction` with payee_id, account_id fields

**Phase 2.2: Error Handling & Validation**
1. Implement `YnabError` enum with thiserror
2. Add input validation functions
3. Add Result<T, YnabError> return types
4. Test all error conditions

Example TDD sequence:
```rust
// 1. Test Payee creation and validation
// 2. Test Account creation with types
// 3. Test enhanced Transaction with all fields
// 4. Test error conditions and validation
// 5. Test CategoryGroup organization
```

### Iteration 3: YNAB API Integration
Implementation of HTTP client and API integration:

**Phase 3.1: HTTP Client Foundation**
1. Add reqwest dependency when tests require it
2. Implement YnabClient with authentication
3. Add base URL configuration and error handling

**Phase 3.2: API Endpoint Implementation**
1. GET /budgets - Fetch budget list
2. GET /budgets/{budget_id} - Fetch budget details
3. GET /budgets/{budget_id}/categories - Fetch categories
4. GET /budgets/{budget_id}/transactions - Fetch transactions
5. GET /budgets/{budget_id}/payees - Fetch payees

**Phase 3.3: Data Mapping Layer**
1. Convert YNAB API responses to domain models
2. Handle API rate limiting and error responses
3. Add comprehensive integration tests with mocks

### Iteration 4: MCP Server Implementation
Build the Model Context Protocol server:

**Phase 4.1: MCP Protocol Foundation**
1. Add MCP protocol message handling
2. Implement tool discovery and registration
3. Add JSON-RPC communication layer

**Phase 4.2: Budget Analysis Tools**
1. `analyze_category_spending` - Category spending analysis
2. `get_budget_overview` - Complete budget summary
3. `search_transactions` - Advanced transaction search
4. `compare_periods` - Period-over-period comparisons

**Phase 4.3: Tool Integration**
1. Connect tools to YNAB API client
2. Add proper error handling and validation
3. Implement response formatting for AI consumption

### Iteration 5: Advanced Features & Polish
Advanced analytics and optimization:

**Phase 5.1: Advanced Analytics**
1. `analyze_spending_trends` - Multi-month trend analysis
2. `budget_health_check` - Budget optimization suggestions
3. `category_insights` - Category performance analysis
4. `transaction_patterns` - Spending pattern detection

**Phase 5.2: Performance & Polish**
1. Add caching for API responses
2. Implement request batching
3. Add comprehensive documentation
4. Performance profiling and optimization

## Dependencies Management

Only add dependencies when a TEST requires it:

```toml
# In Cargo.toml
[dependencies]
# Add ONLY when implementation needs it

[dev-dependencies]
# Add when tests need it
mockall = "0.13"  # Added when you write first mock test
```

## Documentation Requirements

Every public item MUST have:
1. A doc comment explaining what it does
2. An example in the doc comment
3. Clear parameter descriptions

```rust
/// Calculates the total spending for a category within a date range.
///
/// # Arguments
/// * `transactions` - List of transactions to analyze
/// * `category_id` - The category to calculate spending for
/// * `date_range` - Optional date range filter
///
/// # Example
/// ```
/// let spending = calculate_category_spending(&transactions, "groceries", None);
/// assert_eq!(spending.as_milliunits(), 50_000);
/// ```
pub fn calculate_category_spending(
    transactions: &[Transaction],
    category_id: &str,
    date_range: Option<DateRange>,
) -> Money {
    // Implementation
}
```

## Common Pitfalls to Avoid

1. **Writing code without tests** - Always write the test first
2. **Over-engineering** - Start simple, refactor when needed
3. **Large commits** - Keep commits small and focused
4. **Skipping clippy warnings** - Fix all warnings before committing
5. **Missing documentation** - Document as you go

## Quick Reference Commands

```bash
# Run specific test
cargo test test_name -- --nocapture

# Run tests in watch mode
cargo watch -x test

# Check test coverage (HTML report)
cargo tarpaulin --out Html

# Check test coverage (terminal output)
cargo tarpaulin

# Check test coverage with exclusions
cargo tarpaulin --exclude-files src/main.rs --exclude-files src/adapters/ynab_client.rs

# Run benchmarks
cargo bench

# Update dependencies
cargo update --dry-run  # Check first
cargo update           # Then update
```

## When You're Stuck

1. Check if you're following TDD (did you write a test first?)
2. Is the test testing one specific behavior?
3. Is your implementation the simplest possible?
4. Are all previous tests still passing?
5. Have you run clippy and fixed warnings?

## Remember

- **Quality over speed** - Take time to write good tests
- **Small steps** - Each TDD cycle should be completable in 10-15 minutes
- **Refactor regularly** - But only when tests are green
- **Ask for clarification** - If requirements are unclear, ask before coding

This project values maintainability and correctness over performance optimization or clever solutions. When in doubt, choose the simpler approach.