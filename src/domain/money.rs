//! Money value object for handling currency amounts.

/// Represents a monetary amount in milliunits (1/1000th of the base currency unit).
///
/// YNAB stores all monetary amounts as milliunits to avoid floating point precision issues.
/// For example, $1.23 would be stored as 1230 milliunits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Money {
    milliunits: i64,
}

impl Money {
    /// Creates a new Money instance from milliunits.
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::Money;
    ///
    /// let amount = Money::from_milliunits(1230);
    /// assert_eq!(amount.as_milliunits(), 1230);
    /// ```
    pub fn from_milliunits(milliunits: i64) -> Self {
        Self { milliunits }
    }

    /// Returns the amount as milliunits.
    pub fn as_milliunits(&self) -> i64 {
        self.milliunits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_money_from_milliunits() {
        let money = Money::from_milliunits(1230);
        assert_eq!(money.as_milliunits(), 1230);
    }

    #[test]
    fn should_handle_negative_amounts() {
        let money = Money::from_milliunits(-500);
        assert_eq!(money.as_milliunits(), -500);
    }
}
