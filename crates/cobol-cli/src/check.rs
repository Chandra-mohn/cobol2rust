//! `cobol2rust check` -- validate COBOL source without transpiling.

use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, ValueEnum};
use miette::{Context, IntoDiagnostic, Result};
use serde::Serialize;

use cobol_transpiler::ast::{CobolProgram, ProcedureDivision, Statement};
use cobol_transpiler::parser::preprocess::{detect_source_format, SourceFormat};
use cobol_transpiler::parser::{extract_copy_targets, parse_cobol};

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
        let result = check_file(cli, input)?;
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

/// Check a single COBOL file.
fn check_file(cli: &Cli, path: &PathBuf) -> Result<FileResult> {
    let source = fs::read_to_string(path)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read {}", path.display()))?;

    // Detect format.
    let format = detect_source_format(&source);
    let format_str = match format {
        SourceFormat::Fixed => "fixed",
        SourceFormat::Free => "free",
    };

    // Try parsing.
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let mut info = ProgramInfo {
        paragraphs: 0,
        calls: 0,
        file_ops: 0,
        is_subprogram: false,
    };
    let program_id;

    match parse_cobol(&source) {
        Ok(program) => {
            program_id = program.program_id.clone();
            info = collect_stats(&program);

            // Check for unsupported features (text-level scan).
            scan_warnings(&source, &mut warnings);
        }
        Err(e) => {
            errors.push(Diagnostic {
                line: extract_error_line(&e),
                message: e.to_string(),
                code: String::from("E001"),
            });

            // Even on parse error, try to extract program-id from raw text.
            program_id = extract_program_id_text(&source);
        }
    }

    if cli.verbose > 0 {
        // Extract COPY targets for verbose output.
        let copies = extract_copy_targets(&source);
        if !copies.is_empty() && !cli.quiet {
            eprintln!("  COPY targets: {}", copies.join(", "));
        }
    }

    Ok(FileResult {
        path: path.display().to_string(),
        program_id,
        format: format_str.to_string(),
        valid: errors.is_empty(),
        errors,
        warnings,
        info,
    })
}

/// Collect statistics from a parsed program.
fn collect_stats(program: &CobolProgram) -> ProgramInfo {
    let mut paragraphs = 0usize;
    let mut calls = 0usize;
    let mut file_ops = 0usize;
    let mut is_subprogram = false;

    // Check for subprogram indicators.
    if let Some(ref pd) = program.procedure_division {
        if !pd.using_params.is_empty() {
            is_subprogram = true;
        }

        // Count paragraphs and walk statements.
        paragraphs += pd.paragraphs.len();
        for section in &pd.sections {
            paragraphs += section.paragraphs.len();
        }

        count_statements(pd, &mut calls, &mut file_ops);
    }

    // Also check for LINKAGE SECTION items as subprogram indicator.
    if let Some(ref dd) = program.data_division {
        if !dd.linkage.is_empty() {
            is_subprogram = true;
        }
    }

    ProgramInfo {
        paragraphs,
        calls,
        file_ops,
        is_subprogram,
    }
}

/// Walk all statements in the procedure division to count calls and file ops.
fn count_statements(pd: &ProcedureDivision, calls: &mut usize, file_ops: &mut usize) {
    // Walk standalone paragraphs.
    for para in &pd.paragraphs {
        for sentence in &para.sentences {
            for stmt in &sentence.statements {
                count_in_statement(stmt, calls, file_ops);
            }
        }
    }

    // Walk sections.
    for section in &pd.sections {
        for para in &section.paragraphs {
            for sentence in &para.sentences {
                for stmt in &sentence.statements {
                    count_in_statement(stmt, calls, file_ops);
                }
            }
        }
    }
}

/// Count calls and file operations in a single statement (recursive for nested).
fn count_in_statement(stmt: &Statement, calls: &mut usize, file_ops: &mut usize) {
    match stmt {
        Statement::Call(_) => *calls += 1,
        Statement::Open(_)
        | Statement::Close(_)
        | Statement::Read(_)
        | Statement::Write(_)
        | Statement::Rewrite(_)
        | Statement::Delete(_)
        | Statement::Start(_) => *file_ops += 1,
        // Recurse into nested statements (IF/EVALUATE/PERFORM INLINE).
        Statement::If(if_stmt) => {
            for s in &if_stmt.then_body {
                count_in_statement(s, calls, file_ops);
            }
            for s in &if_stmt.else_body {
                count_in_statement(s, calls, file_ops);
            }
        }
        Statement::Evaluate(eval) => {
            for branch in &eval.when_branches {
                for s in &branch.body {
                    count_in_statement(s, calls, file_ops);
                }
            }
            for s in &eval.when_other {
                count_in_statement(s, calls, file_ops);
            }
        }
        Statement::Perform(perf) => {
            for s in &perf.body {
                count_in_statement(s, calls, file_ops);
            }
        }
        _ => {}
    }
}

/// Scan raw source for common warnings.
fn scan_warnings(source: &str, warnings: &mut Vec<Diagnostic>) {
    for (i, line) in source.lines().enumerate() {
        let trimmed = line.trim().to_uppercase();

        if trimmed.contains("EXEC SQL") || trimmed.contains("EXEC CICS") {
            warnings.push(Diagnostic {
                line: Some(i + 1),
                message: String::from("EXEC SQL/CICS not supported (will be skipped)"),
                code: String::from("W001"),
            });
        }

        if trimmed.starts_with("ALTER ") || trimmed.contains(" ALTER ") {
            warnings.push(Diagnostic {
                line: Some(i + 1),
                message: String::from("ALTER verb detected (consider refactoring)"),
                code: String::from("W002"),
            });
        }
    }
}

/// Extract error line number from `TranspileError` (best-effort).
fn extract_error_line(e: &cobol_transpiler::error::TranspileError) -> Option<usize> {
    use cobol_transpiler::error::TranspileError;
    match e {
        TranspileError::Preprocess { line, .. } | TranspileError::Parse { line, .. } => {
            Some(*line)
        }
        _ => None,
    }
}

/// Extract PROGRAM-ID from raw text (fallback when parsing fails).
fn extract_program_id_text(source: &str) -> String {
    for line in source.lines() {
        let trimmed = line.trim().to_uppercase();
        if trimmed.starts_with("PROGRAM-ID") {
            let rest = trimmed
                .trim_start_matches("PROGRAM-ID")
                .trim_start_matches('.')
                .trim_start();
            let name = rest.trim_end_matches('.').trim();
            if !name.is_empty() {
                return name.to_string();
            }
        }
    }
    String::from("UNKNOWN")
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
        "  [INFO] {} paragraph(s), {} CALL statement(s), {} file operation(s)",
        r.info.paragraphs, r.info.calls, r.info.file_ops,
    );
}
