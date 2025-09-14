//! Payee domain entity.

/// Represents a payee in YNAB (e.g., a merchant or person who receives payments).
#[derive(Debug, Clone, PartialEq)]
pub struct Payee {
    id: String,
    name: String,
}

impl Payee {
    /// Creates a new Payee.
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::Payee;
    ///
    /// let payee = Payee::new("payee-123".to_string(), "Whole Foods".to_string());
    /// assert_eq!(payee.id(), "payee-123");
    /// assert_eq!(payee.name(), "Whole Foods");
    /// ```
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }

    /// Returns the payee ID.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the payee name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_payee_with_id_and_name() {
        // RED: This test should fail initially
        let payee = Payee::new("payee-123".to_string(), "Whole Foods Market".to_string());

        assert_eq!(payee.id(), "payee-123");
        assert_eq!(payee.name(), "Whole Foods Market");
    }

    #[test]
    fn should_handle_empty_payee_name() {
        let payee = Payee::new("payee-456".to_string(), "".to_string());

        assert_eq!(payee.id(), "payee-456");
        assert_eq!(payee.name(), "");
    }

    #[test]
    fn should_support_payee_equality_comparison() {
        let payee1 = Payee::new("payee-123".to_string(), "Whole Foods".to_string());
        let payee2 = Payee::new("payee-123".to_string(), "Whole Foods".to_string());
        let payee3 = Payee::new("payee-456".to_string(), "Target".to_string());

        assert_eq!(payee1, payee2);
        assert_ne!(payee1, payee3);
    }
}
