//! Category domain entity.

use crate::domain::{DateRange, Money, Transaction};

/// Represents a budget category in YNAB.
#[derive(Debug, Clone, PartialEq)]
pub struct Category {
    id: String,
    name: String,
}

impl Category {
    /// Creates a new Category.
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::Category;
    ///
    /// let category = Category::new("groceries".to_string(), "Groceries".to_string());
    /// assert_eq!(category.id(), "groceries");
    /// assert_eq!(category.name(), "Groceries");
    /// ```
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }

    /// Returns the category ID.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the category name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Calculates the total spending for this category from a list of transactions.
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::{Category, Transaction, Money};
    ///
    /// let category = Category::new("groceries".to_string(), "Groceries".to_string());
    /// let transactions = vec![
    ///     Transaction::new("txn-1".to_string(), "groceries".to_string(), Money::from_milliunits(-5000)),
    /// ];
    /// let spending = category.calculate_spending(&transactions);
    /// assert_eq!(spending, Money::from_milliunits(-5000));
    /// ```
    pub fn calculate_spending(&self, transactions: &[Transaction]) -> Money {
        transactions
            .iter()
            .filter(|t| t.category_id() == self.id)
            .map(|t| t.amount())
            .sum()
    }

    /// Calculates the total spending for this category with optional date filtering.
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::{Category, Transaction, Money, DateRange};
    ///
    /// let category = Category::new("groceries".to_string(), "Groceries".to_string());
    /// let transactions = vec![
    ///     Transaction::new_with_date("txn-1".to_string(), "groceries".to_string(),
    ///                               Money::from_milliunits(-5000), "2024-01-15".to_string()),
    /// ];
    /// let date_range = Some(DateRange::new("2024-01-01".to_string(), "2024-01-31".to_string()));
    /// let spending = category.calculate_spending_with_date_filter(&transactions, date_range);
    /// assert_eq!(spending, Money::from_milliunits(-5000));
    /// ```
    pub fn calculate_spending_with_date_filter(
        &self,
        transactions: &[Transaction],
        date_range: Option<DateRange>,
    ) -> Money {
        transactions
            .iter()
            .filter(|t| t.category_id() == self.id)
            .filter(|t| {
                if let Some(ref range) = date_range {
                    if let Some(date) = t.date() {
                        range.contains(date)
                    } else {
                        false // Exclude transactions without dates when filtering
                    }
                } else {
                    true // Include all transactions when no date filter
                }
            })
            .map(|t| t.amount())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Money, Transaction};

    #[test]
    fn should_create_category_with_id_and_name() {
        let category = Category::new("test-id".to_string(), "Test Category".to_string());

        assert_eq!(category.id(), "test-id");
        assert_eq!(category.name(), "Test Category");
    }

    #[test]
    fn should_calculate_category_spending_for_single_transaction() {
        let category = Category::new("groceries".to_string(), "Groceries".to_string());
        let transactions = vec![Transaction::new(
            "txn-1".to_string(),
            "groceries".to_string(),
            Money::from_milliunits(-5000), // $5.00 expense
        )];

        let spending = category.calculate_spending(&transactions);

        assert_eq!(spending, Money::from_milliunits(-5000));
    }

    #[test]
    fn should_calculate_category_spending_for_multiple_transactions() {
        let category = Category::new("groceries".to_string(), "Groceries".to_string());
        let transactions = vec![
            Transaction::new(
                "txn-1".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-3000), // $3.00 expense
            ),
            Transaction::new(
                "txn-2".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-2000), // $2.00 expense
            ),
            Transaction::new(
                "txn-3".to_string(),
                "gas".to_string(), // Different category - should be ignored
                Money::from_milliunits(-4000),
            ),
        ];

        let spending = category.calculate_spending(&transactions);

        // Should only include groceries transactions: -3000 + -2000 = -5000
        assert_eq!(spending, Money::from_milliunits(-5000));
    }

    #[test]
    fn should_calculate_category_spending_with_date_filtering() {
        let category = Category::new("groceries".to_string(), "Groceries".to_string());
        let transactions = vec![
            Transaction::new_with_date(
                "txn-1".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-3000),
                "2024-01-15".to_string(),
            ),
            Transaction::new_with_date(
                "txn-2".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-2000),
                "2024-01-25".to_string(),
            ),
            Transaction::new_with_date(
                "txn-3".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-1000),
                "2024-02-05".to_string(), // Outside date range
            ),
        ];

        let date_range = Some(crate::domain::DateRange::new(
            "2024-01-01".to_string(),
            "2024-01-31".to_string(),
        ));

        let spending = category.calculate_spending_with_date_filter(&transactions, date_range);

        // Should only include January transactions: -3000 + -2000 = -5000
        assert_eq!(spending, Money::from_milliunits(-5000));
    }

    #[test]
    fn should_exclude_transactions_without_dates_when_date_filtering() {
        let category = Category::new("groceries".to_string(), "Groceries".to_string());
        let transactions = vec![
            Transaction::new_with_date(
                "txn-1".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-3000),
                "2024-01-15".to_string(),
            ),
            Transaction::new(
                "txn-2".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-2000), // No date - should be excluded
            ),
        ];

        let date_range = Some(crate::domain::DateRange::new(
            "2024-01-01".to_string(),
            "2024-01-31".to_string(),
        ));

        let spending = category.calculate_spending_with_date_filter(&transactions, date_range);

        // Should only include transaction with date: -3000
        assert_eq!(spending, Money::from_milliunits(-3000));
    }

    #[test]
    fn should_include_all_transactions_when_no_date_filter() {
        let category = Category::new("groceries".to_string(), "Groceries".to_string());
        let transactions = vec![
            Transaction::new_with_date(
                "txn-1".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-3000),
                "2024-01-15".to_string(),
            ),
            Transaction::new(
                "txn-2".to_string(),
                "groceries".to_string(),
                Money::from_milliunits(-2000), // No date but should be included
            ),
        ];

        let spending = category.calculate_spending_with_date_filter(&transactions, None);

        // Should include all transactions: -3000 + -2000 = -5000
        assert_eq!(spending, Money::from_milliunits(-5000));
    }
}
