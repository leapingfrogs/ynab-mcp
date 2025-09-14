//! Account domain entity.

/// Represents different types of accounts in YNAB.
#[derive(Debug, Clone, PartialEq)]
pub enum AccountType {
    Checking,
    Savings,
    CreditCard,
    Cash,
    LineOfCredit,
    OtherAsset,
    OtherLiability,
    Mortgage,
    AutoLoan,
    StudentLoan,
    PersonalLoan,
    MedicalDebt,
    OtherDebt,
}

/// Represents a financial account in YNAB.
#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    id: String,
    name: String,
    account_type: AccountType,
    on_budget: bool,
}

impl Account {
    /// Creates a new Account.
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::{Account, AccountType};
    ///
    /// let account = Account::new(
    ///     "acc-123".to_string(),
    ///     "Checking Account".to_string(),
    ///     AccountType::Checking,
    ///     true
    /// );
    /// assert_eq!(account.id(), "acc-123");
    /// assert_eq!(account.name(), "Checking Account");
    /// assert_eq!(account.account_type(), &AccountType::Checking);
    /// assert_eq!(account.is_on_budget(), true);
    /// ```
    pub fn new(id: String, name: String, account_type: AccountType, on_budget: bool) -> Self {
        Self {
            id,
            name,
            account_type,
            on_budget,
        }
    }

    /// Returns the account ID.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the account name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the account type.
    pub fn account_type(&self) -> &AccountType {
        &self.account_type
    }

    /// Returns whether this account is tracked on the budget.
    pub fn is_on_budget(&self) -> bool {
        self.on_budget
    }

    /// Returns whether this account is a liability (debt) account.
    pub fn is_liability(&self) -> bool {
        matches!(
            self.account_type,
            AccountType::CreditCard
                | AccountType::LineOfCredit
                | AccountType::OtherLiability
                | AccountType::Mortgage
                | AccountType::AutoLoan
                | AccountType::StudentLoan
                | AccountType::PersonalLoan
                | AccountType::MedicalDebt
                | AccountType::OtherDebt
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_account_with_all_properties() {
        // RED: This test should fail initially
        let account = Account::new(
            "acc-123".to_string(),
            "My Checking Account".to_string(),
            AccountType::Checking,
            true,
        );

        assert_eq!(account.id(), "acc-123");
        assert_eq!(account.name(), "My Checking Account");
        assert_eq!(account.account_type(), &AccountType::Checking);
        assert!(account.is_on_budget());
    }

    #[test]
    fn should_create_off_budget_account() {
        let account = Account::new(
            "acc-456".to_string(),
            "Investment Account".to_string(),
            AccountType::OtherAsset,
            false,
        );

        assert!(!account.is_on_budget());
        assert_eq!(account.account_type(), &AccountType::OtherAsset);
    }

    #[test]
    fn should_identify_liability_accounts() {
        let credit_card = Account::new(
            "acc-cc".to_string(),
            "Credit Card".to_string(),
            AccountType::CreditCard,
            true,
        );
        let mortgage = Account::new(
            "acc-mort".to_string(),
            "Mortgage".to_string(),
            AccountType::Mortgage,
            false,
        );
        let checking = Account::new(
            "acc-check".to_string(),
            "Checking".to_string(),
            AccountType::Checking,
            true,
        );

        assert!(credit_card.is_liability());
        assert!(mortgage.is_liability());
        assert!(!checking.is_liability());
    }

    #[test]
    fn should_support_all_account_types() {
        let types = vec![
            AccountType::Checking,
            AccountType::Savings,
            AccountType::CreditCard,
            AccountType::Cash,
            AccountType::LineOfCredit,
            AccountType::OtherAsset,
            AccountType::OtherLiability,
            AccountType::Mortgage,
            AccountType::AutoLoan,
            AccountType::StudentLoan,
            AccountType::PersonalLoan,
            AccountType::MedicalDebt,
            AccountType::OtherDebt,
        ];

        for account_type in types {
            let account = Account::new(
                "test-id".to_string(),
                "Test Account".to_string(),
                account_type.clone(),
                true,
            );
            assert_eq!(account.account_type(), &account_type);
        }
    }

    #[test]
    fn should_support_account_equality_comparison() {
        let account1 = Account::new(
            "acc-123".to_string(),
            "Checking".to_string(),
            AccountType::Checking,
            true,
        );
        let account2 = Account::new(
            "acc-123".to_string(),
            "Checking".to_string(),
            AccountType::Checking,
            true,
        );
        let account3 = Account::new(
            "acc-456".to_string(),
            "Savings".to_string(),
            AccountType::Savings,
            true,
        );

        assert_eq!(account1, account2);
        assert_ne!(account1, account3);
    }
}
