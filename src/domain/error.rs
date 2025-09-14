//! Error handling for the YNAB domain.

/// Errors that can occur in the YNAB MCP server domain.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum YnabError {
    /// Invalid budget ID provided.
    #[error("Invalid budget ID: {0}")]
    InvalidBudgetId(String),

    /// Category not found.
    #[error("Category not found: {0}")]
    CategoryNotFound(String),

    /// Account not found.
    #[error("Account not found: {0}")]
    AccountNotFound(String),

    /// Payee not found.
    #[error("Payee not found: {0}")]
    PayeeNotFound(String),

    /// Transaction not found.
    #[error("Transaction not found: {0}")]
    TransactionNotFound(String),

    /// Invalid money amount.
    #[error("Invalid money amount: {0}")]
    InvalidAmount(String),

    /// Invalid date format.
    #[error("Invalid date format: {0}")]
    InvalidDate(String),

    /// API request failed.
    #[error("API request failed: {0}")]
    ApiError(String),
}

impl YnabError {
    /// Creates a new InvalidBudgetId error.
    pub fn invalid_budget_id<S: Into<String>>(id: S) -> Self {
        Self::InvalidBudgetId(id.into())
    }

    /// Creates a new CategoryNotFound error.
    pub fn category_not_found<S: Into<String>>(id: S) -> Self {
        Self::CategoryNotFound(id.into())
    }

    /// Creates a new AccountNotFound error.
    pub fn account_not_found<S: Into<String>>(id: S) -> Self {
        Self::AccountNotFound(id.into())
    }

    /// Creates a new PayeeNotFound error.
    pub fn payee_not_found<S: Into<String>>(id: S) -> Self {
        Self::PayeeNotFound(id.into())
    }

    /// Creates a new TransactionNotFound error.
    pub fn transaction_not_found<S: Into<String>>(id: S) -> Self {
        Self::TransactionNotFound(id.into())
    }

    /// Creates a new InvalidAmount error.
    pub fn invalid_amount<S: Into<String>>(message: S) -> Self {
        Self::InvalidAmount(message.into())
    }

    /// Creates a new InvalidDate error.
    pub fn invalid_date<S: Into<String>>(date: S) -> Self {
        Self::InvalidDate(date.into())
    }

    /// Creates a new ApiError.
    pub fn api_error<S: Into<String>>(message: S) -> Self {
        Self::ApiError(message.into())
    }
}

/// Result type for YNAB operations.
pub type YnabResult<T> = Result<T, YnabError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_invalid_budget_id_error() {
        // RED: This test should fail initially due to missing thiserror dependency
        let error = YnabError::invalid_budget_id("invalid-id");

        assert_eq!(error, YnabError::InvalidBudgetId("invalid-id".to_string()));
        assert_eq!(error.to_string(), "Invalid budget ID: invalid-id");
    }

    #[test]
    fn should_create_category_not_found_error() {
        let error = YnabError::category_not_found("cat-123");

        assert_eq!(error, YnabError::CategoryNotFound("cat-123".to_string()));
        assert_eq!(error.to_string(), "Category not found: cat-123");
    }

    #[test]
    fn should_create_account_not_found_error() {
        let error = YnabError::account_not_found("acc-456");

        assert_eq!(error, YnabError::AccountNotFound("acc-456".to_string()));
        assert_eq!(error.to_string(), "Account not found: acc-456");
    }

    #[test]
    fn should_create_payee_not_found_error() {
        let error = YnabError::payee_not_found("payee-789");

        assert_eq!(error, YnabError::PayeeNotFound("payee-789".to_string()));
        assert_eq!(error.to_string(), "Payee not found: payee-789");
    }

    #[test]
    fn should_create_invalid_amount_error() {
        let error = YnabError::invalid_amount("Amount cannot be zero");

        assert_eq!(
            error,
            YnabError::InvalidAmount("Amount cannot be zero".to_string())
        );
        assert_eq!(
            error.to_string(),
            "Invalid money amount: Amount cannot be zero"
        );
    }

    #[test]
    fn should_create_invalid_date_error() {
        let error = YnabError::invalid_date("2024-13-45");

        assert_eq!(error, YnabError::InvalidDate("2024-13-45".to_string()));
        assert_eq!(error.to_string(), "Invalid date format: 2024-13-45");
    }

    #[test]
    fn should_create_api_error() {
        let error = YnabError::api_error("Connection timeout");

        assert_eq!(error, YnabError::ApiError("Connection timeout".to_string()));
        assert_eq!(error.to_string(), "API request failed: Connection timeout");
    }

    #[test]
    fn should_support_ynab_result_type() {
        let success: YnabResult<i32> = Ok(42);
        let failure: YnabResult<i32> = Err(YnabError::invalid_budget_id("test"));

        assert!(success.is_ok());
        assert!(failure.is_err());
    }
}
