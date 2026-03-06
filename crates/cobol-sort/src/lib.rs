//! cobol-sort: COBOL SORT/MERGE operations.
//!
//! Provides in-memory and external merge sort with collating sequence
//! support, INPUT/OUTPUT PROCEDURE closures, and MERGE via min-heap selection.
//!
//! # Architecture
//!
//! The sort engine uses a two-tier approach:
//! - Small datasets: in-memory sort via `Vec::sort_by` (fast, no disk I/O)
//! - Large datasets: external merge sort with tempfile-backed runs and k-way merge
//!
//! The adaptive `CobolSortEngine` starts in-memory and automatically switches
//! to external mode when the memory limit is exceeded.
//!
//! # MERGE
//!
//! The `CobolMergeEngine` performs k-way merge of pre-sorted input sources
//! using a min-heap for efficient selection.
//!
//! # INPUT/OUTPUT PROCEDURE
//!
//! The `Releaser` and `Returner` types provide the interface for COBOL's
//! INPUT PROCEDURE (RELEASE verb) and OUTPUT PROCEDURE (RETURN verb).

pub mod collating;
pub mod config;
pub mod external;
pub mod in_memory;
pub mod merge;
pub mod procedure;
pub mod sort_engine;
pub mod sort_key;
pub mod sort_return;

// Re-export public API
pub use collating::CollatingTable;
pub use config::SortConfig;
pub use merge::CobolMergeEngine;
pub use procedure::{
    sort_with_input_procedure, sort_with_output_procedure, sort_with_procedures, Releaser,
    Returner,
};
pub use sort_engine::CobolSortEngine;
pub use sort_key::{RecordComparator, SharedComparator, SortKeySpec, SortKeyType};
pub use sort_return::SortReturn;
