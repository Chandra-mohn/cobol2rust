//! File discovery: recursively walk a directory tree to find COBOL sources.

use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

use miette::{IntoDiagnostic, Result};

/// A discovered file with metadata.
#[derive(Debug, Clone)]
pub struct DiscoveredFile {
    /// Path relative to the scan root.
    pub relative_path: String,
    /// Absolute path on disk.
    pub absolute_path: String,
    /// File extension (lowercase).
    pub extension: String,
    /// File size in bytes.
    pub file_size: u64,
    /// Modification time as seconds since UNIX epoch.
    pub mtime_epoch: i64,
    /// Classified file type.
    pub file_type: FileType,
}

/// Classification of discovered files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum FileType {
    Source,
    Copybook,
    Jcl,
    Unknown,
}

impl FileType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Source => "source",
            Self::Copybook => "copybook",
            Self::Jcl => "jcl",
            Self::Unknown => "unknown",
        }
    }
}

/// Default source extensions (case-insensitive).
const SOURCE_EXTS: &[&str] = &["cbl", "cob", "cobol"];
/// Default copybook extensions.
const COPYBOOK_EXTS: &[&str] = &["cpy", "copy", "cpylib"];
/// Default JCL extensions.
const JCL_EXTS: &[&str] = &["jcl", "proc"];

/// Default directory exclusion patterns.
const DEFAULT_EXCLUDES: &[&str] = &[
    "backup", "bak", "archive", "old", "deprecated", ".git", ".svn", "node_modules",
];

/// Discover all relevant files under `root_dir`.
pub fn discover_files(
    root_dir: &Path,
    custom_extensions: &[String],
    exclude_patterns: &[String],
) -> Result<Vec<DiscoveredFile>> {
    let root_dir = root_dir
        .canonicalize()
        .into_diagnostic()
        .map_err(|e| miette::miette!("cannot resolve root directory: {e}"))?;

    let mut files = Vec::new();
    let mut visited_inodes = HashSet::new();

    walk_directory(
        &root_dir,
        &root_dir,
        custom_extensions,
        exclude_patterns,
        &mut files,
        &mut visited_inodes,
    )?;

    files.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
    Ok(files)
}

fn walk_directory(
    dir: &Path,
    root: &Path,
    custom_extensions: &[String],
    exclude_patterns: &[String],
    files: &mut Vec<DiscoveredFile>,
    visited: &mut HashSet<u64>,
) -> Result<()> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!(
                "  [WARN] Cannot read directory {}: {e}",
                dir.display()
            );
            return Ok(());
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();

        // Check for symlink loops via inode tracking.
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if let Ok(meta) = fs::metadata(&path) {
                let inode = meta.ino();
                if meta.is_dir() && !visited.insert(inode) {
                    eprintln!(
                        "  [WARN] Symlink loop detected, skipping: {}",
                        path.display()
                    );
                    continue;
                }
            }
        }

        if path.is_dir() {
            // Check exclusions.
            let dir_name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_lowercase())
                .unwrap_or_default();

            if is_excluded(&dir_name, exclude_patterns) {
                continue;
            }

            walk_directory(&path, root, custom_extensions, exclude_patterns, files, visited)?;
        } else if path.is_file() {
            if let Some(discovered) = classify_file(&path, root, custom_extensions) {
                files.push(discovered);
            }
        }
    }

    Ok(())
}

fn is_excluded(dir_name: &str, custom_excludes: &[String]) -> bool {
    // Check built-in exclusions.
    for excl in DEFAULT_EXCLUDES {
        if dir_name == *excl {
            return true;
        }
    }
    // Check custom exclusions (simple name match).
    for excl in custom_excludes {
        let excl_lower = excl.to_lowercase();
        if dir_name == excl_lower {
            return true;
        }
        // Support glob-like ** patterns via simple contains check.
        if let Some(pattern) = excl_lower.strip_prefix("**/") {
            if let Some(pattern) = pattern.strip_suffix("/**") {
                if dir_name == pattern {
                    return true;
                }
            } else if dir_name == pattern.trim_end_matches('/') {
                return true;
            }
        }
    }
    false
}

fn classify_file(path: &Path, root: &Path, custom_extensions: &[String]) -> Option<DiscoveredFile> {
    let ext = path
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    let file_type = if !custom_extensions.is_empty() {
        // User specified custom extensions -- treat all as source.
        if custom_extensions.iter().any(|e| e.to_lowercase() == ext) {
            FileType::Source
        } else {
            return None;
        }
    } else {
        classify_extension(&ext)?
    };

    let metadata = fs::metadata(path).ok()?;
    let file_size = metadata.len();
    let mtime_epoch = metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let relative_path = path
        .strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string();

    let absolute_path = path.to_string_lossy().to_string();

    Some(DiscoveredFile {
        relative_path,
        absolute_path,
        extension: ext,
        file_size,
        mtime_epoch,
        file_type,
    })
}

fn classify_extension(ext: &str) -> Option<FileType> {
    if SOURCE_EXTS.contains(&ext) {
        Some(FileType::Source)
    } else if COPYBOOK_EXTS.contains(&ext) {
        Some(FileType::Copybook)
    } else if JCL_EXTS.contains(&ext) {
        Some(FileType::Jcl)
    } else {
        None
    }
}
