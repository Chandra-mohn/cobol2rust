//! External merge sort for large datasets.
//!
//! When the dataset exceeds the memory limit, records are sorted in chunks
//! (runs), flushed to temporary files, then merged using a k-way merge
//! with a min-heap.

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufReader, BufWriter, Read, Write};
use tempfile::NamedTempFile;

use crate::sort_key::SharedComparator;

/// External merge sort engine.
///
/// Phase 1 (run generation): buffer records in memory until the memory limit
/// is reached, sort the buffer, and flush to a temporary file.
/// Phase 2 (k-way merge): open all run files and merge them using a min-heap.
#[allow(missing_debug_implementations)]
pub struct ExternalMergeSort {
    run_files: Vec<NamedTempFile>,
    memory_limit: usize,
    fan_in: usize,
    #[allow(dead_code)]
    record_length: usize,
    comparator: SharedComparator,
    stable: bool,
    /// Current in-memory buffer for building the next run.
    buffer: Vec<Vec<u8>>,
    buffer_bytes: usize,
}

impl ExternalMergeSort {
    /// Create a new external merge sort engine.
    pub fn new(
        comparator: SharedComparator,
        memory_limit: usize,
        fan_in: usize,
        record_length: usize,
        stable: bool,
    ) -> Self {
        Self {
            run_files: Vec::new(),
            memory_limit,
            fan_in: fan_in.max(2),
            record_length,
            comparator,
            stable,
            buffer: Vec::new(),
            buffer_bytes: 0,
        }
    }

    /// Add a record. If the buffer exceeds the memory limit, flush a run.
    ///
    /// # Errors
    /// Returns `io::Error` if flushing a run to disk fails.
    pub fn add_record(&mut self, record: Vec<u8>) -> io::Result<()> {
        self.buffer_bytes += record.len();
        self.buffer.push(record);

        if self.buffer_bytes >= self.memory_limit {
            self.flush_run()?;
        }
        Ok(())
    }

    /// Sort the current buffer and write it to a temporary run file.
    #[allow(clippy::cast_possible_truncation)]
    fn flush_run(&mut self) -> io::Result<()> {
        if self.buffer.is_empty() {
            return Ok(());
        }

        let cmp = &self.comparator;
        if self.stable {
            self.buffer.sort_by(|a, b| cmp(a, b));
        } else {
            self.buffer.sort_unstable_by(|a, b| cmp(a, b));
        }

        let mut tmp = NamedTempFile::new()?;
        {
            let mut writer = BufWriter::new(&mut tmp);
            for record in &self.buffer {
                // Write 4-byte length prefix (little-endian) + record data
                let len = record.len() as u32;
                writer.write_all(&len.to_le_bytes())?;
                writer.write_all(record)?;
            }
            writer.flush()?;
        }

        self.run_files.push(tmp);
        self.buffer.clear();
        self.buffer_bytes = 0;
        Ok(())
    }

    /// Finish adding records and produce the sorted output.
    ///
    /// Flushes any remaining buffer, then performs k-way merge of all run files.
    ///
    /// # Errors
    /// Returns `io::Error` if I/O operations fail during merge.
    pub fn finish(mut self) -> io::Result<Vec<Vec<u8>>> {
        // Flush remaining buffer as final run
        self.flush_run()?;

        if self.run_files.is_empty() {
            return Ok(Vec::new());
        }

        // If only one run, just read it back
        if self.run_files.len() == 1 {
            let mut file = self.run_files.remove(0);
            return Self::read_run_file(&mut file);
        }

        // Multi-pass merge if needed (when run count > fan_in)
        #[allow(clippy::cast_possible_truncation)]
        while self.run_files.len() > self.fan_in {
            let mut new_runs = Vec::new();
            let runs_to_merge: Vec<_> = self.run_files.drain(..self.fan_in).collect();
            let merged = self.merge_runs(runs_to_merge)?;

            // Write merged result to new run file
            let mut tmp = NamedTempFile::new()?;
            {
                let mut writer = BufWriter::new(&mut tmp);
                for record in &merged {
                    let len = record.len() as u32;
                    writer.write_all(&len.to_le_bytes())?;
                    writer.write_all(record)?;
                }
                writer.flush()?;
            }
            new_runs.push(tmp);

            // Keep remaining runs
            new_runs.append(&mut self.run_files);
            self.run_files = new_runs;
        }

        // Final merge
        let all_runs: Vec<_> = self.run_files.drain(..).collect();
        self.merge_runs(all_runs)
    }

    /// Read all records from a run file.
    fn read_run_file(file: &mut NamedTempFile) -> io::Result<Vec<Vec<u8>>> {
        use std::io::Seek;
        file.seek(io::SeekFrom::Start(0))?;
        let mut reader = BufReader::new(file);
        let mut records = Vec::new();

        loop {
            let mut len_buf = [0u8; 4];
            match reader.read_exact(&mut len_buf) {
                Ok(()) => {}
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e),
            }
            let len = u32::from_le_bytes(len_buf) as usize;
            let mut record = vec![0u8; len];
            reader.read_exact(&mut record)?;
            records.push(record);
        }

        Ok(records)
    }

    /// Merge multiple sorted run files into a single sorted output.
    fn merge_runs(&self, runs: Vec<NamedTempFile>) -> io::Result<Vec<Vec<u8>>> {
        let mut readers: Vec<RunReader> = Vec::new();
        for file in runs {
            let reader = RunReader::new(file)?;
            readers.push(reader);
        }

        let mut heap: BinaryHeap<MergeEntry> = BinaryHeap::new();
        let comparator = self.comparator.clone();

        // Seed the heap with the first record from each run
        for (run_idx, reader) in readers.iter_mut().enumerate() {
            if let Some(record) = reader.next_record()? {
                heap.push(MergeEntry {
                    record,
                    run_index: run_idx,
                    comparator: comparator.clone(),
                });
            }
        }

        let mut result = Vec::new();

        while let Some(entry) = heap.pop() {
            let run_idx = entry.run_index;
            result.push(entry.record);

            // Get next record from the same run
            if let Some(record) = readers[run_idx].next_record()? {
                heap.push(MergeEntry {
                    record,
                    run_index: run_idx,
                    comparator: comparator.clone(),
                });
            }
        }

        Ok(result)
    }
}

/// Reader for a sorted run file.
struct RunReader {
    reader: BufReader<NamedTempFile>,
}

impl RunReader {
    fn new(mut file: NamedTempFile) -> io::Result<Self> {
        use std::io::Seek;
        file.seek(io::SeekFrom::Start(0))?;
        Ok(Self {
            reader: BufReader::new(file),
        })
    }

    fn next_record(&mut self) -> io::Result<Option<Vec<u8>>> {
        let mut len_buf = [0u8; 4];
        match self.reader.read_exact(&mut len_buf) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(e),
        }
        let len = u32::from_le_bytes(len_buf) as usize;
        let mut record = vec![0u8; len];
        self.reader.read_exact(&mut record)?;
        Ok(Some(record))
    }
}

/// Entry in the merge heap.
///
/// Uses reverse ordering so `BinaryHeap` (max-heap) acts as a min-heap.
struct MergeEntry {
    record: Vec<u8>,
    run_index: usize,
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
        // Reverse ordering for min-heap behavior.
        // On tie, prefer lower run_index for stable-like behavior across runs.
        let ord = (self.comparator)(&self.record, &other.record);
        match ord.reverse() {
            Ordering::Equal => other.run_index.cmp(&self.run_index),
            o => o,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    fn ascii_cmp() -> SharedComparator {
        Arc::new(|a: &[u8], b: &[u8]| a.cmp(b))
    }

    #[test]
    fn test_single_run() {
        let mut ext = ExternalMergeSort::new(ascii_cmp(), 1024, 4, 10, false);
        ext.add_record(b"CCC".to_vec()).unwrap();
        ext.add_record(b"AAA".to_vec()).unwrap();
        ext.add_record(b"BBB".to_vec()).unwrap();

        // Force flush
        ext.flush_run().unwrap();
        assert_eq!(ext.run_files.len(), 1);

        let result = ext.finish().unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], b"AAA");
        assert_eq!(result[1], b"BBB");
        assert_eq!(result[2], b"CCC");
    }

    #[test]
    fn test_two_runs_merged() {
        // Use tiny memory limit to force multiple runs
        let mut ext = ExternalMergeSort::new(ascii_cmp(), 10, 4, 3, false);
        ext.add_record(b"DDD".to_vec()).unwrap();
        ext.add_record(b"BBB".to_vec()).unwrap();
        ext.add_record(b"EEE".to_vec()).unwrap();
        ext.add_record(b"AAA".to_vec()).unwrap();
        ext.add_record(b"CCC".to_vec()).unwrap();

        let result = ext.finish().unwrap();
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], b"AAA");
        assert_eq!(result[1], b"BBB");
        assert_eq!(result[2], b"CCC");
        assert_eq!(result[3], b"DDD");
        assert_eq!(result[4], b"EEE");
    }

    #[test]
    fn test_empty_external_sort() {
        let ext = ExternalMergeSort::new(ascii_cmp(), 1024, 4, 10, false);
        let result = ext.finish().unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_many_runs_with_small_fan_in() {
        // Force multi-pass merge: fan_in=2, create 4 runs
        let mut ext = ExternalMergeSort::new(ascii_cmp(), 5, 2, 3, false);
        for ch in b"HGFEDCBA".iter() {
            ext.add_record(vec![*ch; 3]).unwrap();
        }

        let result = ext.finish().unwrap();
        assert_eq!(result.len(), 8);
        // Verify sorted order
        for i in 1..result.len() {
            assert!(result[i - 1] <= result[i], "Not sorted at index {}", i);
        }
    }
}
