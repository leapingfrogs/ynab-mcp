//! Category group domain entity.

/// Represents a group of categories in YNAB (e.g., "Monthly Bills", "Everyday Expenses").
#[derive(Debug, Clone, PartialEq)]
pub struct CategoryGroup {
    id: String,
    name: String,
    hidden: bool,
}

impl CategoryGroup {
    /// Creates a new CategoryGroup.
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::CategoryGroup;
    ///
    /// let group = CategoryGroup::new("grp-123".to_string(), "Monthly Bills".to_string());
    /// assert_eq!(group.id(), "grp-123");
    /// assert_eq!(group.name(), "Monthly Bills");
    /// assert_eq!(group.is_hidden(), false);
    /// ```
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            hidden: false,
        }
    }

    /// Creates a new CategoryGroup with visibility setting.
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::CategoryGroup;
    ///
    /// let group = CategoryGroup::new_with_visibility(
    ///     "grp-456".to_string(),
    ///     "Hidden Group".to_string(),
    ///     true
    /// );
    /// assert_eq!(group.is_hidden(), true);
    /// ```
    pub fn new_with_visibility(id: String, name: String, hidden: bool) -> Self {
        Self { id, name, hidden }
    }

    /// Returns the category group ID.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the category group name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether this category group is hidden.
    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    /// Sets the hidden status of this category group.
    pub fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_category_group_with_id_and_name() {
        // RED: This test should fail initially
        let group = CategoryGroup::new("grp-123".to_string(), "Monthly Bills".to_string());

        assert_eq!(group.id(), "grp-123");
        assert_eq!(group.name(), "Monthly Bills");
        assert!(!group.is_hidden()); // Default should be visible
    }

    #[test]
    fn should_create_category_group_with_visibility() {
        let visible_group = CategoryGroup::new_with_visibility(
            "grp-456".to_string(),
            "Everyday Expenses".to_string(),
            false,
        );
        let hidden_group = CategoryGroup::new_with_visibility(
            "grp-789".to_string(),
            "Hidden Group".to_string(),
            true,
        );

        assert!(!visible_group.is_hidden());
        assert!(hidden_group.is_hidden());
    }

    #[test]
    fn should_allow_changing_visibility() {
        let mut group = CategoryGroup::new("grp-123".to_string(), "Test Group".to_string());

        assert!(!group.is_hidden());

        group.set_hidden(true);
        assert!(group.is_hidden());

        group.set_hidden(false);
        assert!(!group.is_hidden());
    }

    #[test]
    fn should_support_category_group_equality_comparison() {
        let group1 = CategoryGroup::new("grp-123".to_string(), "Monthly Bills".to_string());
        let group2 = CategoryGroup::new("grp-123".to_string(), "Monthly Bills".to_string());
        let group3 = CategoryGroup::new("grp-456".to_string(), "Everyday Expenses".to_string());

        assert_eq!(group1, group2);
        assert_ne!(group1, group3);
    }

    #[test]
    fn should_handle_empty_group_name() {
        let group = CategoryGroup::new("grp-empty".to_string(), "".to_string());

        assert_eq!(group.name(), "");
        assert_eq!(group.id(), "grp-empty");
    }
}
