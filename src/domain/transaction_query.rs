//! Transaction query and filtering capabilities.

use crate::domain::{Money, Transaction};

/// Sorting criteria for transactions.
#[derive(Debug, Clone, PartialEq)]
pub enum SortBy {
    AmountAscending,
    AmountDescending,
    Date,
}

/// Builder for filtering and querying transactions.
#[derive(Debug, Clone, Default)]
pub struct TransactionQuery {
    min_amount: Option<Money>,
    max_amount: Option<Money>,
    categories: Vec<String>,
    search_text: Option<String>,
    sort_by: Option<SortBy>,
}

impl TransactionQuery {
    /// Creates a new TransactionQuery builder.
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::{TransactionQuery, Money};
    ///
    /// let query = TransactionQuery::new()
    ///     .with_min_amount(Money::from_milliunits(0));
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Filters transactions within the specified amount range (inclusive).
    pub fn with_amount_range(mut self, min: Money, max: Money) -> Self {
        self.min_amount = Some(min);
        self.max_amount = Some(max);
        self
    }

    /// Filters transactions above the minimum amount.
    pub fn with_min_amount(mut self, min: Money) -> Self {
        self.min_amount = Some(min);
        self
    }

    /// Filters transactions below the maximum amount.
    pub fn with_max_amount(mut self, max: Money) -> Self {
        self.max_amount = Some(max);
        self
    }

    /// Filters transactions that belong to the specified categories.
    pub fn with_categories(mut self, categories: Vec<String>) -> Self {
        self.categories = categories;
        self
    }

    /// Filters transactions that belong to a single category.
    pub fn with_category(mut self, category: String) -> Self {
        self.categories = vec![category];
        self
    }

    /// Filters transactions by searching in their description (case-insensitive).
    pub fn with_text_search(mut self, search_text: String) -> Self {
        self.search_text = Some(search_text);
        self
    }

    /// Sorts transactions by amount in ascending order.
    pub fn sort_by_amount_ascending(mut self) -> Self {
        self.sort_by = Some(SortBy::AmountAscending);
        self
    }

    /// Sorts transactions by amount in descending order.
    pub fn sort_by_amount_descending(mut self) -> Self {
        self.sort_by = Some(SortBy::AmountDescending);
        self
    }

    /// Sorts transactions by date.
    pub fn sort_by_date(mut self) -> Self {
        self.sort_by = Some(SortBy::Date);
        self
    }

    /// Applies all filters to a list of transactions and returns matching ones.
    pub fn filter<'a>(&self, transactions: &'a [Transaction]) -> Vec<&'a Transaction> {
        let mut filtered: Vec<&Transaction> = transactions
            .iter()
            .filter(|transaction| self.matches_amount_filter(transaction))
            .filter(|transaction| self.matches_category_filter(transaction))
            .filter(|transaction| self.matches_text_filter(transaction))
            .collect();

        if let Some(ref sort_by) = self.sort_by {
            self.apply_sorting(&mut filtered, sort_by);
        }

        filtered
    }

    /// Checks if a transaction matches the amount filter criteria.
    fn matches_amount_filter(&self, transaction: &Transaction) -> bool {
        let amount = transaction.amount();

        if let Some(min) = self.min_amount
            && amount < min
        {
            return false;
        }

        if let Some(max) = self.max_amount
            && amount > max
        {
            return false;
        }

        true
    }

    /// Checks if a transaction matches the category filter criteria.
    fn matches_category_filter(&self, transaction: &Transaction) -> bool {
        if self.categories.is_empty() {
            return true; // No category filter applied
        }

        self.categories
            .contains(&transaction.category_id().to_string())
    }

    /// Checks if a transaction matches the text search criteria (case-insensitive).
    fn matches_text_filter(&self, transaction: &Transaction) -> bool {
        if let Some(ref search_text) = self.search_text {
            if let Some(description) = transaction.description() {
                return description
                    .to_lowercase()
                    .contains(&search_text.to_lowercase());
            }
            return false; // No description to search in
        }
        true // No text filter applied
    }

    /// Applies the specified sorting to the filtered transactions.
    fn apply_sorting(&self, transactions: &mut Vec<&Transaction>, sort_by: &SortBy) {
        match sort_by {
            SortBy::AmountAscending => {
                transactions.sort_by_key(|a| a.amount());
            }
            SortBy::AmountDescending => {
                transactions.sort_by_key(|b| std::cmp::Reverse(b.amount()));
            }
            SortBy::Date => {
                transactions.sort_by(|a, b| {
                    match (a.date(), b.date()) {
                        (Some(date_a), Some(date_b)) => date_a.cmp(date_b),
                        (Some(_), None) => std::cmp::Ordering::Less, // Transactions with dates come first
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => std::cmp::Ordering::Equal,
                    }
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_filter_transactions_by_amount_range() {
        let transactions = vec![
            Transaction::new(
                "txn-1".to_string(),
                "acc-test".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-5000), // $5.00 expense
            ),
            Transaction::new(
                "txn-2".to_string(),
                "acc-test".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-15000), // $15.00 expense
            ),
            Transaction::new(
                "txn-3".to_string(),
                "acc-test".to_string(),
                "salary".to_string(),
                Money::from_milliunits(100000), // $100.00 income
            ),
        ];

        let query = TransactionQuery::new().with_amount_range(
            Money::from_milliunits(-10000),
            Money::from_milliunits(-1000),
        );

        let filtered = query.filter(&transactions);

        // Should only include the $5.00 expense (within range -$10.00 to -$1.00)
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].amount(), Money::from_milliunits(-5000));
    }

    #[test]
    fn should_filter_transactions_by_minimum_amount() {
        let transactions = vec![
            Transaction::new(
                "txn-1".to_string(),
                "acc-test".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-5000),
            ),
            Transaction::new(
                "txn-2".to_string(),
                "acc-test".to_string(),
                "salary".to_string(),
                Money::from_milliunits(50000),
            ),
        ];

        let query = TransactionQuery::new().with_min_amount(Money::from_milliunits(0));

        let filtered = query.filter(&transactions);

        // Should only include positive amounts (income)
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].amount(), Money::from_milliunits(50000));
    }

    #[test]
    fn should_filter_transactions_by_category_list() {
        let transactions = vec![
            Transaction::new(
                "txn-1".to_string(),
                "acc-test".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-5000),
            ),
            Transaction::new(
                "txn-2".to_string(),
                "acc-test".to_string(),
                "gas".to_string(),
                Money::from_milliunits(-3000),
            ),
            Transaction::new(
                "txn-3".to_string(),
                "acc-test".to_string(),
                "salary".to_string(),
                Money::from_milliunits(50000),
            ),
            Transaction::new(
                "txn-4".to_string(),
                "acc-test".to_string(),
                "restaurants".to_string(),
                Money::from_milliunits(-2000),
            ),
        ];

        let query = TransactionQuery::new()
            .with_categories(vec!["groceries".to_string(), "gas".to_string()]);

        let filtered = query.filter(&transactions);

        // Should include groceries and gas transactions
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().any(|t| t.category_id() == "groceries"));
        assert!(filtered.iter().any(|t| t.category_id() == "gas"));
    }

    #[test]
    fn should_filter_transactions_by_single_category() {
        let transactions = vec![
            Transaction::new(
                "txn-1".to_string(),
                "acc-test".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-5000),
            ),
            Transaction::new(
                "txn-2".to_string(),
                "acc-test".to_string(),
                "gas".to_string(),
                Money::from_milliunits(-3000),
            ),
        ];

        let query = TransactionQuery::new().with_category("groceries".to_string());

        let filtered = query.filter(&transactions);

        // Should only include groceries transaction
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].category_id(), "groceries");
    }

    #[test]
    fn should_filter_transactions_by_text_search() {
        let transactions = vec![
            Transaction::new_with_description(
                "txn-1".to_string(),
                "acc-test".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-5000),
                "Whole Foods Market".to_string(),
            ),
            Transaction::new_with_description(
                "txn-2".to_string(),
                "acc-test".to_string(),
                "gas".to_string(),
                Money::from_milliunits(-3000),
                "Shell Gas Station".to_string(),
            ),
            Transaction::new(
                "txn-3".to_string(),
                "acc-test".to_string(),
                "salary".to_string(),
                Money::from_milliunits(50000),
            ),
        ];

        let query = TransactionQuery::new().with_text_search("market".to_string());

        let filtered = query.filter(&transactions);

        // Should only include transaction with "market" in description (case-insensitive)
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id(), "txn-1");
    }

    #[test]
    fn should_perform_case_insensitive_text_search() {
        let transactions = vec![Transaction::new_with_description(
            "txn-1".to_string(),
            "acc-test".to_string(),
            "groceries".to_string(),
            Money::from_milliunits(-5000),
            "COSTCO WAREHOUSE".to_string(),
        )];

        let query = TransactionQuery::new().with_text_search("costco".to_string()); // lowercase search

        let filtered = query.filter(&transactions);

        // Should match regardless of case
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id(), "txn-1");
    }

    #[test]
    fn should_sort_transactions_by_amount_ascending() {
        let transactions = vec![
            Transaction::new(
                "txn-1".to_string(),
                "acc-test".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-5000),
            ),
            Transaction::new(
                "txn-2".to_string(),
                "acc-test".to_string(),
                "salary".to_string(),
                Money::from_milliunits(100000),
            ),
            Transaction::new(
                "txn-3".to_string(),
                "acc-test".to_string(),
                "gas".to_string(),
                Money::from_milliunits(-3000),
            ),
        ];

        let query = TransactionQuery::new().sort_by_amount_ascending();

        let sorted = query.filter(&transactions);

        // Should be sorted by amount: -$50.00, -$30.00, $1000.00
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0].amount(), Money::from_milliunits(-5000));
        assert_eq!(sorted[1].amount(), Money::from_milliunits(-3000));
        assert_eq!(sorted[2].amount(), Money::from_milliunits(100000));
    }

    #[test]
    fn should_sort_transactions_by_amount_descending() {
        let transactions = vec![
            Transaction::new(
                "txn-1".to_string(),
                "acc-test".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-5000),
            ),
            Transaction::new(
                "txn-2".to_string(),
                "acc-test".to_string(),
                "salary".to_string(),
                Money::from_milliunits(100000),
            ),
            Transaction::new(
                "txn-3".to_string(),
                "acc-test".to_string(),
                "gas".to_string(),
                Money::from_milliunits(-3000),
            ),
        ];

        let query = TransactionQuery::new().sort_by_amount_descending();

        let sorted = query.filter(&transactions);

        // Should be sorted by amount: $1000.00, -$30.00, -$50.00
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0].amount(), Money::from_milliunits(100000));
        assert_eq!(sorted[1].amount(), Money::from_milliunits(-3000));
        assert_eq!(sorted[2].amount(), Money::from_milliunits(-5000));
    }

    #[test]
    fn should_sort_transactions_by_date() {
        let transactions = vec![
            Transaction::new_with_date(
                "txn-1".to_string(),
                "acc-test".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-5000),
                "2024-01-20".to_string(),
            ),
            Transaction::new_with_date(
                "txn-2".to_string(),
                "acc-test".to_string(),
                "salary".to_string(),
                Money::from_milliunits(100000),
                "2024-01-15".to_string(),
            ),
            Transaction::new_with_date(
                "txn-3".to_string(),
                "acc-test".to_string(),
                "gas".to_string(),
                Money::from_milliunits(-3000),
                "2024-01-25".to_string(),
            ),
        ];

        let query = TransactionQuery::new().sort_by_date();

        let sorted = query.filter(&transactions);

        // Should be sorted by date: 2024-01-15, 2024-01-20, 2024-01-25
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0].date(), Some("2024-01-15"));
        assert_eq!(sorted[1].date(), Some("2024-01-20"));
        assert_eq!(sorted[2].date(), Some("2024-01-25"));
    }

    #[test]
    fn should_filter_transactions_by_max_amount() {
        let transactions = vec![
            Transaction::new(
                "txn-1".to_string(),
                "acc-test".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-5000), // Under limit
            ),
            Transaction::new(
                "txn-2".to_string(),
                "acc-test".to_string(),
                "gas".to_string(),
                Money::from_milliunits(-10000), // Over limit
            ),
            Transaction::new(
                "txn-3".to_string(),
                "acc-test".to_string(),
                "salary".to_string(),
                Money::from_milliunits(50000), // Positive, over limit
            ),
        ];

        let query = TransactionQuery::new().with_max_amount(Money::from_milliunits(-3000));

        let filtered = query.filter(&transactions);

        // Should include transactions with amount <= -3000 (i.e., -5000 and -10000, but not 50000)
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().any(|t| t.amount().as_milliunits() == -5000));
        assert!(
            filtered
                .iter()
                .any(|t| t.amount().as_milliunits() == -10000)
        );
        assert!(!filtered.iter().any(|t| t.amount().as_milliunits() == 50000));
    }

    #[test]
    fn should_handle_sorting_with_mixed_date_availability() {
        let transactions = vec![
            Transaction::new(
                "txn-1".to_string(),
                "acc-test".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-5000),
            ), // No date
            Transaction::new_with_date(
                "txn-2".to_string(),
                "acc-test".to_string(),
                "gas".to_string(),
                Money::from_milliunits(-3000),
                "2024-01-20".to_string(),
            ), // Has date
            Transaction::new(
                "txn-3".to_string(),
                "acc-test".to_string(),
                "salary".to_string(),
                Money::from_milliunits(50000),
            ), // No date
        ];

        let query = TransactionQuery::new().sort_by_date();

        let sorted = query.filter(&transactions);

        // Should be sorted: transactions with dates first, then those without
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0].date(), Some("2024-01-20")); // Transaction with date comes first
        assert_eq!(sorted[1].date(), None); // Transactions without dates follow
        assert_eq!(sorted[2].date(), None);
    }

    #[test]
    fn should_apply_sorting_when_sort_by_is_set() {
        let transactions = vec![
            Transaction::new(
                "txn-1".to_string(),
                "acc-test".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-1000), // Smallest expense
            ),
            Transaction::new(
                "txn-2".to_string(),
                "acc-test".to_string(),
                "gas".to_string(),
                Money::from_milliunits(-5000), // Largest expense
            ),
        ];

        // Test that sorting is applied (line 102 coverage)
        let query = TransactionQuery::new().sort_by_amount_descending();

        let sorted = query.filter(&transactions);

        // Should be sorted by amount descending: -1000, -5000
        assert_eq!(sorted.len(), 2);
        assert_eq!(sorted[0].amount().as_milliunits(), -1000); // Closer to zero comes first
        assert_eq!(sorted[1].amount().as_milliunits(), -5000);
    }
}
