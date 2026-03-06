//! MERGE verb implementation.
//!
//! K-way merge of pre-sorted input files into a single sorted output.
//! Uses a `BinaryHeap` (min-heap via reverse ordering) to select the
//! smallest record from multiple input sources at each step.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use cobol_core::SortError;
use cobol_io::file_traits::FileOpenMode;
use cobol_io::file_status::FileStatusCode;
use cobol_io::CobolFile;

use crate::collating::CollatingTable;
use crate::sort_key::{build_comparator, SharedComparator, SortKeySpec};
use crate::sort_return::SortReturn;

/// K-way merge engine for the COBOL MERGE verb.
///
/// Merges multiple pre-sorted input sources into a single sorted output.
/// Each input must already be sorted on the same keys.
#[allow(missing_debug_implementations)]
pub struct CobolMergeEngine {
    comparator: SharedComparator,
}

impl CobolMergeEngine {
    /// Create a new merge engine with the given key specifications.
    #[must_use]
    pub fn new(keys: &[SortKeySpec], collating: Option<&CollatingTable>) -> Self {
        let default_collating = CollatingTable::native_ascii();
        let collating = collating.unwrap_or(&default_collating);
        let comparator = build_comparator(keys, collating);
        let comparator: SharedComparator = std::sync::Arc::from(comparator);

        Self { comparator }
    }

    /// Merge pre-sorted input files into an output file.
    ///
    /// Opens each input for INPUT, reads all records via k-way merge,
    /// and writes the merged result to the output file (opened for OUTPUT).
    ///
    /// # Errors
    /// Returns `SortError` if file I/O fails or inputs are not sorted.
    pub fn merge_files(
        &self,
        inputs: &mut [&mut dyn CobolFile],
        output: &mut dyn CobolFile,
    ) -> Result<SortReturn, SortError> {
        // Open all inputs for reading
        for input in inputs.iter_mut() {
            let status = input.open(FileOpenMode::Input);
            if !status.is_success() {
                return Ok(SortReturn::Failed);
            }
        }

        // Open output for writing
        let status = output.open(FileOpenMode::Output);
        if !status.is_success() {
            return Ok(SortReturn::Failed);
        }

        // Seed the heap with first record from each input
        let mut heap: BinaryHeap<MergeEntry> = BinaryHeap::new();

        for (idx, input) in inputs.iter_mut().enumerate() {
            let (status, record) = input.read_next();
            if status == FileStatusCode::AT_END {
                // Empty input -- skip
                continue;
            }
            if !status.is_success() && status != FileStatusCode::AT_END {
                return Ok(SortReturn::Failed);
            }
            if let Some(rec) = record {
                heap.push(MergeEntry {
                    record: rec,
                    source_index: idx,
                    comparator: self.comparator.clone(),
                });
            }
        }

        // K-way merge: pop smallest, write to output, refill from same source
        while let Some(entry) = heap.pop() {
            let src_idx = entry.source_index;
            let write_status = output.write_record(&entry.record);
            if !write_status.is_success() {
                return Ok(SortReturn::Failed);
            }

            // Get next record from the same source
            let (status, record) = inputs[src_idx].read_next();
            if status == FileStatusCode::AT_END {
                // This source is exhausted
                continue;
            }
            if !status.is_success() {
                return Ok(SortReturn::Failed);
            }
            if let Some(rec) = record {
                heap.push(MergeEntry {
                    record: rec,
                    source_index: src_idx,
                    comparator: self.comparator.clone(),
                });
            }
        }

        // Close all files
        for input in inputs.iter_mut() {
            input.close();
        }
        output.close();

        Ok(SortReturn::Success)
    }

    /// Merge pre-sorted record vectors (for testing and in-memory merge).
    ///
    /// Each input `Vec<Vec<u8>>` must be pre-sorted on the same keys.
    ///
    /// # Errors
    /// Returns `SortError` if merge fails.
    pub fn merge_vecs(&self, inputs: &[Vec<Vec<u8>>]) -> Result<Vec<Vec<u8>>, SortError> {
        let mut heap: BinaryHeap<IndexedMergeEntry> = BinaryHeap::new();
        let mut cursors: Vec<usize> = vec![0; inputs.len()];

        // Seed heap
        for (idx, input) in inputs.iter().enumerate() {
            if !input.is_empty() {
                heap.push(IndexedMergeEntry {
                    record: input[0].clone(),
                    source_index: idx,
                    comparator: self.comparator.clone(),
                });
                cursors[idx] = 1;
            }
        }

        let mut result = Vec::new();

        while let Some(entry) = heap.pop() {
            let src_idx = entry.source_index;
            result.push(entry.record);

            // Advance cursor for this source
            if cursors[src_idx] < inputs[src_idx].len() {
                let next_rec = inputs[src_idx][cursors[src_idx]].clone();
                cursors[src_idx] += 1;
                heap.push(IndexedMergeEntry {
                    record: next_rec,
                    source_index: src_idx,
                    comparator: self.comparator.clone(),
                });
            }
        }

        Ok(result)
    }
}

/// Entry in the merge heap (for file-based merge).
struct MergeEntry {
    record: Vec<u8>,
    source_index: usize,
    comparator: SharedComparator,
}

impl PartialEq for MergeEntry {
    fn eq(&self, other: &Self) -> bool {
        (self.comparator)(&self.record, &other.record) == Ordering::Equal
    }
}

impl Eq for MergeEntry {}

impl PartialOrd for MergeEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MergeEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse for min-heap. On tie, prefer lower source index for stability.
        let ord = (self.comparator)(&self.record, &other.record);
        match ord.reverse() {
            Ordering::Equal => other.source_index.cmp(&self.source_index),
            o => o,
        }
    }
}

/// Entry in the merge heap (for Vec-based merge).
struct IndexedMergeEntry {
    record: Vec<u8>,
    source_index: usize,
    comparator: SharedComparator,
}

impl PartialEq for IndexedMergeEntry {
    fn eq(&self, other: &Self) -> bool {
        (self.comparator)(&self.record, &other.record) == Ordering::Equal
    }
}

impl Eq for IndexedMergeEntry {}

impl PartialOrd for IndexedMergeEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IndexedMergeEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = (self.comparator)(&self.record, &other.record);
        match ord.reverse() {
            Ordering::Equal => other.source_index.cmp(&self.source_index),
            o => o,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_sorted_vecs() {
        let keys = vec![SortKeySpec::alphanumeric(0, 3)];
        let engine = CobolMergeEngine::new(&keys, None);

        let input1 = vec![b"AAA".to_vec(), b"CCC".to_vec(), b"EEE".to_vec()];
        let input2 = vec![b"BBB".to_vec(), b"DDD".to_vec(), b"FFF".to_vec()];

        let result = engine.merge_vecs(&[input1, input2]).unwrap();
        assert_eq!(result.len(), 6);
        assert_eq!(result[0], b"AAA");
        assert_eq!(result[1], b"BBB");
        assert_eq!(result[2], b"CCC");
        assert_eq!(result[3], b"DDD");
        assert_eq!(result[4], b"EEE");
        assert_eq!(result[5], b"FFF");
    }

    #[test]
    fn test_merge_three_sources() {
        let keys = vec![SortKeySpec::alphanumeric(0, 1)];
        let engine = CobolMergeEngine::new(&keys, None);

        let a = vec![b"A".to_vec(), b"D".to_vec(), b"G".to_vec()];
        let b_vec = vec![b"B".to_vec(), b"E".to_vec(), b"H".to_vec()];
        let c = vec![b"C".to_vec(), b"F".to_vec(), b"I".to_vec()];

        let result = engine.merge_vecs(&[a, b_vec, c]).unwrap();
        assert_eq!(result.len(), 9);
        for (i, rec) in result.iter().enumerate() {
            let expected = (b'A' + i as u8) as char;
            assert_eq!(rec[0] as char, expected, "Mismatch at index {}", i);
        }
    }

    #[test]
    fn test_merge_with_duplicates() {
        let keys = vec![SortKeySpec::alphanumeric(0, 3)];
        let engine = CobolMergeEngine::new(&keys, None);

        let input1 = vec![b"AAA".to_vec(), b"BBB".to_vec()];
        let input2 = vec![b"AAA".to_vec(), b"BBB".to_vec()];

        let result = engine.merge_vecs(&[input1, input2]).unwrap();
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], b"AAA");
        assert_eq!(result[1], b"AAA");
        assert_eq!(result[2], b"BBB");
        assert_eq!(result[3], b"BBB");
    }

    #[test]
    fn test_merge_empty_inputs() {
        let keys = vec![SortKeySpec::alphanumeric(0, 3)];
        let engine = CobolMergeEngine::new(&keys, None);

        let result = engine.merge_vecs(&[vec![], vec![]]).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_merge_one_empty_one_populated() {
        let keys = vec![SortKeySpec::alphanumeric(0, 3)];
        let engine = CobolMergeEngine::new(&keys, None);

        let input1: Vec<Vec<u8>> = vec![];
        let input2 = vec![b"AAA".to_vec(), b"BBB".to_vec()];

        let result = engine.merge_vecs(&[input1, input2]).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], b"AAA");
        assert_eq!(result[1], b"BBB");
    }

    #[test]
    fn test_merge_unequal_lengths() {
        let keys = vec![SortKeySpec::alphanumeric(0, 1)];
        let engine = CobolMergeEngine::new(&keys, None);

        let input1 = vec![b"A".to_vec(), b"C".to_vec(), b"E".to_vec(), b"G".to_vec()];
        let input2 = vec![b"B".to_vec()];

        let result = engine.merge_vecs(&[input1, input2]).unwrap();
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], b"A");
        assert_eq!(result[1], b"B");
        assert_eq!(result[2], b"C");
        assert_eq!(result[3], b"E");
        assert_eq!(result[4], b"G");
    }
}
