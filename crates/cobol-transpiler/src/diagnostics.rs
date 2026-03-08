//! Transpilation diagnostics -- coverage reporting and gap detection.
//!
//! Collected during parsing and code generation to report which COBOL
//! constructs were handled, skipped, or unsupported.

use std::collections::HashSet;

use serde::Serialize;

/// A single diagnostic emitted during transpilation.
#[derive(Debug, Clone, Serialize)]
pub struct TranspileDiagnostic {
    /// COBOL source line number (1-based, 0 if unknown).
    pub line: usize,
    /// Severity level.
    pub severity: Severity,
    /// Diagnostic category.
    pub category: DiagCategory,
    /// Human-readable message.
    pub message: String,
    /// The COBOL source text at the diagnostic location (truncated).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub cobol_text: String,
}

/// Diagnostic severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// Diagnostic category for classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DiagCategory {
    /// ANTLR parse error -- source couldn't be parsed.
    ParseError,
    /// Statement parsed by ANTLR but no AST mapping exists.
    UnhandledStatement,
    /// Clause within a handled statement was silently dropped.
    UnhandledClause,
    /// Recognized feature that is explicitly not implemented.
    UnsupportedFeature,
    /// Data division entry skipped or incomplete.
    DataDivisionGap,
}

/// Aggregate statistics from transpilation.
#[derive(Debug, Clone, Default, Serialize)]
pub struct TranspileStats {
    /// Total statement contexts seen by the ANTLR parser.
    pub total_statements: usize,
    /// Statements successfully mapped to AST nodes.
    pub mapped_statements: usize,
    /// Total data entries parsed.
    pub total_data_entries: usize,
    /// COBOL verbs encountered (uppercase).
    pub verbs_used: HashSet<String>,
    /// COBOL verbs that were skipped (not implemented).
    pub verbs_unsupported: HashSet<String>,
}

/// Full result of transpilation with diagnostics.
#[derive(Debug, Clone)]
pub struct TranspileResult {
    /// Generated Rust source code (None if fatal errors prevented generation).
    pub rust_code: Option<String>,
    /// All diagnostics collected during transpilation.
    pub diagnostics: Vec<TranspileDiagnostic>,
    /// Aggregate statistics.
    pub stats: TranspileStats,
}

impl TranspileResult {
    /// Create a successful result with code and diagnostics.
    pub fn success(code: String, diagnostics: Vec<TranspileDiagnostic>, stats: TranspileStats) -> Self {
        Self {
            rust_code: Some(code),
            diagnostics,
            stats,
        }
    }

    /// Check if there are any errors (not just warnings).
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.severity == Severity::Error)
    }

    /// Get diagnostics filtered by severity.
    pub fn errors(&self) -> Vec<&TranspileDiagnostic> {
        self.diagnostics.iter().filter(|d| d.severity == Severity::Error).collect()
    }

    pub fn warnings(&self) -> Vec<&TranspileDiagnostic> {
        self.diagnostics.iter().filter(|d| d.severity == Severity::Warning).collect()
    }

    /// Coverage percentage: mapped statements / total statements.
    #[allow(clippy::cast_precision_loss)]
    pub fn statement_coverage(&self) -> f64 {
        if self.stats.total_statements == 0 {
            100.0
        } else {
            (self.stats.mapped_statements as f64 / self.stats.total_statements as f64) * 100.0
        }
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => write!(f, "ERROR"),
            Self::Warning => write!(f, "WARN"),
            Self::Info => write!(f, "INFO"),
        }
    }
}

impl std::fmt::Display for DiagCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError => write!(f, "ParseError"),
            Self::UnhandledStatement => write!(f, "UnhandledStatement"),
            Self::UnhandledClause => write!(f, "UnhandledClause"),
            Self::UnsupportedFeature => write!(f, "UnsupportedFeature"),
            Self::DataDivisionGap => write!(f, "DataDivisionGap"),
        }
    }
}
