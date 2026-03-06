//! INPUT/OUTPUT PROCEDURE handles for SORT.
//!
//! In COBOL, a SORT statement can specify:
//! - INPUT PROCEDURE: a section that calls RELEASE to feed records into the sort
//! - OUTPUT PROCEDURE: a section that calls RETURN to consume sorted records
//!
//! This module provides `Releaser` and `Returner` types that act as the
//! interface between the user's procedure code and the sort engine.

use cobol_core::SortError;

use crate::collating::CollatingTable;
use crate::config::SortConfig;
use crate::sort_engine::CobolSortEngine;
use crate::sort_key::SortKeySpec;
use crate::sort_return::SortReturn;

/// Handle for the RELEASE verb in an INPUT PROCEDURE.
///
/// The INPUT PROCEDURE receives a `&mut Releaser` and calls `release()`
/// for each record it wants to feed into the sort.
#[allow(missing_debug_implementations)]
pub struct Releaser {
    records: Vec<Vec<u8>>,
}

impl Releaser {
    /// Create a new empty releaser.
    fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    /// RELEASE verb: push a record into the sort input.
    pub fn release(&mut self, record: &[u8]) {
        self.records.push(record.to_vec());
    }

    /// Consume the releaser and return all collected records.
    fn into_records(self) -> Vec<Vec<u8>> {
        self.records
    }
}

/// Handle for the RETURN verb in an OUTPUT PROCEDURE.
///
/// The OUTPUT PROCEDURE receives a `&mut Returner` and calls `return_record()`
/// to get each sorted record. Returns `None` when all records are consumed
/// (AT END condition).
#[allow(missing_debug_implementations)]
pub struct Returner {
    records: Vec<Vec<u8>>,
    cursor: usize,
}

impl Returner {
    /// Create a returner from a vec of sorted records.
    fn new(records: Vec<Vec<u8>>) -> Self {
        Self { records, cursor: 0 }
    }

    /// RETURN verb: get the next sorted record.
    ///
    /// Returns `Some(record)` for the next record, or `None` at end (AT END).
    pub fn return_record(&mut self) -> Option<&[u8]> {
        if self.cursor < self.records.len() {
            let rec = &self.records[self.cursor];
            self.cursor += 1;
            Some(rec)
        } else {
            None
        }
    }

    /// Whether all records have been consumed (AT END condition).
    #[must_use]
    pub fn is_at_end(&self) -> bool {
        self.cursor >= self.records.len()
    }
}

/// Execute a SORT with INPUT PROCEDURE and OUTPUT PROCEDURE.
///
/// 1. Calls `input_proc` with a `Releaser` -- the procedure calls `release()` for each record
/// 2. Sorts all released records using the sort engine
/// 3. Calls `output_proc` with a `Returner` -- the procedure calls `return_record()` to consume
///
/// # Errors
/// Returns `SortError` if the sort operation fails.
pub fn sort_with_procedures<FI, FO>(
    keys: &[SortKeySpec],
    config: &SortConfig,
    collating: Option<&CollatingTable>,
    input_proc: FI,
    output_proc: FO,
) -> Result<SortReturn, SortError>
where
    FI: FnOnce(&mut Releaser),
    FO: FnOnce(&mut Returner),
{
    // Phase 1: INPUT PROCEDURE collects records via Releaser
    let mut releaser = Releaser::new();
    input_proc(&mut releaser);
    let records = releaser.into_records();

    // Phase 2: Sort the records
    let mut engine = CobolSortEngine::new(keys, config.clone(), collating);
    for record in records {
        engine.add_record(record)?;
    }
    let sorted = engine.finish()?;

    // Phase 3: OUTPUT PROCEDURE consumes sorted records via Returner
    let mut returner = Returner::new(sorted);
    output_proc(&mut returner);

    Ok(SortReturn::Success)
}

/// Execute a SORT with INPUT PROCEDURE only (GIVING files handled externally).
///
/// Returns the sorted records as a `Vec`.
///
/// # Errors
/// Returns `SortError` if the sort operation fails.
pub fn sort_with_input_procedure<FI>(
    keys: &[SortKeySpec],
    config: &SortConfig,
    collating: Option<&CollatingTable>,
    input_proc: FI,
) -> Result<Vec<Vec<u8>>, SortError>
where
    FI: FnOnce(&mut Releaser),
{
    let mut releaser = Releaser::new();
    input_proc(&mut releaser);
    let records = releaser.into_records();

    let mut engine = CobolSortEngine::new(keys, config.clone(), collating);
    for record in records {
        engine.add_record(record)?;
    }
    engine.finish()
}

/// Execute a SORT with OUTPUT PROCEDURE only (USING files provide input).
///
/// Takes pre-read records, sorts them, then passes to the output procedure.
///
/// # Errors
/// Returns `SortError` if the sort operation fails.
pub fn sort_with_output_procedure<FO>(
    keys: &[SortKeySpec],
    config: &SortConfig,
    collating: Option<&CollatingTable>,
    records: Vec<Vec<u8>>,
    output_proc: FO,
) -> Result<SortReturn, SortError>
where
    FO: FnOnce(&mut Returner),
{
    let mut engine = CobolSortEngine::new(keys, config.clone(), collating);
    for record in records {
        engine.add_record(record)?;
    }
    let sorted = engine.finish()?;

    let mut returner = Returner::new(sorted);
    output_proc(&mut returner);

    Ok(SortReturn::Success)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_releaser_basic() {
        let mut releaser = Releaser::new();
        releaser.release(b"RECORD1");
        releaser.release(b"RECORD2");
        releaser.release(b"RECORD3");

        let records = releaser.into_records();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0], b"RECORD1");
    }

    #[test]
    fn test_returner_basic() {
        let records = vec![b"AAA".to_vec(), b"BBB".to_vec(), b"CCC".to_vec()];
        let mut returner = Returner::new(records);

        assert!(!returner.is_at_end());
        assert_eq!(returner.return_record(), Some(b"AAA".as_slice()));
        assert_eq!(returner.return_record(), Some(b"BBB".as_slice()));
        assert_eq!(returner.return_record(), Some(b"CCC".as_slice()));
        assert_eq!(returner.return_record(), None);
        assert!(returner.is_at_end());
    }

    #[test]
    fn test_sort_with_procedures() {
        let keys = vec![SortKeySpec::alphanumeric(0, 3)];
        let config = SortConfig::new(3);

        let mut output_records = Vec::new();

        let result = sort_with_procedures(
            &keys,
            &config,
            None,
            |releaser| {
                releaser.release(b"CCC");
                releaser.release(b"AAA");
                releaser.release(b"BBB");
            },
            |returner| {
                while let Some(rec) = returner.return_record() {
                    output_records.push(rec.to_vec());
                }
            },
        )
        .unwrap();

        assert_eq!(result, SortReturn::Success);
        assert_eq!(output_records.len(), 3);
        assert_eq!(output_records[0], b"AAA");
        assert_eq!(output_records[1], b"BBB");
        assert_eq!(output_records[2], b"CCC");
    }

    #[test]
    fn test_sort_with_input_procedure_only() {
        let keys = vec![SortKeySpec::alphanumeric(0, 3)];
        let config = SortConfig::new(3);

        let sorted = sort_with_input_procedure(&keys, &config, None, |releaser| {
            releaser.release(b"ZZZ");
            releaser.release(b"AAA");
            releaser.release(b"MMM");
        })
        .unwrap();

        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0], b"AAA");
        assert_eq!(sorted[1], b"MMM");
        assert_eq!(sorted[2], b"ZZZ");
    }

    #[test]
    fn test_sort_with_output_procedure_only() {
        let keys = vec![SortKeySpec::alphanumeric(0, 3)];
        let config = SortConfig::new(3);

        let records = vec![b"CCC".to_vec(), b"AAA".to_vec(), b"BBB".to_vec()];
        let mut output = Vec::new();

        let result = sort_with_output_procedure(&keys, &config, None, records, |returner| {
            while let Some(rec) = returner.return_record() {
                output.push(rec.to_vec());
            }
        })
        .unwrap();

        assert_eq!(result, SortReturn::Success);
        assert_eq!(output[0], b"AAA");
        assert_eq!(output[1], b"BBB");
        assert_eq!(output[2], b"CCC");
    }

    #[test]
    fn test_sort_with_procedures_empty_input() {
        let keys = vec![SortKeySpec::alphanumeric(0, 3)];
        let config = SortConfig::new(3);
        let mut count = 0usize;

        let result = sort_with_procedures(
            &keys,
            &config,
            None,
            |_releaser| {
                // Release nothing
            },
            |returner| {
                while returner.return_record().is_some() {
                    count += 1;
                }
            },
        )
        .unwrap();

        assert_eq!(result, SortReturn::Success);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_returner_at_end_detection() {
        let records = vec![b"ONE".to_vec()];
        let mut returner = Returner::new(records);

        assert!(!returner.is_at_end());
        let _ = returner.return_record();
        assert!(returner.is_at_end());
        assert_eq!(returner.return_record(), None);
    }

    #[test]
    fn test_sort_with_procedures_large_dataset() {
        let keys = vec![SortKeySpec::alphanumeric(0, 3)];
        let config = SortConfig::new(3);
        let mut output = Vec::new();

        sort_with_procedures(
            &keys,
            &config,
            None,
            |releaser| {
                for ch in (b'A'..=b'Z').rev() {
                    releaser.release(&[ch, ch, ch]);
                }
            },
            |returner| {
                while let Some(rec) = returner.return_record() {
                    output.push(rec.to_vec());
                }
            },
        )
        .unwrap();

        assert_eq!(output.len(), 26);
        // Verify sorted order
        for i in 1..output.len() {
            assert!(output[i - 1] <= output[i]);
        }
    }
}
