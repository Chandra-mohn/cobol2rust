//! cobol-io: COBOL file I/O operations.
//!
//! Provides sequential, indexed (SQLite-backed), relative, and print file
//! operations with COBOL file status codes and record buffer management.

pub mod file_status;
pub mod file_traits;
#[cfg(feature = "sqlite")]
pub mod indexed;
pub mod relative;
pub mod resolver;
pub mod sequential;

// Re-export public API
pub use file_status::FileStatusCode;
pub use file_traits::{CobolFile, FileAccessMode, FileOpenMode, FileOrganization};
#[cfg(feature = "sqlite")]
pub use indexed::IndexedFile;
pub use relative::RelativeFile;
pub use resolver::FileResolver;
pub use sequential::SequentialFile;
