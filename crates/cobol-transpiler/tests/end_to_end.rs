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
        rust_code.contains("ws.ws_ctr") && rust_code.contains("cobol_add"),
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
        rust_code.contains("ws.ws_ctr") && rust_code.contains("cobol_subtract"),
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

// ---------------------------------------------------------------------------
// Test: Level 66 RENAMES -- single field alias
// ---------------------------------------------------------------------------
#[test]
fn e2e_renames_single() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. REN1.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-RECORD.\n",
        "    05  WS-NAME PIC X(20).\n",
        "    05  WS-AGE PIC 9(3).\n",
        "    66  WS-ALIAS RENAMES WS-NAME.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    DISPLAY WS-ALIAS.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // RENAMES alias should appear as a struct field
    assert!(
        rust_code.contains("ws_alias"),
        "missing RENAMES alias field: {rust_code}"
    );
    // Should contain a RENAMES comment
    assert!(
        rust_code.contains("RENAMES"),
        "missing RENAMES annotation: {rust_code}"
    );
}

// ---------------------------------------------------------------------------
// Test: Level 66 RENAMES THRU -- range alias
// ---------------------------------------------------------------------------
#[test]
fn e2e_renames_thru() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. REN2.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-RECORD.\n",
        "    05  WS-YEAR PIC 9(4).\n",
        "    05  WS-MONTH PIC 9(2).\n",
        "    05  WS-DAY PIC 9(2).\n",
        "    66  WS-DATE-RANGE RENAMES WS-YEAR THRU WS-DAY.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    DISPLAY WS-DATE-RANGE.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // RENAMES THRU should generate a PicX field spanning the range
    assert!(
        rust_code.contains("ws_date_range"),
        "missing RENAMES THRU field: {rust_code}"
    );
    assert!(
        rust_code.contains("THRU"),
        "missing THRU annotation: {rust_code}"
    );
    assert!(
        rust_code.contains("PicX"),
        "RENAMES THRU should produce PicX type: {rust_code}"
    );
}

// ---------------------------------------------------------------------------
// Test: Level 66 RENAMES with numeric target
// ---------------------------------------------------------------------------
#[test]
fn e2e_renames_numeric_target() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. REN3.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-RECORD.\n",
        "    05  WS-AMOUNT PIC 9(7)V99.\n",
        "    66  WS-AMT RENAMES WS-AMOUNT.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    DISPLAY WS-AMT.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // RENAMES of numeric should copy the numeric type (PackedDecimal)
    assert!(
        rust_code.contains("ws_amt"),
        "missing RENAMES numeric alias field: {rust_code}"
    );
    assert!(
        rust_code.contains("RENAMES WS-AMOUNT"),
        "missing RENAMES comment: {rust_code}"
    );
}

// ---------------------------------------------------------------------------
// Test: Multiple RENAMES in one record
// ---------------------------------------------------------------------------
#[test]
fn e2e_renames_multiple() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. REN4.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-EMPLOYEE.\n",
        "    05  WS-EMP-NAME PIC X(30).\n",
        "    05  WS-EMP-ID PIC 9(5).\n",
        "    05  WS-EMP-DEPT PIC X(10).\n",
        "    66  WS-NAME-ALIAS RENAMES WS-EMP-NAME.\n",
        "    66  WS-ID-ALIAS RENAMES WS-EMP-ID.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    DISPLAY WS-NAME-ALIAS.\n",
        "    DISPLAY WS-ID-ALIAS.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    assert!(
        rust_code.contains("ws_name_alias"),
        "missing first RENAMES alias: {rust_code}"
    );
    assert!(
        rust_code.contains("ws_id_alias"),
        "missing second RENAMES alias: {rust_code}"
    );
}

// ---------------------------------------------------------------------------
// Test: No RENAMES -- record without level 66 should be unaffected
// ---------------------------------------------------------------------------
#[test]
fn e2e_renames_none_unchanged() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. REN5.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-RECORD.\n",
        "    05  WS-NAME PIC X(20).\n",
        "    05  WS-AGE PIC 9(3).\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    DISPLAY WS-NAME.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // Should not contain any RENAMES artifacts
    assert!(
        !rust_code.contains("RENAMES"),
        "no RENAMES expected: {rust_code}"
    );
    assert!(
        rust_code.contains("ws_name"),
        "normal fields should still work: {rust_code}"
    );
}

// ---------------------------------------------------------------------------
// Test: AlphanumericEdited -- DISPLAY of edited field
// ---------------------------------------------------------------------------
#[test]
fn e2e_alpha_edited_display() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. AE1.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-RECORD.\n",
        "    05  WS-DATE-FMT PIC X(2)/X(2)/X(4).\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    DISPLAY WS-DATE-FMT.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // Should generate AlphanumericEdited type
    assert!(
        rust_code.contains("AlphanumericEdited"),
        "should use AlphanumericEdited type: {rust_code}"
    );
    assert!(
        rust_code.contains("AlphaEditSymbol"),
        "should reference AlphaEditSymbol: {rust_code}"
    );
    assert!(
        rust_code.contains("ws_date_fmt"),
        "should have field name: {rust_code}"
    );
}

// ---------------------------------------------------------------------------
// Test: AlphanumericEdited -- MOVE alphanumeric TO edited
// ---------------------------------------------------------------------------
#[test]
fn e2e_alpha_edited_move() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. AE2.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-RECORD.\n",
        "    05  WS-RAW PIC X(6).\n",
        "    05  WS-EDITED PIC X(3)BX(3).\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    MOVE WS-RAW TO WS-EDITED.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // MOVE should generate cobol_move
    assert!(
        rust_code.contains("cobol_move"),
        "should generate MOVE call: {rust_code}"
    );
    // Edited field should be AlphanumericEdited
    assert!(
        rust_code.contains("AlphanumericEdited"),
        "should use AlphanumericEdited: {rust_code}"
    );
}

// ---------------------------------------------------------------------------
// Test: AlphanumericEdited -- INITIALIZE edited field
// ---------------------------------------------------------------------------
#[test]
fn e2e_alpha_edited_initialize() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. AE3.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-RECORD.\n",
        "    05  WS-FMT PIC X(3)0X(3).\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    INITIALIZE WS-FMT.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    assert!(
        rust_code.contains("AlphanumericEdited"),
        "should use AlphanumericEdited: {rust_code}"
    );
    assert!(
        rust_code.contains("initialize"),
        "should have initialize call: {rust_code}"
    );
}

// ---------------------------------------------------------------------------
// COMP-1/COMP-2 Float Tests
// ---------------------------------------------------------------------------

#[test]
fn e2e_comp1_display() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. FLT1.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-RATE COMP-1 VALUE 3.14.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    DISPLAY WS-RATE.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    assert!(
        rust_code.contains("Comp1Float"),
        "should use Comp1Float type: {rust_code}"
    );
    assert!(
        rust_code.contains("Comp1Float::from_f32(3.14f32)"),
        "should init with from_f32: {rust_code}"
    );
    assert!(
        rust_code.contains("display_bytes"),
        "should call display_bytes for DISPLAY: {rust_code}"
    );
}

#[test]
fn e2e_comp2_arithmetic() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. FLT2.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-A COMP-2 VALUE 100.5.\n",
        "01  WS-B COMP-2 VALUE 50.25.\n",
        "01  WS-RESULT COMP-2.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    ADD WS-A TO WS-B.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    assert!(
        rust_code.contains("Comp2Float"),
        "should use Comp2Float type: {rust_code}"
    );
    assert!(
        rust_code.contains("Comp2Float::from_f64(100.5f64)"),
        "should init WS-A with from_f64: {rust_code}"
    );
    assert!(
        rust_code.contains("cobol_add"),
        "should generate cobol_add call: {rust_code}"
    );
}

#[test]
fn e2e_comp1_move() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. FLT3.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-FLOAT COMP-1 VALUE 42.5.\n",
        "01  WS-NUM PIC 9(5)V99 COMP-3.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    MOVE WS-FLOAT TO WS-NUM.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    assert!(
        rust_code.contains("Comp1Float"),
        "should use Comp1Float for COMP-1: {rust_code}"
    );
    assert!(
        rust_code.contains("cobol_move"),
        "should generate cobol_move call: {rust_code}"
    );
}

// ===========================================================================
// Session 37: Real-World COBOL Test Suite
// ===========================================================================

// ---------------------------------------------------------------------------
// Program 1: PAYROLL -- batch payroll calculation
// Features: Group records, 88-level conditions, PERFORM VARYING, COMPUTE,
//           REDEFINES
// ---------------------------------------------------------------------------

#[test]
fn e2e_payroll_structure() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. PAYROLL.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-EMPLOYEE.\n",
        "    05  WS-EMP-ID       PIC 9(5) VALUE 0.\n",
        "    05  WS-EMP-HOURS    PIC 9(3) VALUE 0.\n",
        "    05  WS-EMP-RATE     PIC 9(3)V99 VALUE 0.\n",
        "    05  WS-EMP-STATUS   PIC X VALUE 'A'.\n",
        "        88  IS-ACTIVE    VALUE 'A'.\n",
        "        88  IS-INACTIVE  VALUE 'I'.\n",
        "    05  WS-EMP-PAY      PIC 9(5)V99 VALUE 0.\n",
        "01  WS-RAW-DATA.\n",
        "    05  WS-RAW-BYTES    PIC X(20).\n",
        "    05  WS-RAW-OVERLAY  REDEFINES WS-RAW-BYTES PIC 9(10).\n",
        "01  WS-COUNTER          PIC 9(3) VALUE 0.\n",
        "01  WS-MAX-EMP          PIC 9(3) VALUE 5.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    PERFORM CALC-PARA VARYING WS-COUNTER\n",
        "        FROM 1 BY 1 UNTIL WS-COUNTER > WS-MAX-EMP.\n",
        "    STOP RUN.\n",
        "CALC-PARA.\n",
        "    IF IS-ACTIVE\n",
        "        COMPUTE WS-EMP-PAY = WS-EMP-HOURS * WS-EMP-RATE\n",
        "    END-IF.\n",
        "    DISPLAY WS-EMP-PAY.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // WorkingStorage with child fields
    assert!(
        rust_code.contains("pub struct WorkingStorage"),
        "missing WorkingStorage struct: {rust_code}"
    );
    assert!(
        rust_code.contains("ws_emp_id"),
        "missing WS-EMP-ID field: {rust_code}"
    );
    assert!(
        rust_code.contains("ws_emp_pay"),
        "missing WS-EMP-PAY field: {rust_code}"
    );

    // RedefinesGroup
    assert!(
        rust_code.contains("RedefinesGroup"),
        "missing RedefinesGroup for REDEFINES: {rust_code}"
    );

    // Dispatch loop with both paragraphs
    assert!(
        rust_code.contains("fn main_para("),
        "missing main_para function: {rust_code}"
    );
    assert!(
        rust_code.contains("fn calc_para("),
        "missing calc_para function: {rust_code}"
    );

    // Balanced braces
    let open = rust_code.chars().filter(|c| *c == '{').count();
    let close = rust_code.chars().filter(|c| *c == '}').count();
    assert_eq!(
        open, close,
        "unbalanced braces: {open} open vs {close} close\n{rust_code}"
    );
}

#[test]
fn e2e_payroll_features() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. PAYROLL.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-EMPLOYEE.\n",
        "    05  WS-EMP-ID       PIC 9(5) VALUE 0.\n",
        "    05  WS-EMP-HOURS    PIC 9(3) VALUE 0.\n",
        "    05  WS-EMP-RATE     PIC 9(3)V99 VALUE 0.\n",
        "    05  WS-EMP-STATUS   PIC X VALUE 'A'.\n",
        "        88  IS-ACTIVE    VALUE 'A'.\n",
        "        88  IS-INACTIVE  VALUE 'I'.\n",
        "    05  WS-EMP-PAY      PIC 9(5)V99 VALUE 0.\n",
        "01  WS-RAW-DATA.\n",
        "    05  WS-RAW-BYTES    PIC X(20).\n",
        "    05  WS-RAW-OVERLAY  REDEFINES WS-RAW-BYTES PIC 9(10).\n",
        "01  WS-COUNTER          PIC 9(3) VALUE 0.\n",
        "01  WS-MAX-EMP          PIC 9(3) VALUE 5.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    PERFORM CALC-PARA VARYING WS-COUNTER\n",
        "        FROM 1 BY 1 UNTIL WS-COUNTER > WS-MAX-EMP.\n",
        "    STOP RUN.\n",
        "CALC-PARA.\n",
        "    IF IS-ACTIVE\n",
        "        COMPUTE WS-EMP-PAY = WS-EMP-HOURS * WS-EMP-RATE\n",
        "    END-IF.\n",
        "    DISPLAY WS-EMP-PAY.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // COMPUTE
    assert!(
        rust_code.contains("cobol_compute"),
        "missing cobol_compute call: {rust_code}"
    );

    // PERFORM VARYING loop (while !(...))
    assert!(
        rust_code.contains("while !("),
        "missing PERFORM VARYING while loop: {rust_code}"
    );

    // 88-level condition inline comparison for IS-ACTIVE
    assert!(
        rust_code.contains("ws_emp_status"),
        "missing 88-level parent field reference: {rust_code}"
    );
}

// ---------------------------------------------------------------------------
// Program 2: REPORT -- string manipulation
// Features: STRING with DELIMITED BY, UNSTRING with delimiter,
//           INSPECT TALLYING, multiple paragraphs
// ---------------------------------------------------------------------------

#[test]
fn e2e_report_structure() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. REPORT.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-FIRST-NAME      PIC X(15) VALUE 'JOHN'.\n",
        "01  WS-LAST-NAME       PIC X(15) VALUE 'SMITH'.\n",
        "01  WS-FULL-NAME       PIC X(35).\n",
        "01  WS-INPUT-LINE      PIC X(40) VALUE 'PART1,PART2,PART3'.\n",
        "01  WS-FIELD1          PIC X(10).\n",
        "01  WS-FIELD2          PIC X(10).\n",
        "01  WS-FIELD3          PIC X(10).\n",
        "01  WS-SPACE-COUNT     PIC 9(3) VALUE 0.\n",
        "01  WS-PTR             PIC 9(3) VALUE 1.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    PERFORM FORMAT-PARA.\n",
        "    PERFORM SPLIT-PARA.\n",
        "    PERFORM COUNT-PARA.\n",
        "    DISPLAY WS-FULL-NAME.\n",
        "    STOP RUN.\n",
        "FORMAT-PARA.\n",
        "    STRING WS-FIRST-NAME DELIMITED BY ' '\n",
        "           ' ' DELIMITED BY SIZE\n",
        "           WS-LAST-NAME DELIMITED BY ' '\n",
        "        INTO WS-FULL-NAME\n",
        "        WITH POINTER WS-PTR.\n",
        "SPLIT-PARA.\n",
        "    UNSTRING WS-INPUT-LINE DELIMITED BY ','\n",
        "        INTO WS-FIELD1 WS-FIELD2 WS-FIELD3.\n",
        "COUNT-PARA.\n",
        "    INSPECT WS-FULL-NAME\n",
        "        TALLYING WS-SPACE-COUNT FOR ALL ' '.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // All paragraph functions present
    assert!(
        rust_code.contains("fn main_para("),
        "missing main_para: {rust_code}"
    );
    assert!(
        rust_code.contains("fn format_para("),
        "missing format_para: {rust_code}"
    );
    assert!(
        rust_code.contains("fn split_para("),
        "missing split_para: {rust_code}"
    );
    assert!(
        rust_code.contains("fn count_para("),
        "missing count_para: {rust_code}"
    );

    // Balanced braces
    let open = rust_code.chars().filter(|c| *c == '{').count();
    let close = rust_code.chars().filter(|c| *c == '}').count();
    assert_eq!(
        open, close,
        "unbalanced braces: {open} open vs {close} close\n{rust_code}"
    );
}

#[test]
fn e2e_report_string_ops() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. REPORT.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-FIRST-NAME      PIC X(15) VALUE 'JOHN'.\n",
        "01  WS-LAST-NAME       PIC X(15) VALUE 'SMITH'.\n",
        "01  WS-FULL-NAME       PIC X(35).\n",
        "01  WS-INPUT-LINE      PIC X(40) VALUE 'PART1,PART2,PART3'.\n",
        "01  WS-FIELD1          PIC X(10).\n",
        "01  WS-FIELD2          PIC X(10).\n",
        "01  WS-FIELD3          PIC X(10).\n",
        "01  WS-SPACE-COUNT     PIC 9(3) VALUE 0.\n",
        "01  WS-PTR             PIC 9(3) VALUE 1.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    PERFORM FORMAT-PARA.\n",
        "    PERFORM SPLIT-PARA.\n",
        "    PERFORM COUNT-PARA.\n",
        "    DISPLAY WS-FULL-NAME.\n",
        "    STOP RUN.\n",
        "FORMAT-PARA.\n",
        "    STRING WS-FIRST-NAME DELIMITED BY ' '\n",
        "           ' ' DELIMITED BY SIZE\n",
        "           WS-LAST-NAME DELIMITED BY ' '\n",
        "        INTO WS-FULL-NAME\n",
        "        WITH POINTER WS-PTR.\n",
        "SPLIT-PARA.\n",
        "    UNSTRING WS-INPUT-LINE DELIMITED BY ','\n",
        "        INTO WS-FIELD1 WS-FIELD2 WS-FIELD3.\n",
        "COUNT-PARA.\n",
        "    INSPECT WS-FULL-NAME\n",
        "        TALLYING WS-SPACE-COUNT FOR ALL ' '.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // STRING verb
    assert!(
        rust_code.contains("cobol_string"),
        "missing cobol_string call: {rust_code}"
    );

    // UNSTRING verb
    assert!(
        rust_code.contains("cobol_unstring") || rust_code.contains("cobol_unstring_simple"),
        "missing cobol_unstring call: {rust_code}"
    );

    // INSPECT TALLYING verb
    assert!(
        rust_code.contains("cobol_inspect_tallying"),
        "missing cobol_inspect_tallying call: {rust_code}"
    );
}

// ---------------------------------------------------------------------------
// Program 3: SEARCH -- table processing
// Features: OCCURS DEPENDING ON, PERFORM UNTIL, SET UP BY, SET TO,
//           EVALUATE TRUE with 88-level conditions
// ---------------------------------------------------------------------------

#[test]
fn e2e_search_structure() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. SEARCH.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-TABLE-SIZE       PIC 9(3) VALUE 10.\n",
        "01  WS-TABLE.\n",
        "    05  WS-ENTRY OCCURS 100 TIMES\n",
        "        DEPENDING ON WS-TABLE-SIZE PIC 9(5).\n",
        "01  WS-INDEX            PIC 9(3) VALUE 1.\n",
        "01  WS-FOUND            PIC 9 VALUE 0.\n",
        "    88  IS-FOUND        VALUE 1.\n",
        "    88  NOT-FOUND       VALUE 0.\n",
        "01  WS-TARGET           PIC 9(5) VALUE 500.\n",
        "01  WS-RESULT           PIC X(10).\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    PERFORM SEARCH-PARA.\n",
        "    EVALUATE TRUE\n",
        "        WHEN IS-FOUND\n",
        "            MOVE 'YES' TO WS-RESULT\n",
        "        WHEN NOT-FOUND\n",
        "            MOVE 'NO' TO WS-RESULT\n",
        "    END-EVALUATE.\n",
        "    DISPLAY WS-RESULT.\n",
        "    STOP RUN.\n",
        "SEARCH-PARA.\n",
        "    SET WS-INDEX TO 1.\n",
        "    PERFORM UNTIL WS-INDEX > WS-TABLE-SIZE\n",
        "        IF WS-TARGET = 500\n",
        "            SET IS-FOUND TO TRUE\n",
        "        END-IF\n",
        "        SET WS-INDEX UP BY 1\n",
        "    END-PERFORM.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // CobolVarArray for OCCURS DEPENDING ON
    assert!(
        rust_code.contains("CobolVarArray"),
        "missing CobolVarArray for OCCURS DEPENDING: {rust_code}"
    );

    // Both paragraphs present
    assert!(
        rust_code.contains("fn main_para("),
        "missing main_para: {rust_code}"
    );
    assert!(
        rust_code.contains("fn search_para("),
        "missing search_para: {rust_code}"
    );

    // Balanced braces
    let open = rust_code.chars().filter(|c| *c == '{').count();
    let close = rust_code.chars().filter(|c| *c == '}').count();
    assert_eq!(
        open, close,
        "unbalanced braces: {open} open vs {close} close\n{rust_code}"
    );
}

#[test]
fn e2e_search_features() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. SEARCH.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-TABLE-SIZE       PIC 9(3) VALUE 10.\n",
        "01  WS-TABLE.\n",
        "    05  WS-ENTRY OCCURS 100 TIMES\n",
        "        DEPENDING ON WS-TABLE-SIZE PIC 9(5).\n",
        "01  WS-INDEX            PIC 9(3) VALUE 1.\n",
        "01  WS-FOUND            PIC 9 VALUE 0.\n",
        "    88  IS-FOUND        VALUE 1.\n",
        "    88  NOT-FOUND       VALUE 0.\n",
        "01  WS-TARGET           PIC 9(5) VALUE 500.\n",
        "01  WS-RESULT           PIC X(10).\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    PERFORM SEARCH-PARA.\n",
        "    EVALUATE TRUE\n",
        "        WHEN IS-FOUND\n",
        "            MOVE 'YES' TO WS-RESULT\n",
        "        WHEN NOT-FOUND\n",
        "            MOVE 'NO' TO WS-RESULT\n",
        "    END-EVALUATE.\n",
        "    DISPLAY WS-RESULT.\n",
        "    STOP RUN.\n",
        "SEARCH-PARA.\n",
        "    SET WS-INDEX TO 1.\n",
        "    PERFORM UNTIL WS-INDEX > WS-TABLE-SIZE\n",
        "        IF WS-TARGET = 500\n",
        "            SET IS-FOUND TO TRUE\n",
        "        END-IF\n",
        "        SET WS-INDEX UP BY 1\n",
        "    END-PERFORM.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // PERFORM UNTIL loop
    assert!(
        rust_code.contains("while !("),
        "missing PERFORM UNTIL while loop: {rust_code}"
    );

    // SET UP BY -> cobol_add
    assert!(
        rust_code.contains("cobol_add"),
        "missing SET UP BY (cobol_add): {rust_code}"
    );

    // EVALUATE TRUE pattern with 88-level condition references
    assert!(
        rust_code.contains("ws.ws_found"),
        "missing ws_found reference in EVALUATE TRUE: {rust_code}"
    );
}

// ---------------------------------------------------------------------------
// Program 4: SUBPROG -- inter-program communication
// Features: CALL with mixed passing modes (BY REF/CONTENT/VALUE),
//           ON EXCEPTION, RETURNING, CANCEL
// ---------------------------------------------------------------------------

#[test]
fn e2e_subprog_structure() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. SUBPROG.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-PARAM-A          PIC X(10) VALUE 'INPUT'.\n",
        "01  WS-PARAM-B          PIC 9(5) VALUE 100.\n",
        "01  WS-RESULT           PIC 9(5) VALUE 0.\n",
        "01  WS-RC               PIC 9(3) VALUE 0.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    CALL 'PROCESSOR' USING\n",
        "        BY REFERENCE WS-PARAM-A\n",
        "        BY CONTENT WS-PARAM-B\n",
        "        BY VALUE WS-PARAM-B\n",
        "        ON EXCEPTION\n",
        "            DISPLAY 'CALL FAILED'\n",
        "        NOT ON EXCEPTION\n",
        "            DISPLAY 'CALL OK'\n",
        "    END-CALL.\n",
        "    CALL 'UTILITY' USING WS-PARAM-A\n",
        "        RETURNING WS-RC.\n",
        "    CANCEL 'PROCESSOR'.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // Two cobol_call invocations
    let call_count = rust_code.matches("cobol_call").count();
    assert!(
        call_count >= 2,
        "expected at least 2 cobol_call invocations, got {call_count}: {rust_code}"
    );

    // cobol_cancel
    assert!(
        rust_code.contains("cobol_cancel"),
        "missing cobol_cancel: {rust_code}"
    );
}

#[test]
fn e2e_subprog_features() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. SUBPROG.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-PARAM-A          PIC X(10) VALUE 'INPUT'.\n",
        "01  WS-PARAM-B          PIC 9(5) VALUE 100.\n",
        "01  WS-RESULT           PIC 9(5) VALUE 0.\n",
        "01  WS-RC               PIC 9(3) VALUE 0.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    CALL 'PROCESSOR' USING\n",
        "        BY REFERENCE WS-PARAM-A\n",
        "        BY CONTENT WS-PARAM-B\n",
        "        BY VALUE WS-PARAM-B\n",
        "        ON EXCEPTION\n",
        "            DISPLAY 'CALL FAILED'\n",
        "        NOT ON EXCEPTION\n",
        "            DISPLAY 'CALL OK'\n",
        "    END-CALL.\n",
        "    CALL 'UTILITY' USING WS-PARAM-A\n",
        "        RETURNING WS-RC.\n",
        "    CANCEL 'PROCESSOR'.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // Mixed passing modes
    assert!(
        rust_code.contains("call_param_by_ref"),
        "missing call_param_by_ref: {rust_code}"
    );
    assert!(
        rust_code.contains("call_param_by_content"),
        "missing call_param_by_content: {rust_code}"
    );
    assert!(
        rust_code.contains("call_param_by_value"),
        "missing call_param_by_value: {rust_code}"
    );

    // Ok/Err exception pattern
    assert!(
        rust_code.contains("Ok(rc)") || rust_code.contains("Ok(_"),
        "missing Ok arm for NOT ON EXCEPTION: {rust_code}"
    );
    assert!(
        rust_code.contains("Err(_e)") || rust_code.contains("Err(_"),
        "missing Err arm for ON EXCEPTION: {rust_code}"
    );

    // CANCEL
    assert!(
        rust_code.contains("cobol_cancel"),
        "missing cobol_cancel: {rust_code}"
    );
}

// ---------------------------------------------------------------------------
// Program 5: CONTROL -- control flow stress test
// Features: GO TO DEPENDING ON, PERFORM THRU, EXIT PARAGRAPH, forward GO TO,
//           paragraph fall-through
// ---------------------------------------------------------------------------

#[test]
fn e2e_control_structure() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. CONTROL.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-BRANCH           PIC 9 VALUE 2.\n",
        "01  WS-FLAG             PIC 9 VALUE 0.\n",
        "01  WS-COUNTER          PIC 9(3) VALUE 0.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    PERFORM INIT-PARA THRU INIT-EXIT.\n",
        "    GO TO BRANCH-PARA SKIP-PARA DONE-PARA\n",
        "        DEPENDING ON WS-BRANCH.\n",
        "    DISPLAY 'FALLTHROUGH'.\n",
        "BRANCH-PARA.\n",
        "    IF WS-FLAG = 1\n",
        "        GO TO DONE-PARA\n",
        "    END-IF.\n",
        "    DISPLAY 'BRANCH'.\n",
        "SKIP-PARA.\n",
        "    DISPLAY 'SKIP'.\n",
        "DONE-PARA.\n",
        "    DISPLAY 'DONE'.\n",
        "    STOP RUN.\n",
        "INIT-PARA.\n",
        "    ADD 1 TO WS-COUNTER.\n",
        "    IF WS-COUNTER > 5\n",
        "        EXIT PARAGRAPH\n",
        "    END-IF.\n",
        "    DISPLAY 'INIT'.\n",
        "INIT-EXIT.\n",
        "    EXIT.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // All 6 paragraphs in dispatch loop
    assert!(
        rust_code.contains("fn main_para("),
        "missing main_para: {rust_code}"
    );
    assert!(
        rust_code.contains("fn branch_para("),
        "missing branch_para: {rust_code}"
    );
    assert!(
        rust_code.contains("fn skip_para("),
        "missing skip_para: {rust_code}"
    );
    assert!(
        rust_code.contains("fn done_para("),
        "missing done_para: {rust_code}"
    );
    assert!(
        rust_code.contains("fn init_para("),
        "missing init_para: {rust_code}"
    );
    assert!(
        rust_code.contains("fn init_exit("),
        "missing init_exit: {rust_code}"
    );

    // Balanced braces
    let open = rust_code.chars().filter(|c| *c == '{').count();
    let close = rust_code.chars().filter(|c| *c == '}').count();
    assert_eq!(
        open, close,
        "unbalanced braces: {open} open vs {close} close\n{rust_code}"
    );
}

#[test]
fn e2e_control_features() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. CONTROL.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-BRANCH           PIC 9 VALUE 2.\n",
        "01  WS-FLAG             PIC 9 VALUE 0.\n",
        "01  WS-COUNTER          PIC 9(3) VALUE 0.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    PERFORM INIT-PARA THRU INIT-EXIT.\n",
        "    GO TO BRANCH-PARA SKIP-PARA DONE-PARA\n",
        "        DEPENDING ON WS-BRANCH.\n",
        "    DISPLAY 'FALLTHROUGH'.\n",
        "BRANCH-PARA.\n",
        "    IF WS-FLAG = 1\n",
        "        GO TO DONE-PARA\n",
        "    END-IF.\n",
        "    DISPLAY 'BRANCH'.\n",
        "SKIP-PARA.\n",
        "    DISPLAY 'SKIP'.\n",
        "DONE-PARA.\n",
        "    DISPLAY 'DONE'.\n",
        "    STOP RUN.\n",
        "INIT-PARA.\n",
        "    ADD 1 TO WS-COUNTER.\n",
        "    IF WS-COUNTER > 5\n",
        "        EXIT PARAGRAPH\n",
        "    END-IF.\n",
        "    DISPLAY 'INIT'.\n",
        "INIT-EXIT.\n",
        "    EXIT.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // GO TO DEPENDING ON (match _goto_idx)
    assert!(
        rust_code.contains("match _goto_idx"),
        "missing match _goto_idx for GO TO DEPENDING: {rust_code}"
    );

    // PERFORM THRU (_perf_pc)
    assert!(
        rust_code.contains("_perf_pc"),
        "missing _perf_pc for PERFORM THRU: {rust_code}"
    );

    // EXIT PARAGRAPH (return;)
    assert!(
        rust_code.contains("return;"),
        "missing return for EXIT PARAGRAPH: {rust_code}"
    );
}

// ---------------------------------------------------------------------------
// Program 6: COPYBOOK -- COPY REPLACING integration
// Features: COPY with pseudo-text REPLACING, copybook-defined fields used in
//           MOVE/ADD/DISPLAY
// ---------------------------------------------------------------------------

#[test]
fn e2e_copybook_structure() {
    use cobol_transpiler::transpile::{transpile_with_config, TranspileConfig};
    use std::collections::HashMap;
    use std::path::PathBuf;

    let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
    let copybook_path = temp_dir.path().join("COMMON.cpy");
    std::fs::write(
        &copybook_path,
        concat!(
            "01  :PREFIX:-RECORD.\n",
            "    05  :PREFIX:-NAME   PIC X(20).\n",
            "    05  :PREFIX:-AMOUNT PIC 9(5)V99.\n",
            "    05  :PREFIX:-FLAG   PIC X VALUE 'N'.\n",
        ),
    )
    .expect("failed to write copybook");

    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. COPYTEST.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "COPY COMMON REPLACING ==:PREFIX:== BY ==WS==.\n",
        "01  WS-TOTAL            PIC 9(7)V99 VALUE 0.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    MOVE 'SMITH' TO WS-NAME.\n",
        "    MOVE 100 TO WS-AMOUNT.\n",
        "    ADD WS-AMOUNT TO WS-TOTAL.\n",
        "    DISPLAY WS-NAME.\n",
        "    DISPLAY WS-TOTAL.\n",
        "    STOP RUN.\n",
    );

    let config = TranspileConfig {
        copybook_paths: vec![temp_dir.path().to_path_buf()],
        library_map: HashMap::new(),
        max_copy_depth: 10,
    };

    let rust_code = transpile_with_config(cobol, &config).expect("transpile_with_config failed");

    // Expanded copybook fields (after REPLACING :PREFIX: -> WS)
    assert!(
        rust_code.contains("ws_record_ws_name") || rust_code.contains("ws_name"),
        "missing WS-NAME field from copybook: {rust_code}"
    );
    assert!(
        rust_code.contains("ws_record_ws_amount") || rust_code.contains("ws_amount"),
        "missing WS-AMOUNT field from copybook: {rust_code}"
    );
    assert!(
        rust_code.contains("ws_total"),
        "missing WS-TOTAL field: {rust_code}"
    );
}

#[test]
fn e2e_copybook_features() {
    use cobol_transpiler::transpile::{transpile_with_config, TranspileConfig};
    use std::collections::HashMap;
    use std::path::PathBuf;

    let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
    let copybook_path = temp_dir.path().join("COMMON.cpy");
    std::fs::write(
        &copybook_path,
        concat!(
            "01  :PREFIX:-RECORD.\n",
            "    05  :PREFIX:-NAME   PIC X(20).\n",
            "    05  :PREFIX:-AMOUNT PIC 9(5)V99.\n",
            "    05  :PREFIX:-FLAG   PIC X VALUE 'N'.\n",
        ),
    )
    .expect("failed to write copybook");

    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. COPYTEST.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "COPY COMMON REPLACING ==:PREFIX:== BY ==WS==.\n",
        "01  WS-TOTAL            PIC 9(7)V99 VALUE 0.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    MOVE 'SMITH' TO WS-NAME.\n",
        "    MOVE 100 TO WS-AMOUNT.\n",
        "    ADD WS-AMOUNT TO WS-TOTAL.\n",
        "    DISPLAY WS-NAME.\n",
        "    DISPLAY WS-TOTAL.\n",
        "    STOP RUN.\n",
    );

    let config = TranspileConfig {
        copybook_paths: vec![temp_dir.path().to_path_buf()],
        library_map: HashMap::new(),
        max_copy_depth: 10,
    };

    let rust_code = transpile_with_config(cobol, &config).expect("transpile_with_config failed");

    // MOVE and ADD should be present
    assert!(
        rust_code.contains("cobol_move"),
        "missing cobol_move: {rust_code}"
    );
    assert!(
        rust_code.contains("cobol_add"),
        "missing cobol_add: {rust_code}"
    );

    // No unexpanded :PREFIX: should remain
    assert!(
        !rust_code.contains(":PREFIX:"),
        "unexpanded :PREFIX: found in output: {rust_code}"
    );
}

// ---------------------------------------------------------------------------
// Regression Tests
// ---------------------------------------------------------------------------

#[test]
fn e2e_regression_nested_if_with_88_level() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. REGR1.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-TYPE PIC X VALUE 'A'.\n",
        "    88  IS-TYPE-A VALUE 'A'.\n",
        "    88  IS-TYPE-B VALUE 'B'.\n",
        "01  WS-STATUS PIC 9 VALUE 1.\n",
        "    88  IS-ACTIVE VALUE 1.\n",
        "01  WS-COUNT PIC 9(3) VALUE 0.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    PERFORM CHECK-PARA 3 TIMES.\n",
        "    STOP RUN.\n",
        "CHECK-PARA.\n",
        "    IF IS-TYPE-A\n",
        "        IF IS-ACTIVE\n",
        "            ADD 1 TO WS-COUNT\n",
        "        END-IF\n",
        "    ELSE\n",
        "        IF IS-TYPE-B\n",
        "            ADD 2 TO WS-COUNT\n",
        "        END-IF\n",
        "    END-IF.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // Balanced braces -- critical for nested IF
    let open = rust_code.chars().filter(|c| *c == '{').count();
    let close = rust_code.chars().filter(|c| *c == '}').count();
    assert_eq!(
        open, close,
        "unbalanced braces in nested IF with 88-level: {open} open vs {close} close\n{rust_code}"
    );

    // 88-level conditions referenced
    assert!(
        rust_code.contains("ws.ws_type"),
        "missing 88-level parent ws_type: {rust_code}"
    );
    assert!(
        rust_code.contains("ws.ws_status"),
        "missing 88-level parent ws_status: {rust_code}"
    );
}

#[test]
fn e2e_regression_ref_mod_in_move() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. REGR2.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-SOURCE PIC X(20) VALUE 'HELLO WORLD'.\n",
        "01  WS-DEST   PIC X(5).\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    MOVE WS-SOURCE(1:5) TO WS-DEST.\n",
        "    DISPLAY WS-DEST.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // Reference modification should generate ref_mod_read
    assert!(
        rust_code.contains("ref_mod_read") || rust_code.contains("ref_mod"),
        "missing ref_mod_read for reference modification: {rust_code}"
    );
}

#[test]
fn e2e_regression_compute_with_parens() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. REGR3.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-A      PIC 9(5) VALUE 10.\n",
        "01  WS-B      PIC 9(5) VALUE 20.\n",
        "01  WS-C      PIC 9(5) VALUE 3.\n",
        "01  WS-RESULT PIC 9(7).\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    COMPUTE WS-RESULT = (WS-A + WS-B) * WS-C.\n",
        "    DISPLAY WS-RESULT.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // COMPUTE with parenthesized expression
    assert!(
        rust_code.contains("cobol_compute"),
        "missing cobol_compute: {rust_code}"
    );

    // Parentheses should be preserved in the arithmetic expression
    assert!(
        rust_code.contains("(") && rust_code.contains(")"),
        "missing parentheses in compute expression: {rust_code}"
    );
}

// ---------------------------------------------------------------------------
// Test: Group child fields use unprefixed names matching proc_gen references
// ---------------------------------------------------------------------------
#[test]
fn e2e_group_child_field_names_match() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. GRPTEST.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-RECORD.\n",
        "    05  WS-NAME PIC X(20).\n",
        "    05  WS-AGE PIC 9(3).\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    DISPLAY WS-NAME.\n",
        "    DISPLAY WS-AGE.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // Struct fields should use child's own name (no group prefix)
    assert!(
        rust_code.contains("pub ws_name: PicX"),
        "field should be ws_name not ws_record_ws_name: {rust_code}"
    );
    assert!(
        rust_code.contains("pub ws_age: PackedDecimal"),
        "field should be ws_age not ws_record_ws_age: {rust_code}"
    );

    // Procedure division references should match struct field names
    assert!(
        rust_code.contains("ws.ws_name"),
        "proc_gen should reference ws.ws_name: {rust_code}"
    );
    assert!(
        rust_code.contains("ws.ws_age"),
        "proc_gen should reference ws.ws_age: {rust_code}"
    );
}

// ---------------------------------------------------------------------------
// Test: COMPUTE uses .to_decimal() for field operands (not raw operators)
// ---------------------------------------------------------------------------
#[test]
fn e2e_compute_uses_decimal_conversion() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. COMPFIX.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-A PIC 9(5) VALUE 10.\n",
        "01  WS-B PIC 9(5) VALUE 20.\n",
        "01  WS-RESULT PIC 9(7).\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    COMPUTE WS-RESULT = WS-A * WS-B.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // Operands should use .to_decimal() for Decimal arithmetic
    assert!(
        rust_code.contains(".to_decimal()"),
        "COMPUTE operands should call .to_decimal(): {rust_code}"
    );
    // Should still call cobol_compute
    assert!(
        rust_code.contains("cobol_compute"),
        "should generate cobol_compute call: {rust_code}"
    );
    // Should NOT have raw ws.ws_a * ws.ws_b (without .to_decimal())
    assert!(
        !rust_code.contains("ws.ws_a *") || rust_code.contains("ws.ws_a.to_decimal()"),
        "should not use raw operators on PackedDecimal: {rust_code}"
    );
}

// ---------------------------------------------------------------------------
// Test: COMPUTE with literal operands uses dec!() (not PackedDecimal wrapper)
// ---------------------------------------------------------------------------
#[test]
fn e2e_compute_literal_uses_decimal_macro() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. COMPFIX2.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-A PIC 9(5) VALUE 10.\n",
        "01  WS-RESULT PIC 9(7).\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    COMPUTE WS-RESULT = WS-A + 5.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // Literal 5 in COMPUTE should become dec!(5), not a PackedDecimal temp
    assert!(
        rust_code.contains("dec!(5)"),
        "literal in COMPUTE should use dec!(): {rust_code}"
    );
    // Field should use .to_decimal()
    assert!(
        rust_code.contains("ws.ws_a.to_decimal()"),
        "field in COMPUTE should call .to_decimal(): {rust_code}"
    );
}

// ---------------------------------------------------------------------------
// Test: Nested group children also use unprefixed names
// ---------------------------------------------------------------------------
#[test]
fn e2e_nested_group_field_names() {
    let cobol = concat!(
        "IDENTIFICATION DIVISION.\n",
        "PROGRAM-ID. NESTGRP.\n",
        "DATA DIVISION.\n",
        "WORKING-STORAGE SECTION.\n",
        "01  WS-OUTER.\n",
        "    05  WS-INNER.\n",
        "        10  WS-VALUE PIC 9(5) VALUE 42.\n",
        "PROCEDURE DIVISION.\n",
        "MAIN-PARA.\n",
        "    DISPLAY WS-VALUE.\n",
        "    STOP RUN.\n",
    );

    let rust_code = transpile(cobol).expect("transpile failed");

    // Deeply nested field should use its own name
    assert!(
        rust_code.contains("pub ws_value: PackedDecimal"),
        "nested field should be ws_value: {rust_code}"
    );
    // Reference in procedure division should match
    assert!(
        rust_code.contains("ws.ws_value"),
        "proc reference should be ws.ws_value: {rust_code}"
    );
}
