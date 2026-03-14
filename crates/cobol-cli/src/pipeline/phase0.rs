//! Phase 0: Configure -- discover copybooks and write `.cobol2rust.toml`.
//!
//! If `.cobol2rust.toml` already exists, this phase is a no-op.
//! Otherwise, auto-discovers copybook directories using 4 heuristics.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use miette::{miette, IntoDiagnostic, Result};

use crate::workspace::PROJECT_CONFIG_FILE;

/// Heuristic directory names that commonly hold copybooks (case-insensitive).
const COPYBOOK_DIR_NAMES: &[&str] = &[
    "copy",
    "copylib",
    "cpy",
    "copybooks",
    "includes",
    "copybook",
    "cpylib",
    "cpybook",
];

/// Extensions that indicate copybook files.
const COPYBOOK_EXTENSIONS: &[&str] = &["cpy", "copy", "cpylib"];

/// Run Phase 0: discover copybooks and write `.cobol2rust.toml` if needed.
///
/// Returns the list of discovered copy paths (relative to project_dir).
pub fn run_phase0(project_dir: &Path, quiet: bool) -> Result<Vec<PathBuf>> {
    let config_path = project_dir.join(PROJECT_CONFIG_FILE);

    if config_path.exists() {
        if !quiet {
            eprintln!(
                "Phase 0: Configure -- skipped ({} exists)",
                PROJECT_CONFIG_FILE
            );
        }
        // Return empty -- caller should use load_project_config() for actual paths
        return Ok(vec![]);
    }

    if !quiet {
        eprintln!("Phase 0: Configure -- discovering copybook directories...");
    }

    let mut copy_dirs: BTreeSet<PathBuf> = BTreeSet::new();

    // Heuristic 1: directories with known copybook names
    discover_by_dir_name(project_dir, project_dir, &mut copy_dirs, 0, 5);

    // Heuristic 2: directories containing .cpy/.copy files
    discover_by_copybook_extension(project_dir, project_dir, &mut copy_dirs, 0, 5);

    // Heuristic 3: .cbl/.cob files without PROCEDURE DIVISION (data-only = copybook)
    discover_data_only_sources(project_dir, project_dir, &mut copy_dirs, 0, 5);

    // Convert to relative paths
    let relative_dirs: Vec<PathBuf> = copy_dirs
        .iter()
        .filter_map(|p| p.strip_prefix(project_dir).ok())
        .map(|p| p.to_path_buf())
        .collect();

    // Write .cobol2rust.toml
    let toml_content = build_toml(&relative_dirs);
    fs::write(&config_path, &toml_content)
        .into_diagnostic()
        .map_err(|e| miette!("failed to write {}: {e}", config_path.display()))?;

    if !quiet {
        if relative_dirs.is_empty() {
            eprintln!("  No copybook directories found");
        } else {
            eprintln!("  Found {} copybook directories:", relative_dirs.len());
            for d in &relative_dirs {
                eprintln!("    {}", d.display());
            }
        }
        eprintln!("  Wrote {}", config_path.display());
    }

    Ok(relative_dirs)
}

/// Heuristic 1: find directories whose name matches known copybook directory names.
fn discover_by_dir_name(
    root: &Path,
    dir: &Path,
    found: &mut BTreeSet<PathBuf>,
    depth: usize,
    max_depth: usize,
) {
    if depth > max_depth {
        return;
    }
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            let lower = name.to_lowercase();
            // Skip hidden dirs and common non-source dirs
            if lower.starts_with('.') || lower == "node_modules" || lower == ".git" {
                continue;
            }
            if COPYBOOK_DIR_NAMES.contains(&lower.as_str()) {
                found.insert(path.clone());
            }
        }
        // Recurse even if matched (might have nested copybook dirs)
        discover_by_dir_name(root, &path, found, depth + 1, max_depth);
    }
}

/// Heuristic 2: find directories that contain files with copybook extensions.
fn discover_by_copybook_extension(
    root: &Path,
    dir: &Path,
    found: &mut BTreeSet<PathBuf>,
    depth: usize,
    max_depth: usize,
) {
    if depth > max_depth {
        return;
    }
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    let mut has_copybook_files = false;
    let mut subdirs = vec![];

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                let lower = name.to_lowercase();
                if !lower.starts_with('.') && lower != "node_modules" {
                    subdirs.push(path);
                }
            }
        } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if COPYBOOK_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
                has_copybook_files = true;
            }
        }
    }

    if has_copybook_files {
        found.insert(dir.to_path_buf());
    }

    for subdir in subdirs {
        discover_by_copybook_extension(root, &subdir, found, depth + 1, max_depth);
    }
}

/// Heuristic 3: find directories containing .cbl/.cob files without PROCEDURE DIVISION.
/// These are likely copybook/data-only files that should be on the copy path.
fn discover_data_only_sources(
    root: &Path,
    dir: &Path,
    found: &mut BTreeSet<PathBuf>,
    depth: usize,
    max_depth: usize,
) {
    if depth > max_depth {
        return;
    }
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    let mut has_data_only = false;
    let mut subdirs = vec![];

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                let lower = name.to_lowercase();
                if !lower.starts_with('.') && lower != "node_modules" {
                    subdirs.push(path);
                }
            }
            continue;
        }

        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let lower_ext = ext.to_lowercase();
            if lower_ext == "cbl" || lower_ext == "cob" || lower_ext == "cobol" {
                // Quick check: read first ~200 lines looking for PROCEDURE DIVISION
                if is_data_only_file(&path) {
                    has_data_only = true;
                }
            }
        }
    }

    if has_data_only {
        found.insert(dir.to_path_buf());
    }

    for subdir in subdirs {
        discover_data_only_sources(root, &subdir, found, depth + 1, max_depth);
    }
}

/// Check if a COBOL source file appears to be data-only (no PROCEDURE DIVISION).
fn is_data_only_file(path: &Path) -> bool {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return false,
    };

    // Quick scan: if the file has no PROCEDURE DIVISION, it's data-only
    let upper = content.to_uppercase();
    !upper.contains("PROCEDURE DIVISION")
}

/// Build the `.cobol2rust.toml` content from discovered directories.
fn build_toml(copy_dirs: &[PathBuf]) -> String {
    let mut out = String::new();
    writeln!(out, "[workspace]").unwrap();

    if copy_dirs.is_empty() {
        writeln!(out, "copy_paths = []").unwrap();
    } else {
        writeln!(out, "copy_paths = [").unwrap();
        for (i, dir) in copy_dirs.iter().enumerate() {
            let path_str = dir.to_string_lossy();
            if i < copy_dirs.len() - 1 {
                writeln!(out, "    \"{}\",", path_str).unwrap();
            } else {
                writeln!(out, "    \"{}\"", path_str).unwrap();
            }
        }
        writeln!(out, "]").unwrap();
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_toml_empty() {
        let result = build_toml(&[]);
        assert!(result.contains("copy_paths = []"));
    }

    #[test]
    fn test_build_toml_with_dirs() {
        let dirs = vec![PathBuf::from("copy"), PathBuf::from("src/copylib")];
        let result = build_toml(&dirs);
        assert!(result.contains("\"copy\""));
        assert!(result.contains("\"src/copylib\""));
    }
}
