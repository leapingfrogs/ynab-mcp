//! Transaction domain entity.

use crate::domain::Money;

/// Represents a financial transaction in YNAB.
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    id: String,
    account_id: String,
    category_id: String,
    payee_id: Option<String>,
    amount: Money,
    date: Option<String>,
    description: Option<String>,
}

impl Transaction {
    /// Creates a new TransactionBuilder for constructing transactions.
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::{Transaction, Money};
    ///
    /// let transaction = Transaction::builder()
    ///     .id("txn-123".to_string())
    ///     .account_id("acc-123".to_string())
    ///     .category_id("groceries".to_string())
    ///     .amount(Money::from_milliunits(-5000))
    ///     .build();
    /// assert_eq!(transaction.id(), "txn-123");
    /// assert_eq!(transaction.account_id(), "acc-123");
    /// assert_eq!(transaction.category_id(), "groceries");
    /// ```
    pub fn builder() -> TransactionBuilder {
        TransactionBuilder::new()
    }

    /// Creates a new Transaction with minimal required fields (legacy constructor).
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::{Transaction, Money};
    ///
    /// let transaction = Transaction::new(
    ///     "txn-123".to_string(),
    ///     "acc-123".to_string(),
    ///     "groceries".to_string(),
    ///     Money::from_milliunits(-5000),
    /// );
    /// assert_eq!(transaction.id(), "txn-123");
    /// assert_eq!(transaction.category_id(), "groceries");
    /// ```
    pub fn new(id: String, account_id: String, category_id: String, amount: Money) -> Self {
        Self {
            id,
            account_id,
            category_id,
            payee_id: None,
            amount,
            date: None,
            description: None,
        }
    }

    /// Creates a new Transaction with a date (legacy constructor).
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::{Transaction, Money};
    ///
    /// let transaction = Transaction::new_with_date(
    ///     "txn-123".to_string(),
    ///     "acc-123".to_string(),
    ///     "groceries".to_string(),
    ///     Money::from_milliunits(-5000),
    ///     "2024-01-15".to_string(),
    /// );
    /// assert_eq!(transaction.date(), Some("2024-01-15"));
    /// ```
    pub fn new_with_date(
        id: String,
        account_id: String,
        category_id: String,
        amount: Money,
        date: String,
    ) -> Self {
        Self {
            id,
            account_id,
            category_id,
            payee_id: None,
            amount,
            date: Some(date),
            description: None,
        }
    }

    /// Creates a new Transaction with a description (legacy constructor).
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::{Transaction, Money};
    ///
    /// let transaction = Transaction::new_with_description(
    ///     "txn-123".to_string(),
    ///     "acc-123".to_string(),
    ///     "groceries".to_string(),
    ///     Money::from_milliunits(-5000),
    ///     "Whole Foods Market".to_string(),
    /// );
    /// assert_eq!(transaction.description(), Some("Whole Foods Market"));
    /// ```
    pub fn new_with_description(
        id: String,
        account_id: String,
        category_id: String,
        amount: Money,
        description: String,
    ) -> Self {
        Self {
            id,
            account_id,
            category_id,
            payee_id: None,
            amount,
            date: None,
            description: Some(description),
        }
    }

    /// Returns the transaction ID.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the account ID this transaction belongs to.
    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    /// Returns the category ID this transaction belongs to.
    pub fn category_id(&self) -> &str {
        &self.category_id
    }

    /// Returns the payee ID if this transaction has a payee.
    pub fn payee_id(&self) -> Option<&str> {
        self.payee_id.as_deref()
    }

    /// Returns the transaction amount.
    pub fn amount(&self) -> Money {
        self.amount
    }

    /// Returns the transaction date if available.
    pub fn date(&self) -> Option<&str> {
        self.date.as_deref()
    }

    /// Returns the transaction description if available.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

/// Builder for constructing Transaction objects.
#[derive(Debug, Default)]
pub struct TransactionBuilder {
    id: Option<String>,
    account_id: Option<String>,
    category_id: Option<String>,
    payee_id: Option<String>,
    amount: Option<Money>,
    date: Option<String>,
    description: Option<String>,
}

impl TransactionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn account_id(mut self, account_id: String) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn category_id(mut self, category_id: String) -> Self {
        self.category_id = Some(category_id);
        self
    }

    pub fn payee_id(mut self, payee_id: String) -> Self {
        self.payee_id = Some(payee_id);
        self
    }

    pub fn amount(mut self, amount: Money) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn date(mut self, date: String) -> Self {
        self.date = Some(date);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn build(self) -> Transaction {
        Transaction {
            id: self.id.expect("Transaction ID is required"),
            account_id: self.account_id.expect("Account ID is required"),
            category_id: self.category_id.expect("Category ID is required"),
            payee_id: self.payee_id,
            amount: self.amount.expect("Transaction amount is required"),
            date: self.date,
            description: self.description,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_transaction_with_required_fields() {
        let transaction = Transaction::new(
            "txn-123".to_string(),
            "acc-123".to_string(),
            "category-456".to_string(),
            Money::from_milliunits(-5000), // Negative for expense
        );

        assert_eq!(transaction.id(), "txn-123");
        assert_eq!(transaction.account_id(), "acc-123");
        assert_eq!(transaction.category_id(), "category-456");
        assert_eq!(transaction.amount(), Money::from_milliunits(-5000));
        assert_eq!(transaction.payee_id(), None);
    }

    #[test]
    fn should_handle_positive_amount_for_income() {
        let transaction = Transaction::new(
            "txn-income".to_string(),
            "acc-checking".to_string(),
            "salary".to_string(),
            Money::from_milliunits(100000), // Positive for income
        );

        assert_eq!(transaction.amount(), Money::from_milliunits(100000));
    }

    #[test]
    fn should_create_transaction_with_builder_pattern() {
        // RED: This test should fail initially
        let transaction = Transaction::builder()
            .id("txn-456".to_string())
            .account_id("acc-checking".to_string())
            .category_id("groceries".to_string())
            .payee_id("payee-wholefood".to_string())
            .amount(Money::from_milliunits(-7500))
            .date("2024-01-15".to_string())
            .description("Whole Foods Market".to_string())
            .build();

        assert_eq!(transaction.id(), "txn-456");
        assert_eq!(transaction.account_id(), "acc-checking");
        assert_eq!(transaction.category_id(), "groceries");
        assert_eq!(transaction.payee_id(), Some("payee-wholefood"));
        assert_eq!(transaction.amount(), Money::from_milliunits(-7500));
        assert_eq!(transaction.date(), Some("2024-01-15"));
        assert_eq!(transaction.description(), Some("Whole Foods Market"));
    }

    #[test]
    fn should_create_transaction_with_minimal_builder_fields() {
        let transaction = Transaction::builder()
            .id("txn-minimal".to_string())
            .account_id("acc-123".to_string())
            .category_id("gas".to_string())
            .amount(Money::from_milliunits(-3000))
            .build();

        assert_eq!(transaction.id(), "txn-minimal");
        assert_eq!(transaction.account_id(), "acc-123");
        assert_eq!(transaction.category_id(), "gas");
        assert_eq!(transaction.payee_id(), None);
        assert_eq!(transaction.date(), None);
        assert_eq!(transaction.description(), None);
    }
}
