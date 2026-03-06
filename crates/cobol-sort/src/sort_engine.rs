//! Adaptive sort engine combining in-memory and external merge sort.
//!
//! Starts in-memory for small datasets. When the memory limit is exceeded,
//! automatically switches to external merge sort.

use std::sync::Arc;

use cobol_core::SortError;
use cobol_io::file_traits::FileOpenMode;
use cobol_io::file_status::FileStatusCode;
use cobol_io::CobolFile;

use crate::collating::CollatingTable;
use crate::config::SortConfig;
use crate::external::ExternalMergeSort;
use crate::in_memory::InMemorySort;
use crate::sort_key::{build_comparator, SharedComparator, SortKeySpec};
use crate::sort_return::SortReturn;

/// Adaptive sort engine state.
enum SortState {
    /// Still buffering in memory.
    InMemory(InMemorySort),
    /// Switched to external merge sort.
    External(ExternalMergeSort),
}

/// Adaptive COBOL sort engine.
///
/// Starts in-memory, auto-switches to external merge sort when the memory
/// limit is exceeded. This mirrors IBM DFSORT's adaptive behavior.
#[allow(missing_debug_implementations)]
pub struct CobolSortEngine {
    state: SortState,
    config: SortConfig,
    comparator: SharedComparator,
}

impl CobolSortEngine {
    /// Create a new adaptive sort engine.
    #[must_use]
    pub fn new(
        keys: &[SortKeySpec],
        config: SortConfig,
        collating: Option<&CollatingTable>,
    ) -> Self {
        let default_collating = CollatingTable::native_ascii();
        let collating = collating.unwrap_or(&default_collating);

        let comparator = build_comparator(keys, collating);
        let comparator: SharedComparator =
            Arc::from(comparator);

        let in_mem = InMemorySort::new(
            {
                let c = comparator.clone();
                Box::new(move |a: &[u8], b: &[u8]| c(a, b))
            },
            config.stable,
        );

        Self {
            state: SortState::InMemory(in_mem),
            config,
            comparator,
        }
    }

    /// Add a record to the sort.
    ///
    /// May trigger a switch from in-memory to external mode.
    ///
    /// # Errors
    /// Returns `SortError::Io` if external merge sort I/O fails.
    pub fn add_record(&mut self, record: Vec<u8>) -> Result<(), SortError> {
        match &mut self.state {
            SortState::InMemory(mem) => {
                mem.add_record(record);

                // Check if we need to switch to external
                if mem.memory_usage() >= self.config.memory_limit {
                    self.switch_to_external()?;
                }
                Ok(())
            }
            SortState::External(ext) => ext.add_record(record).map_err(SortError::Io),
        }
    }

    /// Switch from in-memory to external merge sort, transferring buffered records.
    fn switch_to_external(&mut self) -> Result<(), SortError> {
        // Take the in-memory state
        let old_state = std::mem::replace(
            &mut self.state,
            // Temporary placeholder -- will be replaced below
            SortState::External(ExternalMergeSort::new(
                self.comparator.clone(),
                self.config.memory_limit,
                self.config.fan_in,
                self.config.record_length,
                self.config.stable,
            )),
        );

        if let SortState::InMemory(mem) = old_state {
            if let SortState::External(ext) = &mut self.state {
                // Transfer all buffered records
                for record in mem.into_sorted_vec() {
                    ext.add_record(record).map_err(SortError::Io)?;
                }
            }
        }

        Ok(())
    }

    /// Finish adding records and produce sorted output.
    ///
    /// # Errors
    /// Returns `SortError::Io` if external merge sort I/O fails.
    pub fn finish(self) -> Result<Vec<Vec<u8>>, SortError> {
        match self.state {
            SortState::InMemory(mem) => Ok(mem.into_sorted_vec()),
            SortState::External(ext) => ext.finish().map_err(SortError::Io),
        }
    }

    /// Finish and return sorted records as an iterator.
    ///
    /// # Errors
    /// Returns `SortError::Io` if external merge sort I/O fails.
    pub fn finish_iter(self) -> Result<Box<dyn Iterator<Item = Vec<u8>>>, SortError> {
        match self.state {
            SortState::InMemory(mem) => Ok(Box::new(mem.drain())),
            SortState::External(ext) => {
                let records = ext.finish().map_err(SortError::Io)?;
                Ok(Box::new(records.into_iter()))
            }
        }
    }

    /// Whether the engine has switched to external mode.
    #[must_use]
    pub fn is_external(&self) -> bool {
        matches!(self.state, SortState::External(_))
    }

    /// SORT ... USING input-files GIVING output-files.
    ///
    /// Reads all records from input files, sorts them, writes to output files.
    /// This is the most common form of COBOL SORT.
    ///
    /// # Errors
    /// Returns `SortError` if file I/O or sort operation fails.
    pub fn sort_using_giving(
        keys: &[SortKeySpec],
        config: &SortConfig,
        collating: Option<&CollatingTable>,
        inputs: &mut [&mut dyn CobolFile],
        outputs: &mut [&mut dyn CobolFile],
    ) -> Result<SortReturn, SortError> {
        let mut engine = CobolSortEngine::new(keys, config.clone(), collating);

        // Phase 1: Read all records from input files
        for input in inputs.iter_mut() {
            let status = input.open(FileOpenMode::Input);
            if !status.is_success() {
                return Ok(SortReturn::Failed);
            }

            loop {
                let (status, record) = input.read_next();
                if status == FileStatusCode::AT_END {
                    break;
                }
                if !status.is_success() {
                    return Ok(SortReturn::Failed);
                }
                if let Some(rec) = record {
                    engine.add_record(rec)?;
                }
            }

            input.close();
        }

        // Phase 2: Sort
        let sorted = engine.finish()?;

        // Phase 3: Write sorted records to output files
        for output in outputs.iter_mut() {
            let status = output.open(FileOpenMode::Output);
            if !status.is_success() {
                return Ok(SortReturn::Failed);
            }

            for record in &sorted {
                let status = output.write_record(record);
                if !status.is_success() {
                    return Ok(SortReturn::Failed);
                }
            }

            output.close();
        }

        Ok(SortReturn::Success)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_dataset_stays_in_memory() {
        let keys = vec![SortKeySpec::alphanumeric(0, 5)];
        let config = SortConfig::new(5).with_memory_limit(1024);
        let mut engine = CobolSortEngine::new(&keys, config, None);

        engine.add_record(b"EEEEE".to_vec()).unwrap();
        engine.add_record(b"AAAAA".to_vec()).unwrap();
        engine.add_record(b"CCCCC".to_vec()).unwrap();

        assert!(!engine.is_external());

        let result = engine.finish().unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], b"AAAAA");
        assert_eq!(result[1], b"CCCCC");
        assert_eq!(result[2], b"EEEEE");
    }

    #[test]
    fn test_large_dataset_switches_to_external() {
        let keys = vec![SortKeySpec::alphanumeric(0, 3)];
        // Tiny memory limit to force external
        let config = SortConfig::new(3).with_memory_limit(20);
        let mut engine = CobolSortEngine::new(&keys, config, None);

        for ch in b"ZYXWVUTSRQPONMLKJIHGFEDCBA".iter() {
            engine.add_record(vec![*ch; 3]).unwrap();
        }

        assert!(engine.is_external());

        let result = engine.finish().unwrap();
        assert_eq!(result.len(), 26);
        // Verify sorted
        for i in 1..result.len() {
            assert!(result[i - 1] <= result[i], "Not sorted at index {}", i);
        }
    }

    #[test]
    fn test_stable_sort() {
        let keys = vec![SortKeySpec::alphanumeric(0, 1)]; // Sort by first byte only
        let config = SortConfig::new(3).with_stable(true).with_memory_limit(1024);
        let mut engine = CobolSortEngine::new(&keys, config, None);

        engine.add_record(b"A_1".to_vec()).unwrap();
        engine.add_record(b"B_1".to_vec()).unwrap();
        engine.add_record(b"A_2".to_vec()).unwrap();
        engine.add_record(b"B_2".to_vec()).unwrap();

        let result = engine.finish().unwrap();
        assert_eq!(result[0], b"A_1");
        assert_eq!(result[1], b"A_2");
        assert_eq!(result[2], b"B_1");
        assert_eq!(result[3], b"B_2");
    }

    #[test]
    fn test_empty_sort() {
        let keys = vec![SortKeySpec::alphanumeric(0, 5)];
        let config = SortConfig::new(5);
        let engine = CobolSortEngine::new(&keys, config, None);

        let result = engine.finish().unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_ebcdic_collating() {
        let keys = vec![SortKeySpec::alphanumeric(0, 1)];
        let config = SortConfig::new(1).with_memory_limit(1024);
        let ebcdic = CollatingTable::ebcdic_cp037();
        let mut engine = CobolSortEngine::new(&keys, config, Some(&ebcdic));

        // In EBCDIC: lowercase < uppercase < digits
        engine.add_record(b"5".to_vec()).unwrap();
        engine.add_record(b"A".to_vec()).unwrap();
        engine.add_record(b"a".to_vec()).unwrap();

        let result = engine.finish().unwrap();
        // EBCDIC order: a(0x81) < A(0xC1) < 5(0xF5)
        assert_eq!(result[0], b"a");
        assert_eq!(result[1], b"A");
        assert_eq!(result[2], b"5");
    }

    #[test]
    fn test_finish_iter() {
        let keys = vec![SortKeySpec::alphanumeric(0, 3)];
        let config = SortConfig::new(3).with_memory_limit(1024);
        let mut engine = CobolSortEngine::new(&keys, config, None);

        engine.add_record(b"CCC".to_vec()).unwrap();
        engine.add_record(b"AAA".to_vec()).unwrap();
        engine.add_record(b"BBB".to_vec()).unwrap();

        let result: Vec<_> = engine.finish_iter().unwrap().collect();
        assert_eq!(result[0], b"AAA");
        assert_eq!(result[1], b"BBB");
        assert_eq!(result[2], b"CCC");
    }
}
