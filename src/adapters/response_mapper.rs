//! Response mapper for converting YNAB API JSON responses to domain entities.

use crate::domain::{Budget, Category, Money, Transaction, YnabError, YnabResult};
use serde_json::Value;

/// Maps YNAB API responses to domain entities.
#[derive(Debug, Clone)]
pub struct ResponseMapper;

impl ResponseMapper {
    /// Creates a new ResponseMapper.
    pub fn new() -> Self {
        Self
    }

    /// Maps a YNAB budget JSON response to a Budget domain entity.
    ///
    /// # Arguments
    /// * `json` - The JSON response from the YNAB API
    ///
    /// # Example
    /// ```no_run
    /// use ynab_mcp::adapters::ResponseMapper;
    /// use serde_json::json;
    ///
    /// let mapper = ResponseMapper::new();
    /// let json = json!({"id": "budget-123", "name": "My Budget"});
    /// let budget = mapper.map_budget(&json)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn map_budget(&self, json: &Value) -> YnabResult<Budget> {
        let id = json["id"].as_str().unwrap_or("").to_string();
        let name = json["name"].as_str().unwrap_or("").to_string();
        Ok(Budget::new(id, name))
    }

    /// Maps a YNAB category JSON response to a Category domain entity.
    ///
    /// # Arguments
    /// * `json` - The JSON response from the YNAB API
    ///
    /// # Example
    /// ```no_run
    /// use ynab_mcp::adapters::ResponseMapper;
    /// use serde_json::json;
    ///
    /// let mapper = ResponseMapper::new();
    /// let json = json!({"id": "cat-123", "name": "Groceries", "category_group_id": "group-1"});
    /// let category = mapper.map_category(&json)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn map_category(&self, json: &Value) -> YnabResult<Category> {
        let id = json["id"].as_str().unwrap_or("").to_string();
        let name = json["name"].as_str().unwrap_or("").to_string();
        let group_id = json["category_group_id"].as_str().map(|s| s.to_string());

        Ok(match group_id {
            Some(gid) => Category::new_with_group(id, name, gid),
            None => Category::new(id, name),
        })
    }

    /// Maps a YNAB transaction JSON response to a Transaction domain entity.
    ///
    /// # Arguments
    /// * `json` - The JSON response from the YNAB API
    ///
    /// # Example
    /// ```no_run
    /// use ynab_mcp::adapters::ResponseMapper;
    /// use serde_json::json;
    ///
    /// let mapper = ResponseMapper::new();
    /// let json = json!({
    ///     "id": "trans-123",
    ///     "account_id": "acc-456",
    ///     "category_id": "cat-789",
    ///     "amount": -25000,
    ///     "date": "2024-01-15"
    /// });
    /// let transaction = mapper.map_transaction(&json)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn map_transaction(&self, json: &Value) -> YnabResult<Transaction> {
        let id = json["id"].as_str().unwrap_or("").to_string();
        let account_id = json["account_id"].as_str().unwrap_or("").to_string();
        let category_id = json["category_id"].as_str().unwrap_or("").to_string();
        let payee_id = json["payee_id"].as_str().map(|s| s.to_string());
        let amount_milliunits = json["amount"].as_i64().unwrap_or(0);
        let amount = Money::from_milliunits(amount_milliunits);

        let date = json["date"].as_str().map(|s| s.to_string());
        let description = json["memo"].as_str().map(|s| s.to_string());

        let mut builder = Transaction::builder()
            .id(id)
            .account_id(account_id)
            .category_id(category_id)
            .amount(amount);

        if let Some(pid) = payee_id {
            builder = builder.payee_id(pid);
        }

        if let Some(d) = date {
            builder = builder.date(d);
        }

        if let Some(desc) = description {
            builder = builder.description(desc);
        }

        Ok(builder.build())
    }

    /// Maps a YNAB transactions API response to a vector of Transaction domain entities.
    ///
    /// # Arguments
    /// * `json` - The JSON response from the YNAB transactions API
    ///
    /// # Example
    /// ```no_run
    /// use ynab_mcp::adapters::ResponseMapper;
    /// use serde_json::json;
    ///
    /// let mapper = ResponseMapper::new();
    /// let response = json!({
    ///     "data": {
    ///         "transactions": [
    ///             {
    ///                 "id": "trans-123",
    ///                 "account_id": "acc-456",
    ///                 "category_id": "cat-789",
    ///                 "amount": -25000,
    ///                 "date": "2024-01-15"
    ///             }
    ///         ]
    ///     }
    /// });
    /// let transactions = mapper.map_transactions_from_response(&response).unwrap();
    /// assert_eq!(transactions.len(), 1);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn map_transactions_from_response(&self, json: &Value) -> YnabResult<Vec<Transaction>> {
        let transactions_array = json["data"]["transactions"].as_array().ok_or_else(|| {
            YnabError::ApiError("Invalid transactions response format".to_string())
        })?;

        let mut transactions = Vec::new();
        for transaction_json in transactions_array {
            transactions.push(self.map_transaction(transaction_json)?);
        }

        Ok(transactions)
    }
}

impl Default for ResponseMapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn should_create_response_mapper() {
        let mapper = ResponseMapper::new();

        // Just verify it can be created
        assert!(format!("{:?}", mapper).contains("ResponseMapper"));
    }

    #[test]
    fn should_map_budget_from_json() {
        let mapper = ResponseMapper::new();
        let json = json!({
            "id": "budget-123",
            "name": "My Personal Budget"
        });

        // This test will initially pass since we have a basic implementation
        let budget = mapper.map_budget(&json).unwrap();

        assert_eq!(budget.id(), "budget-123");
        assert_eq!(budget.name(), "My Personal Budget");
    }

    #[test]
    fn should_handle_missing_budget_fields() {
        let mapper = ResponseMapper::new();
        let json = json!({}); // Empty JSON

        // This should still work but with empty values
        let budget = mapper.map_budget(&json).unwrap();

        assert_eq!(budget.id(), "");
        assert_eq!(budget.name(), "");
    }

    #[test]
    fn should_map_category_from_json() {
        let mapper = ResponseMapper::new();
        let json = json!({
            "id": "category-456",
            "name": "Groceries",
            "category_group_id": "group-123"
        });

        let category = mapper.map_category(&json).unwrap();

        assert_eq!(category.id(), "category-456");
        assert_eq!(category.name(), "Groceries");
        assert_eq!(category.group_id(), Some("group-123"));
    }

    #[test]
    fn should_map_transaction_from_json() {
        let mapper = ResponseMapper::new();
        let json = json!({
            "id": "trans-789",
            "account_id": "account-123",
            "category_id": "category-456",
            "payee_id": "payee-789",
            "amount": -50000, // YNAB uses milliunits
            "date": "2024-01-15",
            "memo": "Grocery shopping"
        });

        // This test will fail initially - we need to implement map_transaction
        let transaction = mapper.map_transaction(&json).unwrap();

        assert_eq!(transaction.id(), "trans-789");
        assert_eq!(transaction.account_id(), "account-123");
        assert_eq!(transaction.category_id(), "category-456");
        assert_eq!(transaction.payee_id(), Some("payee-789"));
        assert_eq!(transaction.amount(), Money::from_milliunits(-50000));
        assert_eq!(transaction.date(), Some("2024-01-15"));
        assert_eq!(transaction.description(), Some("Grocery shopping"));
    }

    #[test]
    fn should_map_multiple_transactions_from_api_response() {
        let mapper = ResponseMapper::new();
        let json = json!({
            "data": {
                "transactions": [
                    {
                        "id": "trans-1",
                        "account_id": "account-123",
                        "category_id": "category-456",
                        "amount": -25000,
                        "date": "2024-01-15",
                        "memo": "Grocery store"
                    },
                    {
                        "id": "trans-2",
                        "account_id": "account-456",
                        "category_id": "category-789",
                        "amount": -15000,
                        "date": "2024-01-16"
                    }
                ]
            }
        });

        let transactions = mapper.map_transactions_from_response(&json).unwrap();

        assert_eq!(transactions.len(), 2);
        assert_eq!(transactions[0].id(), "trans-1");
        assert_eq!(transactions[0].amount(), Money::from_milliunits(-25000));
        assert_eq!(transactions[1].id(), "trans-2");
        assert_eq!(transactions[1].amount(), Money::from_milliunits(-15000));
    }

    #[test]
    fn should_handle_invalid_transactions_response_format() {
        let mapper = ResponseMapper::new();
        let json = json!({
            "data": {
                "invalid_field": "not transactions"
            }
        });

        let result = mapper.map_transactions_from_response(&json);
        assert!(result.is_err());

        if let Err(YnabError::ApiError(msg)) = result {
            assert_eq!(msg, "Invalid transactions response format");
        } else {
            panic!("Expected ApiError");
        }
    }
}
