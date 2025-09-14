//! Date range value object for filtering transactions by date.

/// Represents a date range for filtering transactions.
#[derive(Debug, Clone, PartialEq)]
pub struct DateRange {
    start: String, // Using String for simplicity, following TDD
    end: String,
}

impl DateRange {
    /// Creates a new DateRange.
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::DateRange;
    ///
    /// let range = DateRange::new("2024-01-01".to_string(), "2024-01-31".to_string());
    /// assert_eq!(range.start(), "2024-01-01");
    /// assert_eq!(range.end(), "2024-01-31");
    /// ```
    pub fn new(start: String, end: String) -> Self {
        Self { start, end }
    }

    /// Returns the start date.
    pub fn start(&self) -> &str {
        &self.start
    }

    /// Returns the end date.
    pub fn end(&self) -> &str {
        &self.end
    }

    /// Checks if a date falls within this range (inclusive).
    pub fn contains(&self, date: &str) -> bool {
        date >= &self.start && date <= &self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_date_range() {
        let range = DateRange::new("2024-01-01".to_string(), "2024-01-31".to_string());

        assert_eq!(range.start(), "2024-01-01");
        assert_eq!(range.end(), "2024-01-31");
    }

    #[test]
    fn should_check_if_date_is_within_range() {
        let range = DateRange::new("2024-01-01".to_string(), "2024-01-31".to_string());

        assert!(range.contains("2024-01-15"));
        assert!(range.contains("2024-01-01")); // Inclusive start
        assert!(range.contains("2024-01-31")); // Inclusive end
        assert!(!range.contains("2023-12-31")); // Before range
        assert!(!range.contains("2024-02-01")); // After range
    }
}
