//! `cobol2rust transpile` -- transpile COBOL source to Rust.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Instant;

use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use miette::{miette, Context, IntoDiagnostic, Result};
use rayon::prelude::*;

use cobol_transpiler::transpile::{transpile_with_config, transpile_with_config_and_diagnostics, TranspileConfig};

use crate::scan::ndjson::{self, NdjsonWriter, TranspileResultRecord};
use crate::workspace::{
    analyze_workspace, build_manifest, cobol_name_to_crate, discover_copybook_files,
    load_manifest_overrides, manifest_to_toml, ProgramType,
};
use crate::Cli;

/// Arguments for `cobol2rust transpile`.
#[derive(Debug, Args)]
#[allow(clippy::struct_excessive_bools)]
pub struct TranspileArgs {
    /// COBOL source file or directory to transpile.
    pub input: PathBuf,

    /// Output file or directory (default: stdout for single file).
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Force main program output (generates `fn main()`).
    #[arg(long, conflicts_with = "lib")]
    pub main: bool,

    /// Force library output (no `fn main()`).
    #[arg(long, conflicts_with = "main")]
    pub lib: bool,

    /// COPY library mapping NAME=DIR (repeatable).
    #[arg(short = 'L', long = "library-map")]
    pub library_map: Vec<String>,

    /// Generate a Cargo workspace (required for directory input).
    #[arg(long)]
    pub workspace: bool,

    /// Skip files that fail, report at end.
    #[arg(long)]
    pub continue_on_error: bool,

    /// Read/write manifest for main/lib overrides.
    #[arg(long)]
    pub manifest: Option<PathBuf>,

    /// Path to cobol-runtime crate (for workspace mode path dependency).
    /// If not specified, the workspace Cargo.toml uses a crates.io version spec.
    #[arg(long)]
    pub runtime_path: Option<PathBuf>,

    /// Transpile files in parallel (workspace mode only).
    #[arg(long)]
    pub parallel: bool,

    /// Number of parallel jobs (default: number of CPUs, implies --parallel).
    #[arg(short = 'j', long)]
    pub jobs: Option<usize>,

    /// Skip unchanged files (workspace mode only).
    /// Compares source mtime against output mtime.
    #[arg(long)]
    pub incremental: bool,
}

/// Run the transpile subcommand.
pub fn run(cli: &Cli, args: &TranspileArgs) -> Result<ExitCode> {
    if args.input.is_dir() {
        if !args.workspace {
            return Err(miette!(
                "input is a directory; use --workspace for directory mode"
            ));
        }
        return run_workspace(cli, args);
    }

    // Single-file mode
    let source = fs::read_to_string(&args.input)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read {}", args.input.display()))?;

    let config = build_config(cli, args)?;

    if !cli.quiet {
        eprintln!("Transpiling {}...", args.input.display());
    }

    let rust_source =
        transpile_with_config(&source, &config).map_err(|e| miette!("{e}"))?;

    match &args.output {
        Some(path) => {
            fs::write(path, &rust_source)
                .into_diagnostic()
                .wrap_err_with(|| format!("failed to write {}", path.display()))?;
            if !cli.quiet {
                eprintln!("Wrote {}", path.display());
            }
        }
        None => {
            std::io::stdout()
                .write_all(rust_source.as_bytes())
                .into_diagnostic()
                .wrap_err("failed to write to stdout")?;
        }
    }

    Ok(ExitCode::SUCCESS)
}

/// A single program ready for transpilation.
struct TranspileJob {
    crate_name: String,
    source_path: PathBuf,
    crate_dir: PathBuf,
    entry_file: &'static str,
    cargo_toml_content: String,
}

/// Result of transpiling one program.
enum TranspileOutcome {
    Success {
        crate_name: String,
        rust_lines: i32,
        duration_ms: i32,
        coverage_pct: f64,
        total_statements: i32,
        mapped_statements: i32,
        verbs_used: String,
        verbs_unsupported: String,
    },
    Skipped { crate_name: String },
    Failed { crate_name: String, error: String, duration_ms: i32 },
}

/// Run workspace mode: transpile a directory of COBOL files into a Cargo workspace.
fn run_workspace(cli: &Cli, args: &TranspileArgs) -> Result<ExitCode> {
    let output_dir = args.output.as_ref().ok_or_else(|| {
        miette!("--output is required for workspace mode")
    })?;

    let overrides = load_manifest_overrides(args.manifest.as_deref())?;

    if !cli.quiet {
        eprintln!(
            "Analyzing {} for workspace transpilation...",
            args.input.display()
        );
    }

    let analysis =
        analyze_workspace(&args.input, &overrides, args.continue_on_error)?;

    for (path, err) in &analysis.errors {
        eprintln!("  warning: skipped {}: {err}", path.display());
    }

    if analysis.programs.is_empty() {
        return Err(miette!(
            "no programs found in {}",
            args.input.display()
        ));
    }

    let config = build_config(cli, args)?;

    // Create output directory structure
    fs::create_dir_all(output_dir)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to create {}", output_dir.display()))?;

    // Build workspace member list
    let has_copybooks = !analysis.all_copybooks.is_empty();
    let mut members = Vec::new();

    if has_copybooks {
        members.push("copybooks".to_string());
    }
    for (crate_name, info) in &analysis.programs {
        if info.program_type != ProgramType::Skip {
            members.push(format!("programs/{crate_name}"));
        }
    }

    // Write workspace Cargo.toml
    let runtime_path_str = args.runtime_path.as_ref().map(|p| {
        let canonical = p.canonicalize().unwrap_or_else(|_| p.clone());
        canonical.to_string_lossy().to_string()
    });
    let workspace_toml =
        generate_workspace_cargo_toml(&members, runtime_path_str.as_deref());
    fs::write(output_dir.join("Cargo.toml"), &workspace_toml)
        .into_diagnostic()
        .wrap_err("failed to write workspace Cargo.toml")?;

    // Create copybooks crate if needed
    if has_copybooks {
        create_copybooks_crate(output_dir, &analysis)?;
    }

    // Prepare transpilation jobs
    let programs_dir = output_dir.join("programs");
    fs::create_dir_all(&programs_dir)
        .into_diagnostic()
        .wrap_err("failed to create programs/")?;

    let mut jobs: Vec<TranspileJob> = Vec::new();

    for (crate_name, info) in &analysis.programs {
        if info.program_type == ProgramType::Skip {
            continue;
        }

        let source_path = args.input.join(&info.source);
        let crate_dir = programs_dir.join(crate_name);

        // Build dependencies list
        let mut deps: Vec<String> = Vec::new();
        if has_copybooks {
            deps.push("copybooks".to_string());
        }
        for call_target in &info.calls {
            let target_crate = cobol_name_to_crate(call_target);
            if analysis.programs.contains_key(&target_crate) {
                deps.push(target_crate);
            }
        }

        let cargo_toml_content =
            generate_program_cargo_toml(crate_name, info.program_type, &deps);

        let entry_file = if info.program_type == ProgramType::Main {
            "main.rs"
        } else {
            "lib.rs"
        };

        jobs.push(TranspileJob {
            crate_name: crate_name.clone(),
            source_path,
            crate_dir,
            entry_file,
            cargo_toml_content,
        });
    }

    let total = jobs.len();
    let use_parallel = args.parallel || args.jobs.is_some();

    // Configure rayon thread pool if --jobs is specified
    if let Some(num_jobs) = args.jobs {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_jobs)
            .build_global()
            .ok(); // Ignore error if already initialized
    }

    // Create progress bar
    let pb = if !cli.quiet && total > 1 {
        let bar = ProgressBar::new(total as u64);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40}] {pos}/{len} {msg}")
                .expect("valid template")
                .progress_chars("=> "),
        );
        Some(bar)
    } else {
        None
    };

    let success_count = AtomicU32::new(0);
    let fail_count = AtomicU32::new(0);
    let skip_count = AtomicU32::new(0);

    // Transpile function for one job
    let transpile_one = |job: &TranspileJob| -> TranspileOutcome {
        let start = Instant::now();
        let src_dir = job.crate_dir.join("src");

        // Create directories (sequential I/O, but fast)
        if fs::create_dir_all(&src_dir).is_err() {
            return TranspileOutcome::Failed {
                crate_name: job.crate_name.clone(),
                error: format!("failed to create {}/src", job.crate_dir.display()),
                duration_ms: start.elapsed().as_millis() as i32,
            };
        }

        // Write Cargo.toml
        if fs::write(job.crate_dir.join("Cargo.toml"), &job.cargo_toml_content).is_err() {
            return TranspileOutcome::Failed {
                crate_name: job.crate_name.clone(),
                error: format!(
                    "failed to write {}/Cargo.toml",
                    job.crate_dir.display()
                ),
                duration_ms: start.elapsed().as_millis() as i32,
            };
        }

        let output_path = src_dir.join(job.entry_file);

        // Incremental: skip if output is newer than source
        if args.incremental && is_up_to_date(&job.source_path, &output_path) {
            return TranspileOutcome::Skipped {
                crate_name: job.crate_name.clone(),
            };
        }

        // Transpile with diagnostics for reporting
        match transpile_single_with_diagnostics(&job.source_path, &config) {
            Ok(result) => {
                let rust_code = match result.rust_code {
                    Some(ref code) => code,
                    None => {
                        return TranspileOutcome::Failed {
                            crate_name: job.crate_name.clone(),
                            error: "transpilation produced no output".to_string(),
                            duration_ms: start.elapsed().as_millis() as i32,
                        };
                    }
                };
                let rust_lines = rust_code.lines().count() as i32;
                if fs::write(&output_path, rust_code).is_err() {
                    return TranspileOutcome::Failed {
                        crate_name: job.crate_name.clone(),
                        error: format!(
                            "failed to write {}",
                            output_path.display()
                        ),
                        duration_ms: start.elapsed().as_millis() as i32,
                    };
                }
                let coverage_pct = result.statement_coverage();
                let verbs_used: Vec<&str> = result.stats.verbs_used.iter()
                    .map(|s| s.as_str()).collect();
                let verbs_unsupported: Vec<&str> = result.stats.verbs_unsupported.iter()
                    .map(|s| s.as_str()).collect();
                TranspileOutcome::Success {
                    crate_name: job.crate_name.clone(),
                    rust_lines,
                    duration_ms: start.elapsed().as_millis() as i32,
                    coverage_pct,
                    total_statements: result.stats.total_statements as i32,
                    mapped_statements: result.stats.mapped_statements as i32,
                    verbs_used: verbs_used.join(","),
                    verbs_unsupported: verbs_unsupported.join(","),
                }
            }
            Err(e) => TranspileOutcome::Failed {
                crate_name: job.crate_name.clone(),
                error: format!("{e}"),
                duration_ms: start.elapsed().as_millis() as i32,
            },
        }
    };

    // Execute transpilation (parallel or sequential)
    let update_bar = |outcome: &TranspileOutcome, bar: &Option<ProgressBar>| {
        if let Some(bar) = bar {
            bar.inc(1);
            match outcome {
                TranspileOutcome::Success { crate_name, .. } => {
                    bar.set_message(crate_name.clone());
                }
                TranspileOutcome::Skipped { crate_name } => {
                    bar.set_message(format!("{crate_name} (skipped)"));
                }
                TranspileOutcome::Failed { crate_name, .. } => {
                    bar.set_message(format!("{crate_name} (FAILED)"));
                }
            }
        }
    };

    let outcomes: Vec<(usize, TranspileOutcome)> = if use_parallel {
        jobs.par_iter()
            .enumerate()
            .map(|(idx, job)| {
                let outcome = transpile_one(job);
                update_bar(&outcome, &pb);
                (idx, outcome)
            })
            .collect()
    } else {
        jobs.iter()
            .enumerate()
            .map(|(idx, job)| {
                let outcome = transpile_one(job);
                update_bar(&outcome, &pb);
                (idx, outcome)
            })
            .collect()
    };

    if let Some(ref bar) = pb {
        bar.finish_and_clear();
    }

    // Initialize NDJSON writer for reports
    let reports_dir = output_dir.join("reports");
    let mut ndjson_writer = NdjsonWriter::new(&reports_dir, args.incremental)?;

    // Register all source files
    let discovered_files: Vec<crate::scan::discover::DiscoveredFile> = jobs
        .iter()
        .map(|job| crate::scan::discover::DiscoveredFile {
            relative_path: job.source_path.to_string_lossy().to_string(),
            absolute_path: job.source_path.canonicalize()
                .unwrap_or_else(|_| job.source_path.clone())
                .to_string_lossy()
                .to_string(),
            extension: job.source_path.extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            file_size: fs::metadata(&job.source_path).map(|m| m.len()).unwrap_or(0),
            mtime_epoch: fs::metadata(&job.source_path)
                .and_then(|m| m.modified())
                .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs() as i64)
                .unwrap_or(0),
            file_type: crate::scan::discover::FileType::Source,
        })
        .collect();

    let run_id: i64 = 1;
    let file_id_map = ndjson_writer.register_files(&discovered_files, run_id)?;

    // Write transpile_meta.json
    let started_at = crate::scan::chrono_now();
    ndjson_writer.write_meta(&ndjson::ScanMeta {
        run_id,
        started_at: started_at.clone(),
        finished_at: None,
        root_dir: args.input.to_string_lossy().to_string(),
        phase: "transpile".to_string(),
        status: "running".to_string(),
        total_files: jobs.len() as i64,
        processed_files: 0,
        skipped_files: 0,
        failed_files: 0,
        worker_count: args.jobs.unwrap_or(1) as i64,
        batch_size: 0,
        incremental: args.incremental,
    })?;

    // Process outcomes and write NDJSON records
    for (idx, outcome) in &outcomes {
        let job = &jobs[*idx];
        let source_rel = job.source_path.to_string_lossy().to_string();
        let file_id = file_id_map.get(&source_rel).copied().unwrap_or(0);

        match outcome {
            TranspileOutcome::Success {
                crate_name,
                rust_lines,
                duration_ms,
                coverage_pct,
                total_statements,
                mapped_statements,
                verbs_used,
                verbs_unsupported,
            } => {
                success_count.fetch_add(1, Ordering::Relaxed);
                if cli.verbose > 0 && !cli.quiet {
                    eprintln!("  Transpiled {crate_name}");
                }
                let output_rel = format!("programs/{crate_name}/src/{}", job.entry_file);
                ndjson_writer.write_transpile_result(&TranspileResultRecord {
                    file_id,
                    run_id,
                    path: source_rel,
                    success: true,
                    output_path: output_rel,
                    rust_lines: *rust_lines,
                    duration_ms: *duration_ms,
                    error: None,
                    coverage_pct: *coverage_pct,
                    total_statements: *total_statements,
                    mapped_statements: *mapped_statements,
                    verbs_used: verbs_used.clone(),
                    verbs_unsupported: verbs_unsupported.clone(),
                })?;
            }
            TranspileOutcome::Skipped { crate_name } => {
                skip_count.fetch_add(1, Ordering::Relaxed);
                if cli.verbose > 0 && !cli.quiet {
                    eprintln!("  Skipped {crate_name} (up-to-date)");
                }
            }
            TranspileOutcome::Failed { crate_name, error, duration_ms } => {
                fail_count.fetch_add(1, Ordering::Relaxed);
                ndjson_writer.write_transpile_result(&TranspileResultRecord {
                    file_id,
                    run_id,
                    path: source_rel,
                    success: false,
                    output_path: String::new(),
                    rust_lines: 0,
                    duration_ms: *duration_ms,
                    error: Some(error.clone()),
                    coverage_pct: 0.0,
                    total_statements: 0,
                    mapped_statements: 0,
                    verbs_used: String::new(),
                    verbs_unsupported: String::new(),
                })?;
                if args.continue_on_error {
                    eprintln!("  error: failed to transpile {crate_name}: {error}");
                } else {
                    // Finalize NDJSON before returning error
                    ndjson_writer.flush()?;
                    return Err(miette!("failed to transpile {crate_name}: {error}"));
                }
            }
        }
    }

    ndjson_writer.flush()?;

    // Finalize metadata
    let s = success_count.load(Ordering::Relaxed);
    let f = fail_count.load(Ordering::Relaxed);
    let k = skip_count.load(Ordering::Relaxed);

    ndjson_writer.write_meta(&ndjson::ScanMeta {
        run_id,
        started_at,
        finished_at: Some(crate::scan::chrono_now()),
        root_dir: args.input.to_string_lossy().to_string(),
        phase: "transpile".to_string(),
        status: if f > 0 { "completed_with_errors" } else { "completed" }.to_string(),
        total_files: jobs.len() as i64,
        processed_files: s as i64,
        skipped_files: k as i64,
        failed_files: f as i64,
        worker_count: args.jobs.unwrap_or(1) as i64,
        batch_size: 0,
        incremental: args.incremental,
    })?;

    // Write manifest
    let manifest = build_manifest(&analysis);
    let manifest_toml = manifest_to_toml(&manifest);
    let manifest_path = args
        .manifest
        .clone()
        .unwrap_or_else(|| output_dir.join("cobol2rust-manifest.toml"));
    fs::write(&manifest_path, &manifest_toml)
        .into_diagnostic()
        .wrap_err_with(|| {
            format!("failed to write manifest {}", manifest_path.display())
        })?;

    if !cli.quiet {
        let mut summary = format!("Workspace transpiled: {s} succeeded, {f} failed");
        if k > 0 {
            let _ = write!(summary, ", {k} skipped (up-to-date)");
        }
        eprintln!("{summary}");
        eprintln!("Output: {}", output_dir.display());
        eprintln!("Reports: {}", reports_dir.display());
    }

    if f > 0 {
        Ok(ExitCode::from(1))
    } else {
        Ok(ExitCode::SUCCESS)
    }
}

/// Check if `output` is newer than `source` (for incremental mode).
fn is_up_to_date(source: &Path, output: &Path) -> bool {
    let Ok(src_meta) = fs::metadata(source) else {
        return false;
    };
    let Ok(out_meta) = fs::metadata(output) else {
        return false;
    };
    let Ok(src_mtime) = src_meta.modified() else {
        return false;
    };
    let Ok(out_mtime) = out_meta.modified() else {
        return false;
    };
    out_mtime >= src_mtime
}

/// Transpile a single source file and return diagnostics + coverage stats.
fn transpile_single_with_diagnostics(
    source_path: &Path,
    config: &TranspileConfig,
) -> Result<cobol_transpiler::diagnostics::TranspileResult> {
    let source = fs::read_to_string(source_path)
        .into_diagnostic()
        .wrap_err_with(|| {
            format!("failed to read {}", source_path.display())
        })?;
    transpile_with_config_and_diagnostics(&source, config).map_err(|e| miette!("{e}"))
}

/// Build a `TranspileConfig` from CLI flags.
fn build_config(cli: &Cli, args: &TranspileArgs) -> Result<TranspileConfig> {
    let mut library_map = HashMap::new();
    for mapping in &args.library_map {
        let (name, dir) = mapping
            .split_once('=')
            .ok_or_else(|| {
                miette!("invalid library mapping '{mapping}': expected NAME=DIR")
            })?;
        library_map.insert(name.to_uppercase(), PathBuf::from(dir));
    }

    Ok(TranspileConfig {
        copybook_paths: cli.copybook_paths.clone(),
        library_map,
        max_copy_depth: 10,
    })
}

/// Generate workspace root `Cargo.toml`.
///
/// The `runtime_path` specifies how to resolve the `cobol-runtime` dependency.
/// If `None`, uses a crates.io version spec; otherwise uses a path dependency.
fn generate_workspace_cargo_toml(members: &[String], runtime_path: Option<&str>) -> String {
    let mut out = String::from("[workspace]\n");
    out.push_str("resolver = \"2\"\n");
    out.push_str("members = [\n");
    for m in members {
        let _ = writeln!(out, "    \"{m}\",");
    }
    out.push_str("]\n\n");
    out.push_str("[workspace.dependencies]\n");
    if let Some(path) = runtime_path {
        let _ = writeln!(
            out,
            "cobol-runtime = {{ path = \"{path}\", features = [\"full\"] }}"
        );
    } else {
        out.push_str("cobol-runtime = { version = \"0.1\", features = [\"full\"] }\n");
    }
    out
}

/// Generate a program crate `Cargo.toml`.
fn generate_program_cargo_toml(
    crate_name: &str,
    program_type: ProgramType,
    deps: &[String],
) -> String {
    let mut out = String::new();
    out.push_str("[package]\n");
    let _ = writeln!(out, "name = \"{crate_name}\"");
    out.push_str("version = \"0.1.0\"\n");
    out.push_str("edition = \"2021\"\n\n");

    if program_type == ProgramType::Main {
        out.push_str("[[bin]]\n");
        let _ = writeln!(out, "name = \"{crate_name}\"");
        out.push_str("path = \"src/main.rs\"\n\n");
    }

    out.push_str("[dependencies]\n");
    out.push_str("cobol-runtime = { workspace = true }\n");
    for dep in deps {
        let _ = writeln!(out, "{dep} = {{ path = \"../{dep}\" }}");
    }
    out
}

/// Create the copybooks crate with placeholder lib.rs.
fn create_copybooks_crate(
    output_dir: &Path,
    analysis: &crate::workspace::WorkspaceAnalysis,
) -> Result<()> {
    let cb_dir = output_dir.join("copybooks").join("src");
    fs::create_dir_all(&cb_dir)
        .into_diagnostic()
        .wrap_err("failed to create copybooks/src")?;

    let mut cargo = String::new();
    cargo.push_str("[package]\n");
    cargo.push_str("name = \"copybooks\"\n");
    cargo.push_str("version = \"0.1.0\"\n");
    cargo.push_str("edition = \"2021\"\n\n");
    cargo.push_str("[dependencies]\n");
    cargo.push_str("cobol-runtime = { workspace = true }\n");

    fs::write(output_dir.join("copybooks/Cargo.toml"), &cargo)
        .into_diagnostic()
        .wrap_err("failed to write copybooks/Cargo.toml")?;

    let mut lib = String::from("//! Shared copybook types.\n");
    lib.push_str("//! Auto-generated by `cobol2rust transpile --workspace`.\n\n");

    let mut all_cpy_files = Vec::new();
    for dir in &analysis.copybook_dirs {
        all_cpy_files.extend(discover_copybook_files(dir));
    }

    if all_cpy_files.is_empty() {
        lib.push_str("// No copybook files discovered.\n");
    } else {
        lib.push_str("// Copybook files to transpile:\n");
        for f in &all_cpy_files {
            let _ = writeln!(lib, "//   {f}");
        }
    }

    fs::write(cb_dir.join("lib.rs"), &lib)
        .into_diagnostic()
        .wrap_err("failed to write copybooks/src/lib.rs")?;

    Ok(())
}
