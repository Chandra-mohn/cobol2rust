//! Workspace analysis: scan COBOL sources, build CALL graph, detect main/lib.
//!
//! Shared logic used by both `init` and `transpile --workspace`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use miette::{miette, Context, IntoDiagnostic, Result};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use toml;

use cobol_transpiler::ast::{Literal, Operand, Statement};
use cobol_transpiler::parser::preprocess::preprocess;
use cobol_transpiler::parser::{extract_copy_targets, extract_program_id, parse_cobol};

/// Information about a single COBOL program in the workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramInfo {
    /// Program ID (from PROGRAM-ID paragraph).
    pub program_id: String,
    /// Source file path (relative to scan root).
    pub source: PathBuf,
    /// Whether this is a main program or a library/subprogram.
    #[serde(rename = "type")]
    pub program_type: ProgramType,
    /// Programs this one CALLs.
    pub calls: BTreeSet<String>,
    /// Programs that CALL this one (populated after cross-reference).
    pub called_by: BTreeSet<String>,
    /// Copybook names referenced via COPY statements.
    pub copybooks: Vec<String>,
    /// Whether detection was overridden by user.
    #[serde(default)]
    pub overridden: bool,
    /// True if the program has LINKAGE SECTION.
    #[serde(skip)]
    pub has_linkage: bool,
    /// True if the program has PROCEDURE DIVISION USING.
    #[serde(skip)]
    pub has_using: bool,
}

/// Program type: main executable or library subprogram.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProgramType {
    Main,
    Lib,
    Skip,
}

impl std::fmt::Display for ProgramType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Main => write!(f, "main"),
            Self::Lib => write!(f, "lib"),
            Self::Skip => write!(f, "skip"),
        }
    }
}

/// Manifest file for a COBOL workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// Programs in the workspace, keyed by Rust crate name.
    pub programs: BTreeMap<String, ProgramInfo>,
    /// Copybook configuration.
    pub copybooks: CopybookConfig,
}

/// Copybook configuration in the manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopybookConfig {
    /// Directories containing copybook files.
    pub sources: Vec<PathBuf>,
    /// Individual copybook files discovered.
    pub files: Vec<String>,
}

/// Result of scanning a directory of COBOL sources.
pub struct WorkspaceAnalysis {
    /// Programs discovered, keyed by Rust crate name.
    pub programs: BTreeMap<String, ProgramInfo>,
    /// All copybook names referenced across programs.
    pub all_copybooks: BTreeSet<String>,
    /// Copybook source directories found.
    pub copybook_dirs: Vec<PathBuf>,
    /// Files that failed to parse (path, error message).
    pub errors: Vec<(PathBuf, String)>,
}

/// Project configuration loaded from `.cobol2rust.toml`.
///
/// This file is placed in a repo root (by `discover_copybooks.sh` or manually)
/// and provides per-project settings that `cobol2rust transpile --workspace`
/// reads automatically.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ProjectConfig {
    /// Workspace-level settings.
    #[serde(default)]
    pub workspace: WorkspaceConfig,
}

/// Workspace configuration section of `.cobol2rust.toml`.
#[derive(Debug, Clone, Default, Deserialize)]
#[allow(dead_code)]
pub struct WorkspaceConfig {
    /// Directories containing copybook files, relative to the project root.
    #[serde(default)]
    pub copy_paths: Vec<PathBuf>,

    /// File extensions to scan (default: cbl, cob, cobol).
    #[serde(default)]
    pub extensions: Vec<String>,

    /// Glob patterns to exclude.
    #[serde(default)]
    pub exclude: Vec<String>,
}

/// Default config file name.
pub const PROJECT_CONFIG_FILE: &str = ".cobol2rust.toml";

/// Load a `.cobol2rust.toml` from the given directory, if it exists.
///
/// Returns `None` if the file does not exist. Returns an error if the
/// file exists but cannot be parsed.
pub fn load_project_config(dir: &Path) -> Result<Option<ProjectConfig>> {
    let config_path = dir.join(PROJECT_CONFIG_FILE);
    if !config_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&config_path)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read {}", config_path.display()))?;

    let config: ProjectConfig = toml::from_str(&content)
        .map_err(|e| miette!("failed to parse {}: {e}", config_path.display()))?;

    Ok(Some(config))
}

/// Resolve copy_paths from a ProjectConfig relative to the project root,
/// returning absolute PathBufs.
pub fn resolve_copy_paths(dir: &Path, config: &ProjectConfig) -> Vec<PathBuf> {
    config
        .workspace
        .copy_paths
        .iter()
        .map(|p| {
            if p.is_absolute() {
                p.clone()
            } else {
                dir.join(p)
            }
        })
        .filter(|p| p.is_dir())
        .collect()
}

/// Scan a directory for `.cbl` files and analyze the workspace.
///
/// This builds the CALL graph, detects main vs lib for each program,
/// and identifies copybook dependencies.
pub fn analyze_workspace(
    input_dir: &Path,
    overrides: &BTreeMap<String, ProgramType>,
    continue_on_error: bool,
) -> Result<WorkspaceAnalysis> {
    // 1. Discover .cbl files
    let cbl_files = discover_cobol_files(input_dir)?;
    if cbl_files.is_empty() {
        return Err(miette!(
            "no .cbl files found in {}",
            input_dir.display()
        ));
    }

    // 2. Discover copybook directories (.cpy files)
    let copybook_dirs = discover_copybook_dirs(input_dir);

    // 3. Parse each file and extract program info (parallel)
    let programs_mutex: Mutex<BTreeMap<String, ProgramInfo>> = Mutex::new(BTreeMap::new());
    let errors_mutex: Mutex<Vec<(PathBuf, String)>> = Mutex::new(Vec::new());
    let all_copybooks_mutex: Mutex<BTreeSet<String>> = Mutex::new(BTreeSet::new());

    cbl_files.par_iter().for_each(|cbl_path| {
        match analyze_single_file(cbl_path, input_dir) {
            Ok(info) => {
                let crate_name = cobol_name_to_crate(&info.program_id);
                {
                    let mut cbs = all_copybooks_mutex.lock().unwrap();
                    for cb in &info.copybooks {
                        cbs.insert(cb.clone());
                    }
                }
                programs_mutex.lock().unwrap().insert(crate_name, info);
            }
            Err(e) => {
                if continue_on_error {
                    errors_mutex.lock().unwrap().push((cbl_path.clone(), format!("{e}")));
                }
                // When not continue_on_error, we skip the error here and check below.
                // Parallel iteration can't return early, so we collect and check after.
            }
        }
    });

    let mut programs = programs_mutex.into_inner().unwrap();
    let errors = errors_mutex.into_inner().unwrap();
    let all_copybooks = all_copybooks_mutex.into_inner().unwrap();

    // If not continue_on_error and we had errors, report the first one.
    if !continue_on_error && !errors.is_empty() {
        let (path, err) = &errors[0];
        return Err(miette!("failed to analyze {}: {err}", path.display()));
    }

    // 4. Cross-reference: populate called_by from calls
    let call_targets: Vec<(String, BTreeSet<String>)> = programs
        .iter()
        .map(|(name, info)| (name.clone(), info.calls.clone()))
        .collect();

    let program_ids: BTreeMap<String, String> = programs
        .iter()
        .map(|(k, v)| (k.clone(), v.program_id.clone()))
        .collect();

    for (caller_crate, calls) in &call_targets {
        for call_target in calls {
            let target_crate = cobol_name_to_crate(call_target);
            if let Some(target_info) = programs.get_mut(&target_crate) {
                if let Some(caller_id) = program_ids.get(caller_crate) {
                    target_info.called_by.insert(caller_id.clone());
                }
            }
        }
    }

    // 5. Detect main vs lib using the cross-file CALL graph
    detect_program_types(&mut programs, overrides);

    Ok(WorkspaceAnalysis {
        programs,
        all_copybooks,
        copybook_dirs,
        errors,
    })
}

/// Analyze a single COBOL source file.
fn analyze_single_file(path: &Path, base_dir: &Path) -> Result<ProgramInfo> {
    let source = fs::read_to_string(path)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read {}", path.display()))?;

    // Extract copybook targets from raw source (before preprocessing)
    let copybooks = extract_copy_targets(&source);

    // Preprocess and extract program ID
    let preprocessed = preprocess(&source).map_err(|e| miette!("{e}"))?;
    let program_id = extract_program_id(&preprocessed);

    // Parse the full AST for deeper analysis
    let ast = parse_cobol(&source).map_err(|e| miette!("{e}"))?;

    // Check for LINKAGE SECTION and USING via text scan.
    // parse_cobol() doesn't populate linkage/using_params yet,
    // so we detect them from the preprocessed source.
    let upper = preprocessed.to_uppercase();
    let has_linkage = upper.contains("LINKAGE SECTION");
    let has_using = upper.contains("PROCEDURE DIVISION USING");

    // Extract CALL targets from the AST
    let mut calls = BTreeSet::new();

    if let Some(ref pd) = ast.procedure_division {
        for section in &pd.sections {
            for para in &section.paragraphs {
                for sentence in &para.sentences {
                    collect_calls(&sentence.statements, &mut calls);
                }
            }
        }
        for para in &pd.paragraphs {
            for sentence in &para.sentences {
                collect_calls(&sentence.statements, &mut calls);
            }
        }
    }

    let relative_source = path
        .strip_prefix(base_dir)
        .unwrap_or(path)
        .to_path_buf();

    Ok(ProgramInfo {
        program_id,
        source: relative_source,
        program_type: ProgramType::Main, // will be resolved later
        calls,
        called_by: BTreeSet::new(),
        copybooks,
        overridden: false,
        has_linkage,
        has_using,
    })
}

/// Recursively collect CALL targets from statements.
fn collect_calls(stmts: &[Statement], calls: &mut BTreeSet<String>) {
    for stmt in stmts {
        match stmt {
            Statement::Call(call) => {
                if let Operand::Literal(Literal::Alphanumeric(name)) = &call.program {
                    calls.insert(name.to_uppercase());
                }
                collect_calls(&call.on_exception, calls);
                collect_calls(&call.not_on_exception, calls);
            }
            Statement::If(if_stmt) => {
                collect_calls(&if_stmt.then_body, calls);
                collect_calls(&if_stmt.else_body, calls);
            }
            Statement::Evaluate(eval) => {
                for branch in &eval.when_branches {
                    collect_calls(&branch.body, calls);
                }
                collect_calls(&eval.when_other, calls);
            }
            Statement::Perform(perf) => {
                collect_calls(&perf.body, calls);
            }
            _ => {}
        }
    }
}

/// Detect program types using the CALL graph analysis per spec:
///
/// 1. Programs that are CALL targets = subprograms (lib)
/// 2. Programs with LINKAGE SECTION + USING = subprograms (lib)
/// 3. Remaining programs = main (conservative default)
/// 4. Override per-file via config
fn detect_program_types(
    programs: &mut BTreeMap<String, ProgramInfo>,
    overrides: &BTreeMap<String, ProgramType>,
) {
    // Collect all CALL targets across all programs
    let call_targets: BTreeSet<String> = programs
        .values()
        .flat_map(|p| p.calls.iter().cloned())
        .collect();

    for info in programs.values_mut() {
        // Check for user override first
        if let Some(&override_type) = overrides.get(&info.program_id) {
            info.program_type = override_type;
            info.overridden = true;
            continue;
        }

        // Rule 1: Programs that are CALL targets = lib
        if call_targets.contains(&info.program_id) {
            info.program_type = ProgramType::Lib;
            continue;
        }

        // Rule 2: LINKAGE + USING = lib
        if info.has_linkage && info.has_using {
            info.program_type = ProgramType::Lib;
            continue;
        }

        // Rule 3: Default = main (conservative)
        info.program_type = ProgramType::Main;
    }
}

/// Convert a COBOL program name to a valid Rust crate name.
///
/// - Converts to lowercase
/// - Replaces hyphens with underscores
/// - Strips leading digits
pub fn cobol_name_to_crate(name: &str) -> String {
    let lower = name.to_lowercase();
    let sanitized = lower.replace('-', "_");
    // Strip leading digits (invalid for crate names)
    let trimmed = sanitized.trim_start_matches(|c: char| c.is_ascii_digit());
    if trimmed.is_empty() {
        format!("program_{sanitized}")
    } else {
        trimmed.to_string()
    }
}

/// Discover all `.cbl` files in a directory (recursive).
fn discover_cobol_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_cobol_files_recursive(dir, &mut files)?;
    files.sort();
    Ok(files)
}

/// Recursively collect `.cbl`/`.cob`/`.cobol` files from a directory tree.
fn collect_cobol_files_recursive(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    let entries = fs::read_dir(dir)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read directory {}", dir.display()))?;

    for entry in entries {
        let entry = entry.into_diagnostic()?;
        let path = entry.path();
        if path.is_dir() {
            collect_cobol_files_recursive(&path, files)?;
        } else if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext_lower = ext.to_string_lossy().to_lowercase();
                if ext_lower == "cbl" || ext_lower == "cob" || ext_lower == "cobol" {
                    files.push(path);
                }
            }
        }
    }
    Ok(())
}

/// Discover directories containing `.cpy` files.
fn discover_copybook_dirs(dir: &Path) -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    // Check the input directory itself for .cpy files
    if has_copybook_files(dir) {
        dirs.push(dir.to_path_buf());
    }

    // Check immediate subdirectories
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && has_copybook_files(&path) {
                dirs.push(path);
            }
        }
    }

    dirs.sort();
    dirs
}

/// Check if a directory contains any `.cpy` files.
fn has_copybook_files(dir: &Path) -> bool {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Some(ext) = entry.path().extension() {
                if ext.to_string_lossy().to_lowercase() == "cpy" {
                    return true;
                }
            }
        }
    }
    false
}

/// Discover individual `.cpy` files in a directory.
pub fn discover_copybook_files(dir: &Path) -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext.to_string_lossy().to_lowercase() == "cpy" {
                        if let Some(name) = path.file_name() {
                            files.push(name.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
    }
    files.sort();
    files
}

/// Build a manifest from a workspace analysis.
pub fn build_manifest(analysis: &WorkspaceAnalysis) -> Manifest {
    Manifest {
        programs: analysis.programs.clone(),
        copybooks: CopybookConfig {
            sources: analysis.copybook_dirs.clone(),
            files: analysis.all_copybooks.iter().cloned().collect(),
        },
    }
}

/// Load program type overrides from an existing manifest file.
pub fn load_manifest_overrides(
    manifest_path: Option<&Path>,
) -> Result<BTreeMap<String, ProgramType>> {
    let mut overrides = BTreeMap::new();

    if let Some(path) = manifest_path {
        if path.exists() {
            let content = fs::read_to_string(path)
                .into_diagnostic()
                .wrap_err_with(|| {
                    format!("failed to read manifest {}", path.display())
                })?;

            let mut current_program: Option<String> = None;
            for line in content.lines() {
                let trimmed = line.trim();
                if let Some(rest) = trimmed.strip_prefix("[programs.") {
                    if let Some(name) = rest.strip_suffix(']') {
                        current_program = Some(name.to_string());
                    }
                } else if current_program.is_some() {
                    if let Some(rest) = trimmed.strip_prefix("type = ") {
                        let type_str = rest
                            .trim_matches('"')
                            .split_whitespace()
                            .next()
                            .unwrap_or("");
                        let program_type = match type_str {
                            "main" => ProgramType::Main,
                            "lib" => ProgramType::Lib,
                            "skip" => ProgramType::Skip,
                            _ => continue,
                        };
                        if let Some(prog) = current_program.take() {
                            overrides.insert(
                                prog.to_uppercase().replace('_', "-"),
                                program_type,
                            );
                        }
                    }
                }
            }
        }
    }

    Ok(overrides)
}

/// Serialize a manifest to TOML string.
pub fn manifest_to_toml(manifest: &Manifest) -> String {
    let mut out = String::new();
    out.push_str("# Auto-generated by cobol2rust init. Edit to override detection.\n");
    out.push_str("# Re-run `cobol2rust transpile --workspace` to apply changes.\n\n");

    for (crate_name, info) in &manifest.programs {
        let _ = writeln!(out, "[programs.{crate_name}]");
        let _ = writeln!(out, "source = \"{}\"", info.source.display());
        let _ = write!(out, "type = \"{}\"", info.program_type);
        if info.overridden {
            out.push_str("  # user override");
        } else {
            out.push_str("  # auto-detected");
        }
        out.push('\n');

        if !info.calls.is_empty() {
            let calls: Vec<&str> = info.calls.iter().map(String::as_str).collect();
            let _ = writeln!(out, "calls = {calls:?}");
        }
        if !info.called_by.is_empty() {
            let called_by: Vec<&str> = info.called_by.iter().map(String::as_str).collect();
            let _ = writeln!(out, "called_by = {called_by:?}");
        }
        out.push('\n');
    }

    out.push_str("[copybooks]\n");
    if manifest.copybooks.sources.is_empty() {
        out.push_str("sources = []\n");
    } else {
        let sources: Vec<String> = manifest
            .copybooks
            .sources
            .iter()
            .map(|p| p.display().to_string())
            .collect();
        let _ = writeln!(out, "sources = {sources:?}");
    }
    if manifest.copybooks.files.is_empty() {
        out.push_str("files = []\n");
    } else {
        let files: Vec<&str> = manifest.copybooks.files.iter().map(String::as_str).collect();
        let _ = writeln!(out, "files = {files:?}");
    }

    out
}
