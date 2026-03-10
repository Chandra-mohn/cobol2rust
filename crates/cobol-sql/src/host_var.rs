//! Host variable binding for EXEC SQL statements.
//!
//! In COBOL, host variables are WORKING-STORAGE fields referenced inside
//! EXEC SQL blocks with a `:` prefix. They serve as parameters (input)
//! and result targets (output) for SQL operations.
//!
//! The transpiler resolves `:WS-FIELD` references to WorkingStorage fields
//! and passes them to the `CobolSqlRuntime` trait methods via `HostVar`
//! (read-only, for input params) and `HostVarMut` (read-write, for
//! SELECT INTO targets).

use std::fmt;

use cobol_core::CobolField;

/// A read-only host variable reference (input parameter direction).
///
/// Used for WHERE clause parameters, INSERT VALUES, etc.
/// Optionally includes a null indicator field.
///
pub struct HostVar<'a> {
    /// The COBOL field providing the value.
    pub field: &'a dyn CobolField,
    /// Optional null indicator. Negative value means NULL.
    pub indicator: Option<&'a dyn CobolField>,
}

impl fmt::Debug for HostVar<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HostVar")
            .field("field_bytes", &self.field.as_bytes())
            .field("has_indicator", &self.indicator.is_some())
            .finish()
    }
}

impl<'a> HostVar<'a> {
    /// Create a host variable without a null indicator.
    #[inline]
    pub fn new(field: &'a dyn CobolField) -> Self {
        Self {
            field,
            indicator: None,
        }
    }

    /// Create a host variable with a null indicator.
    #[inline]
    pub fn with_indicator(field: &'a dyn CobolField, indicator: &'a dyn CobolField) -> Self {
        Self {
            field,
            indicator: Some(indicator),
        }
    }

    /// Returns true if the indicator signals NULL (negative value).
    ///
    /// Reads the indicator field bytes as a display numeric and checks
    /// for a negative value. If no indicator is present, returns false.
    pub fn is_null(&self) -> bool {
        if let Some(ind) = self.indicator {
            // Indicator fields are typically PIC S9(4) COMP.
            // Check if first byte indicates negative sign in display format,
            // or parse the trimmed bytes as a number.
            let bytes = ind.as_bytes();
            let s = std::str::from_utf8(bytes).unwrap_or("0").trim();
            s.starts_with('-') && s != "-0" && s != "-00" && s != "-000" && s != "-0000"
        } else {
            false
        }
    }

    /// Get the field value as bytes for the SQL runtime to read.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.field.as_bytes()
    }
}

/// A mutable host variable reference (output/bidirectional direction).
///
/// Used for SELECT INTO targets and FETCH results.
/// Optionally includes a mutable null indicator field.
///
pub struct HostVarMut<'a> {
    /// The COBOL field to write results into.
    pub field: &'a mut dyn CobolField,
    /// Optional null indicator. Set to -1 for NULL, 0 for non-NULL,
    /// positive for truncation length.
    pub indicator: Option<&'a mut dyn CobolField>,
}

impl fmt::Debug for HostVarMut<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HostVarMut")
            .field("field_bytes", &self.field.as_bytes())
            .field("has_indicator", &self.indicator.is_some())
            .finish()
    }
}

impl<'a> HostVarMut<'a> {
    /// Create a mutable host variable without a null indicator.
    #[inline]
    pub fn new(field: &'a mut dyn CobolField) -> Self {
        Self {
            field,
            indicator: None,
        }
    }

    /// Create a mutable host variable with a null indicator.
    #[inline]
    pub fn with_indicator(
        field: &'a mut dyn CobolField,
        indicator: &'a mut dyn CobolField,
    ) -> Self {
        Self {
            field,
            indicator: Some(indicator),
        }
    }
}
