//! In-memory sort for small datasets.
//!
//! Buffers all records in a Vec, then sorts them using the built comparator.
//! Supports both stable and unstable sort modes.

use crate::sort_key::RecordComparator;

/// In-memory sort engine for datasets that fit in memory.
#[allow(missing_debug_implementations)]
pub struct InMemorySort {
    records: Vec<Vec<u8>>,
    comparator: RecordComparator,
    stable: bool,
    sorted: bool,
}

impl InMemorySort {
    /// Create a new in-memory sort with the given comparator and stability flag.
    pub fn new(comparator: RecordComparator, stable: bool) -> Self {
        Self {
            records: Vec::new(),
            comparator,
            stable,
            sorted: false,
        }
    }

    /// Add a record to the sort buffer.
    pub fn add_record(&mut self, record: Vec<u8>) {
        self.sorted = false;
        self.records.push(record);
    }

    /// Total bytes consumed by buffered records (approximate).
    #[must_use]
    pub fn memory_usage(&self) -> usize {
        self.records.iter().map(|r| r.len() + std::mem::size_of::<Vec<u8>>()).sum()
    }

    /// Number of buffered records.
    #[must_use]
    pub fn record_count(&self) -> usize {
        self.records.len()
    }

    /// Sort the buffered records.
    pub fn sort(&mut self) {
        if self.sorted {
            return;
        }
        let cmp = &self.comparator;
        if self.stable {
            self.records.sort_by(|a, b| cmp(a, b));
        } else {
            self.records.sort_unstable_by(|a, b| cmp(a, b));
        }
        self.sorted = true;
    }

    /// Drain all sorted records as an iterator.
    ///
    /// Automatically sorts if not already sorted.
    pub fn drain(mut self) -> impl Iterator<Item = Vec<u8>> {
        if !self.sorted {
            self.sort();
        }
        self.records.into_iter()
    }

    /// Take all sorted records as a `Vec`.
    ///
    /// Automatically sorts if not already sorted.
    #[must_use]
    pub fn into_sorted_vec(mut self) -> Vec<Vec<u8>> {
        if !self.sorted {
            self.sort();
        }
        self.records
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    type SortComparator = Box<dyn Fn(&[u8], &[u8]) -> Ordering + Send + Sync>;

    fn ascii_comparator() -> SortComparator {
        Box::new(|a: &[u8], b: &[u8]| a.cmp(b))
    }

    #[test]
    fn test_basic_sort() {
        let mut sorter = InMemorySort::new(ascii_comparator(), false);
        sorter.add_record(b"CHARLIE".to_vec());
        sorter.add_record(b"ALICE__".to_vec());
        sorter.add_record(b"BOB____".to_vec());

        let result = sorter.into_sorted_vec();
        assert_eq!(result[0], b"ALICE__");
        assert_eq!(result[1], b"BOB____");
        assert_eq!(result[2], b"CHARLIE");
    }

    #[test]
    fn test_empty_sort() {
        let sorter = InMemorySort::new(ascii_comparator(), false);
        let result = sorter.into_sorted_vec();
        assert!(result.is_empty());
    }

    #[test]
    fn test_single_record() {
        let mut sorter = InMemorySort::new(ascii_comparator(), false);
        sorter.add_record(b"ONLY".to_vec());
        let result = sorter.into_sorted_vec();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], b"ONLY");
    }

    #[test]
    fn test_stable_sort_preserves_order() {
        // Records with same key prefix but different suffixes
        let cmp: SortComparator =
            Box::new(|a: &[u8], b: &[u8]| {
                // Compare only first 3 bytes
                a[..3].cmp(&b[..3])
            });
        let mut sorter = InMemorySort::new(cmp, true);
        sorter.add_record(b"AAA1".to_vec());
        sorter.add_record(b"BBB1".to_vec());
        sorter.add_record(b"AAA2".to_vec());
        sorter.add_record(b"BBB2".to_vec());

        let result = sorter.into_sorted_vec();
        // Stable: AAA1 before AAA2 (insertion order preserved for equal keys)
        assert_eq!(result[0], b"AAA1");
        assert_eq!(result[1], b"AAA2");
        assert_eq!(result[2], b"BBB1");
        assert_eq!(result[3], b"BBB2");
    }

    #[test]
    fn test_drain_iterator() {
        let mut sorter = InMemorySort::new(ascii_comparator(), false);
        sorter.add_record(b"CCC".to_vec());
        sorter.add_record(b"AAA".to_vec());
        sorter.add_record(b"BBB".to_vec());

        let result: Vec<_> = sorter.drain().collect();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], b"AAA");
    }

    #[test]
    fn test_memory_usage() {
        let mut sorter = InMemorySort::new(ascii_comparator(), false);
        assert_eq!(sorter.memory_usage(), 0);
        sorter.add_record(vec![0u8; 100]);
        assert!(sorter.memory_usage() >= 100);
    }
}
