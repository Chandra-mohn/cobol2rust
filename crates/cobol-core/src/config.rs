use serde::{Deserialize, Serialize};

/// Target COBOL dialect for runtime behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CobolDialect {
    IbmEnterprise,
    GnuCobol,
    MicroFocus,
    AnsiStandard,
}

/// NUMPROC compiler option (IBM-specific sign handling).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NumProc {
    /// Correct signs on MOVE (default)
    Nopfd,
    /// Assume valid signs (performance optimization)
    Pfd,
    /// Migration mode
    Mig,
}

/// ARITH compiler option (IBM intermediate result precision).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArithMode {
    /// 18-digit intermediates (IBM ARITH(COMPAT))
    Compat,
    /// 31-digit intermediates (IBM ARITH(EXTEND))
    Extend,
}

/// Diagnostic level for MOVE and arithmetic operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticLevel {
    /// No diagnostics -- matches COBOL production behavior
    Silent,
    /// Log warnings for data loss (development/testing)
    Warn,
    /// Panic on data loss (strict testing mode)
    Strict,
}

/// Collating sequence for comparisons and SORT operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollatingSequence {
    /// Native platform sequence (ASCII in Rust)
    Native,
    /// EBCDIC collating order (for migrated programs)
    Ebcdic,
    /// Custom collating sequence (256-byte mapping table)
    Custom(Box<[u8; 256]>),
}

/// COBOL ROUNDED phrase modes.
///
/// IBM Enterprise COBOL v4+ and COBOL 2014 standard define 8 rounding modes.
/// Default (when ROUNDED is specified without MODE) is `NearestAwayFromZero`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundingMode {
    /// Always round away from zero (ceiling of absolute value)
    AwayFromZero,
    /// Round to nearest; ties round away from zero (default ROUNDED)
    NearestAwayFromZero,
    /// Round to nearest; ties round to even digit ("banker's rounding")
    NearestEven,
    /// Round to nearest; ties round toward zero
    NearestTowardZero,
    /// Round toward positive infinity (ceiling)
    TowardGreater,
    /// Round toward negative infinity (floor)
    TowardLesser,
    /// Truncate toward zero (same as no ROUNDED)
    Truncation,
    /// Prohibited -- raise SIZE ERROR if rounding needed
    Prohibited,
}

/// Master runtime configuration.
///
/// Controls dialect-specific behavior for the entire COBOL runtime.
/// A single `RuntimeConfig` is shared across all operations in a program.
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub dialect: CobolDialect,
    pub numproc: NumProc,
    pub arith_mode: ArithMode,
    pub diagnostic_level: DiagnosticLevel,
    pub allow_de_editing: bool,
    pub default_collating: CollatingSequence,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            dialect: CobolDialect::IbmEnterprise,
            numproc: NumProc::Nopfd,
            arith_mode: ArithMode::Compat,
            diagnostic_level: DiagnosticLevel::Silent,
            allow_de_editing: true,
            default_collating: CollatingSequence::Native,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_ibm_enterprise() {
        let config = RuntimeConfig::default();
        assert_eq!(config.dialect, CobolDialect::IbmEnterprise);
        assert_eq!(config.numproc, NumProc::Nopfd);
        assert_eq!(config.arith_mode, ArithMode::Compat);
        assert_eq!(config.diagnostic_level, DiagnosticLevel::Silent);
        assert!(config.allow_de_editing);
        assert_eq!(config.default_collating, CollatingSequence::Native);
    }
}
