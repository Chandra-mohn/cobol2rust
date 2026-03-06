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

// ---------------------------------------------------------------------------
// CALL / CANCEL tests
// ---------------------------------------------------------------------------

#[test]
fn e2e_call_simple() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. CALLER.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    CALL 'SUBPROG'.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    assert!(
        rust_code.contains("cobol_call(&mut ctx.dispatcher"),
        "missing cobol_call: {rust_code}"
    );
    assert!(
        rust_code.contains("SUBPROG"),
        "missing program name: {rust_code}"
    );
    // ProgramContext should have dispatcher field
    assert!(
        rust_code.contains("dispatcher: CallDispatcher::new()"),
        "missing dispatcher in ProgramContext: {rust_code}"
    );
}

#[test]
fn e2e_call_using_by_ref() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. CALLER.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-PARAM PIC X(10) VALUE 'HELLO'.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    CALL 'SUBPROG' USING WS-PARAM.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    assert!(
        rust_code.contains("call_param_by_ref"),
        "missing call_param_by_ref: {rust_code}"
    );
    assert!(
        rust_code.contains("ws.ws_param"),
        "missing ws.ws_param reference: {rust_code}"
    );
    assert!(
        rust_code.contains("_call_params"),
        "missing _call_params array: {rust_code}"
    );
}

#[test]
fn e2e_call_with_exception() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. CALLER.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    CALL 'MISSING'\n",
        "        ON EXCEPTION\n",
        "            DISPLAY 'NOT FOUND'\n",
        "        NOT ON EXCEPTION\n",
        "            DISPLAY 'CALLED OK'\n",
        "    END-CALL.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    assert!(
        rust_code.contains("match cobol_call("),
        "should generate match for exception handling: {rust_code}"
    );
    assert!(
        rust_code.contains("Ok(rc)"),
        "should have Ok arm: {rust_code}"
    );
    assert!(
        rust_code.contains("Err(_e)"),
        "should have Err arm: {rust_code}"
    );
    assert!(
        rust_code.contains("NOT FOUND"),
        "should contain ON EXCEPTION display: {rust_code}"
    );
    assert!(
        rust_code.contains("CALLED OK"),
        "should contain NOT ON EXCEPTION display: {rust_code}"
    );
}

#[test]
fn e2e_cancel() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. CALLER.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    CANCEL 'SUBPROG'.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    assert!(
        rust_code.contains("cobol_cancel(&mut ctx.dispatcher"),
        "missing cobol_cancel: {rust_code}"
    );
    assert!(
        rust_code.contains("SUBPROG"),
        "missing program name in cancel: {rust_code}"
    );
}

// ---------------------------------------------------------------------------
// Session 32: Paragraph Fall-Through Execution Model
// ---------------------------------------------------------------------------

// Test: 3 paragraphs all execute sequentially via dispatch loop
#[test]
fn e2e_fall_through_basic() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. FALLTHRU.\n",
        "PROCEDURE DIVISION.\n",
        "PARA-A.\n",
        "    DISPLAY 'A'.\n",
        "PARA-B.\n",
        "    DISPLAY 'B'.\n",
        "PARA-C.\n",
        "    DISPLAY 'C'.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // Dispatch loop with all three paragraphs
    assert!(rust_code.contains("let mut _pc: usize = 0;"), "missing _pc: {rust_code}");
    assert!(rust_code.contains("0 => para_a(ws, ctx),"), "missing para_a dispatch: {rust_code}");
    assert!(rust_code.contains("1 => para_b(ws, ctx),"), "missing para_b dispatch: {rust_code}");
    assert!(rust_code.contains("2 => para_c(ws, ctx),"), "missing para_c dispatch: {rust_code}");
    assert!(rust_code.contains("_pc += 1;"), "missing _pc increment: {rust_code}");
}

// Test: STOP RUN in 2nd paragraph, 3rd skipped
#[test]
fn e2e_fall_through_stop_run() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. STOPTEST.\n",
        "PROCEDURE DIVISION.\n",
        "PARA-A.\n",
        "    DISPLAY 'A'.\n",
        "PARA-B.\n",
        "    STOP RUN.\n",
        "PARA-C.\n",
        "    DISPLAY 'C'.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // STOP RUN sets flag and returns
    assert!(rust_code.contains("ctx.stop_run();"), "missing stop_run: {rust_code}");
    assert!(rust_code.contains("if ctx.stopped || ctx.exit_program { break; }"), "missing break check: {rust_code}");
}

// Test: EXIT PROGRAM stops fall-through
#[test]
fn e2e_fall_through_exit_program() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. EXITTEST.\n",
        "PROCEDURE DIVISION.\n",
        "PARA-A.\n",
        "    DISPLAY 'A'.\n",
        "PARA-B.\n",
        "    EXIT PROGRAM.\n",
        "PARA-C.\n",
        "    DISPLAY 'C'.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    assert!(rust_code.contains("ctx.exit_program = true;"), "missing exit_program flag: {rust_code}");
}

// Test: GO TO skips paragraphs forward
#[test]
fn e2e_goto_forward() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. GOTOTEST.\n",
        "PROCEDURE DIVISION.\n",
        "PARA-A.\n",
        "    GO TO PARA-C.\n",
        "PARA-B.\n",
        "    DISPLAY 'SKIPPED'.\n",
        "PARA-C.\n",
        "    DISPLAY 'JUMPED TO C'.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // GO TO sets goto_target
    assert!(
        rust_code.contains("ctx.goto_target = Some(\"PARA-C\".to_string());"),
        "missing goto_target for PARA-C: {rust_code}"
    );
    // Dispatch loop resolves target
    assert!(rust_code.contains("\"PARA-C\" => 2,"), "missing PARA-C lookup: {rust_code}");
}

// Test: GO TO jumps backward (with guard to prevent infinite loop)
#[test]
fn e2e_goto_backward() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. GOBACK.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-CTR PIC 9(3) VALUE 0.\n",
        "PROCEDURE DIVISION.\n",
        "PARA-A.\n",
        "    ADD 1 TO WS-CTR.\n",
        "    IF WS-CTR > 3\n",
        "        STOP RUN\n",
        "    END-IF.\n",
        "    GO TO PARA-A.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // Backward GO TO
    assert!(
        rust_code.contains("ctx.goto_target = Some(\"PARA-A\".to_string());"),
        "missing backward goto: {rust_code}"
    );
    assert!(rust_code.contains("\"PARA-A\" => 0,"), "missing PARA-A lookup: {rust_code}");
}

// Test: PERFORM doesn't break caller's fall-through
#[test]
fn e2e_perform_returns() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. PERFTEST.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    PERFORM WORK-PARA.\n",
        "    DISPLAY 'AFTER PERFORM'.\n",
        "    STOP RUN.\n",
        "WORK-PARA.\n",
        "    DISPLAY 'WORKING'.\n",
        "FINAL-PARA.\n",
        "    DISPLAY 'FINAL'.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // PERFORM is still a direct function call
    assert!(rust_code.contains("work_para(ws, ctx);"), "missing PERFORM call: {rust_code}");
    // But all paragraphs are in the dispatch loop
    assert!(rust_code.contains("0 => main_para(ws, ctx),"), "missing main dispatch: {rust_code}");
    assert!(rust_code.contains("1 => work_para(ws, ctx),"), "missing work dispatch: {rust_code}");
    assert!(rust_code.contains("2 => final_para(ws, ctx),"), "missing final dispatch: {rust_code}");
}

// Test: PERFORM A THRU C executes range
#[test]
fn e2e_perform_thru() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. THRUTEST.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    PERFORM STEP-A THRU STEP-C.\n",
        "    STOP RUN.\n",
        "STEP-A.\n",
        "    DISPLAY 'A'.\n",
        "STEP-B.\n",
        "    DISPLAY 'B'.\n",
        "STEP-C.\n",
        "    DISPLAY 'C'.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // PERFORM THRU generates inline dispatch loop
    assert!(rust_code.contains("_perf_pc"), "missing _perf_pc for THRU: {rust_code}");
    assert!(rust_code.contains("step_a(ws, ctx)"), "missing step_a in THRU range: {rust_code}");
    assert!(rust_code.contains("step_b(ws, ctx)"), "missing step_b in THRU range: {rust_code}");
    assert!(rust_code.contains("step_c(ws, ctx)"), "missing step_c in THRU range: {rust_code}");
}

// Test: verify run() has loop/match/_pc structure
#[test]
fn e2e_dispatch_loop_structure() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. LOOPTEST.\n",
        "PROCEDURE DIVISION.\n",
        "PARA-ONE.\n",
        "    DISPLAY 'ONE'.\n",
        "PARA-TWO.\n",
        "    DISPLAY 'TWO'.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    assert!(rust_code.contains("loop {"), "missing loop in run(): {rust_code}");
    assert!(rust_code.contains("match _pc {"), "missing match _pc: {rust_code}");
    assert!(rust_code.contains("continue;"), "missing continue after goto resolution: {rust_code}");
}

// Test: GO TO inside IF branch
#[test]
fn e2e_goto_from_if() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. GOTOCOND.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-FLAG PIC 9 VALUE 1.\n",
        "PROCEDURE DIVISION.\n",
        "CHECK-PARA.\n",
        "    IF WS-FLAG = 1\n",
        "        GO TO DONE-PARA\n",
        "    END-IF.\n",
        "    DISPLAY 'NOT REACHED'.\n",
        "DONE-PARA.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // GO TO inside IF
    assert!(
        rust_code.contains("ctx.goto_target = Some(\"DONE-PARA\".to_string());"),
        "missing conditional goto: {rust_code}"
    );
}

// Test: single paragraph still works
#[test]
fn e2e_single_paragraph() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. SINGLE.\n",
        "PROCEDURE DIVISION.\n",
        "ONLY-PARA.\n",
        "    DISPLAY 'ALONE'.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    assert!(rust_code.contains("0 => only_para(ws, ctx),"), "missing single para dispatch: {rust_code}");
    assert!(rust_code.contains("fn only_para("), "missing only_para fn: {rust_code}");
}

// Test: paragraphs in sections also fall through
#[test]
fn e2e_sections_fall_through() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. SECTEST.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-SECTION SECTION.\n",
        "SETUP-PARA.\n",
        "    DISPLAY 'SETUP'.\n",
        "PROCESS-PARA.\n",
        "    DISPLAY 'PROCESS'.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // Both section paragraphs should be in the dispatch loop
    assert!(rust_code.contains("0 => setup_para(ws, ctx),"), "missing setup_para dispatch: {rust_code}");
    assert!(rust_code.contains("1 => process_para(ws, ctx),"), "missing process_para dispatch: {rust_code}");
    // Section name may include the SECTION keyword depending on parser behavior
    assert!(rust_code.contains("// --- Section:"), "missing section comment: {rust_code}");
}

// =========================================================================
// Session 33: SET, START, EXIT variants, GO TO DEPENDING ON
// =========================================================================

// Test: SET TO literal value
#[test]
fn e2e_set_to_literal() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. SETTEST.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01 WS-INDEX PIC 9(3) VALUE 0.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    SET WS-INDEX TO 5.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");
    // SET TO should produce an assignment
    assert!(
        rust_code.contains("ws.ws_index") && rust_code.contains("5"),
        "missing SET TO assignment: {rust_code}"
    );
}

// Test: SET condition TO TRUE
#[test]
fn e2e_set_to_true() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. SETBOOL.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01 WS-STATUS PIC X VALUE 'N'.\n",
        "   88 IS-ACTIVE VALUE 'Y'.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    SET IS-ACTIVE TO TRUE.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");
    // SET TO TRUE should MOVE the 88-level value to the parent
    assert!(
        rust_code.contains("cobol_move") || rust_code.contains("pack"),
        "missing SET TO TRUE -> MOVE: {rust_code}"
    );
}

// Test: SET UP BY
#[test]
fn e2e_set_up_by() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. SETUP.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01 WS-CTR PIC 9(3) VALUE 0.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    SET WS-CTR UP BY 1.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");
    assert!(
        rust_code.contains("ws.ws_ctr") && rust_code.contains("+="),
        "missing SET UP BY: {rust_code}"
    );
}

// Test: SET DOWN BY
#[test]
fn e2e_set_down_by() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. SETDOWN.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01 WS-CTR PIC 9(3) VALUE 10.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    SET WS-CTR DOWN BY 3.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");
    assert!(
        rust_code.contains("ws.ws_ctr") && rust_code.contains("-="),
        "missing SET DOWN BY: {rust_code}"
    );
}

// Test: EXIT PARAGRAPH produces return
#[test]
fn e2e_exit_paragraph() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. EXITPARA.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    DISPLAY 'BEFORE'.\n",
        "    EXIT PARAGRAPH.\n",
        "    DISPLAY 'AFTER'.\n",
        "NEXT-PARA.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");
    // EXIT PARAGRAPH should generate return;
    assert!(
        rust_code.contains("return;"),
        "missing return for EXIT PARAGRAPH: {rust_code}"
    );
}

// Test: GO TO DEPENDING ON generates match
#[test]
fn e2e_goto_depending_on() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. GOTODEP.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01 WS-INDEX PIC 9 VALUE 2.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    GO TO PARA-A PARA-B PARA-C\n",
        "        DEPENDING ON WS-INDEX.\n",
        "    STOP RUN.\n",
        "PARA-A.\n",
        "    DISPLAY 'A'.\n",
        "    STOP RUN.\n",
        "PARA-B.\n",
        "    DISPLAY 'B'.\n",
        "    STOP RUN.\n",
        "PARA-C.\n",
        "    DISPLAY 'C'.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");
    // Should generate a match on the index
    assert!(
        rust_code.contains("match _goto_idx"),
        "missing match _goto_idx: {rust_code}"
    );
    assert!(
        rust_code.contains("1 => ctx.goto_target = Some(\"PARA-A\""),
        "missing PARA-A target: {rust_code}"
    );
    assert!(
        rust_code.contains("2 => ctx.goto_target = Some(\"PARA-B\""),
        "missing PARA-B target: {rust_code}"
    );
    assert!(
        rust_code.contains("3 => ctx.goto_target = Some(\"PARA-C\""),
        "missing PARA-C target: {rust_code}"
    );
}

// Test: START statement with KEY condition
#[test]
fn e2e_start_statement() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. STARTTEST.\n",
        "ENVIRONMENT DIVISION.\n",
        "INPUT-OUTPUT SECTION.\n",
        "FILE-CONTROL.\n",
        "    SELECT IDX-FILE ASSIGN TO 'IDX.DAT'\n",
        "        ORGANIZATION IS INDEXED\n",
        "        ACCESS MODE IS DYNAMIC\n",
        "        RECORD KEY IS IDX-KEY\n",
        "        FILE STATUS IS WS-STATUS.\n",
        "DATA DIVISION.\n",
        "FILE SECTION.\n",
        "FD IDX-FILE.\n",
        "01 IDX-RECORD.\n",
        "   05 IDX-KEY PIC X(10).\n",
        "   05 IDX-DATA PIC X(40).\n",
        "WORKING-STORAGE SECTION.\n",
        "01 WS-STATUS PIC XX.\n",
        "01 WS-SEARCH-KEY PIC X(10).\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    OPEN INPUT IDX-FILE.\n",
        "    START IDX-FILE KEY >= WS-SEARCH-KEY\n",
        "        INVALID KEY\n",
        "            DISPLAY 'NOT FOUND'\n",
        "        NOT INVALID KEY\n",
        "            DISPLAY 'FOUND'\n",
        "    END-START.\n",
        "    CLOSE IDX-FILE.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");
    // Should have a start call
    assert!(
        rust_code.contains("idx_file.start("),
        "missing start call: {rust_code}"
    );
    // Should have match for INVALID KEY handling
    assert!(
        rust_code.contains("Ok(())") && rust_code.contains("Err(_)"),
        "missing INVALID KEY match: {rust_code}"
    );
}

// Test: plain EXIT (no qualifier) treated as EXIT PARAGRAPH
#[test]
fn e2e_plain_exit() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. EXITTEST.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    PERFORM DO-WORK.\n",
        "    STOP RUN.\n",
        "DO-WORK.\n",
        "    DISPLAY 'WORKING'.\n",
        "DO-WORK-EXIT.\n",
        "    EXIT.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");
    // Plain EXIT -> return; (same as EXIT PARAGRAPH)
    assert!(
        rust_code.contains("return;"),
        "missing return for plain EXIT: {rust_code}"
    );
    // Should NOT contain exit_program for plain EXIT
    // (though other code might set exit_program; check the do_work_exit fn specifically)
    assert!(
        rust_code.contains("fn do_work_exit"),
        "missing do_work_exit paragraph: {rust_code}"
    );
}
