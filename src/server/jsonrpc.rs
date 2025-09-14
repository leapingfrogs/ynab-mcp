//! JSON-RPC 2.0 message handling for MCP protocol.

use crate::domain::{YnabError, YnabResult};
use serde_json::Value;

/// A JSON-RPC 2.0 request message.
#[derive(Debug, Clone, PartialEq)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

impl JsonRpcRequest {
    /// Parses a JSON-RPC request from a JSON string.
    pub fn from_json(json: &str) -> YnabResult<Self> {
        let value: Value = serde_json::from_str(json)
            .map_err(|e| YnabError::api_error(format!("Invalid JSON: {}", e)))?;

        let jsonrpc = value["jsonrpc"].as_str()
            .ok_or_else(|| YnabError::api_error("Missing jsonrpc field".to_string()))?
            .to_string();

        let method = value["method"].as_str()
            .ok_or_else(|| YnabError::api_error("Missing method field".to_string()))?
            .to_string();

        let id = if value["id"].is_null() { None } else { Some(value["id"].clone()) };
        let params = if value["params"].is_null() { None } else { Some(value["params"].clone()) };

        Ok(JsonRpcRequest {
            jsonrpc,
            id,
            method,
            params,
        })
    }
}

/// A JSON-RPC 2.0 response message.
#[derive(Debug, Clone, PartialEq)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Value,
    pub result: Option<Value>,
    pub error: Option<JsonRpcError>,
}

/// A JSON-RPC 2.0 error object.
#[derive(Debug, Clone, PartialEq)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}

impl JsonRpcResponse {
    /// Creates a success response with the given result.
    pub fn success(id: impl Into<Value>, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: id.into(),
            result: Some(result),
            error: None,
        }
    }

    /// Creates an error response with the given error.
    pub fn error(id: impl Into<Value>, code: i32, message: String, data: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: id.into(),
            result: None,
            error: Some(JsonRpcError { code, message, data }),
        }
    }

    /// Converts the response to a JSON string.
    pub fn to_json(&self) -> String {
        let mut response = serde_json::Map::new();
        response.insert("jsonrpc".to_string(), Value::String(self.jsonrpc.clone()));
        response.insert("id".to_string(), self.id.clone());

        if let Some(result) = &self.result {
            response.insert("result".to_string(), result.clone());
        }

        if let Some(error) = &self.error {
            let mut error_obj = serde_json::Map::new();
            error_obj.insert("code".to_string(), Value::Number(error.code.into()));
            error_obj.insert("message".to_string(), Value::String(error.message.clone()));
            if let Some(data) = &error.data {
                error_obj.insert("data".to_string(), data.clone());
            }
            response.insert("error".to_string(), Value::Object(error_obj));
        }

        serde_json::to_string(&response).expect("JSON serialization should not fail")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn should_parse_jsonrpc_request_with_method_and_params() {
        let json = r#"{"jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": {}}"#;
        let request = JsonRpcRequest::from_json(json).unwrap();

        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.id, Some(json!(1)));
        assert_eq!(request.method, "tools/list");
        assert_eq!(request.params, Some(json!({})));
    }

    #[test]
    fn should_format_jsonrpc_success_response() {
        let response = JsonRpcResponse::success(1, json!({"tools": []}));
        let json = response.to_json();

        assert!(json.contains("\"jsonrpc\":\"2.0\""));
        assert!(json.contains("\"id\":1"));
        assert!(json.contains("\"result\""));
        assert!(json.contains("\"tools\":[]"));
    }

    #[test]
    fn should_parse_jsonrpc_request_without_params() {
        let json = r#"{"jsonrpc": "2.0", "id": "test", "method": "tools/list"}"#;
        let request = JsonRpcRequest::from_json(json).unwrap();

        assert_eq!(request.method, "tools/list");
        assert_eq!(request.id, Some(json!("test")));
        assert_eq!(request.params, None);
    }

    #[test]
    fn should_handle_invalid_json() {
        let json = r#"{"invalid": json"#;
        let result = JsonRpcRequest::from_json(json);

        assert!(result.is_err());
        match result.unwrap_err() {
            YnabError::ApiError(msg) => assert!(msg.contains("Invalid JSON")),
            other => panic!("Expected ApiError, got: {:?}", other),
        }
    }

    #[test]
    fn should_handle_missing_method_field() {
        let json = r#"{"jsonrpc": "2.0", "id": 1}"#;
        let result = JsonRpcRequest::from_json(json);

        assert!(result.is_err());
        match result.unwrap_err() {
            YnabError::ApiError(msg) => assert_eq!(msg, "Missing method field"),
            other => panic!("Expected ApiError, got: {:?}", other),
        }
    }

    #[test]
    fn should_format_jsonrpc_error_response() {
        let response = JsonRpcResponse::error(
            "test-id",
            -32600,
            "Invalid Request".to_string(),
            Some(json!({"details": "Missing required field"}))
        );
        let json = response.to_json();

        assert!(json.contains("\"jsonrpc\":\"2.0\""));
        assert!(json.contains("\"id\":\"test-id\""));
        assert!(json.contains("\"error\""));
        assert!(json.contains("\"code\":-32600"));
        assert!(json.contains("\"message\":\"Invalid Request\""));
        assert!(json.contains("\"details\":\"Missing required field\""));
    }
}