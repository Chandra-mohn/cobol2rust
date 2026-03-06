//! Sort/merge configuration.

/// Configuration for SORT and MERGE operations.
#[derive(Debug, Clone)]
pub struct SortConfig {
    /// Maximum memory (bytes) for in-memory sort before switching to external merge.
    /// Default: 64 MB.
    pub memory_limit: usize,
    /// Maximum number of run files to merge simultaneously in external sort.
    /// Default: 16.
    pub fan_in: usize,
    /// Whether to use stable sort (`WITH DUPLICATES IN ORDER`).
    /// Default: false (unstable sort is faster).
    pub stable: bool,
    /// Fixed record length in bytes. All records must be this size.
    pub record_length: usize,
}

impl SortConfig {
    /// Default memory limit: 64 MB.
    pub const DEFAULT_MEMORY_LIMIT: usize = 64 * 1024 * 1024;

    /// Default fan-in for k-way merge.
    pub const DEFAULT_FAN_IN: usize = 16;

    /// Create a new `SortConfig` with the given record length and defaults.
    #[must_use]
    pub fn new(record_length: usize) -> Self {
        Self {
            memory_limit: Self::DEFAULT_MEMORY_LIMIT,
            fan_in: Self::DEFAULT_FAN_IN,
            stable: false,
            record_length,
        }
    }

    /// Builder: set memory limit.
    #[must_use]
    pub fn with_memory_limit(mut self, limit: usize) -> Self {
        self.memory_limit = limit;
        self
    }

    /// Builder: set fan-in.
    #[must_use]
    pub fn with_fan_in(mut self, fan_in: usize) -> Self {
        self.fan_in = fan_in.max(2); // minimum 2-way merge
        self
    }

    /// Builder: enable stable sort.
    #[must_use]
    pub fn with_stable(mut self, stable: bool) -> Self {
        self.stable = stable;
        self
    }
}
