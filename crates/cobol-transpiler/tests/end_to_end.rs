//! End-to-end integration tests for the COBOL-to-Rust transpiler.
//!
//! Each test transpiles a complete COBOL program and verifies that the
//! generated Rust code contains the correct structure, API calls, and
//! control flow patterns.

use cobol_transpiler::transpile::transpile;

// ---------------------------------------------------------------------------
// Test 1: Hello World
// ---------------------------------------------------------------------------
#[test]
fn e2e_hello_world() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. HELLO.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    DISPLAY 'HELLO WORLD'.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // File header
    assert!(
        rust_code.contains("//! Program: HELLO"),
        "missing program id header"
    );
    assert!(
        rust_code.contains("use cobol_runtime::prelude::*;"),
        "missing prelude import"
    );

    // WorkingStorage (empty but present)
    assert!(
        rust_code.contains("pub struct WorkingStorage"),
        "missing WorkingStorage struct"
    );
    assert!(
        rust_code.contains("impl WorkingStorage"),
        "missing WorkingStorage impl"
    );

    // ProgramContext
    assert!(
        rust_code.contains("pub struct ProgramContext"),
        "missing ProgramContext struct"
    );

    // Procedure division
    assert!(
        rust_code.contains("fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext)"),
        "missing run() function"
    );
    assert!(
        rust_code.contains("fn main_para("),
        "missing main_para function"
    );

    // DISPLAY generates display call with string literal
    assert!(
        rust_code.contains("print!") || rust_code.contains("println!"),
        "missing DISPLAY -> print!/println!"
    );
    assert!(
        rust_code.contains("HELLO WORLD"),
        "missing HELLO WORLD literal"
    );

    // STOP RUN
    assert!(
        rust_code.contains("ctx.stop_run()"),
        "missing STOP RUN -> ctx.stop_run()"
    );

    // main() entry point
    assert!(
        rust_code.contains("fn main()"),
        "missing main function"
    );
    assert!(
        rust_code.contains("WorkingStorage::new()"),
        "missing WorkingStorage construction in main"
    );
    assert!(
        rust_code.contains("ProgramContext::new()"),
        "missing ProgramContext construction in main"
    );
    assert!(
        rust_code.contains("run(&mut ws, &mut ctx)"),
        "missing run() call in main"
    );
}

// ---------------------------------------------------------------------------
// Test 2: Simple Arithmetic (ADD, MOVE, DISPLAY)
// ---------------------------------------------------------------------------
#[test]
fn e2e_simple_arithmetic() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. ARITH.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-A PIC 9(5) VALUE 10.\n",
        "01  WS-B PIC 9(5) VALUE 20.\n",
        "01  WS-C PIC 9(5).\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    ADD WS-A TO WS-B.\n",
        "    MOVE WS-B TO WS-C.\n",
        "    DISPLAY WS-C.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // Data fields
    assert!(
        rust_code.contains("ws_a"),
        "missing WS-A field"
    );
    assert!(
        rust_code.contains("ws_b"),
        "missing WS-B field"
    );
    assert!(
        rust_code.contains("ws_c"),
        "missing WS-C field"
    );

    // ADD WS-A TO WS-B -> cobol_add
    assert!(
        rust_code.contains("cobol_add"),
        "missing ADD -> cobol_add"
    );

    // MOVE WS-B TO WS-C -> cobol_move
    assert!(
        rust_code.contains("cobol_move"),
        "missing MOVE -> cobol_move"
    );

    // DISPLAY WS-C
    assert!(
        rust_code.contains("print!") || rust_code.contains("display_bytes"),
        "missing DISPLAY call"
    );

    // STOP RUN
    assert!(
        rust_code.contains("ctx.stop_run()"),
        "missing STOP RUN"
    );

    // VALUE clauses in initialization
    assert!(
        rust_code.contains("10") || rust_code.contains("dec!(10)"),
        "missing VALUE 10 initialization"
    );
    assert!(
        rust_code.contains("20") || rust_code.contains("dec!(20)"),
        "missing VALUE 20 initialization"
    );
}

// ---------------------------------------------------------------------------
// Test 3: IF/ELSE Branching
// ---------------------------------------------------------------------------
#[test]
fn e2e_if_else_branching() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. TESTIF.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-X PIC 9(3) VALUE 5.\n",
        "01  WS-RESULT PIC X(20).\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    IF WS-X > 0\n",
        "        DISPLAY 'POSITIVE'\n",
        "    ELSE\n",
        "        DISPLAY 'NOT POSITIVE'\n",
        "    END-IF.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // IF condition generates Rust if
    assert!(
        rust_code.contains("if "),
        "missing if keyword"
    );

    // Comparison: WS-X > 0
    assert!(
        rust_code.contains("ws.ws_x") || rust_code.contains("ws_x"),
        "missing WS-X reference in condition"
    );

    // ELSE branch
    assert!(
        rust_code.contains("} else {"),
        "missing else branch"
    );

    // Both DISPLAY statements
    assert!(
        rust_code.contains("POSITIVE"),
        "missing POSITIVE display"
    );
    assert!(
        rust_code.contains("NOT POSITIVE"),
        "missing NOT POSITIVE display"
    );
}

// ---------------------------------------------------------------------------
// Test 4: PERFORM with paragraph call
// ---------------------------------------------------------------------------
#[test]
fn e2e_perform_paragraph() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. TESTPERF.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-I PIC 9(3).\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    PERFORM WORK-PARA.\n",
        "    STOP RUN.\n",
        "WORK-PARA.\n",
        "    DISPLAY 'WORKING'.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // PERFORM WORK-PARA -> function call
    assert!(
        rust_code.contains("work_para(ws, ctx)"),
        "missing PERFORM -> work_para() call"
    );

    // WORK-PARA generates its own function
    assert!(
        rust_code.contains("fn work_para("),
        "missing work_para function definition"
    );

    // DISPLAY inside work paragraph
    assert!(
        rust_code.contains("WORKING"),
        "missing WORKING display in work_para"
    );
}

// ---------------------------------------------------------------------------
// Test 5: EVALUATE (COBOL's CASE/SWITCH)
// ---------------------------------------------------------------------------
#[test]
fn e2e_evaluate_statement() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. TESTEVAL.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-CODE PIC 9(2) VALUE 1.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    EVALUATE WS-CODE\n",
        "        WHEN 1\n",
        "            DISPLAY 'ONE'\n",
        "        WHEN 2\n",
        "            DISPLAY 'TWO'\n",
        "        WHEN OTHER\n",
        "            DISPLAY 'OTHER'\n",
        "    END-EVALUATE.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // EVALUATE generates cascading if/else if/else
    assert!(
        rust_code.contains("if "),
        "missing if from EVALUATE"
    );
    assert!(
        rust_code.contains("} else if ") || rust_code.contains("else if"),
        "missing else if from WHEN 2"
    );
    assert!(
        rust_code.contains("} else {"),
        "missing else from WHEN OTHER"
    );

    // All display values
    assert!(
        rust_code.contains("ONE"),
        "missing 'ONE' display"
    );
    assert!(
        rust_code.contains("TWO"),
        "missing 'TWO' display"
    );
    assert!(
        rust_code.contains("OTHER"),
        "missing 'OTHER' display"
    );
}

// ---------------------------------------------------------------------------
// Test 6: Multiple paragraphs with fall-through pattern
// ---------------------------------------------------------------------------
#[test]
fn e2e_multiple_paragraphs() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. MULTIPARA.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-NAME PIC X(20) VALUE 'COBOL'.\n",
        "01  WS-COUNT PIC 9(5) VALUE 0.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    PERFORM SETUP-PARA.\n",
        "    PERFORM PROCESS-PARA.\n",
        "    DISPLAY WS-COUNT.\n",
        "    STOP RUN.\n",
        "SETUP-PARA.\n",
        "    MOVE 100 TO WS-COUNT.\n",
        "PROCESS-PARA.\n",
        "    ADD 1 TO WS-COUNT.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // Three separate paragraph functions
    assert!(
        rust_code.contains("fn main_para("),
        "missing main_para function"
    );
    assert!(
        rust_code.contains("fn setup_para("),
        "missing setup_para function"
    );
    assert!(
        rust_code.contains("fn process_para("),
        "missing process_para function"
    );

    // PERFORM calls
    assert!(
        rust_code.contains("setup_para(ws, ctx)"),
        "missing PERFORM SETUP-PARA call"
    );
    assert!(
        rust_code.contains("process_para(ws, ctx)"),
        "missing PERFORM PROCESS-PARA call"
    );

    // Value initialization
    assert!(
        rust_code.contains("COBOL") || rust_code.contains("PicX::from_str"),
        "missing WS-NAME value initialization"
    );
}

// ---------------------------------------------------------------------------
// Test 7: SUBTRACT and MULTIPLY
// ---------------------------------------------------------------------------
#[test]
fn e2e_subtract_multiply() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. SUBMUL.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-X PIC 9(5) VALUE 100.\n",
        "01  WS-Y PIC 9(5) VALUE 10.\n",
        "01  WS-Z PIC 9(5) VALUE 3.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    SUBTRACT WS-Y FROM WS-X.\n",
        "    MULTIPLY WS-Z BY WS-X.\n",
        "    DISPLAY WS-X.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // SUBTRACT generates cobol_subtract
    assert!(
        rust_code.contains("cobol_subtract"),
        "missing SUBTRACT -> cobol_subtract"
    );

    // MULTIPLY generates cobol_multiply
    assert!(
        rust_code.contains("cobol_multiply"),
        "missing MULTIPLY -> cobol_multiply"
    );
}

// ---------------------------------------------------------------------------
// Test 8: COMPUTE with arithmetic expression
// ---------------------------------------------------------------------------
#[test]
fn e2e_compute_expression() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. TESTCOMP.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-A PIC 9(5) VALUE 10.\n",
        "01  WS-B PIC 9(5) VALUE 5.\n",
        "01  WS-RESULT PIC 9(5).\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    COMPUTE WS-RESULT = WS-A + WS-B.\n",
        "    DISPLAY WS-RESULT.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // COMPUTE generates cobol_compute
    assert!(
        rust_code.contains("cobol_compute"),
        "missing COMPUTE -> cobol_compute"
    );

    // Result field reference
    assert!(
        rust_code.contains("ws.ws_result") || rust_code.contains("ws_result"),
        "missing WS-RESULT reference"
    );
}

// ---------------------------------------------------------------------------
// Test 9: INITIALIZE verb
// ---------------------------------------------------------------------------
#[test]
fn e2e_initialize() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. TESTINIT.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-REC.\n",
        "    05  WS-NAME PIC X(20) VALUE 'TEST'.\n",
        "    05  WS-COUNT PIC 9(5) VALUE 99.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    INITIALIZE WS-REC.\n",
        "    DISPLAY WS-NAME.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // INITIALIZE generates cobol_initialize
    assert!(
        rust_code.contains("cobol_initialize"),
        "missing INITIALIZE -> cobol_initialize"
    );

    // Group fields flattened into struct
    assert!(
        rust_code.contains("ws_name") || rust_code.contains("ws_rec_ws_name"),
        "missing group child field WS-NAME"
    );
    assert!(
        rust_code.contains("ws_count") || rust_code.contains("ws_rec_ws_count"),
        "missing group child field WS-COUNT"
    );
}

// ---------------------------------------------------------------------------
// Test 10: Generated code is syntactically well-formed
// ---------------------------------------------------------------------------
#[test]
fn e2e_balanced_braces() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. BRACES.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-X PIC 9(3) VALUE 5.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    IF WS-X > 0\n",
        "        DISPLAY 'YES'\n",
        "    ELSE\n",
        "        DISPLAY 'NO'\n",
        "    END-IF.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // Count braces to verify they're balanced
    let open_braces = rust_code.chars().filter(|c| *c == '{').count();
    let close_braces = rust_code.chars().filter(|c| *c == '}').count();
    assert_eq!(
        open_braces, close_braces,
        "unbalanced braces: {open_braces} open vs {close_braces} close\n\nGenerated code:\n{rust_code}"
    );
}

// ---------------------------------------------------------------------------
// Test 11: DIVIDE with INTO
// ---------------------------------------------------------------------------
#[test]
fn e2e_divide() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. TESTDIV.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-A PIC 9(5) VALUE 100.\n",
        "01  WS-B PIC 9(5) VALUE 5.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    DIVIDE WS-B INTO WS-A.\n",
        "    DISPLAY WS-A.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // DIVIDE generates cobol_divide
    assert!(
        rust_code.contains("cobol_divide"),
        "missing DIVIDE -> cobol_divide"
    );
}

// ---------------------------------------------------------------------------
// Test 12: MOVE with figurative constant
// ---------------------------------------------------------------------------
#[test]
fn e2e_move_figurative() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. TESTFIG.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-NAME PIC X(20) VALUE 'TEST'.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    MOVE SPACES TO WS-NAME.\n",
        "    DISPLAY WS-NAME.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // MOVE SPACES generates cobol_move with SPACES
    assert!(
        rust_code.contains("cobol_move"),
        "missing MOVE -> cobol_move"
    );
    assert!(
        rust_code.contains("SPACES"),
        "missing SPACES figurative constant"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Standalone (level 77) fields
// ---------------------------------------------------------------------------
#[test]
fn e2e_level_77_fields() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. LVL77.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "77  WS-COUNTER PIC 9(5) VALUE 0.\n",
        "77  WS-FLAG PIC X VALUE 'N'.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    ADD 1 TO WS-COUNTER.\n",
        "    DISPLAY WS-COUNTER.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // Level 77 fields appear in struct
    assert!(
        rust_code.contains("ws_counter"),
        "missing level 77 WS-COUNTER field"
    );
    assert!(
        rust_code.contains("ws_flag"),
        "missing level 77 WS-FLAG field"
    );

    // ADD generates cobol_add
    assert!(
        rust_code.contains("cobol_add"),
        "missing ADD -> cobol_add"
    );
}

// ---------------------------------------------------------------------------
// Test 14: No-data program (minimal)
// ---------------------------------------------------------------------------
#[test]
fn e2e_minimal_no_data() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. MINIMAL.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // Even without data division, struct and main should exist
    assert!(
        rust_code.contains("fn main()"),
        "missing main function"
    );
    assert!(
        rust_code.contains("ctx.stop_run()"),
        "missing STOP RUN"
    );
}

// ---------------------------------------------------------------------------
// Test 15: File I/O statements (OPEN, READ, WRITE, CLOSE)
// ---------------------------------------------------------------------------
#[test]
fn e2e_file_io_statements() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. FILEIO.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01 WS-RECORD PIC X(80).\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    OPEN INPUT INPUT-FILE\n",
        "    OPEN OUTPUT OUTPUT-FILE\n",
        "    READ INPUT-FILE INTO WS-RECORD\n",
        "        AT END DISPLAY 'EOF'\n",
        "    END-READ\n",
        "    WRITE WS-RECORD\n",
        "    CLOSE INPUT-FILE\n",
        "    CLOSE OUTPUT-FILE\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // OPEN INPUT generates correct code
    assert!(
        rust_code.contains("FileOpenMode::Input"),
        "missing OPEN INPUT: {rust_code}"
    );
    // OPEN OUTPUT generates correct code
    assert!(
        rust_code.contains("FileOpenMode::Output"),
        "missing OPEN OUTPUT: {rust_code}"
    );
    // READ with AT END
    assert!(
        rust_code.contains("read_next()"),
        "missing read_next call: {rust_code}"
    );
    assert!(
        rust_code.contains("Err(_)"),
        "missing AT END error branch: {rust_code}"
    );
    assert!(
        rust_code.contains("EOF"),
        "missing AT END display: {rust_code}"
    );
    // WRITE
    assert!(
        rust_code.contains("write_record"),
        "missing write_record call: {rust_code}"
    );
    // CLOSE
    assert!(
        rust_code.contains(".close()"),
        "missing close call: {rust_code}"
    );
}

// ---------------------------------------------------------------------------
// Test 16: DELETE and REWRITE with INVALID KEY
// ---------------------------------------------------------------------------
#[test]
fn e2e_delete_rewrite_statements() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. DELRW.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01 MASTER-REC PIC X(100).\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    DELETE MASTER-FILE\n",
        "        INVALID KEY DISPLAY 'NOT FOUND'\n",
        "    END-DELETE\n",
        "    REWRITE MASTER-REC\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // DELETE with INVALID KEY
    assert!(
        rust_code.contains("delete_record"),
        "missing delete_record call: {rust_code}"
    );
    assert!(
        rust_code.contains("NOT FOUND"),
        "missing INVALID KEY display: {rust_code}"
    );
    // REWRITE
    assert!(
        rust_code.contains("rewrite_record"),
        "missing rewrite_record call: {rust_code}"
    );
}
