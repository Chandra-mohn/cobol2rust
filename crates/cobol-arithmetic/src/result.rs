/// Result of an arithmetic operation.
///
/// Contains both the outcome and whether a size error occurred.
/// Used to drive ON SIZE ERROR / NOT ON SIZE ERROR logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArithResult {
    /// True if the result exceeded the destination capacity.
    pub size_error: bool,
}

impl ArithResult {
    /// Successful operation (no size error).
    pub fn ok() -> Self {
        Self { size_error: false }
    }

    /// Operation caused a size error.
    pub fn size_error() -> Self {
        Self { size_error: true }
    }
}
