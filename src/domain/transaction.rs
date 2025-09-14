//! Transaction domain entity.

use crate::domain::Money;

/// Represents a financial transaction in YNAB.
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    id: String,
    category_id: String,
    amount: Money,
    date: Option<String>,
    description: Option<String>,
}

impl Transaction {
    /// Creates a new Transaction.
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::{Transaction, Money};
    ///
    /// let transaction = Transaction::new(
    ///     "txn-123".to_string(),
    ///     "groceries".to_string(),
    ///     Money::from_milliunits(-5000),
    /// );
    /// assert_eq!(transaction.id(), "txn-123");
    /// assert_eq!(transaction.category_id(), "groceries");
    /// ```
    pub fn new(id: String, category_id: String, amount: Money) -> Self {
        Self {
            id,
            category_id,
            amount,
            date: None,
            description: None,
        }
    }

    /// Creates a new Transaction with a date.
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::{Transaction, Money};
    ///
    /// let transaction = Transaction::new_with_date(
    ///     "txn-123".to_string(),
    ///     "groceries".to_string(),
    ///     Money::from_milliunits(-5000),
    ///     "2024-01-15".to_string(),
    /// );
    /// assert_eq!(transaction.date(), Some("2024-01-15"));
    /// ```
    pub fn new_with_date(id: String, category_id: String, amount: Money, date: String) -> Self {
        Self {
            id,
            category_id,
            amount,
            date: Some(date),
            description: None,
        }
    }

    /// Creates a new Transaction with a description.
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::{Transaction, Money};
    ///
    /// let transaction = Transaction::new_with_description(
    ///     "txn-123".to_string(),
    ///     "groceries".to_string(),
    ///     Money::from_milliunits(-5000),
    ///     "Whole Foods Market".to_string(),
    /// );
    /// assert_eq!(transaction.description(), Some("Whole Foods Market"));
    /// ```
    pub fn new_with_description(
        id: String,
        category_id: String,
        amount: Money,
        description: String,
    ) -> Self {
        Self {
            id,
            category_id,
            amount,
            date: None,
            description: Some(description),
        }
    }

    /// Returns the transaction ID.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the category ID this transaction belongs to.
    pub fn category_id(&self) -> &str {
        &self.category_id
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_transaction_with_required_fields() {
        let transaction = Transaction::new(
            "txn-123".to_string(),
            "category-456".to_string(),
            Money::from_milliunits(-5000), // Negative for expense
        );

        assert_eq!(transaction.id(), "txn-123");
        assert_eq!(transaction.category_id(), "category-456");
        assert_eq!(transaction.amount(), Money::from_milliunits(-5000));
    }

    #[test]
    fn should_handle_positive_amount_for_income() {
        let transaction = Transaction::new(
            "txn-income".to_string(),
            "salary".to_string(),
            Money::from_milliunits(100000), // Positive for income
        );

        assert_eq!(transaction.amount(), Money::from_milliunits(100000));
    }
}
