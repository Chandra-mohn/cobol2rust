//! Shared analysis logic used by both `check` and `scan` subcommands.
//!
//! Extracts the core file-analysis pipeline (parse, stats, coverage) into
//! reusable functions so that `check` and `scan` produce identical results.

use std::time::Instant;

use serde::Serialize;

use cobol_transpiler::ast::{CobolProgram, ProcedureDivision, Statement};
use cobol_transpiler::diagnostics::Severity;
use cobol_transpiler::parser::preprocess::{detect_source_format, SourceFormat};
use cobol_transpiler::parser::{extract_copy_targets, parse_cobol};
use cobol_transpiler::transpile::transpile_with_diagnostics;

/// Result of analyzing a single COBOL source file.
#[derive(Debug)]
pub struct AnalysisResult {
    pub program_id: String,
    pub source_format: String,
    pub valid: bool,
    pub info: ProgramStats,
    pub errors: Vec<DiagnosticEntry>,
    pub warnings: Vec<DiagnosticEntry>,
    pub copy_targets: Vec<String>,
    pub coverage: Option<CoverageResult>,
    pub parse_time_ms: u64,
    pub line_count: usize,
}

/// Program statistics extracted from the AST.
#[derive(Debug, Serialize)]
pub struct ProgramStats {
    pub paragraphs: usize,
    pub sections: usize,
    pub calls: usize,
    pub file_ops: usize,
    pub sql_statements: usize,
    pub is_subprogram: bool,
    pub has_linkage: bool,
    pub has_using: bool,
    pub data_items: usize,
}

/// A single diagnostic entry (error, warning, or info).
#[derive(Debug, Serialize)]
pub struct DiagnosticEntry {
    pub line: Option<usize>,
    pub message: String,
    pub code: String,
    pub category: String,
}

/// Coverage statistics from transpilation analysis.
#[derive(Debug, Serialize)]
pub struct CoverageResult {
    pub total_statements: usize,
    pub mapped_statements: usize,
    pub coverage_pct: f64,
    pub total_data_entries: usize,
    pub unhandled: Vec<DiagnosticEntry>,
    pub coverage_time_ms: u64,
}

/// Analyze a single COBOL source string (parse + stats + optional coverage).
///
/// This is the core analysis function shared by `check` and `scan`.
pub fn analyze_source(source: &str, with_coverage: bool) -> AnalysisResult {
    let start = Instant::now();
    let line_count = source.lines().count();

    // Detect format.
    let format = detect_source_format(source);
    let format_str = match format {
        SourceFormat::Fixed => "fixed",
        SourceFormat::Free => "free",
    };

    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let mut info = ProgramStats {
        paragraphs: 0,
        sections: 0,
        calls: 0,
        file_ops: 0,
        sql_statements: 0,
        is_subprogram: false,
        has_linkage: false,
        has_using: false,
        data_items: 0,
    };
    let program_id;
    let mut coverage = None;

    // Extract copy targets from raw source.
    let copy_targets = extract_copy_targets(source);

    match parse_cobol(source) {
        Ok(program) => {
            program_id = program.program_id.clone();
            info = collect_stats(&program);

            // Scan for common warnings.
            scan_warnings(source, &mut warnings);
        }
        Err(e) => {
            errors.push(DiagnosticEntry {
                line: extract_error_line(&e),
                message: e.to_string(),
                code: String::from("E001"),
                category: String::from("parse"),
            });

            // Fallback: extract program-id from raw text.
            program_id = extract_program_id_text(source);
        }
    }

    let parse_time_ms = start.elapsed().as_millis() as u64;

    // Run coverage analysis if requested and parsing succeeded.
    if with_coverage && errors.is_empty() {
        coverage = Some(run_coverage(source));
    }

    AnalysisResult {
        program_id,
        source_format: format_str.to_string(),
        valid: errors.is_empty(),
        info,
        errors,
        warnings,
        copy_targets,
        coverage,
        parse_time_ms,
        line_count,
    }
}

/// Run transpilation and collect coverage diagnostics.
fn run_coverage(source: &str) -> CoverageResult {
    let start = Instant::now();
    match transpile_with_diagnostics(source) {
        Ok(result) => {
            let unhandled: Vec<DiagnosticEntry> = result
                .diagnostics
                .iter()
                .map(|d| DiagnosticEntry {
                    line: if d.line > 0 { Some(d.line) } else { None },
                    message: format!("{}: {}", d.category, d.message),
                    code: match d.severity {
                        Severity::Error => String::from("C-ERR"),
                        Severity::Warning => String::from("C-WARN"),
                        Severity::Info => String::from("C-INFO"),
                    },
                    category: String::from("coverage"),
                })
                .collect();

            CoverageResult {
                total_statements: result.stats.total_statements,
                mapped_statements: result.stats.mapped_statements,
                coverage_pct: result.statement_coverage(),
                total_data_entries: result.stats.total_data_entries,
                unhandled,
                coverage_time_ms: start.elapsed().as_millis() as u64,
            }
        }
        Err(e) => CoverageResult {
            total_statements: 0,
            mapped_statements: 0,
            coverage_pct: 0.0,
            total_data_entries: 0,
            unhandled: vec![DiagnosticEntry {
                line: None,
                message: format!("Transpilation failed: {e}"),
                code: String::from("C-FATAL"),
                category: String::from("coverage"),
            }],
            coverage_time_ms: start.elapsed().as_millis() as u64,
        },
    }
}

/// Collect statistics from a parsed program.
fn collect_stats(program: &CobolProgram) -> ProgramStats {
    let mut paragraphs = 0usize;
    let mut sections = 0usize;
    let mut calls = 0usize;
    let mut file_ops = 0usize;
    let mut is_subprogram = false;
    let mut has_linkage = false;
    let mut has_using = false;
    let mut data_items = 0usize;

    // Check for subprogram indicators.
    if let Some(ref pd) = program.procedure_division {
        if !pd.using_params.is_empty() {
            is_subprogram = true;
            has_using = true;
        }

        sections = pd.sections.len();
        paragraphs += pd.paragraphs.len();
        for section in &pd.sections {
            paragraphs += section.paragraphs.len();
        }

        count_statements(pd, &mut calls, &mut file_ops);
    }

    // Check LINKAGE SECTION.
    if let Some(ref dd) = program.data_division {
        if !dd.linkage.is_empty() {
            is_subprogram = true;
            has_linkage = true;
        }
        data_items = dd.working_storage.len() + dd.linkage.len();
    }

    let sql_statements = program.exec_sql_statements.len();

    ProgramStats {
        paragraphs,
        sections,
        calls,
        file_ops,
        sql_statements,
        is_subprogram,
        has_linkage,
        has_using,
        data_items,
    }
}

/// Walk all statements in the procedure division to count calls and file ops.
fn count_statements(pd: &ProcedureDivision, calls: &mut usize, file_ops: &mut usize) {
    for para in &pd.paragraphs {
        for sentence in &para.sentences {
            for stmt in &sentence.statements {
                count_in_statement(stmt, calls, file_ops);
            }
        }
    }
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
        Statement::ExecSql(_) => {}
        _ => {}
    }
}

/// Scan raw source for common warnings.
fn scan_warnings(source: &str, warnings: &mut Vec<DiagnosticEntry>) {
    for (i, line) in source.lines().enumerate() {
        let trimmed = line.trim().to_uppercase();

        if trimmed.contains("EXEC SQL") || trimmed.contains("EXEC CICS") {
            warnings.push(DiagnosticEntry {
                line: Some(i + 1),
                message: String::from("EXEC SQL/CICS detected"),
                code: String::from("W001"),
                category: String::from("unsupported"),
            });
        }

        if trimmed.starts_with("ALTER ") || trimmed.contains(" ALTER ") {
            warnings.push(DiagnosticEntry {
                line: Some(i + 1),
                message: String::from("ALTER verb detected (consider refactoring)"),
                code: String::from("W002"),
                category: String::from("unsupported"),
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
pub fn extract_program_id_text(source: &str) -> String {
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
