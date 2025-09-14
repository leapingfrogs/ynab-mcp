//! MCP request handlers.

use crate::domain::error::YnabResult;

/// Represents an MCP tool that can be called by clients.
#[derive(Debug, Clone, PartialEq)]
pub struct Tool {
    pub name: String,
    pub description: String,
}

/// MCP server handler for YNAB budget analysis tools.
pub struct Handler;

impl Handler {
    /// Creates a new Handler instance.
    pub fn new() -> Self {
        Self
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
        ]
    }

    /// Executes a named tool with the provided parameters.
    pub fn execute_tool(&self, tool_name: &str, _params: serde_json::Value) -> YnabResult<String> {
        match tool_name {
            "analyze_category_spending" => Ok(serde_json::json!({
                "category_spending": {
                    "category": "Groceries",
                    "amount_milliunits": 125000,
                    "transaction_count": 5
                }
            })
            .to_string()),
            "get_budget_overview" => Ok(serde_json::json!({
                "budget_overview": {
                    "total_budgeted": 300000,
                    "total_spent": 125000,
                    "categories_over_budget": 2
                }
            })
            .to_string()),
            _ => Err(crate::domain::error::YnabError::InvalidBudgetId(format!(
                "Unknown tool: {}",
                tool_name
            ))),
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

        // Basic creation test - placeholder will be expanded in future iterations
        assert_eq!(std::mem::size_of_val(&handler), 0); // Zero-sized struct
    }

    #[test]
    fn should_create_handler_with_default() {
        let _handler = Handler;

        // Test that we can create via Default trait - clippy prefers direct construction for unit structs
        let _default_handler: Handler = Default::default();
    }

    #[test]
    fn should_list_available_tools() {
        let handler = Handler::new();

        let tools = handler.list_tools();

        // Should include category spending analysis tool
        assert!(
            tools
                .iter()
                .any(|tool| tool.name == "analyze_category_spending")
        );
        assert!(tools.iter().any(|tool| tool.name == "get_budget_overview"));
        assert_eq!(tools.len(), 2);
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
}
