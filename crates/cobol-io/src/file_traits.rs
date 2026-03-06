//! Core traits and enums for COBOL file I/O.
//!
//! `CobolFile` is the unified interface for all file organizations
//! (sequential, indexed, relative). Each operation returns a `FileStatusCode`
//! matching COBOL's FILE STATUS semantics.

use crate::file_status::FileStatusCode;

/// File organization (SELECT ... ORGANIZATION IS ...).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileOrganization {
    /// SEQUENTIAL (fixed-length records, no key).
    Sequential,
    /// LINE SEQUENTIAL (text lines, newline-delimited).
    LineSequential,
    /// INDEXED (keyed access, backed by `SQLite` in our implementation).
    Indexed,
    /// RELATIVE (slot-based, integer key).
    Relative,
}

/// File open mode (OPEN INPUT/OUTPUT/I-O/EXTEND).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileOpenMode {
    /// OPEN INPUT -- read only.
    Input,
    /// OPEN OUTPUT -- write only (creates/truncates).
    Output,
    /// OPEN I-O -- read and write.
    InputOutput,
    /// OPEN EXTEND -- append.
    Extend,
}

/// File access mode (ACCESS MODE IS ...).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileAccessMode {
    /// SEQUENTIAL -- records accessed in order.
    Sequential,
    /// RANDOM -- records accessed by key.
    Random,
    /// DYNAMIC -- both sequential and random access.
    Dynamic,
}

/// Unified COBOL file interface.
///
/// Every COBOL I/O verb maps to a method here.
/// All methods return a `FileStatusCode` to match COBOL's
/// FILE STATUS checking semantics.
pub trait CobolFile {
    /// OPEN verb.
    fn open(&mut self, mode: FileOpenMode) -> FileStatusCode;

    /// CLOSE verb.
    fn close(&mut self) -> FileStatusCode;

    /// READ verb (sequential read: next record).
    ///
    /// Reads the next record into the internal buffer.
    /// Returns the record data and status.
    fn read_next(&mut self) -> (FileStatusCode, Option<Vec<u8>>);

    /// WRITE verb.
    ///
    /// Writes the given record data.
    fn write_record(&mut self, record: &[u8]) -> FileStatusCode;

    /// REWRITE verb (update current record).
    fn rewrite_record(&mut self, record: &[u8]) -> FileStatusCode;

    /// The file's organization.
    fn organization(&self) -> FileOrganization;

    /// The current access mode.
    fn access_mode(&self) -> FileAccessMode;

    /// Whether the file is currently open.
    fn is_open(&self) -> bool;

    /// The COBOL SELECT name for this file.
    fn select_name(&self) -> &str;

    /// The record length (fixed-length files).
    fn record_length(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_organization_values() {
        assert_ne!(FileOrganization::Sequential, FileOrganization::Indexed);
        assert_ne!(FileOrganization::LineSequential, FileOrganization::Relative);
    }

    #[test]
    fn open_mode_values() {
        assert_ne!(FileOpenMode::Input, FileOpenMode::Output);
        assert_ne!(FileOpenMode::InputOutput, FileOpenMode::Extend);
    }

    #[test]
    fn access_mode_values() {
        assert_ne!(FileAccessMode::Sequential, FileAccessMode::Random);
        assert_ne!(FileAccessMode::Dynamic, FileAccessMode::Sequential);
    }
}
