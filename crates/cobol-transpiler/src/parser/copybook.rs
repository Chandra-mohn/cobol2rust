//! Copybook resolution -- locating and reading COPY member content.
//!
//! Provides a trait `CopybookResolver` for pluggable resolution strategies,
//! with two concrete implementations:
//! - `FileSystemResolver` searches directories for copybook files
//! - `InMemoryResolver` for testing (maps names to content strings)

use std::collections::HashMap;
use std::path::PathBuf;

use crate::error::{Result, TranspileError};

/// Resolves a COPY member name to its source content.
pub trait CopybookResolver {
    /// Look up a copybook by name and optional library qualifier.
    ///
    /// The `library` parameter corresponds to `COPY name OF library`.
    /// Returns the file content as a String.
    /// # Errors
    ///
    /// Returns `TranspileError::CopyNotFound` if the copybook cannot be located.
    fn resolve(&self, name: &str, library: Option<&str>) -> Result<String>;
}

/// Resolves copybooks by searching filesystem directories.
///
/// For each directory in `paths`, searches for files matching the copybook
/// name with common extensions (`.cpy`, `.cbl`, `.CBL`, no extension).
/// Matching is case-insensitive on the copybook name portion.
#[derive(Debug)]
pub struct FileSystemResolver {
    /// Directories to search, in priority order.
    paths: Vec<PathBuf>,
    /// Library name -> directory mapping (for `COPY name OF library`).
    library_map: HashMap<String, PathBuf>,
}

impl FileSystemResolver {
    pub fn new(paths: Vec<PathBuf>) -> Self {
        Self {
            paths,
            library_map: HashMap::new(),
        }
    }

    #[must_use]
    pub fn with_library_map(mut self, map: HashMap<String, PathBuf>) -> Self {
        self.library_map = map;
        self
    }

    /// Search a single directory for the copybook, returning content if found.
    fn search_dir(dir: &std::path::Path, name: &str) -> Option<String> {
        let name_upper = name.to_uppercase();
        let extensions = ["cpy", "cbl", "CBL", ""];

        // Try exact-name matches with each extension first
        for ext in &extensions {
            let filename = if ext.is_empty() {
                name.to_string()
            } else {
                format!("{name}.{ext}")
            };
            let path = dir.join(&filename);
            if path.is_file() {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    return Some(content);
                }
            }
        }

        // Case-insensitive fallback: scan directory entries
        let Ok(entries) = std::fs::read_dir(dir) else {
            return None;
        };

        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            let stem = std::path::Path::new(&*file_name_str)
                .file_stem()
                .map(|s| s.to_string_lossy().to_uppercase())
                .unwrap_or_default();

            if stem == name_upper && entry.path().is_file() {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    return Some(content);
                }
            }
        }

        None
    }
}

impl CopybookResolver for FileSystemResolver {
    fn resolve(&self, name: &str, library: Option<&str>) -> Result<String> {
        // If library specified, search library-specific directory first
        if let Some(lib) = library {
            let lib_upper = lib.to_uppercase();
            if let Some(lib_dir) = self.library_map.get(&lib_upper) {
                if let Some(content) = Self::search_dir(lib_dir, name) {
                    return Ok(content);
                }
            }
        }

        // Search general paths
        for dir in &self.paths {
            if let Some(content) = Self::search_dir(dir, name) {
                return Ok(content);
            }
        }

        Err(TranspileError::CopyNotFound {
            name: name.to_string(),
            paths_searched: self.paths.clone(),
        })
    }
}

/// In-memory resolver for testing -- maps copybook names to content strings.
#[derive(Debug, Default)]
pub struct InMemoryResolver {
    members: HashMap<String, String>,
}

impl InMemoryResolver {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn add(mut self, name: &str, content: &str) -> Self {
        self.members.insert(name.to_uppercase(), content.to_string());
        self
    }
}

impl CopybookResolver for InMemoryResolver {
    fn resolve(&self, name: &str, _library: Option<&str>) -> Result<String> {
        let key = name.to_uppercase();
        self.members.get(&key).cloned().ok_or_else(|| TranspileError::CopyNotFound {
            name: name.to_string(),
            paths_searched: vec![],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn filesystem_resolver_found() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("ACCTFILE.cpy"), "01 ACCT-REC PIC X(80).\n").unwrap();

        let resolver = FileSystemResolver::new(vec![dir.path().to_path_buf()]);
        let content = resolver.resolve("ACCTFILE", None).unwrap();
        assert!(content.contains("ACCT-REC"));
    }

    #[test]
    fn filesystem_resolver_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let resolver = FileSystemResolver::new(vec![dir.path().to_path_buf()]);
        let result = resolver.resolve("MISSING", None);
        assert!(result.is_err());
        match result.unwrap_err() {
            TranspileError::CopyNotFound { name, .. } => assert_eq!(name, "MISSING"),
            other => panic!("expected CopyNotFound, got {other:?}"),
        }
    }

    #[test]
    fn filesystem_resolver_case_insensitive() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("acctfile.cpy"), "01 ACCT-REC PIC X(80).\n").unwrap();

        let resolver = FileSystemResolver::new(vec![dir.path().to_path_buf()]);
        // Search with uppercase name should find lowercase file
        let content = resolver.resolve("ACCTFILE", None).unwrap();
        assert!(content.contains("ACCT-REC"));
    }

    #[test]
    fn in_memory_resolver() {
        let resolver = InMemoryResolver::new()
            .add("COMMON", "01 WS-COMMON PIC X(10).\n")
            .add("DATES", "01 WS-DATE PIC 9(8).\n");

        let content = resolver.resolve("common", None).unwrap();
        assert!(content.contains("WS-COMMON"));

        let content = resolver.resolve("DATES", None).unwrap();
        assert!(content.contains("WS-DATE"));

        assert!(resolver.resolve("MISSING", None).is_err());
    }
}
