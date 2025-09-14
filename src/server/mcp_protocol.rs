//! MCP (Model Context Protocol) implementation.

use crate::domain::YnabResult;
use crate::server::handler::Handler;
use crate::server::jsonrpc::{JsonRpcRequest, JsonRpcResponse};
use serde_json::json;

/// MCP server that wraps the Handler and provides MCP protocol methods.
pub struct McpServer {
    handler: Handler,
}

impl McpServer {
    /// Creates a new MCP server with the given handler.
    pub fn new(handler: Handler) -> Self {
        Self { handler }
    }

    /// Handles an MCP request and returns an appropriate response.
    pub fn handle_request(&self, request: JsonRpcRequest) -> YnabResult<JsonRpcResponse> {
        let id = request.id.clone().unwrap_or_else(|| json!(null));

        match request.method.as_str() {
            "initialize" => self.handle_initialize(id, request.params),
            "tools/list" => self.handle_tools_list(id),
            "tools/call" => self.handle_tools_call(id, request.params),
            _ => Ok(JsonRpcResponse::error(
                id,
                -32601,
                "Method not found".to_string(),
                None,
            )),
        }
    }

    /// Handles the initialize method.
    fn handle_initialize(&self, id: serde_json::Value, params: Option<serde_json::Value>) -> YnabResult<JsonRpcResponse> {
        // Extract protocol version from params
        let _params = params.unwrap_or_else(|| json!({}));

        // Return MCP initialization response
        let result = json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "ynab-mcp-server",
                "version": "0.1.0"
            }
        });

        Ok(JsonRpcResponse::success(id, result))
    }

    /// Handles the tools/list method.
    fn handle_tools_list(&self, id: serde_json::Value) -> YnabResult<JsonRpcResponse> {
        let tools = self.handler.list_tools();
        let tool_objects: Vec<serde_json::Value> = tools
            .into_iter()
            .map(|tool| {
                json!({
                    "name": tool.name,
                    "description": tool.description
                })
            })
            .collect();

        let result = json!({
            "tools": tool_objects
        });

        Ok(JsonRpcResponse::success(id, result))
    }

    /// Handles the tools/call method.
    fn handle_tools_call(&self, id: serde_json::Value, params: Option<serde_json::Value>) -> YnabResult<JsonRpcResponse> {
        let params = params.ok_or_else(|| {
            crate::domain::YnabError::api_error("Missing params for tools/call".to_string())
        })?;

        let tool_name = params["name"]
            .as_str()
            .ok_or_else(|| crate::domain::YnabError::api_error("Missing tool name".to_string()))?;

        let arguments = params["arguments"].clone();

        match self.handler.execute_tool(tool_name, arguments) {
            Ok(content) => {
                let result = json!({
                    "content": [
                        {
                            "type": "text",
                            "text": content
                        }
                    ]
                });
                Ok(JsonRpcResponse::success(id, result))
            }
            Err(e) => Ok(JsonRpcResponse::error(
                id,
                -32000,
                format!("Tool execution failed: {}", e),
                None,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::jsonrpc::JsonRpcRequest;
    use crate::server::handler::Handler;
    use serde_json::json;

    #[test]
    fn should_handle_initialize_request() {
        let handler = Handler::new();
        let mcp_server = McpServer::new(handler);
        let request = JsonRpcRequest::from_json(r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "clientInfo": {"name": "test-client"}
            }
        }"#).unwrap();

        let response = mcp_server.handle_request(request).unwrap();

        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, json!(1));
        assert!(response.result.is_some());

        let result = response.result.unwrap();
        assert_eq!(result["protocolVersion"], json!("2024-11-05"));
        assert!(result["capabilities"].is_object());
        assert!(result["serverInfo"].is_object());
    }

    #[test]
    fn should_list_available_tools() {
        use crate::adapters::YnabClient;
        use crate::domain::TransactionService;

        let transaction_service = TransactionService::new();
        let ynab_client = YnabClient::new("test-token".to_string());
        let handler = Handler::with_full_integration(transaction_service, ynab_client);
        let mcp_server = McpServer::new(handler);

        let request = JsonRpcRequest::from_json(r#"{
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list",
            "params": {}
        }"#).unwrap();

        let response = mcp_server.handle_request(request).unwrap();

        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, json!(2));
        assert!(response.result.is_some());

        let result = response.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 5); // Our 5 analytical tools

        // Verify tool structure
        let first_tool = &tools[0];
        assert!(first_tool["name"].is_string());
        assert!(first_tool["description"].is_string());
    }

    #[test]
    fn should_execute_tool_via_mcp_protocol() {
        use crate::adapters::YnabClient;
        use crate::domain::TransactionService;

        let transaction_service = TransactionService::new();
        let ynab_client = YnabClient::new("test-token".to_string());
        let handler = Handler::with_full_integration(transaction_service, ynab_client);
        let mcp_server = McpServer::new(handler);

        let request = JsonRpcRequest::from_json(r#"{
            "jsonrpc": "2.0",
            "id": 3,
            "method": "tools/call",
            "params": {
                "name": "analyze_category_spending",
                "arguments": {
                    "budget_id": "test-budget-123",
                    "category_name": "Groceries"
                }
            }
        }"#).unwrap();

        let response = mcp_server.handle_request(request).unwrap();

        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, json!(3));
        assert!(response.result.is_some());

        let result = response.result.unwrap();
        assert!(result["content"].is_array());

        let content_array = result["content"].as_array().unwrap();
        assert_eq!(content_array.len(), 1);

        let content_item = &content_array[0];
        assert_eq!(content_item["type"], json!("text"));
        assert!(content_item["text"].is_string());
    }

    #[test]
    fn should_handle_unknown_mcp_method() {
        let handler = Handler::new();
        let mcp_server = McpServer::new(handler);

        let request = JsonRpcRequest::from_json(r#"{
            "jsonrpc": "2.0",
            "id": 4,
            "method": "unknown/method",
            "params": {}
        }"#).unwrap();

        let response = mcp_server.handle_request(request).unwrap();

        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, json!(4));
        assert!(response.error.is_some());

        let error = response.error.unwrap();
        assert_eq!(error.code, -32601);
        assert_eq!(error.message, "Method not found");
    }

    #[test]
    fn should_handle_tools_call_with_invalid_tool_name() {
        use crate::adapters::YnabClient;
        use crate::domain::TransactionService;

        let transaction_service = TransactionService::new();
        let ynab_client = YnabClient::new("test-token".to_string());
        let handler = Handler::with_full_integration(transaction_service, ynab_client);
        let mcp_server = McpServer::new(handler);

        let request = JsonRpcRequest::from_json(r#"{
            "jsonrpc": "2.0",
            "id": 5,
            "method": "tools/call",
            "params": {
                "name": "nonexistent_tool",
                "arguments": {}
            }
        }"#).unwrap();

        let response = mcp_server.handle_request(request).unwrap();

        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, json!(5));
        assert!(response.error.is_some());

        let error = response.error.unwrap();
        assert_eq!(error.code, -32000);
        assert!(error.message.contains("Tool execution failed"));
    }
}