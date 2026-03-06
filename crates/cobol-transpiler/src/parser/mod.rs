//! COBOL parser -- ANTLR4 orchestration and public API.
//!
//! Provides the entry point for parsing COBOL source into a typed AST.
//! Uses the ANTLR4-generated lexer/parser with custom listener walks
//! to build the AST incrementally.
//!
//! Each listener walk function creates its own parser instance (following
//! the coqu-di pattern) because the ANTLR4 parse tree has lifetime ties
//! to the parser/token stream.

pub mod copy_expand;
pub mod copybook;
pub(crate) mod data_listener;
pub mod hierarchy;
pub mod pic_parser;
pub mod preprocess;
pub(crate) mod proc_listener;

use crate::ast::{CobolProgram, DataDivision, DataEntry, ProcedureDivision};
use crate::error::{Result, TranspileError};
use crate::generated::cobol85lexer::Cobol85Lexer;
use crate::generated::cobol85listener::Cobol85Listener;
use crate::generated::cobol85parser::{Cobol85Parser, Cobol85ParserContextType, Cobol85TreeWalker};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;
use antlr_rust::parser::Parser as _;
use antlr_rust::tree::ParseTreeListener;

use data_listener::DataDivisionListener;
use hierarchy::{build_hierarchy, compute_layout};
use preprocess::preprocess;
use proc_listener::ProcedureDivisionListener;

/// Parse COBOL source into a typed AST.
///
/// Automatically detects fixed vs free format, preprocesses the source,
/// and then runs the ANTLR4 lexer/parser with listener walks.
///
/// # Errors
///
/// Returns `TranspileError::AntlrError` if the ANTLR4 parser fails,
/// or `TranspileError::Preprocess` if preprocessing fails.
pub fn parse_cobol(source: &str) -> Result<CobolProgram> {
    // Preprocess (strip columns, comments, handle continuations)
    let preprocessed = preprocess(source)?;

    // Wrap standalone copybooks in a minimal program skeleton
    let input = wrap_if_copybook(&preprocessed);

    // Parse DATA DIVISION
    let working_storage = parse_data_division(&input)?;

    // Parse PROCEDURE DIVISION
    let procedure_division = parse_procedure_division(&input)?;

    // Extract PROGRAM-ID
    let program_id = extract_program_id(&input);

    Ok(CobolProgram {
        program_id,
        data_division: Some(DataDivision {
            working_storage,
            local_storage: Vec::new(),
            linkage: Vec::new(),
            file_section: Vec::new(),
        }),
        procedure_division,
        source_path: None,
    })
}

/// Parse already-preprocessed COBOL source into a typed AST.
///
/// Unlike `parse_cobol()`, this skips the preprocessing step. Use when
/// the source has already been through `preprocess()` and COPY expansion.
///
/// # Errors
///
/// Returns `TranspileError::AntlrError` if the ANTLR4 parser fails.
pub fn parse_cobol_from_source(source: &str) -> Result<CobolProgram> {
    // Wrap standalone copybooks in a minimal program skeleton
    let input = wrap_if_copybook(source);

    // Parse DATA DIVISION
    let working_storage = parse_data_division(&input)?;

    // Parse PROCEDURE DIVISION
    let procedure_division = parse_procedure_division(&input)?;

    // Extract PROGRAM-ID
    let program_id = extract_program_id(&input);

    Ok(CobolProgram {
        program_id,
        data_division: Some(DataDivision {
            working_storage,
            local_storage: Vec::new(),
            linkage: Vec::new(),
            file_section: Vec::new(),
        }),
        procedure_division,
        source_path: None,
    })
}

/// Parse COBOL source and extract DATA DIVISION into a hierarchical tree.
///
/// Runs the ANTLR4 parser with `DataDivisionListener`, then builds
/// the hierarchy from flat items and computes byte layouts.
///
/// # Errors
///
/// Returns `TranspileError::AntlrError` if the ANTLR4 parser fails.
pub fn parse_data_division(source: &str) -> Result<Vec<DataEntry>> {
    let listener = run_data_listener(source)?;
    let mut records = build_hierarchy(listener.items);

    // Compute byte offsets for all records
    for record in &mut records {
        compute_layout(record);
    }

    Ok(records)
}

/// Parse COBOL source and extract PROCEDURE DIVISION into AST.
///
/// Returns `None` if the source has no procedure division.
pub fn parse_procedure_division(source: &str) -> Result<Option<ProcedureDivision>> {
    let upper = source.to_uppercase();
    if !upper.contains("PROCEDURE DIVISION") {
        return Ok(None);
    }

    let listener = run_proc_listener(source)?;

    // If we got nothing, return None
    if listener.sections.is_empty() && listener.paragraphs.is_empty() {
        return Ok(None);
    }

    Ok(Some(ProcedureDivision {
        using_params: Vec::new(),
        returning: None,
        sections: listener.sections,
        paragraphs: listener.paragraphs,
    }))
}

/// Run ANTLR4 parse and walk with `ProcedureDivisionListener`.
fn run_proc_listener(source: &str) -> Result<ProcedureDivisionListener> {
    let input: InputStream<&str> = InputStream::new(source);
    let mut lexer = Cobol85Lexer::new(input);
    lexer.remove_error_listeners();
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = Cobol85Parser::new(token_stream);
    parser.remove_error_listeners();

    let tree = parser.startRule().map_err(|e| TranspileError::AntlrError {
        message: format!("{e:?}"),
    })?;

    let listener = Box::new(ProcedureDivisionListener::new());
    let listener = Cobol85TreeWalker::walk(listener, &*tree);

    Ok(*listener)
}

/// Run ANTLR4 parse and walk with `DataDivisionListener`.
fn run_data_listener(source: &str) -> Result<DataDivisionListener> {
    let input: InputStream<&str> = InputStream::new(source);
    let mut lexer = Cobol85Lexer::new(input);
    lexer.remove_error_listeners();
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = Cobol85Parser::new(token_stream);
    parser.remove_error_listeners();

    let tree = parser.startRule().map_err(|e| TranspileError::AntlrError {
        message: format!("{e:?}"),
    })?;

    let listener = Box::new(DataDivisionListener::new());
    let listener = Cobol85TreeWalker::walk(listener, &*tree);

    Ok(*listener)
}

/// Run ANTLR4 parse with a validation-only listener.
///
/// Verifies that the source is syntactically valid COBOL.
#[allow(dead_code)]
fn run_validation_walk(source: &str) -> Result<()> {
    let input: InputStream<&str> = InputStream::new(source);
    let mut lexer = Cobol85Lexer::new(input);
    lexer.remove_error_listeners();
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = Cobol85Parser::new(token_stream);
    parser.remove_error_listeners();

    let tree = parser.startRule().map_err(|e| TranspileError::AntlrError {
        message: format!("{e:?}"),
    })?;

    let listener = Box::new(ValidationListener);
    let _listener = Cobol85TreeWalker::walk(listener, &*tree);

    Ok(())
}

/// Placeholder listener that collects nothing (validates parse only).
#[derive(Debug, Default)]
struct ValidationListener;

impl ParseTreeListener<'_, Cobol85ParserContextType> for ValidationListener {}

#[allow(unused_variables)]
impl Cobol85Listener<'_> for ValidationListener {}

/// Wrap standalone copybook content in a minimal COBOL program skeleton
/// so the ANTLR4 `startRule` entry point can parse it.
fn wrap_if_copybook(source: &str) -> String {
    let upper = source.to_uppercase();
    if upper.contains("IDENTIFICATION DIVISION") || upper.contains("ID DIVISION") {
        return source.to_string();
    }

    let mut wrapped = String::with_capacity(source.len() + 200);
    wrapped.push_str("       IDENTIFICATION DIVISION.\n");
    wrapped.push_str("       PROGRAM-ID. WRAPPER.\n");
    wrapped.push_str("       DATA DIVISION.\n");
    wrapped.push_str("       WORKING-STORAGE SECTION.\n");
    wrapped.push_str(source);
    wrapped.push('\n');
    wrapped
}

/// Extract PROGRAM-ID from preprocessed source via simple text scan.
///
/// Falls back to "UNKNOWN" if not found.
fn extract_program_id(source: &str) -> String {
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
    "UNKNOWN".to_string()
}

/// Extract COPY statement targets from raw COBOL source.
///
/// Text-level scan (no ANTLR4 parsing) since COPY is a preprocessor
/// directive that should be resolved before parsing.
///
/// Returns uppercased copy-member names (e.g., `["ACCTFILE", "COMMON"]`).
pub fn extract_copy_targets(source: &str) -> Vec<String> {
    let mut targets = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim().to_uppercase();
        if trimmed.starts_with("COPY ") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[1].trim_end_matches('.');
                if !name.is_empty() {
                    targets.push(name.to_string());
                }
            }
        }
    }
    targets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_program_id_basic() {
        let source = "\
       IDENTIFICATION DIVISION.
       PROGRAM-ID. HELLO.
       DATA DIVISION.";
        assert_eq!(extract_program_id(source), "HELLO");
    }

    #[test]
    fn extract_program_id_missing() {
        let source = "\
       IDENTIFICATION DIVISION.
       DATA DIVISION.";
        assert_eq!(extract_program_id(source), "UNKNOWN");
    }

    #[test]
    fn wrap_if_copybook_full_program() {
        let source = "       IDENTIFICATION DIVISION.\n       PROGRAM-ID. TEST.";
        let result = wrap_if_copybook(source);
        assert!(!result.contains("WRAPPER"));
    }

    #[test]
    fn wrap_if_copybook_copybook() {
        let source = "       01  WS-FIELD PIC X(10).";
        let result = wrap_if_copybook(source);
        assert!(result.contains("IDENTIFICATION DIVISION"));
        assert!(result.contains("WRAPPER"));
        assert!(result.contains("WS-FIELD"));
    }

    #[test]
    fn extract_copy_targets_basic() {
        let source = "       FD  ACCT-FILE.\n           COPY ACCTFILE.\n";
        let targets = extract_copy_targets(source);
        assert_eq!(targets, vec!["ACCTFILE"]);
    }

    #[test]
    fn extract_copy_targets_none() {
        let source = "       01  WS-FIELD PIC X(10).";
        let targets = extract_copy_targets(source);
        assert!(targets.is_empty());
    }

    #[test]
    fn parse_cobol_minimal_program() {
        // Free-format COBOL source (no column restrictions)
        let source = "IDENTIFICATION DIVISION.\nPROGRAM-ID. HELLO.\nDATA DIVISION.\nWORKING-STORAGE SECTION.\n01  WS-NAME PIC X(20).\nPROCEDURE DIVISION.\n    DISPLAY 'HELLO WORLD'.\n    STOP RUN.";

        let result = parse_cobol(source);
        assert!(result.is_ok(), "parse_cobol failed: {result:?}");
        let program = result.unwrap();
        assert_eq!(program.program_id, "HELLO");
        assert!(program.data_division.is_some());
    }

    #[test]
    fn parse_procedure_division_statements() {
        let source = concat!(
            "IDENTIFICATION DIVISION.\n",
            "PROGRAM-ID. TESTSTMTS.\n",
            "DATA DIVISION.\n",
            "WORKING-STORAGE SECTION.\n",
            "01  WS-A PIC 9(5) VALUE 10.\n",
            "01  WS-B PIC 9(5) VALUE 20.\n",
            "01  WS-C PIC 9(5).\n",
            "01  WS-NAME PIC X(20).\n",
            "PROCEDURE DIVISION.\n",
            "MAIN-PARA.\n",
            "    MOVE WS-A TO WS-C.\n",
            "    ADD WS-A TO WS-B.\n",
            "    DISPLAY 'RESULT: ' WS-B.\n",
            "    STOP RUN.\n",
        );

        let result = parse_cobol(source);
        assert!(result.is_ok(), "parse failed: {result:?}");
        let program = result.unwrap();

        let proc_div = program.procedure_division.as_ref()
            .expect("should have procedure division");

        // Should have at least one paragraph
        assert!(!proc_div.paragraphs.is_empty(), "should have paragraphs");
        let para = &proc_div.paragraphs[0];
        assert_eq!(para.name, "MAIN-PARA");

        // Collect all statements across sentences
        let stmts: Vec<&crate::ast::Statement> = para.sentences.iter()
            .flat_map(|s| &s.statements)
            .collect();
        assert!(stmts.len() >= 3, "should have at least 3 statements, got {}", stmts.len());

        // Verify MOVE
        assert!(matches!(stmts[0], crate::ast::Statement::Move(_)));
        // Verify ADD
        assert!(matches!(stmts[1], crate::ast::Statement::Add(_)));
        // Verify DISPLAY
        assert!(matches!(stmts[2], crate::ast::Statement::Display(_)));
    }

    #[test]
    fn parse_if_statement() {
        let source = concat!(
            "IDENTIFICATION DIVISION.\n",
            "PROGRAM-ID. TESTIF.\n",
            "DATA DIVISION.\n",
            "WORKING-STORAGE SECTION.\n",
            "01  WS-X PIC 9(3) VALUE 5.\n",
            "PROCEDURE DIVISION.\n",
            "MAIN-PARA.\n",
            "    IF WS-X > 0\n",
            "        DISPLAY 'POSITIVE'\n",
            "    ELSE\n",
            "        DISPLAY 'NOT POSITIVE'\n",
            "    END-IF.\n",
            "    STOP RUN.\n",
        );

        let result = parse_cobol(source);
        assert!(result.is_ok(), "parse failed: {result:?}");
        let program = result.unwrap();
        let proc_div = program.procedure_division.as_ref()
            .expect("should have procedure division");

        let stmts: Vec<&crate::ast::Statement> = proc_div.paragraphs[0].sentences.iter()
            .flat_map(|s| &s.statements)
            .collect();

        // First statement should be IF
        match &stmts[0] {
            crate::ast::Statement::If(if_stmt) => {
                assert!(!if_stmt.then_body.is_empty(), "IF should have then body");
                assert!(!if_stmt.else_body.is_empty(), "IF should have else body");
            }
            other => panic!("expected If, got {other:?}"),
        }
    }

    #[test]
    fn parse_perform_procedure() {
        let source = concat!(
            "IDENTIFICATION DIVISION.\n",
            "PROGRAM-ID. TESTPERF.\n",
            "DATA DIVISION.\n",
            "WORKING-STORAGE SECTION.\n",
            "01  WS-COUNT PIC 9(3).\n",
            "PROCEDURE DIVISION.\n",
            "MAIN-PARA.\n",
            "    PERFORM WORK-PARA.\n",
            "    STOP RUN.\n",
            "WORK-PARA.\n",
            "    DISPLAY 'WORKING'.\n",
        );

        let result = parse_cobol(source);
        assert!(result.is_ok(), "parse failed: {result:?}");
        let program = result.unwrap();
        let proc_div = program.procedure_division.as_ref()
            .expect("should have procedure division");

        assert!(proc_div.paragraphs.len() >= 2,
            "should have at least 2 paragraphs, got {}", proc_div.paragraphs.len());

        let main_stmts: Vec<&crate::ast::Statement> = proc_div.paragraphs[0].sentences.iter()
            .flat_map(|s| &s.statements)
            .collect();

        // First statement should be PERFORM
        match &main_stmts[0] {
            crate::ast::Statement::Perform(perf) => {
                assert!(perf.target.is_some(), "should have target paragraph");
                let target = perf.target.as_ref().unwrap();
                assert_eq!(target.name, "WORK-PARA");
            }
            other => panic!("expected Perform, got {other:?}"),
        }
    }

    #[test]
    fn parse_cobol_fixed_format() {
        // Fixed-format COBOL source with sequence numbers and indicators
        let source = concat!(
            "000100 IDENTIFICATION DIVISION.                                         \n",
            "000200 PROGRAM-ID. TESTPG.                                              \n",
            "000300 DATA DIVISION.                                                    \n",
            "000400 WORKING-STORAGE SECTION.                                          \n",
            "000500 01  WS-COUNTER PIC 9(5).                                         \n",
            "000600 PROCEDURE DIVISION.                                               \n",
            "000700     DISPLAY 'HELLO'.                                              \n",
            "000800     STOP RUN.                                                     \n",
        );

        let result = parse_cobol(source);
        assert!(result.is_ok(), "parse_cobol fixed-format failed: {result:?}");
        let program = result.unwrap();
        assert_eq!(program.program_id, "TESTPG");
    }

    #[test]
    fn parse_cobol_from_source_pre_expanded() {
        // Source that has already been preprocessed (free format, no columns)
        let source = concat!(
            "IDENTIFICATION DIVISION.\n",
            "PROGRAM-ID. EXPANDED.\n",
            "DATA DIVISION.\n",
            "WORKING-STORAGE SECTION.\n",
            "01  WS-FLD PIC X(10).\n",
            "PROCEDURE DIVISION.\n",
            "MAIN-PARA.\n",
            "    DISPLAY WS-FLD.\n",
            "    STOP RUN.\n",
        );

        let result = parse_cobol_from_source(source);
        assert!(result.is_ok(), "parse_cobol_from_source failed: {result:?}");
        let program = result.unwrap();
        assert_eq!(program.program_id, "EXPANDED");
        assert!(program.data_division.is_some());
        assert!(program.procedure_division.is_some());
    }
}
