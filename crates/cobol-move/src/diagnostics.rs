/// Move diagnostic level (mirrors `DiagnosticLevel` from config but specific to MOVE).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveDiagnostic {
    /// No diagnostics (production mode -- matches COBOL behavior)
    Silent,
    /// Log warnings for data loss (development/testing mode)
    Warn,
    /// Panic on data loss (strict testing mode)
    Strict,
}

/// Emitted when MOVE causes data loss.
#[derive(Debug)]
pub struct MoveWarning {
    pub kind: MoveWarningKind,
    pub source_desc: String,
    pub dest_desc: String,
}

/// Type of data loss during MOVE.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveWarningKind {
    /// Integer digits lost (left-truncation)
    LeftTruncation,
    /// Decimal digits lost (right-truncation)
    RightTruncation,
    /// Signed -> unsigned, value was negative
    SignLoss,
    /// Alphanumeric source contained non-numeric chars
    InvalidNumericData,
    /// Group move may corrupt subordinate numeric fields
    GroupOverlay,
}

impl MoveWarning {
    /// Handle the warning according to the diagnostic level.
    pub fn handle(&self, level: MoveDiagnostic) {
        match level {
            MoveDiagnostic::Silent => {}
            MoveDiagnostic::Warn => {
                eprintln!(
                    "[MOVE WARNING] {:?}: {} -> {}",
                    self.kind, self.source_desc, self.dest_desc
                );
            }
            MoveDiagnostic::Strict => {
                panic!(
                    "MOVE data loss ({:?}): {} -> {}",
                    self.kind, self.source_desc, self.dest_desc
                );
            }
        }
    }
}
