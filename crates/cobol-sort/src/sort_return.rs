//! SORT-RETURN special register.
//!
//! Maps COBOL's SORT-RETURN special register to a Rust enum.
//! Set after every SORT or MERGE operation.

/// SORT-RETURN special register value.
///
/// - 0 = successful completion
/// - 16 = sort/merge failed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortReturn {
    /// Sort/merge completed successfully (SORT-RETURN = 0).
    Success,
    /// Sort/merge failed (SORT-RETURN = 16).
    Failed,
}

impl SortReturn {
    /// Get the numeric value matching COBOL's SORT-RETURN register.
    #[must_use]
    pub fn code(&self) -> i32 {
        match self {
            Self::Success => 0,
            Self::Failed => 16,
        }
    }

    /// Whether the sort/merge succeeded.
    #[must_use]
    pub fn is_success(&self) -> bool {
        *self == Self::Success
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_return_codes() {
        assert_eq!(SortReturn::Success.code(), 0);
        assert_eq!(SortReturn::Failed.code(), 16);
    }

    #[test]
    fn test_is_success() {
        assert!(SortReturn::Success.is_success());
        assert!(!SortReturn::Failed.is_success());
    }
}
