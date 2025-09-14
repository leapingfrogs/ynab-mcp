//! Budget domain entity.

/// Represents a YNAB budget.
#[derive(Debug, Clone, PartialEq)]
pub struct Budget {
    id: String,
    name: String,
}

impl Budget {
    /// Creates a new Budget.
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::Budget;
    ///
    /// let budget = Budget::new("my-budget".to_string(), "My Budget".to_string());
    /// assert_eq!(budget.id(), "my-budget");
    /// assert_eq!(budget.name(), "My Budget");
    /// ```
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }

    /// Returns the budget ID.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the budget name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_budget_with_id_and_name() {
        let budget = Budget::new("test-budget".to_string(), "Test Budget".to_string());

        assert_eq!(budget.id(), "test-budget");
        assert_eq!(budget.name(), "Test Budget");
    }
}
