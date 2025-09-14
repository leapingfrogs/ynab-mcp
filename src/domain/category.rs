//! Category domain entity.

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_category_with_id_and_name() {
        let category = Category::new("test-id".to_string(), "Test Category".to_string());

        assert_eq!(category.id(), "test-id");
        assert_eq!(category.name(), "Test Category");
    }
}
