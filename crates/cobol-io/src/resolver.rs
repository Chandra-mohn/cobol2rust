//! File name resolution.
//!
//! In COBOL, the SELECT clause maps a logical file name to a DD name,
//! and the JCL maps DD names to physical datasets. In our Rust runtime,
//! `FileResolver` maps COBOL SELECT names to filesystem paths.
//!
//! Resolution order:
//! 1. Environment variable: `COBOL_FILE_{NAME}` (uppercase, hyphens -> underscores)
//! 2. Explicit mapping (from config or programmatic registration)
//! 3. Default: same name as file in current directory

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Maps COBOL SELECT file names to filesystem paths.
///
/// Analogous to JCL DD statements or ASSIGN TO clauses.
#[derive(Debug, Clone)]
pub struct FileResolver {
    /// Explicit name -> path mappings.
    mappings: HashMap<String, PathBuf>,
    /// Base directory for default resolution.
    base_dir: PathBuf,
}

impl FileResolver {
    /// Create a new resolver with the given base directory.
    pub fn new(base_dir: PathBuf) -> Self {
        Self {
            mappings: HashMap::new(),
            base_dir,
        }
    }

    /// Create a resolver using the current working directory.
    pub fn current_dir() -> Self {
        Self::new(std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
    }

    /// Register a mapping from a COBOL file name to a filesystem path.
    pub fn register(&mut self, cobol_name: &str, path: PathBuf) {
        self.mappings.insert(cobol_name.to_uppercase(), path);
    }

    /// Resolve a COBOL file name to a filesystem path.
    ///
    /// Resolution order:
    /// 1. Environment variable `COBOL_FILE_{NAME}`
    /// 2. Explicit mapping
    /// 3. Default: base_dir / name
    pub fn resolve(&self, cobol_name: &str) -> PathBuf {
        let upper = cobol_name.to_uppercase();

        // 1. Environment variable
        let env_key = format!("COBOL_FILE_{}", upper.replace('-', "_"));
        if let Ok(path) = std::env::var(&env_key) {
            return PathBuf::from(path);
        }

        // 2. Explicit mapping
        if let Some(path) = self.mappings.get(&upper) {
            return path.clone();
        }

        // 3. Default: base_dir / name (lowercased, hyphens to underscores)
        let filename = upper.to_lowercase().replace('-', "_");
        self.base_dir.join(filename)
    }

    /// Set the base directory for default resolution.
    pub fn set_base_dir(&mut self, dir: PathBuf) {
        self.base_dir = dir;
    }

    /// Get the base directory.
    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }
}

impl Default for FileResolver {
    fn default() -> Self {
        Self::current_dir()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_mapping() {
        let mut resolver = FileResolver::new(PathBuf::from("/tmp"));
        resolver.register("CUSTOMER-FILE", PathBuf::from("/data/customers.dat"));
        assert_eq!(
            resolver.resolve("CUSTOMER-FILE"),
            PathBuf::from("/data/customers.dat")
        );
    }

    #[test]
    fn explicit_mapping_case_insensitive() {
        let mut resolver = FileResolver::new(PathBuf::from("/tmp"));
        resolver.register("MY-FILE", PathBuf::from("/data/myfile.dat"));
        // Resolve with different case
        assert_eq!(
            resolver.resolve("my-file"),
            PathBuf::from("/data/myfile.dat")
        );
    }

    #[test]
    fn default_resolution() {
        let resolver = FileResolver::new(PathBuf::from("/tmp/data"));
        // No explicit mapping, no env var -> base_dir / lowercased name
        let path = resolver.resolve("REPORT-FILE");
        assert_eq!(path, PathBuf::from("/tmp/data/report_file"));
    }

    #[test]
    fn base_dir_getter() {
        let resolver = FileResolver::new(PathBuf::from("/my/dir"));
        assert_eq!(resolver.base_dir(), Path::new("/my/dir"));
    }

    #[test]
    fn set_base_dir() {
        let mut resolver = FileResolver::new(PathBuf::from("/old"));
        resolver.set_base_dir(PathBuf::from("/new"));
        assert_eq!(resolver.base_dir(), Path::new("/new"));
    }
}
