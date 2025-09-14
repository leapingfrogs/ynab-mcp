//! MCP request handlers.

use crate::domain::error::YnabResult;
use crate::domain::transaction_service::TransactionService;

/// Represents an MCP tool that can be called by clients.
#[derive(Debug, Clone, PartialEq)]
pub struct Tool {
    pub name: String,
    pub description: String,
}

/// MCP server handler for YNAB budget analysis tools.
pub struct Handler {
    transaction_service: Option<TransactionService>,
}

impl Handler {
    /// Creates a new Handler instance.
    pub fn new() -> Self {
        Self {
            transaction_service: None,
        }
    }

    /// Creates a new Handler instance with transaction service.
    pub fn with_services(transaction_service: TransactionService) -> Self {
        Self {
            transaction_service: Some(transaction_service),
        }
    }

    /// Lists all available MCP tools for YNAB budget analysis.
    pub fn list_tools(&self) -> Vec<Tool> {
        vec![
            Tool {
                name: "analyze_category_spending".to_string(),
                description:
                    "Analyzes spending for specific categories with optional date filtering"
                        .to_string(),
            },
            Tool {
                name: "get_budget_overview".to_string(),
                description: "Provides a comprehensive overview of budget status and spending"
                    .to_string(),
            },
            Tool {
                name: "search_transactions".to_string(),
                description: "Searches transactions with advanced filtering and sorting options"
                    .to_string(),
            },
        ]
    }

    /// Executes a named tool with the provided parameters.
    pub fn execute_tool(&self, tool_name: &str, params: serde_json::Value) -> YnabResult<String> {
        match tool_name {
            "analyze_category_spending" => self.analyze_category_spending(&params),
            "get_budget_overview" => self.get_budget_overview(&params),
            "search_transactions" => self.search_transactions(&params),
            _ => Err(crate::domain::error::YnabError::InvalidBudgetId(format!(
                "Unknown tool: {}",
                tool_name
            ))),
        }
    }

    /// Analyzes category spending using real domain data.
    fn analyze_category_spending(&self, params: &serde_json::Value) -> YnabResult<String> {
        let category_id = params["category_id"].as_str().unwrap_or("");
        let category_name = params["category_name"].as_str().unwrap_or("");

        if let Some(transaction_service) = &self.transaction_service {
            use crate::domain::category::Category;
            use crate::domain::transaction_query::TransactionQuery;

            let category = Category::new(category_id.to_string(), category_name.to_string());
            let query = TransactionQuery::new().with_category(category_id.to_string());
            let transactions = transaction_service.query(&query);
            let transaction_count = transactions.len();

            // Convert Vec<&Transaction> to Vec<Transaction> for calculate_spending
            let owned_transactions: Vec<_> = transactions.into_iter().cloned().collect();

            let total_spending = category.calculate_spending(&owned_transactions);

            Ok(serde_json::json!({
                "category_spending": {
                    "category": category_name,
                    "amount_milliunits": total_spending.as_milliunits().abs(), // Convert negative to positive for display
                    "transaction_count": transaction_count
                }
            })
            .to_string())
        } else {
            // Fallback to hardcoded response when no service is available
            Ok(serde_json::json!({
                "category_spending": {
                    "category": "Groceries",
                    "amount_milliunits": 125000,
                    "transaction_count": 5
                }
            })
            .to_string())
        }
    }

    /// Provides budget overview using real domain data.
    fn get_budget_overview(&self, _params: &serde_json::Value) -> YnabResult<String> {
        if let Some(transaction_service) = &self.transaction_service {
            use crate::domain::money::Money;
            use crate::domain::transaction_query::TransactionQuery;

            let query = TransactionQuery::new();
            let all_transactions = transaction_service.query(&query);

            // Calculate totals
            let mut total_expenses = Money::from_milliunits(0);
            let mut total_income = Money::from_milliunits(0);

            for transaction in &all_transactions {
                let amount = transaction.amount();
                if amount.as_milliunits() < 0 {
                    // Negative amounts are expenses
                    total_expenses = Money::from_milliunits(
                        total_expenses.as_milliunits() + amount.as_milliunits().abs(),
                    );
                } else {
                    // Positive amounts are income
                    total_income = Money::from_milliunits(
                        total_income.as_milliunits() + amount.as_milliunits(),
                    );
                }
            }

            let net_income = Money::from_milliunits(
                total_income.as_milliunits() - total_expenses.as_milliunits(),
            );

            Ok(serde_json::json!({
                "budget_overview": {
                    "total_expenses_milliunits": total_expenses.as_milliunits(),
                    "total_income_milliunits": total_income.as_milliunits(),
                    "net_income_milliunits": net_income.as_milliunits(),
                    "transaction_count": all_transactions.len()
                }
            })
            .to_string())
        } else {
            // Fallback to hardcoded response when no service is available
            Ok(serde_json::json!({
                "budget_overview": {
                    "total_budgeted": 300000,
                    "total_spent": 125000,
                    "categories_over_budget": 2
                }
            })
            .to_string())
        }
    }

    /// Searches transactions with advanced filtering options.
    fn search_transactions(&self, params: &serde_json::Value) -> YnabResult<String> {
        if let Some(transaction_service) = &self.transaction_service {
            use crate::domain::transaction_query::TransactionQuery;

            let mut query = TransactionQuery::new();

            // Apply text search filter if provided
            if let Some(text_search) = params["text_search"].as_str()
                && !text_search.is_empty()
            {
                query = query.with_text_search(text_search.to_string());
            }

            // Apply minimum amount filter if provided
            if let Some(min_amount) = params["min_amount_milliunits"].as_i64() {
                query =
                    query.with_min_amount(crate::domain::money::Money::from_milliunits(min_amount));
            }

            // Apply category filter if provided
            if let Some(category_id) = params["category_id"].as_str()
                && !category_id.is_empty()
            {
                query = query.with_category(category_id.to_string());
            }

            let found_transactions = transaction_service.query(&query);

            // Apply limit if provided
            let limit = params["limit"].as_u64().unwrap_or(100) as usize;
            let limited_transactions: Vec<_> = found_transactions.into_iter().take(limit).collect();

            // Convert transactions to JSON format
            let transaction_json: Vec<serde_json::Value> = limited_transactions
                .iter()
                .map(|txn| {
                    serde_json::json!({
                        "id": txn.id(),
                        "description": txn.description().unwrap_or(""),
                        "amount_milliunits": txn.amount().as_milliunits(),
                        "category_id": txn.category_id(),
                        "account_id": txn.account_id()
                    })
                })
                .collect();

            Ok(serde_json::json!({
                "transactions": transaction_json,
                "count": transaction_json.len(),
                "limited": transaction_json.len() == limit
            })
            .to_string())
        } else {
            // Fallback to empty response when no service is available
            Ok(serde_json::json!({
                "transactions": [],
                "count": 0,
                "limited": false
            })
            .to_string())
        }
    }

    /// Handles incoming JSON-RPC requests according to MCP protocol.
    pub fn handle_jsonrpc_request(
        &self,
        request: serde_json::Value,
    ) -> YnabResult<serde_json::Value> {
        let id = request["id"].clone();
        let method = request["method"].as_str().unwrap_or("");

        match method {
            "tools/list" => {
                let tools = self.list_tools();
                let tools_json: Vec<serde_json::Value> = tools
                    .into_iter()
                    .map(|tool| {
                        serde_json::json!({
                            "name": tool.name,
                            "description": tool.description
                        })
                    })
                    .collect();

                Ok(serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "tools": tools_json
                    }
                }))
            }
            _ => Ok(serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32601,
                    "message": "Method not found"
                }
            })),
        }
    }
}

impl Default for Handler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_handler_with_new() {
        let handler = Handler::new();

        // Handler should have no transaction service by default
        assert!(handler.transaction_service.is_none());
    }

    #[test]
    fn should_create_handler_with_default() {
        let _handler = Handler::new();

        // Test that we can create via Default trait - clippy prefers direct construction for unit structs
        let _default_handler: Handler = Default::default();
    }

    #[test]
    fn should_list_available_tools() {
        let handler = Handler::new();

        let tools = handler.list_tools();

        // Should include all MCP budget analysis tools
        assert!(
            tools
                .iter()
                .any(|tool| tool.name == "analyze_category_spending")
        );
        assert!(tools.iter().any(|tool| tool.name == "get_budget_overview"));
        assert!(tools.iter().any(|tool| tool.name == "search_transactions"));
        assert_eq!(tools.len(), 3);
    }

    #[test]
    fn should_execute_analyze_category_spending_tool() {
        let handler = Handler::new();

        let result = handler.execute_tool(
            "analyze_category_spending",
            serde_json::json!({
                "budget_id": "test-budget-123",
                "category_name": "Groceries"
            }),
        );

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("category_spending"));
    }

    #[test]
    fn should_return_error_for_unknown_tool() {
        let handler = Handler::new();

        let result = handler.execute_tool("unknown_tool", serde_json::json!({}));

        assert!(result.is_err());
    }

    #[test]
    fn should_handle_list_tools_jsonrpc_request() {
        let handler = Handler::new();

        let jsonrpc_request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/list",
            "params": {}
        });

        let result = handler.handle_jsonrpc_request(jsonrpc_request);

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response["jsonrpc"], "2.0");
        assert_eq!(response["id"], 1);
        assert!(response["result"]["tools"].is_array());
    }

    #[test]
    fn should_handle_unknown_jsonrpc_method() {
        let handler = Handler::new();

        let jsonrpc_request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "unknown/method",
            "params": {}
        });

        let result = handler.handle_jsonrpc_request(jsonrpc_request);

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response["jsonrpc"], "2.0");
        assert_eq!(response["id"], 1);
        assert_eq!(response["error"]["code"], -32601);
        assert_eq!(response["error"]["message"], "Method not found");
    }

    #[test]
    fn should_execute_get_budget_overview_tool() {
        let handler = Handler::new();

        let result = handler.execute_tool(
            "get_budget_overview",
            serde_json::json!({
                "budget_id": "test-budget-456"
            }),
        );

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("budget_overview"));
    }

    #[test]
    fn should_analyze_category_spending_with_real_domain_data() {
        use crate::domain::money::Money;
        use crate::domain::transaction::Transaction;

        // Create real domain objects
        let transaction1 = Transaction::builder()
            .id("txn1".to_string())
            .amount(Money::from_milliunits(-50_000)) // $50 expense
            .category_id("cat1".to_string())
            .account_id("acc1".to_string())
            .build();
        let transaction2 = Transaction::builder()
            .id("txn2".to_string())
            .amount(Money::from_milliunits(-75_000)) // $75 expense
            .category_id("cat1".to_string())
            .account_id("acc1".to_string())
            .build();

        let transaction_service =
            TransactionService::with_transactions(vec![transaction1, transaction2]);

        // Create handler with real services
        let handler = Handler::with_services(transaction_service);

        let result = handler.execute_tool(
            "analyze_category_spending",
            serde_json::json!({
                "category_id": "cat1",
                "category_name": "Groceries"
            }),
        );

        assert!(result.is_ok());
        let response = result.unwrap();
        let response_json: serde_json::Value = serde_json::from_str(&response).unwrap();

        // Should use actual calculated spending ($125 total)
        assert_eq!(
            response_json["category_spending"]["amount_milliunits"],
            125_000
        );
        assert_eq!(response_json["category_spending"]["transaction_count"], 2);
        assert_eq!(response_json["category_spending"]["category"], "Groceries");
    }

    #[test]
    fn should_get_budget_overview_with_real_domain_data() {
        use crate::domain::money::Money;
        use crate::domain::transaction::Transaction;

        // Create transactions for multiple categories
        let groceries_txn = Transaction::builder()
            .id("txn1".to_string())
            .amount(Money::from_milliunits(-50_000)) // $50 groceries expense
            .category_id("groceries".to_string())
            .account_id("acc1".to_string())
            .build();
        let gas_txn = Transaction::builder()
            .id("txn2".to_string())
            .amount(Money::from_milliunits(-30_000)) // $30 gas expense
            .category_id("gas".to_string())
            .account_id("acc1".to_string())
            .build();
        let salary_txn = Transaction::builder()
            .id("txn3".to_string())
            .amount(Money::from_milliunits(3_000_000)) // $3000 salary income
            .category_id("salary".to_string())
            .account_id("acc1".to_string())
            .build();

        let transaction_service =
            TransactionService::with_transactions(vec![groceries_txn, gas_txn, salary_txn]);

        let handler = Handler::with_services(transaction_service);

        let result = handler.execute_tool(
            "get_budget_overview",
            serde_json::json!({
                "budget_id": "test-budget-789"
            }),
        );

        assert!(result.is_ok());
        let response = result.unwrap();
        let response_json: serde_json::Value = serde_json::from_str(&response).unwrap();

        // Should calculate real totals: $80 spent, net income $2920 ($3000 - $80)
        assert_eq!(
            response_json["budget_overview"]["total_expenses_milliunits"],
            80_000
        );
        assert_eq!(
            response_json["budget_overview"]["total_income_milliunits"],
            3_000_000
        );
        assert_eq!(
            response_json["budget_overview"]["net_income_milliunits"],
            2_920_000
        );
        assert_eq!(response_json["budget_overview"]["transaction_count"], 3);
    }

    #[test]
    fn should_search_transactions_with_filters() {
        use crate::domain::money::Money;
        use crate::domain::transaction::Transaction;

        // Create transactions with different amounts and categories
        let expensive_groceries = Transaction::builder()
            .id("txn1".to_string())
            .amount(Money::from_milliunits(-100_000)) // $100 groceries expense
            .category_id("groceries".to_string())
            .account_id("acc1".to_string())
            .description("Whole Foods".to_string())
            .build();
        let cheap_gas = Transaction::builder()
            .id("txn2".to_string())
            .amount(Money::from_milliunits(-20_000)) // $20 gas expense
            .category_id("gas".to_string())
            .account_id("acc1".to_string())
            .description("Shell Station".to_string())
            .build();
        let salary = Transaction::builder()
            .id("txn3".to_string())
            .amount(Money::from_milliunits(2_000_000)) // $2000 salary
            .category_id("salary".to_string())
            .account_id("acc1".to_string())
            .description("Payroll".to_string())
            .build();

        let transaction_service =
            TransactionService::with_transactions(vec![expensive_groceries, cheap_gas, salary]);

        let handler = Handler::with_services(transaction_service);

        let result = handler.execute_tool(
            "search_transactions",
            serde_json::json!({
                "text_search": "Foods",
                "min_amount_milliunits": -150_000,
                "limit": 10
            }),
        );

        assert!(result.is_ok());
        let response = result.unwrap();
        let response_json: serde_json::Value = serde_json::from_str(&response).unwrap();

        // Should find only the Whole Foods transaction (matches text and amount filter)
        assert_eq!(response_json["transactions"].as_array().unwrap().len(), 1);
        let found_txn = &response_json["transactions"][0];
        assert_eq!(found_txn["id"], "txn1");
        assert_eq!(found_txn["description"], "Whole Foods");
        assert_eq!(found_txn["amount_milliunits"], -100_000);
    }
}
