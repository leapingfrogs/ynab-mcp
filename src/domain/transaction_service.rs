//! Transaction service for querying and aggregating transaction data.

use crate::domain::{Transaction, TransactionQuery};

/// Service for executing transaction queries and aggregations.
#[derive(Debug, Clone, Default)]
pub struct TransactionService {
    transactions: Vec<Transaction>,
}

impl TransactionService {
    /// Creates a new TransactionService.
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::TransactionService;
    ///
    /// let service = TransactionService::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new TransactionService with initial transactions.
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::{TransactionService, Transaction, Money};
    ///
    /// let transactions = vec![
    ///     Transaction::new("txn-1".to_string(), "groceries".to_string(), Money::from_milliunits(-5000)),
    /// ];
    /// let service = TransactionService::with_transactions(transactions);
    /// ```
    pub fn with_transactions(transactions: Vec<Transaction>) -> Self {
        Self { transactions }
    }

    /// Executes a transaction query and returns matching transactions.
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::{TransactionService, TransactionQuery, Money};
    ///
    /// let service = TransactionService::new();
    /// let query = TransactionQuery::new()
    ///     .with_min_amount(Money::from_milliunits(0));
    ///
    /// let results = service.query(&query);
    /// ```
    pub fn query(&self, query: &TransactionQuery) -> Vec<&Transaction> {
        query.filter(&self.transactions)
    }

    /// Returns the total count of transactions in the service.
    pub fn total_count(&self) -> usize {
        self.transactions.len()
    }

    /// Adds a transaction to the service.
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    /// Adds multiple transactions to the service.
    pub fn add_transactions(&mut self, transactions: Vec<Transaction>) {
        self.transactions.extend(transactions);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Money;

    #[test]
    fn should_create_transaction_service_with_new() {
        let service = TransactionService::new();

        assert_eq!(service.total_count(), 0);
    }

    #[test]
    fn should_create_transaction_service_with_transactions() {
        let transactions = vec![
            Transaction::new(
                "txn-1".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-5000),
            ),
            Transaction::new(
                "txn-2".to_string(),
                "salary".to_string(),
                Money::from_milliunits(100000),
            ),
        ];

        let service = TransactionService::with_transactions(transactions);

        assert_eq!(service.total_count(), 2);
    }

    #[test]
    fn should_execute_queries_on_transaction_data() {
        let transactions = vec![
            Transaction::new(
                "txn-1".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-5000),
            ),
            Transaction::new(
                "txn-2".to_string(),
                "salary".to_string(),
                Money::from_milliunits(100000),
            ),
            Transaction::new(
                "txn-3".to_string(),
                "gas".to_string(),
                Money::from_milliunits(-3000),
            ),
        ];

        let service = TransactionService::with_transactions(transactions);
        let query = TransactionQuery::new().with_min_amount(Money::from_milliunits(0));

        let results = service.query(&query);

        // Should only return positive amounts (income)
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].amount(), Money::from_milliunits(100000));
    }

    #[test]
    fn should_add_single_transaction() {
        let mut service = TransactionService::new();
        let transaction = Transaction::new(
            "txn-1".to_string(),
            "groceries".to_string(),
            Money::from_milliunits(-5000),
        );

        service.add_transaction(transaction);

        assert_eq!(service.total_count(), 1);
    }

    #[test]
    fn should_add_multiple_transactions() {
        let mut service = TransactionService::new();
        let transactions = vec![
            Transaction::new(
                "txn-1".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-5000),
            ),
            Transaction::new(
                "txn-2".to_string(),
                "salary".to_string(),
                Money::from_milliunits(100000),
            ),
        ];

        service.add_transactions(transactions);

        assert_eq!(service.total_count(), 2);
    }
}
