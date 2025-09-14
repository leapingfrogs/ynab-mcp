//! YNAB API client.

/// Placeholder for YNAB API client.
/// This will be implemented in later iterations following TDD.
pub struct YnabClient;

impl YnabClient {
    /// Creates a new YNAB client instance.
    pub fn new() -> Self {
        Self
    }
}

impl Default for YnabClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_ynab_client_with_new() {
        let client = YnabClient::new();

        // Basic creation test - placeholder will be expanded in future iterations
        assert_eq!(std::mem::size_of_val(&client), 0); // Zero-sized struct
    }

    #[test]
    fn should_create_ynab_client_with_default() {
        let _client = YnabClient;

        // Test that we can create via Default trait - clippy prefers direct construction for unit structs
        let _default_client: YnabClient = Default::default();
    }
}
