//! SQL Communication Area (SQLCA).
//!
//! Mirrors the IBM COBOL SQLCA structure. Auto-injected into WorkingStorage
//! when any EXEC SQL statement is detected in a COBOL program.

/// SQL Communication Area -- matches IBM COBOL SQLCA layout.
///
/// COBOL programs reference fields directly:
/// - `SQLCODE` -> `sqlca.sqlcode`
/// - `SQLSTATE` -> `sqlca.sqlstate`
/// - `SQLERRD(3)` -> `sqlca.sqlerrd[2]` (1-based to 0-based)
/// - `SQLWARN0` through `SQLWARNA` -> `sqlca.sqlwarn.flags[0..11]`
#[derive(Debug, Clone)]
pub struct Sqlca {
    /// SQL return code: 0 = success, 100 = not found, negative = error.
    pub sqlcode: i32,

    /// ANSI SQL state (5 chars, e.g., "00000" = success, "02000" = not found).
    pub sqlstate: [u8; 5],

    /// Error message area.
    pub sqlerrm: SqlErrm,

    /// Diagnostic information array (6 elements).
    /// - `sqlerrd[0]`: reserved
    /// - `sqlerrd[1]`: reserved
    /// - `sqlerrd[2]`: number of rows affected by last statement
    /// - `sqlerrd[3]`: reserved
    /// - `sqlerrd[4]`: position of syntax error (if applicable)
    /// - `sqlerrd[5]`: reserved
    pub sqlerrd: [i32; 6],

    /// Warning flags.
    pub sqlwarn: SqlWarnings,
}

impl Default for Sqlca {
    fn default() -> Self {
        Self {
            sqlcode: 0,
            sqlstate: *b"00000",
            sqlerrm: SqlErrm::default(),
            sqlerrd: [0; 6],
            sqlwarn: SqlWarnings::default(),
        }
    }
}

impl Sqlca {
    /// Reset all fields to their initial (success) state.
    pub fn reset(&mut self) {
        self.sqlcode = 0;
        self.sqlstate = *b"00000";
        self.sqlerrm.clear();
        self.sqlerrd = [0; 6];
        self.sqlwarn.clear();
    }

    /// Returns true if the last statement succeeded (SQLCODE == 0).
    #[inline]
    pub fn is_ok(&self) -> bool {
        self.sqlcode == 0
    }

    /// Returns true if no rows were found (SQLCODE == 100).
    #[inline]
    pub fn is_not_found(&self) -> bool {
        self.sqlcode == 100
    }

    /// Returns true if an error occurred (SQLCODE < 0).
    #[inline]
    pub fn is_error(&self) -> bool {
        self.sqlcode < 0
    }

    /// Get the number of rows affected by the last statement.
    #[inline]
    pub fn rows_affected(&self) -> i32 {
        self.sqlerrd[2]
    }

    /// Set the SQLCODE and derive the SQLSTATE from it.
    pub fn set_sqlcode(&mut self, code: i32) {
        self.sqlcode = code;
        self.sqlstate = match code {
            0 => *b"00000",
            100 => *b"02000",
            _ if code < 0 => *b"HZ000", // generic error
            _ => *b"00000",
        };
    }

    /// Set an error with message.
    pub fn set_error(&mut self, code: i32, message: &str) {
        self.set_sqlcode(code);
        self.sqlerrm.set(message);
    }
}

/// Error message area within SQLCA.
#[derive(Debug, Clone)]
pub struct SqlErrm {
    /// Length of error message.
    pub sqlerrml: i16,
    /// Error message content (up to 70 bytes, IBM standard).
    pub sqlerrmc: [u8; 70],
}

impl Default for SqlErrm {
    fn default() -> Self {
        Self {
            sqlerrml: 0,
            sqlerrmc: [b' '; 70],
        }
    }
}

impl SqlErrm {
    /// Set the error message, truncating to 70 bytes if necessary.
    pub fn set(&mut self, msg: &str) {
        let bytes = msg.as_bytes();
        let len = bytes.len().min(70);
        self.sqlerrmc[..len].copy_from_slice(&bytes[..len]);
        // Pad remainder with spaces
        self.sqlerrmc[len..].fill(b' ');
        #[allow(clippy::cast_possible_wrap)]
        {
            self.sqlerrml = len as i16; // len <= 70, safe
        }
    }

    /// Clear the error message.
    pub fn clear(&mut self) {
        self.sqlerrml = 0;
        self.sqlerrmc.fill(b' ');
    }

    /// Get the error message as a string (trimmed).
    pub fn as_str(&self) -> &str {
        let len = self.sqlerrml.max(0) as usize;
        std::str::from_utf8(&self.sqlerrmc[..len]).unwrap_or("")
    }
}

/// Warning flags within SQLCA.
///
/// IBM COBOL defines SQLWARN0 through SQLWARNA (11 flags).
/// Each is a single byte: b' ' = no warning, b'W' = warning.
#[derive(Debug, Clone)]
pub struct SqlWarnings {
    /// 11 warning flags (indexes 0-10 map to SQLWARN0-SQLWARNA).
    /// - `[0]` (SQLWARN0): set to 'W' if any other warning flag is set
    /// - `[1]` (SQLWARN1): string column truncated on retrieval
    /// - `[2]` (SQLWARN2): null values eliminated from aggregate function
    /// - `[3]` (SQLWARN3): number of result columns exceeds host variables
    /// - `[4]` (SQLWARN4): prepared UPDATE or DELETE without WHERE
    /// - `[5]` (SQLWARN5): reserved
    /// - `[6]` (SQLWARN6): date arithmetic results adjusted
    /// - `[7]` (SQLWARN7): reserved
    /// - `[8]` (SQLWARN8): character conversion substitution
    /// - `[9]` (SQLWARN9): reserved
    /// - `[10]` (SQLWARNA): reserved
    pub flags: [u8; 11],
}

impl Default for SqlWarnings {
    fn default() -> Self {
        Self { flags: [b' '; 11] }
    }
}

impl SqlWarnings {
    /// Set a warning flag by index (0-10). Automatically sets SQLWARN0.
    pub fn set_flag(&mut self, index: usize) {
        if index < 11 {
            self.flags[index] = b'W';
            if index != 0 {
                self.flags[0] = b'W';
            }
        }
    }

    /// Check if a warning flag is set.
    #[inline]
    pub fn is_set(&self, index: usize) -> bool {
        index < 11 && self.flags[index] == b'W'
    }

    /// Check if any warning is active.
    #[inline]
    pub fn any_warning(&self) -> bool {
        self.flags[0] == b'W'
    }

    /// Clear all warning flags.
    pub fn clear(&mut self) {
        self.flags.fill(b' ');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqlca_default_is_success() {
        let sqlca = Sqlca::default();
        assert!(sqlca.is_ok());
        assert!(!sqlca.is_not_found());
        assert!(!sqlca.is_error());
        assert_eq!(sqlca.sqlcode, 0);
        assert_eq!(&sqlca.sqlstate, b"00000");
        assert_eq!(sqlca.rows_affected(), 0);
    }

    #[test]
    fn sqlca_set_not_found() {
        let mut sqlca = Sqlca::default();
        sqlca.set_sqlcode(100);
        assert!(sqlca.is_not_found());
        assert!(!sqlca.is_ok());
        assert!(!sqlca.is_error());
        assert_eq!(&sqlca.sqlstate, b"02000");
    }

    #[test]
    fn sqlca_set_error() {
        let mut sqlca = Sqlca::default();
        sqlca.set_error(-803, "DUPLICATE KEY");
        assert!(sqlca.is_error());
        assert_eq!(sqlca.sqlcode, -803);
        assert_eq!(sqlca.sqlerrm.as_str(), "DUPLICATE KEY");
    }

    #[test]
    fn sqlca_reset() {
        let mut sqlca = Sqlca::default();
        sqlca.set_error(-1, "error");
        sqlca.sqlerrd[2] = 42;
        sqlca.sqlwarn.set_flag(1);
        sqlca.reset();
        assert!(sqlca.is_ok());
        assert_eq!(sqlca.rows_affected(), 0);
        assert!(!sqlca.sqlwarn.any_warning());
    }

    #[test]
    fn sqlerrm_truncates_at_70() {
        let mut errm = SqlErrm::default();
        let long_msg = "A".repeat(100);
        errm.set(&long_msg);
        assert_eq!(errm.sqlerrml, 70);
        assert_eq!(errm.as_str(), "A".repeat(70));
    }

    #[test]
    fn sqlwarn_flags() {
        let mut warn = SqlWarnings::default();
        assert!(!warn.any_warning());

        warn.set_flag(3);
        assert!(warn.any_warning());
        assert!(warn.is_set(0)); // auto-set
        assert!(warn.is_set(3));
        assert!(!warn.is_set(2));

        warn.clear();
        assert!(!warn.any_warning());
    }

    // -- Additional SQLCA tests --

    #[test]
    fn sqlca_positive_sqlcode_not_100() {
        let mut sqlca = Sqlca::default();
        sqlca.set_sqlcode(1);
        // Positive non-100 codes: is_ok() is false (only 0 is ok),
        // but is_error() is also false (only negative is error)
        assert!(!sqlca.is_ok());
        assert!(!sqlca.is_error());
        assert!(!sqlca.is_not_found());
        assert_eq!(&sqlca.sqlstate, b"00000");
    }

    #[test]
    fn sqlca_sqlerrd_row_count() {
        let mut sqlca = Sqlca::default();
        sqlca.sqlerrd[2] = 42;
        assert_eq!(sqlca.rows_affected(), 42);
    }

    #[test]
    fn sqlca_set_error_derives_sqlstate() {
        let mut sqlca = Sqlca::default();
        sqlca.set_error(-805, "DBRM NOT FOUND");
        assert_eq!(&sqlca.sqlstate, b"HZ000");
        assert!(sqlca.is_error());
    }

    #[test]
    fn sqlca_not_found_sqlstate() {
        let mut sqlca = Sqlca::default();
        sqlca.set_sqlcode(100);
        assert_eq!(&sqlca.sqlstate, b"02000");
    }

    #[test]
    fn sqlerrm_empty_message() {
        let errm = SqlErrm::default();
        assert_eq!(errm.sqlerrml, 0);
        assert_eq!(errm.as_str(), "");
    }

    #[test]
    fn sqlerrm_set_and_clear() {
        let mut errm = SqlErrm::default();
        errm.set("SOME ERROR");
        assert_eq!(errm.sqlerrml, 10);
        assert_eq!(errm.as_str(), "SOME ERROR");

        errm.clear();
        assert_eq!(errm.sqlerrml, 0);
        assert_eq!(errm.as_str(), "");
    }

    #[test]
    fn sqlerrm_exact_70_chars() {
        let mut errm = SqlErrm::default();
        let msg = "B".repeat(70);
        errm.set(&msg);
        assert_eq!(errm.sqlerrml, 70);
        assert_eq!(errm.as_str(), msg);
    }

    #[test]
    fn sqlerrm_pads_with_spaces() {
        let mut errm = SqlErrm::default();
        errm.set("SHORT");
        // Bytes after "SHORT" should be spaces
        assert_eq!(errm.sqlerrmc[5], b' ');
        assert_eq!(errm.sqlerrmc[69], b' ');
    }

    #[test]
    fn sqlwarn_set_flag_0_directly() {
        let mut warn = SqlWarnings::default();
        warn.set_flag(0);
        assert!(warn.is_set(0));
        // Other flags should not be set
        assert!(!warn.is_set(1));
    }

    #[test]
    fn sqlwarn_out_of_bounds() {
        let mut warn = SqlWarnings::default();
        warn.set_flag(11); // out of bounds, should be no-op
        assert!(!warn.any_warning());
        assert!(!warn.is_set(11));
    }

    #[test]
    fn sqlwarn_multiple_flags() {
        let mut warn = SqlWarnings::default();
        warn.set_flag(1); // truncation
        warn.set_flag(4); // no WHERE
        warn.set_flag(8); // char conversion
        assert!(warn.any_warning());
        assert!(warn.is_set(1));
        assert!(warn.is_set(4));
        assert!(warn.is_set(8));
        assert!(!warn.is_set(5));
    }

    #[test]
    fn sqlca_clone() {
        let mut sqlca = Sqlca::default();
        sqlca.set_error(-1, "test");
        sqlca.sqlerrd[2] = 5;
        sqlca.sqlwarn.set_flag(2);

        let cloned = sqlca.clone();
        assert_eq!(cloned.sqlcode, -1);
        assert_eq!(cloned.sqlerrd[2], 5);
        assert!(cloned.sqlwarn.is_set(2));
        assert_eq!(cloned.sqlerrm.as_str(), "test");
    }

    #[test]
    fn sqlca_multiple_errors_reset_between() {
        let mut sqlca = Sqlca::default();

        // First error
        sqlca.set_error(-803, "DUP KEY");
        assert_eq!(sqlca.sqlcode, -803);

        // Reset and set different error
        sqlca.reset();
        sqlca.set_error(-501, "CURSOR NOT OPEN");
        assert_eq!(sqlca.sqlcode, -501);
        assert_eq!(sqlca.sqlerrm.as_str(), "CURSOR NOT OPEN");
    }
}
