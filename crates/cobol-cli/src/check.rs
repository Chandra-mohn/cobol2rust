//! `cobol2rust check` -- validate COBOL source without transpiling.

use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, ValueEnum};
use miette::{Context, IntoDiagnostic, Result};
use serde::Serialize;

use cobol_transpiler::parser::extract_copy_targets;

use crate::analyze::{self, AnalysisResult, CoverageResult, DiagnosticEntry};
use crate::Cli;

/// Arguments for `cobol2rust check`.
#[derive(Debug, Args)]
pub struct CheckArgs {
    /// COBOL source file(s) to check.
    pub inputs: Vec<PathBuf>,

    /// Output format.
    #[arg(long, default_value = "text")]
    pub format: CheckFormat,

    /// Treat warnings as errors.
    #[arg(long)]
    pub strict: bool,

    /// Run transpilation coverage analysis.
    ///
    /// Reports which COBOL statements can be transpiled and which are
    /// unhandled, with source line numbers and coverage percentage.
    #[arg(long)]
    pub coverage: bool,
}

/// Output format for check results.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CheckFormat {
    Text,
    Json,
}

/// JSON-serializable check result for a single file.
#[derive(Debug, Serialize)]
struct FileResult {
    path: String,
    program_id: String,
    format: String,
    valid: bool,
    errors: Vec<Diagnostic>,
    warnings: Vec<Diagnostic>,
    info: ProgramInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    coverage: Option<CoverageInfo>,
}

/// Coverage statistics from transpilation analysis.
#[derive(Debug, Serialize)]
struct CoverageInfo {
    total_statements: usize,
    mapped_statements: usize,
    coverage_pct: f64,
    total_data_entries: usize,
    unhandled: Vec<Diagnostic>,
}

/// A diagnostic message.
#[derive(Debug, Serialize)]
struct Diagnostic {
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<usize>,
    message: String,
    code: String,
}

/// Program statistics extracted from the AST.
#[derive(Debug, Serialize)]
struct ProgramInfo {
    paragraphs: usize,
    calls: usize,
    file_ops: usize,
    sql_statements: usize,
    is_subprogram: bool,
}

/// JSON-serializable summary.
#[derive(Debug, Serialize)]
struct CheckSummary {
    files: Vec<FileResult>,
    summary: Summary,
}

#[derive(Debug, Serialize)]
struct Summary {
    files: usize,
    errors: usize,
    warnings: usize,
}

/// Run the check subcommand.
pub fn run(cli: &Cli, args: &CheckArgs) -> Result<ExitCode> {
    if args.inputs.is_empty() {
        return Err(miette::miette!("no input files specified"));
    }

    let mut results = Vec::new();
    let mut total_errors = 0usize;
    let mut total_warnings = 0usize;

    for input in &args.inputs {
        let result = check_file(cli, input, args.coverage)?;
        total_errors += result.errors.len();
        total_warnings += result.warnings.len();
        results.push(result);
    }

    // Output results.
    match args.format {
        CheckFormat::Text => {
            for r in &results {
                print_text_result(r);
            }
            if results.len() > 1 || !cli.quiet {
                eprintln!(
                    "\nSummary: {} file(s) checked, {} error(s), {} warning(s)",
                    results.len(),
                    total_errors,
                    total_warnings,
                );
            }
        }
        CheckFormat::Json => {
            let output = CheckSummary {
                summary: Summary {
                    files: results.len(),
                    errors: total_errors,
                    warnings: total_warnings,
                },
                files: results,
            };
            let json = serde_json::to_string_pretty(&output)
                .into_diagnostic()
                .wrap_err("failed to serialize JSON")?;
            println!("{json}");
        }
    }

    // Exit codes: 0=valid, 1=errors, 2=warnings-only.
    if total_errors > 0 {
        Ok(ExitCode::from(1))
    } else if total_warnings > 0 && args.strict {
        Ok(ExitCode::from(2))
    } else {
        Ok(ExitCode::SUCCESS)
    }
}

/// Check a single COBOL file using the shared analysis pipeline.
fn check_file(cli: &Cli, path: &PathBuf, with_coverage: bool) -> Result<FileResult> {
    let source = fs::read_to_string(path)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read {}", path.display()))?;

    let analysis = analyze::analyze_source(&source, with_coverage);

    if cli.verbose > 0 {
        let copies = extract_copy_targets(&source);
        if !copies.is_empty() && !cli.quiet {
            eprintln!("  COPY targets: {}", copies.join(", "));
        }
    }

    Ok(analysis_to_file_result(path, &analysis))
}

/// Convert an `AnalysisResult` to the check-specific `FileResult`.
fn analysis_to_file_result(path: &PathBuf, a: &AnalysisResult) -> FileResult {
    FileResult {
        path: path.display().to_string(),
        program_id: a.program_id.clone(),
        format: a.source_format.clone(),
        valid: a.valid,
        errors: a.errors.iter().map(entry_to_diagnostic).collect(),
        warnings: a.warnings.iter().map(entry_to_diagnostic).collect(),
        info: ProgramInfo {
            paragraphs: a.info.paragraphs,
            calls: a.info.calls,
            file_ops: a.info.file_ops,
            sql_statements: a.info.sql_statements,
            is_subprogram: a.info.is_subprogram,
        },
        coverage: a.coverage.as_ref().map(coverage_to_info),
    }
}

fn entry_to_diagnostic(e: &DiagnosticEntry) -> Diagnostic {
    Diagnostic {
        line: e.line,
        message: e.message.clone(),
        code: e.code.clone(),
    }
}

fn coverage_to_info(c: &CoverageResult) -> CoverageInfo {
    CoverageInfo {
        total_statements: c.total_statements,
        mapped_statements: c.mapped_statements,
        coverage_pct: c.coverage_pct,
        total_data_entries: c.total_data_entries,
        unhandled: c.unhandled.iter().map(entry_to_diagnostic).collect(),
    }
}

/// Print a text-format check result.
fn print_text_result(r: &FileResult) {
    eprintln!("Checking {}...", r.path);
    eprintln!("  Format: {} (detected)", r.format.to_uppercase());
    eprintln!("  Program-ID: {}", r.program_id);

    if r.valid {
        eprintln!("  [OK] Syntax valid");
    }

    for e in &r.errors {
        if let Some(line) = e.line {
            eprintln!("  [ERR] Line {line}: {}", e.message);
        } else {
            eprintln!("  [ERR] {}", e.message);
        }
    }

    for w in &r.warnings {
        if let Some(line) = w.line {
            eprintln!("  [WARN] Line {line}: {}", w.message);
        } else {
            eprintln!("  [WARN] {}", w.message);
        }
    }

    if r.info.is_subprogram {
        eprintln!("  [INFO] Subprogram (has LINKAGE SECTION + USING)");
    }

    eprintln!(
        "  [INFO] {} paragraph(s), {} CALL statement(s), {} file operation(s), {} SQL statement(s)",
        r.info.paragraphs, r.info.calls, r.info.file_ops, r.info.sql_statements,
    );

    // Coverage report
    if let Some(ref cov) = r.coverage {
        eprintln!();
        eprintln!(
            "  Coverage: {:.1}% ({}/{} statements mapped)",
            cov.coverage_pct, cov.mapped_statements, cov.total_statements,
        );
        eprintln!("  Data entries: {}", cov.total_data_entries);
        if !cov.unhandled.is_empty() {
            eprintln!("  Unhandled constructs:");
            for d in &cov.unhandled {
                if let Some(line) = d.line {
                    eprintln!("    Line {line}: {}", d.message);
                } else {
                    eprintln!("    {}", d.message);
                }
            }
        }
    }
}
