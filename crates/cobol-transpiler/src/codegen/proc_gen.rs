//! Procedure division code generator.
//!
//! Generates Rust functions from COBOL PROCEDURE DIVISION statements.
//! Each paragraph becomes a function. Statements map to runtime API calls.

use std::collections::{HashMap, HashSet};

use crate::ast::{ConditionValue, DataEntry, PicClause, PicCategory, ProcedureDivision, Paragraph, Statement, MoveStatement, Operand, Literal, FigurativeConstant, DisplayStatement, AddStatement, SubtractStatement, MultiplyStatement, DivideStatement, DivideDirection, ComputeStatement, IfStatement, EvaluateStatement, EvaluateSubject, WhenValue, PerformStatement, PerformLoopType, GoToStatement, InitializeStatement, InitializeCategory, InitializeReplacing, CallStatement, PassingMode, CancelStatement, AcceptStatement, AcceptSource, OpenStatement, OpenMode, CloseStatement, ReadStatement, WriteStatement, Advancing, RewriteStatement, DeleteStatement, StartStatement, ComparisonOp, SetStatement, SetAction, DataReference, Subscript, ArithExpr, ArithOp, Condition, ClassCondition, SignCondition, SortStatement, SortInput, SortOutput, MergeStatement, ReleaseStatement, ReturnStatement, InspectStatement, InspectWhat, StringStatement, StringDelimiter, UnstringStatement, FunctionCall, ExecSqlStatement, SqlStatementType};
use crate::codegen::data_gen::cobol_to_rust_name;
use crate::codegen::rust_writer::RustWriter;

/// Maps record names (uppercase) to their parent file names (uppercase).
/// Built from `FileDescription` entries so WRITE can find the correct file handle.
pub type RecordFileMap = HashMap<String, String>;

/// Maps SD record field names (uppercase) to (byte_offset, byte_length).
/// Used by SORT/MERGE codegen to emit correct `SortKeySpec::new(...)`.
pub type SortFieldMap = HashMap<String, (usize, usize)>;

/// Maps group name (uppercase) to list of (child_cobol_name, is_numeric) pairs.
/// Used by MOVE CORRESPONDING codegen to expand into individual MOVEs.
pub type GroupChildMap = HashMap<String, Vec<(String, bool)>>;

/// Build a GroupChildMap from working-storage records and file-section records.
fn build_group_child_map(ws_records: &[DataEntry], file_records: &[DataEntry]) -> GroupChildMap {
    let mut map = GroupChildMap::new();
    for record in ws_records.iter().chain(file_records.iter()) {
        collect_group_children(record, &mut map);
    }
    map
}

/// Recursively collect group -> children mappings.
fn collect_group_children(entry: &DataEntry, map: &mut GroupChildMap) {
    if entry.level == 88 || entry.level == 66 {
        return;
    }
    if !entry.children.is_empty() {
        let group_name = entry.name.to_uppercase();
        let mut children = Vec::new();
        for child in &entry.children {
            if child.level == 88 || child.level == 66 {
                continue;
            }
            let child_name = child.name.to_uppercase();
            if child_name == "FILLER" || child_name.is_empty() {
                continue;
            }
            let is_numeric = child.pic.as_ref().is_some_and(|p| {
                matches!(p.category, PicCategory::Numeric | PicCategory::NumericEdited)
            });
            children.push((child.name.clone(), is_numeric));
            // Recurse into sub-groups
            collect_group_children(child, map);
        }
        if !children.is_empty() {
            map.insert(group_name, children);
        }
    }
}

/// Names of sections/paragraphs used as INPUT or OUTPUT PROCEDURE in SORT/MERGE.
#[derive(Debug, Default)]
struct SortProcedureNames {
    /// Section/para names used as INPUT PROCEDURE (need `releaser: &mut Releaser`).
    input: HashSet<String>,
    /// Section/para names used as OUTPUT PROCEDURE (need `returner: &mut Returner`).
    output: HashSet<String>,
}

/// Collect all section/paragraph names used as INPUT/OUTPUT PROCEDURE from SORT/MERGE.
fn collect_sort_procedure_names(proc_div: &ProcedureDivision) -> SortProcedureNames {
    let mut names = SortProcedureNames::default();
    fn walk_stmts(stmts: &[Statement], names: &mut SortProcedureNames) {
        for stmt in stmts {
            match stmt {
                Statement::Sort(s) => {
                    if let SortInput::InputProcedure { name, .. } = &s.input {
                        names.input.insert(name.to_uppercase());
                    }
                    if let SortOutput::OutputProcedure { name, .. } = &s.output {
                        names.output.insert(name.to_uppercase());
                    }
                }
                Statement::Merge(m) => {
                    if let SortOutput::OutputProcedure { name, .. } = &m.output {
                        names.output.insert(name.to_uppercase());
                    }
                }
                Statement::If(i) => {
                    walk_stmts(&i.then_body, names);
                    walk_stmts(&i.else_body, names);
                }
                Statement::Evaluate(e) => {
                    for branch in &e.when_branches {
                        walk_stmts(&branch.body, names);
                    }
                    walk_stmts(&e.when_other, names);
                }
                Statement::Perform(p) => {
                    walk_stmts(&p.body, names);
                }
                _ => {}
            }
        }
    }
    for para in &proc_div.paragraphs {
        for sentence in &para.sentences {
            walk_stmts(&sentence.statements, &mut names);
        }
    }
    for section in &proc_div.sections {
        for para in &section.paragraphs {
            for sentence in &para.sentences {
                walk_stmts(&sentence.statements, &mut names);
            }
        }
    }
    names
}

/// Information about an 88-level condition for codegen.
#[derive(Debug, Clone)]
pub struct ConditionInfo {
    /// Rust expression for the parent field (e.g., "`ws.ws_status`").
    pub parent_field: String,
    /// Whether the parent field is numeric (affects comparison codegen).
    pub parent_is_numeric: bool,
    /// The condition values (Single or Range).
    pub values: Vec<ConditionValue>,
}

/// Map from condition name (uppercase COBOL name) to its info.
pub type ConditionMap = HashMap<String, ConditionInfo>;

/// Build a `ConditionMap` by walking the `DataEntry` hierarchy.
///
/// 88-level entries are children of their parent field. We walk the tree,
/// and for each 88-level child, record its parent's Rust field path and
/// the condition values.
pub fn build_condition_map(records: &[DataEntry]) -> ConditionMap {
    let mut map = ConditionMap::new();
    for record in records {
        collect_conditions(&mut map, record, "");
    }
    map
}

fn collect_conditions(map: &mut ConditionMap, entry: &DataEntry, prefix: &str) {
    let field_name = cobol_to_rust_name(&entry.name, prefix);
    let rust_path = format!("ws.{field_name}");

    // Check if this entry's children include 88-level conditions
    let is_numeric = entry
        .pic
        .as_ref()
        .is_some_and(is_numeric_pic);

    for child in &entry.children {
        if child.level == 88 && !child.condition_values.is_empty() {
            map.insert(
                child.name.to_uppercase(),
                ConditionInfo {
                    parent_field: rust_path.clone(),
                    parent_is_numeric: is_numeric,
                    values: child.condition_values.clone(),
                },
            );
        } else if child.level != 88 && child.level != 66 {
            // Recurse into non-condition children
            collect_conditions(map, child, prefix);
        }
    }
}

/// Check if a PIC clause represents a numeric field.
fn is_numeric_pic(pic: &PicClause) -> bool {
    matches!(pic.category, PicCategory::Numeric | PicCategory::NumericEdited)
}

/// Entry in the paragraph dispatch table for the program counter loop.
struct ParagraphIndex {
    /// Original COBOL paragraph name (uppercase).
    name: String,
    /// Rust function name (`snake_case`).
    rust_name: String,
    /// Dispatch index in the `run()` match.
    index: usize,
}

/// Generate all procedure division functions.
pub fn generate_procedure_division(
    w: &mut RustWriter,
    proc_div: &ProcedureDivision,
    cmap: &ConditionMap,
    record_file_map: &RecordFileMap,
    sort_field_map: &SortFieldMap,
    ws_records: &[DataEntry],
    file_section_records: &[DataEntry],
    has_sql: bool,
) {
    let gcm = build_group_child_map(ws_records, file_section_records);
    // Collect section/paragraph names used as INPUT/OUTPUT PROCEDURE (needed early for dispatch)
    let spn = collect_sort_procedure_names(proc_div);

    // Build set of paragraph names belonging to INPUT/OUTPUT PROCEDURE sections
    let mut proc_para_names: HashSet<String> = HashSet::new();
    for section in &proc_div.sections {
        let section_upper = section.name.to_uppercase();
        if spn.input.contains(&section_upper) || spn.output.contains(&section_upper) {
            for para in &section.paragraphs {
                proc_para_names.insert(para.name.to_uppercase());
            }
        }
    }

    // Build flat paragraph index table (skip INPUT/OUTPUT PROCEDURE paragraphs)
    let mut para_table: Vec<ParagraphIndex> = Vec::new();
    for para in &proc_div.paragraphs {
        let idx = para_table.len();
        para_table.push(ParagraphIndex {
            name: para.name.to_uppercase(),
            rust_name: cobol_to_rust_name(&para.name, ""),
            index: idx,
        });
    }
    for section in &proc_div.sections {
        let section_upper = section.name.to_uppercase();
        // Skip sections that are INPUT/OUTPUT PROCEDURE -- they're called from SORT closures
        if spn.input.contains(&section_upper) || spn.output.contains(&section_upper) {
            continue;
        }
        for para in &section.paragraphs {
            let idx = para_table.len();
            para_table.push(ParagraphIndex {
                name: para.name.to_uppercase(),
                rust_name: cobol_to_rust_name(&para.name, ""),
                index: idx,
            });
        }
    }

    // Generate run() with program counter dispatch loop
    w.line("/// Execute the COBOL program.");
    if has_sql {
        w.open_block("pub fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext, sql: &mut dyn CobolSqlRuntime) {");
    } else {
        w.open_block("pub fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {");
    }

    if para_table.is_empty() {
        w.close_block("}");
        w.blank_line();
        return;
    }

    w.line("let mut _pc: usize = 0;");
    w.open_block("loop {");

    // Dispatch match
    w.open_block("match _pc {");
    for pi in &para_table {
        if has_sql {
            w.line(&format!("{} => {}(ws, ctx, sql),", pi.index, pi.rust_name));
        } else {
            w.line(&format!("{} => {}(ws, ctx),", pi.index, pi.rust_name));
        }
    }
    w.line("_ => break,");
    w.close_block("}");

    // Control flow checks after each paragraph
    w.line("if ctx.stopped || ctx.exit_program { break; }");
    w.open_block("if let Some(target) = ctx.goto_target.take() {");
    w.open_block("_pc = match target.as_str() {");
    for pi in &para_table {
        w.line(&format!("\"{}\" => {},", pi.name, pi.index));
    }
    w.line("_ => break,");
    w.close_block("};");
    w.line("continue;");
    w.close_block("}");
    w.line("_pc += 1;");

    w.close_block("}"); // loop
    w.close_block("}"); // fn run
    w.blank_line();

    // Generate paragraph functions (outside sections)
    for para in &proc_div.paragraphs {
        generate_paragraph_fn(w, para, cmap, &para_table, record_file_map, sort_field_map, &spn, None, &gcm, has_sql);
    }

    // Generate section paragraphs and section wrapper functions
    for section in &proc_div.sections {
        w.line(&format!(
            "// --- Section: {} ---",
            section.name
        ));
        w.blank_line();

        let section_upper = section.name.to_uppercase();
        let section_is_input = spn.input.contains(&section_upper);
        let section_is_output = spn.output.contains(&section_upper);

        // Determine extra param for paragraphs in this section
        let extra_param = if section_is_input {
            Some("releaser")
        } else if section_is_output {
            Some("returner")
        } else {
            None
        };

        // Generate individual paragraph functions
        for para in &section.paragraphs {
            generate_paragraph_fn(w, para, cmap, &para_table, record_file_map, sort_field_map, &spn, extra_param, &gcm, has_sql);
        }

        // Generate section-level wrapper function that calls all paragraphs
        if !section.paragraphs.is_empty() {
            let section_fn = cobol_to_rust_name(&section.name, "");
            w.line("#[allow(non_snake_case, unused_variables)]");
            if section_is_input {
                w.open_block(&format!(
                    "fn {section_fn}(ws: &mut WorkingStorage, ctx: &mut ProgramContext, releaser: &mut Releaser) {{"
                ));
            } else if section_is_output {
                w.open_block(&format!(
                    "fn {section_fn}(ws: &mut WorkingStorage, ctx: &mut ProgramContext, returner: &mut Returner) {{"
                ));
            } else if has_sql {
                w.open_block(&format!(
                    "fn {section_fn}(ws: &mut WorkingStorage, ctx: &mut ProgramContext, sql: &mut dyn CobolSqlRuntime) {{"
                ));
            } else {
                w.open_block(&format!(
                    "fn {section_fn}(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {{"
                ));
            }
            for para in &section.paragraphs {
                let para_fn = cobol_to_rust_name(&para.name, "");
                if section_is_input {
                    w.line(&format!("{para_fn}(ws, ctx, releaser);"));
                } else if section_is_output {
                    w.line(&format!("{para_fn}(ws, ctx, returner);"));
                } else if has_sql {
                    w.line(&format!("{para_fn}(ws, ctx, sql);"));
                } else {
                    w.line(&format!("{para_fn}(ws, ctx);"));
                }
                w.line("if ctx.stopped || ctx.exit_program || ctx.goto_target.is_some() { return; }");
            }
            w.close_block("}");
            w.blank_line();
        }
    }
}

/// Generate a Rust function for a single paragraph.
///
/// `extra_param`: If `Some("releaser")`, adds `releaser: &mut Releaser` param.
///                If `Some("returner")`, adds `returner: &mut Returner` param.
fn generate_paragraph_fn(w: &mut RustWriter, para: &Paragraph, cmap: &ConditionMap, ptable: &[ParagraphIndex], rfm: &RecordFileMap, sfm: &SortFieldMap, _spn: &SortProcedureNames, extra_param: Option<&str>, gcm: &GroupChildMap, has_sql: bool) {
    let fn_name = cobol_to_rust_name(&para.name, "");
    w.line("#[allow(non_snake_case, unused_variables)]");
    match extra_param {
        Some("releaser") => w.open_block(&format!(
            "fn {fn_name}(ws: &mut WorkingStorage, ctx: &mut ProgramContext, releaser: &mut Releaser) {{"
        )),
        Some("returner") => w.open_block(&format!(
            "fn {fn_name}(ws: &mut WorkingStorage, ctx: &mut ProgramContext, returner: &mut Returner) {{"
        )),
        _ => {
            if has_sql {
                w.open_block(&format!(
                    "fn {fn_name}(ws: &mut WorkingStorage, ctx: &mut ProgramContext, sql: &mut dyn CobolSqlRuntime) {{"
                ));
            } else {
                w.open_block(&format!(
                    "fn {fn_name}(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {{"
                ));
            }
        }
    }

    for sentence in &para.sentences {
        for stmt in &sentence.statements {
            generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
        }
    }

    w.close_block("}");
    w.blank_line();
}

/// Generate Rust code for a single statement.
fn generate_statement(w: &mut RustWriter, stmt: &Statement, cmap: &ConditionMap, ptable: &[ParagraphIndex], rfm: &RecordFileMap, sfm: &SortFieldMap, gcm: &GroupChildMap, has_sql: bool) {
    match stmt {
        Statement::Move(m) => generate_move(w, m, gcm),
        Statement::Display(d) => generate_display(w, d),
        Statement::Add(a) => generate_add(w, a),
        Statement::Subtract(s) => generate_subtract(w, s),
        Statement::Multiply(m) => generate_multiply(w, m),
        Statement::Divide(d) => generate_divide(w, d),
        Statement::Compute(c) => generate_compute(w, c),
        Statement::If(i) => generate_if(w, i, cmap, ptable, rfm, sfm, gcm, has_sql),
        Statement::Evaluate(e) => generate_evaluate(w, e, cmap, ptable, rfm, sfm, gcm, has_sql),
        Statement::Perform(p) => generate_perform(w, p, cmap, ptable, rfm, sfm, gcm, has_sql),
        Statement::GoTo(g) => generate_goto(w, g),
        Statement::StopRun => {
            w.line("ctx.stop_run();");
            w.line("return;");
        }
        Statement::GoBack => {
            w.line("ctx.goback();");
            w.line("return;");
        }
        Statement::Continue => w.line("// CONTINUE"),
        Statement::NextSentence => w.line("// NEXT SENTENCE"),
        Statement::ExitProgram => {
            w.line("ctx.exit_program = true;");
            w.line("return;");
        }
        Statement::ExitParagraph | Statement::ExitSection => w.line("return;"),
        Statement::Initialize(init) => generate_initialize(w, init, gcm),
        Statement::Call(call) => generate_call(w, call, cmap, ptable, rfm, sfm, gcm, has_sql),
        Statement::Cancel(cancel) => generate_cancel(w, cancel),
        Statement::Accept(acc) => generate_accept(w, acc),
        Statement::Open(open) => generate_open(w, open),
        Statement::Close(close) => generate_close(w, close),
        Statement::Read(read) => generate_read(w, read, cmap, ptable, rfm, sfm, gcm, has_sql),
        Statement::Write(write) => generate_write(w, write, cmap, ptable, rfm, sfm, gcm, has_sql),
        Statement::Rewrite(rw) => generate_rewrite(w, rw, cmap, ptable, rfm, sfm, gcm, has_sql),
        Statement::Delete(del) => generate_delete(w, del, cmap, ptable, rfm, sfm, gcm, has_sql),
        Statement::Sort(sort) => generate_sort(w, sort, cmap, sfm),
        Statement::Merge(merge) => generate_merge(w, merge, cmap, sfm),
        Statement::Release(rel) => generate_release(w, rel),
        Statement::Return(ret) => generate_return(w, ret, cmap, ptable, rfm, sfm, gcm, has_sql),
        Statement::Inspect(insp) => generate_inspect(w, insp, cmap),
        Statement::String(s) => generate_string(w, s, cmap, ptable, rfm, sfm, gcm, has_sql),
        Statement::Unstring(u) => generate_unstring(w, u, cmap, ptable, rfm, sfm, gcm, has_sql),
        Statement::Set(set) => generate_set(w, set, cmap),
        Statement::Start(start) => generate_start(w, start, cmap, ptable, rfm, sfm, gcm, has_sql),
        Statement::Alter(_) => {
            w.line(&format!("// TODO: unsupported statement: {stmt:?}"));
        }
        Statement::ExecSql(exec) => generate_exec_sql(w, exec),
    }
}

// ---------------------------------------------------------------------------
// SQL statement generator
// ---------------------------------------------------------------------------

/// Generate Rust code for an EXEC SQL statement.
///
/// Maps each `SqlStatementType` variant to the corresponding `CobolSqlRuntime`
/// trait method call. Host variables are converted to `HostVar` / `HostVarMut`
/// arrays using partial struct borrows from `ws`.
fn generate_exec_sql(w: &mut RustWriter, exec: &ExecSqlStatement) {
    match &exec.sql_type {
        SqlStatementType::IncludeSqlca => {
            // EXEC SQL INCLUDE SQLCA -- no codegen needed, SQLCA is injected
            // into WorkingStorage by data_gen when has_sql is true.
            w.line("// EXEC SQL INCLUDE SQLCA (handled by data_gen)");
        }
        SqlStatementType::SelectInto => {
            // SELECT ... INTO :host-var ... WHERE :host-var ...
            generate_sql_query(w, exec);
        }
        SqlStatementType::Insert | SqlStatementType::Update | SqlStatementType::Delete => {
            generate_sql_update(w, exec);
        }
        SqlStatementType::Commit => {
            w.line("sql.commit(&mut ws.sqlca);");
        }
        SqlStatementType::Rollback => {
            w.line("sql.rollback(&mut ws.sqlca);");
        }
        SqlStatementType::DeclareCursor => {
            generate_sql_declare_cursor(w, exec);
        }
        SqlStatementType::OpenCursor => {
            if let Some(ref cursor_name) = exec.cursor_name {
                w.line(&format!(
                    "sql.open_cursor(\"{}\", &mut ws.sqlca);",
                    cursor_name.to_uppercase()
                ));
            }
        }
        SqlStatementType::FetchCursor => {
            generate_sql_fetch_cursor(w, exec);
        }
        SqlStatementType::CloseCursor => {
            if let Some(ref cursor_name) = exec.cursor_name {
                w.line(&format!(
                    "sql.close_cursor(\"{}\", &mut ws.sqlca);",
                    cursor_name.to_uppercase()
                ));
            }
        }
        SqlStatementType::Prepare => {
            generate_sql_prepare(w, exec);
        }
        SqlStatementType::Execute => {
            generate_sql_execute(w, exec);
        }
        SqlStatementType::ExecuteImmediate => {
            generate_sql_execute_immediate(w, exec);
        }
        SqlStatementType::Savepoint => {
            // Extract savepoint name from raw SQL: "SAVEPOINT name"
            let sp_name = exec.raw_sql.trim()
                .strip_prefix("SAVEPOINT")
                .or_else(|| exec.raw_sql.trim().strip_prefix("savepoint"))
                .unwrap_or("SP1")
                .trim();
            w.line(&format!("sql.savepoint(\"{sp_name}\", &mut ws.sqlca);"));
        }
        SqlStatementType::Other(keyword) => {
            w.line(&format!(
                "// EXEC SQL {keyword} -- unsupported SQL statement type"
            ));
            w.line(&format!(
                "// raw SQL: {}",
                exec.raw_sql.replace('\n', " ")
            ));
        }
    }
}

/// Generate `sql.exec_query(...)` for SELECT INTO.
fn generate_sql_query(w: &mut RustWriter, exec: &ExecSqlStatement) {
    let sql_text = host_vars_to_placeholders(&exec.raw_sql);
    w.open_block("{");

    // Build input params array
    if !exec.input_vars.is_empty() {
        for (i, hv) in exec.input_vars.iter().enumerate() {
            let field = cobol_to_rust_name(&hv.field_name, "");
            if let Some(ref ind) = hv.indicator {
                let ind_field = cobol_to_rust_name(ind, "");
                w.line(&format!(
                    "let _p{i} = HostVar::with_indicator(&ws.{field}, &ws.{ind_field});"
                ));
            } else {
                w.line(&format!("let _p{i} = HostVar::new(&ws.{field});"));
            }
        }
        let params: Vec<String> = (0..exec.input_vars.len()).map(|i| format!("_p{i}")).collect();
        w.line(&format!("let _params = [{}];", params.join(", ")));
    } else {
        w.line("let _params: [HostVar<'_>; 0] = [];");
    }

    // Build output fields array
    if !exec.output_vars.is_empty() {
        for (i, hv) in exec.output_vars.iter().enumerate() {
            let field = cobol_to_rust_name(&hv.field_name, "");
            if let Some(ref ind) = hv.indicator {
                let ind_field = cobol_to_rust_name(ind, "");
                w.line(&format!(
                    "let mut _o{i} = HostVarMut::with_indicator(&mut ws.{field}, &mut ws.{ind_field});"
                ));
            } else {
                w.line(&format!(
                    "let mut _o{i} = HostVarMut::new(&mut ws.{field});"
                ));
            }
        }
        let outputs: Vec<String> = (0..exec.output_vars.len()).map(|i| format!("_o{i}")).collect();
        w.line(&format!(
            "let mut _into = [{}];",
            outputs.join(", ")
        ));
    } else {
        w.line("let mut _into: [HostVarMut<'_>; 0] = [];");
    }

    w.line(&format!(
        "sql.exec_query(\"{}\", &_params, &mut _into, &mut ws.sqlca);",
        escape_sql(&sql_text)
    ));
    w.close_block("}");
}

/// Generate `sql.exec_update(...)` for INSERT, UPDATE, DELETE.
fn generate_sql_update(w: &mut RustWriter, exec: &ExecSqlStatement) {
    let sql_text = host_vars_to_placeholders(&exec.raw_sql);
    w.open_block("{");

    if !exec.input_vars.is_empty() {
        for (i, hv) in exec.input_vars.iter().enumerate() {
            let field = cobol_to_rust_name(&hv.field_name, "");
            if let Some(ref ind) = hv.indicator {
                let ind_field = cobol_to_rust_name(ind, "");
                w.line(&format!(
                    "let _p{i} = HostVar::with_indicator(&ws.{field}, &ws.{ind_field});"
                ));
            } else {
                w.line(&format!("let _p{i} = HostVar::new(&ws.{field});"));
            }
        }
        let params: Vec<String> = (0..exec.input_vars.len()).map(|i| format!("_p{i}")).collect();
        w.line(&format!("let _params = [{}];", params.join(", ")));
    } else {
        w.line("let _params: [HostVar<'_>; 0] = [];");
    }

    w.line(&format!(
        "sql.exec_update(\"{}\", &_params, &mut ws.sqlca);",
        escape_sql(&sql_text)
    ));
    w.close_block("}");
}

/// Generate `sql.declare_cursor(...)`.
fn generate_sql_declare_cursor(w: &mut RustWriter, exec: &ExecSqlStatement) {
    let cursor_name = exec.cursor_name.as_deref().unwrap_or("UNKNOWN");
    let sql_text = host_vars_to_placeholders(&exec.raw_sql);
    w.open_block("{");

    if !exec.input_vars.is_empty() {
        for (i, hv) in exec.input_vars.iter().enumerate() {
            let field = cobol_to_rust_name(&hv.field_name, "");
            if let Some(ref ind) = hv.indicator {
                let ind_field = cobol_to_rust_name(ind, "");
                w.line(&format!(
                    "let _p{i} = HostVar::with_indicator(&ws.{field}, &ws.{ind_field});"
                ));
            } else {
                w.line(&format!("let _p{i} = HostVar::new(&ws.{field});"));
            }
        }
        let params: Vec<String> = (0..exec.input_vars.len()).map(|i| format!("_p{i}")).collect();
        w.line(&format!("let _params = [{}];", params.join(", ")));
    } else {
        w.line("let _params: [HostVar<'_>; 0] = [];");
    }

    w.line(&format!(
        "sql.declare_cursor(\"{}\", \"{}\", &_params, &mut ws.sqlca);",
        cursor_name.to_uppercase(),
        escape_sql(&sql_text)
    ));
    w.close_block("}");
}

/// Generate `sql.fetch_cursor(...)`.
fn generate_sql_fetch_cursor(w: &mut RustWriter, exec: &ExecSqlStatement) {
    let cursor_name = exec.cursor_name.as_deref().unwrap_or("UNKNOWN");
    w.open_block("{");

    if !exec.output_vars.is_empty() {
        for (i, hv) in exec.output_vars.iter().enumerate() {
            let field = cobol_to_rust_name(&hv.field_name, "");
            if let Some(ref ind) = hv.indicator {
                let ind_field = cobol_to_rust_name(ind, "");
                w.line(&format!(
                    "let mut _o{i} = HostVarMut::with_indicator(&mut ws.{field}, &mut ws.{ind_field});"
                ));
            } else {
                w.line(&format!(
                    "let mut _o{i} = HostVarMut::new(&mut ws.{field});"
                ));
            }
        }
        let outputs: Vec<String> = (0..exec.output_vars.len()).map(|i| format!("_o{i}")).collect();
        w.line(&format!(
            "let mut _into = [{}];",
            outputs.join(", ")
        ));
    } else {
        w.line("let mut _into: [HostVarMut<'_>; 0] = [];");
    }

    w.line(&format!(
        "sql.fetch_cursor(\"{}\", &mut _into, &mut ws.sqlca);",
        cursor_name.to_uppercase()
    ));
    w.close_block("}");
}

/// Generate `sql.prepare(...)`.
fn generate_sql_prepare(w: &mut RustWriter, exec: &ExecSqlStatement) {
    let stmt_name = exec.prepared_name.as_deref().unwrap_or("UNKNOWN");
    // PREPARE stmt-name FROM :host-var
    // The input_vars[0] contains the SQL source text field
    if let Some(hv) = exec.input_vars.first() {
        let field = cobol_to_rust_name(&hv.field_name, "");
        w.open_block("{");
        w.line(&format!("let _src = HostVar::new(&ws.{field});"));
        w.line(&format!(
            "sql.prepare(\"{}\", &_src, &mut ws.sqlca);",
            stmt_name.to_uppercase()
        ));
        w.close_block("}");
    }
}

/// Generate `sql.execute_prepared(...)`.
fn generate_sql_execute(w: &mut RustWriter, exec: &ExecSqlStatement) {
    let stmt_name = exec.prepared_name.as_deref().unwrap_or("UNKNOWN");
    w.open_block("{");

    if !exec.input_vars.is_empty() {
        for (i, hv) in exec.input_vars.iter().enumerate() {
            let field = cobol_to_rust_name(&hv.field_name, "");
            if let Some(ref ind) = hv.indicator {
                let ind_field = cobol_to_rust_name(ind, "");
                w.line(&format!(
                    "let _p{i} = HostVar::with_indicator(&ws.{field}, &ws.{ind_field});"
                ));
            } else {
                w.line(&format!("let _p{i} = HostVar::new(&ws.{field});"));
            }
        }
        let params: Vec<String> = (0..exec.input_vars.len()).map(|i| format!("_p{i}")).collect();
        w.line(&format!("let _params = [{}];", params.join(", ")));
    } else {
        w.line("let _params: [HostVar<'_>; 0] = [];");
    }

    w.line(&format!(
        "sql.execute_prepared(\"{}\", &_params, &mut ws.sqlca);",
        stmt_name.to_uppercase()
    ));
    w.close_block("}");
}

/// Generate `sql.execute_immediate(...)`.
fn generate_sql_execute_immediate(w: &mut RustWriter, exec: &ExecSqlStatement) {
    if let Some(hv) = exec.input_vars.first() {
        let field = cobol_to_rust_name(&hv.field_name, "");
        w.open_block("{");
        w.line(&format!("let _src = HostVar::new(&ws.{field});"));
        w.line("sql.execute_immediate(&_src, &mut ws.sqlca);");
        w.close_block("}");
    }
}

/// Replace `:HOST-VAR` references in SQL text with `?` placeholders.
fn host_vars_to_placeholders(sql: &str) -> String {
    let mut result = String::with_capacity(sql.len());
    let mut chars = sql.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == ':' {
            // Check if this starts a host variable name (letter or digit after colon)
            if chars.peek().is_some_and(|c| c.is_ascii_alphanumeric() || *c == '_') {
                result.push('?');
                // Skip the host variable name
                while chars.peek().is_some_and(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-') {
                    chars.next();
                }
            } else {
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }
    result
}

/// Escape double quotes and backslashes for embedding SQL in a Rust string literal.
fn escape_sql(sql: &str) -> String {
    sql.replace('\\', "\\\\").replace('"', "\\\"")
}

// ---------------------------------------------------------------------------
// Statement generators
// ---------------------------------------------------------------------------

/// Format a ROUNDED clause as a Rust expression string.
fn rounded_str(rounded: bool) -> &'static str {
    if rounded {
        "Some(RoundingMode::NearestAwayFromZero)"
    } else {
        "None"
    }
}

fn generate_move(w: &mut RustWriter, m: &MoveStatement, gcm: &GroupChildMap) {
    if m.corresponding {
        // Expand MOVE CORRESPONDING at codegen time: find matching children
        // between source and destination groups, generate individual MOVEs.
        let src_name = match &m.source {
            Operand::DataRef(dr) => dr.name.to_uppercase(),
            _ => String::new(),
        };
        let src_children = gcm.get(&src_name);

        for dest in &m.destinations {
            let dest_name = dest.name.to_uppercase();
            let dest_children = gcm.get(&dest_name);

            if let (Some(sc), Some(dc)) = (src_children, dest_children) {
                // Find matching children by COBOL name (case-insensitive)
                let dest_names: HashSet<String> = dc.iter()
                    .map(|(n, _)| n.to_uppercase())
                    .collect();
                w.line("// MOVE CORRESPONDING expanded");
                for (child_name, _is_numeric) in sc {
                    if dest_names.contains(&child_name.to_uppercase()) {
                        // Build qualified field refs for src and dest children
                        let src_field = cobol_to_rust_name(child_name, &src_name);
                        let dst_field = cobol_to_rust_name(child_name, &dest_name);
                        w.line(&format!(
                            "cobol_move(&ws.{src_field}, &mut ws.{dst_field}, &ctx.config);"
                        ));
                    }
                }
            } else {
                // Fallback: emit a comment if group not found in map
                w.line(&format!(
                    "// TODO: MOVE CORRESPONDING - group not found: {src_name} / {dest_name}"
                ));
            }
        }
    } else {
        for dest in &m.destinations {
            if let Some(rm) = &dest.ref_mod {
                // Destination has reference modification -- use ref_mod_write
                let dest_base = data_ref_base_expr(dest);
                let offset = ref_mod_index_expr(&rm.offset);
                // Get source bytes expression
                let src_bytes = operand_to_source_bytes(&m.source);
                if let Some(ref len) = rm.length {
                    let length = ref_mod_index_expr(len);
                    w.line(&format!(
                        "ref_mod_write({src_bytes}, &mut {dest_base}, {offset}, {length});"
                    ));
                } else {
                    w.line(&format!(
                        "ref_mod_write_to_end({src_bytes}, &mut {dest_base}, {offset});"
                    ));
                }
            } else {
                // Normal MOVE -- dispatch based on source type
                let dest_expr = data_ref_base_expr(dest);
                match &m.source {
                    Operand::Literal(Literal::Numeric(n)) => {
                        w.line(&format!(
                            "move_numeric_literal(\"{n}\".parse::<Decimal>().unwrap(), &mut {dest_expr}, &ctx.config);"
                        ));
                    }
                    Operand::Literal(Literal::Alphanumeric(s)) => {
                        let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
                        w.line(&format!(
                            "move_alphanumeric_literal(b\"{escaped}\", &mut {dest_expr}, &ctx.config);"
                        ));
                    }
                    Operand::Literal(Literal::Figurative(fig)) => {
                        match fig {
                            FigurativeConstant::All(pattern) => {
                                // ALL "x" -- fill field by repeating the pattern
                                let escaped = pattern.replace('\\', "\\\\").replace('"', "\\\"");
                                w.line(&format!(
                                    "{{ let pat = b\"{escaped}\"; let mut buf = Vec::new(); while buf.len() < {dest_expr}.as_bytes().len() {{ buf.extend_from_slice(pat); }} buf.truncate({dest_expr}.as_bytes().len()); {dest_expr}.set_raw_bytes(&buf); }}"
                                ));
                            }
                            _ => {
                                let fig_expr = match fig {
                                    FigurativeConstant::Spaces => "FigurativeConstant::Spaces",
                                    FigurativeConstant::Zeros => "FigurativeConstant::Zeros",
                                    FigurativeConstant::HighValues => "FigurativeConstant::HighValues",
                                    FigurativeConstant::LowValues => "FigurativeConstant::LowValues",
                                    FigurativeConstant::Quotes => "FigurativeConstant::Quotes",
                                    FigurativeConstant::Nulls => "FigurativeConstant::Nulls",
                                    FigurativeConstant::All(_) => unreachable!(),
                                };
                                w.line(&format!(
                                    "{fig_expr}.fill_field(&mut {dest_expr});"
                                ));
                            }
                        }
                    }
                    Operand::Function(f) => {
                        // Intrinsic function results -- handle by return type
                        let (_param_kind, return_kind) = classify_intrinsic(&f.name);
                        let call = generate_intrinsic_call(f);
                        if return_kind == "bytes" {
                            w.line(&format!(
                                "{{ let _fv = {call}; {dest_expr}.set_raw_bytes(&_fv); }}"
                            ));
                        } else {
                            w.line(&format!(
                                "move_numeric_literal({call}, &mut {dest_expr}, &ctx.config);"
                            ));
                        }
                    }
                    _ => {
                        // DataRef: use cobol_move with trait object
                        let src = operand_expr(&m.source);
                        w.line(&format!(
                            "cobol_move(&{src}, &mut {dest_expr}, &ctx.config);"
                        ));
                    }
                }
            }
        }
    }
}

/// Get an operand's bytes for use as `ref_mod_write` source.
fn operand_to_source_bytes(op: &Operand) -> String {
    match op {
        Operand::Literal(Literal::Alphanumeric(s)) => format!("b\"{s}\""),
        Operand::Literal(Literal::Numeric(n)) => format!("b\"{n}\""),
        Operand::Literal(Literal::Figurative(FigurativeConstant::Zeros)) => "b\"0\"".to_string(),
        Operand::Literal(Literal::Figurative(FigurativeConstant::All(s))) => {
            let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
            format!("b\"{escaped}\"")
        }
        Operand::Literal(Literal::Figurative(_)) => "b\" \"".to_string(),
        Operand::DataRef(dr) => {
            if let Some(rm) = &dr.ref_mod {
                // Source also has ref_mod -- use ref_mod_read directly
                let base = data_ref_base_expr(dr);
                let offset = ref_mod_index_expr(&rm.offset);
                if let Some(ref len) = rm.length {
                    let length = ref_mod_index_expr(len);
                    format!("ref_mod_read(&{base}, {offset}, {length})")
                } else {
                    format!("ref_mod_read_to_end(&{base}, {offset})")
                }
            } else {
                let expr = data_ref_base_expr(dr);
                format!("{expr}.as_bytes()")
            }
        }
        Operand::Function(f) => generate_intrinsic_call_bytes(f),
    }
}

fn generate_display(w: &mut RustWriter, d: &DisplayStatement) {
    // DISPLAY generates print statements directly instead of using a ctx method,
    // since items can be heterogeneous types (literals, field display strings).
    for item in &d.items {
        match item {
            Operand::Literal(Literal::Alphanumeric(s)) => {
                w.line(&format!("print!(\"{s}\");"));
            }
            Operand::Literal(Literal::Numeric(n)) => {
                w.line(&format!("print!(\"{n}\");"));
            }
            Operand::DataRef(dr) => {
                let field = data_ref_expr(dr);
                w.line(&format!(
                    "print!(\"{{}}\", String::from_utf8_lossy(&{field}.display_bytes()));"
                ));
            }
            _ => {
                let expr = operand_expr(item);
                w.line(&format!("print!(\"{{:?}}\", {expr});"));
            }
        }
    }
    if !d.no_advancing {
        w.line("println!();");
    }
}

fn generate_add(w: &mut RustWriter, a: &AddStatement) {
    if a.giving.is_empty() {
        // ADD ... TO: add each operand to each TO target
        let operands: Vec<String> = a.operands.iter().map(operand_numeric_expr).collect();
        for target in &a.to {
            let dest = data_ref_expr(&target.field);
            let r = rounded_str(target.rounded);
            for op in &operands {
                w.line(&format!(
                    "cobol_add(&{op}, &mut {dest}, {r}, &ctx.config);"
                ));
            }
        }
    } else {
        // ADD ... GIVING: sum the operands, store in giving targets
        let operands: Vec<String> = a.operands.iter().map(operand_numeric_expr).collect();
        // For GIVING, first operand is src, second is src2
        if operands.len() >= 2 {
            for target in &a.giving {
                let dest = data_ref_expr(&target.field);
                let r = rounded_str(target.rounded);
                w.line(&format!(
                    "cobol_add_giving(&{}, &{}, &mut {dest}, {r}, &ctx.config);",
                    operands[0], operands[1]
                ));
            }
        } else if operands.len() == 1 {
            // Single operand GIVING -- add to first giving target
            for target in &a.giving {
                let dest = data_ref_expr(&target.field);
                let r = rounded_str(target.rounded);
                w.line(&format!(
                    "cobol_add(&{}, &mut {dest}, {r}, &ctx.config);",
                    operands[0]
                ));
            }
        }
    }
}

fn generate_subtract(w: &mut RustWriter, s: &SubtractStatement) {
    if s.giving.is_empty() {
        let operands: Vec<String> = s.operands.iter().map(operand_numeric_expr).collect();
        for target in &s.from {
            let dest = data_ref_expr(&target.field);
            let r = rounded_str(target.rounded);
            for op in &operands {
                w.line(&format!(
                    "cobol_subtract(&{op}, &mut {dest}, {r}, &ctx.config);"
                ));
            }
        }
    } else {
        let operands: Vec<String> = s.operands.iter().map(operand_numeric_expr).collect();
        if operands.len() >= 2 {
            for target in &s.giving {
                let dest = data_ref_expr(&target.field);
                let r = rounded_str(target.rounded);
                w.line(&format!(
                    "cobol_subtract_giving(&{}, &{}, &mut {dest}, {r}, &ctx.config);",
                    operands[0], operands[1]
                ));
            }
        } else if operands.len() == 1 {
            for target in &s.giving {
                let dest = data_ref_expr(&target.field);
                let r = rounded_str(target.rounded);
                w.line(&format!(
                    "cobol_subtract(&{}, &mut {dest}, {r}, &ctx.config);",
                    operands[0]
                ));
            }
        }
    }
}

fn generate_multiply(w: &mut RustWriter, m: &MultiplyStatement) {
    let multiplicand = operand_numeric_expr(&m.operand);

    if m.giving.is_empty() {
        for target in &m.by {
            let dest = data_ref_expr(&target.field);
            let r = rounded_str(target.rounded);
            w.line(&format!(
                "cobol_multiply(&{multiplicand}, &mut {dest}, {r}, &ctx.config);"
            ));
        }
    } else {
        let by_field = m.by.first().map_or_else(|| "0".to_string(), |t| data_ref_expr(&t.field));
        for target in &m.giving {
            let dest = data_ref_expr(&target.field);
            let r = rounded_str(target.rounded);
            w.line(&format!(
                "cobol_multiply_giving(&{multiplicand}, &{by_field}, &mut {dest}, {r}, &ctx.config);"
            ));
        }
    }
}

fn generate_divide(w: &mut RustWriter, d: &DivideStatement) {
    let operand = operand_numeric_expr(&d.operand);
    let remainder_expr = d.remainder.as_ref().map_or_else(|| "None".to_string(), |rem| format!("Some(&mut {})", data_ref_expr(&rem.field)));

    if d.giving.is_empty() {
        // DIVIDE x INTO y -> cobol_divide(x, y, remainder, rounded, config)
        for target in &d.into {
            let dest = data_ref_expr(&target.field);
            let r = rounded_str(target.rounded);
            w.line(&format!(
                "cobol_divide(&{operand}, &mut {dest}, {remainder_expr}, {r}, &ctx.config);"
            ));
        }
    } else {
        match d.direction {
            DivideDirection::Into => {
                let into_field = d.into.first().map_or_else(|| "0".to_string(), |t| data_ref_expr(&t.field));
                // DIVIDE x INTO y GIVING z -> cobol_divide_giving(x, y, z, remainder, rounded, config)
                for target in &d.giving {
                    let dest = data_ref_expr(&target.field);
                    let r = rounded_str(target.rounded);
                    w.line(&format!(
                        "cobol_divide_giving(&{operand}, &{into_field}, &mut {dest}, {remainder_expr}, {r}, &ctx.config);"
                    ));
                }
            }
            DivideDirection::By => {
                // DIVIDE x BY y GIVING z -> cobol_divide_by_giving(x, y, z, rounded, config)
                let by_field = d.by_operand.as_ref().map_or_else(
                    || "0".to_string(),
                    |op| operand_numeric_expr(op),
                );
                for target in &d.giving {
                    let dest = data_ref_expr(&target.field);
                    let r = rounded_str(target.rounded);
                    w.line(&format!(
                        "cobol_divide_by_giving(&{operand}, &{by_field}, &mut {dest}, {r}, &ctx.config);"
                    ));
                }
            }
        }
    }
}

fn generate_compute(w: &mut RustWriter, c: &ComputeStatement) {
    let expr = arith_expr_str(&c.expression);
    for target in &c.targets {
        let dest = data_ref_expr(&target.field);
        let r = rounded_str(target.rounded);
        w.line(&format!(
            "cobol_compute({expr}, &mut {dest}, {r}, &ctx.config);"
        ));
    }
}

fn generate_if(w: &mut RustWriter, i: &IfStatement, cmap: &ConditionMap, ptable: &[ParagraphIndex], rfm: &RecordFileMap, sfm: &SortFieldMap, gcm: &GroupChildMap, has_sql: bool) {
    let cond = condition_expr(&i.condition, cmap);
    w.open_block(&format!("if {cond} {{"));
    for stmt in &i.then_body {
        generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
    }
    if i.else_body.is_empty() {
        w.close_block("}");
    } else {
        w.dedent();
        w.open_block("} else {");
        for stmt in &i.else_body {
            generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
        }
        w.close_block("}");
    }
}

fn generate_evaluate(w: &mut RustWriter, e: &EvaluateStatement, cmap: &ConditionMap, ptable: &[ParagraphIndex], rfm: &RecordFileMap, sfm: &SortFieldMap, gcm: &GroupChildMap, has_sql: bool) {
    let subject_op = e.subjects.first().and_then(|subj| match subj {
        EvaluateSubject::Expr(op) => Some(op.clone()),
        EvaluateSubject::Bool(_) => None,
    });
    let subject_is_true = e.subjects.first().is_none_or(|s| matches!(s, EvaluateSubject::Bool(true)));

    for (i, branch) in e.when_branches.iter().enumerate() {
        let keyword = if i == 0 { "if" } else { "} else if" };
        let values: Vec<String> = branch.values.iter().map(|v| match v {
            WhenValue::Value(op) => {
                if let Some(ref subj_op) = subject_op {
                    // Use smart comparison dispatch
                    generate_comparison_expr(subj_op, ComparisonOp::Equal, op)
                } else {
                    // EVALUATE TRUE/FALSE: WHEN values are conditions
                    // Check if the operand is an 88-level condition name
                    if let Operand::DataRef(dr) = op {
                        let key = dr.name.to_uppercase();
                        if cmap.contains_key(&key) {
                            let cond_expr = condition_name_expr(dr, cmap);
                            if subject_is_true {
                                format!("({cond_expr})")
                            } else {
                                format!("!({cond_expr})")
                            }
                        } else {
                            let val = operand_expr(op);
                            if subject_is_true {
                                format!("({val})")
                            } else {
                                format!("!({val})")
                            }
                        }
                    } else {
                        let val = operand_expr(op);
                        if subject_is_true {
                            format!("({val})")
                        } else {
                            format!("!({val})")
                        }
                    }
                }
            }
            WhenValue::Range { low, high } => {
                if let Some(ref subj_op) = subject_op {
                    let lo = generate_comparison_expr(subj_op, ComparisonOp::GreaterOrEqual, low);
                    let hi = generate_comparison_expr(subj_op, ComparisonOp::LessOrEqual, high);
                    format!("{lo} && {hi}")
                } else {
                    "true".to_string()
                }
            }
            WhenValue::Condition(c) => condition_expr(c, cmap),
            WhenValue::Any => "true".to_string(),
        }).collect();
        let cond = if values.is_empty() {
            "true".to_string()
        } else {
            values.join(" || ")
        };

        if i > 0 {
            w.dedent();
        }
        w.open_block(&format!("{keyword} {cond} {{"));
        for stmt in &branch.body {
            generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
        }
    }

    if !e.when_other.is_empty() {
        w.dedent();
        w.open_block("} else {");
        for stmt in &e.when_other {
            generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
        }
    }

    if !e.when_branches.is_empty() || !e.when_other.is_empty() {
        w.close_block("}");
    }
}

fn generate_perform(w: &mut RustWriter, p: &PerformStatement, cmap: &ConditionMap, ptable: &[ParagraphIndex], rfm: &RecordFileMap, sfm: &SortFieldMap, gcm: &GroupChildMap, has_sql: bool) {
    match &p.loop_type {
        PerformLoopType::Once => {
            if let Some(ref target) = p.target {
                if let Some(ref thru_name) = p.thru {
                    generate_perform_thru_inline(w, &target.name, thru_name, ptable, has_sql);
                } else {
                    let fn_name = cobol_to_rust_name(&target.name, "");
                    if has_sql {
                        w.line(&format!("{fn_name}(ws, ctx, sql);"));
                    } else {
                        w.line(&format!("{fn_name}(ws, ctx);"));
                    }
                }
            } else {
                // Inline perform (once)
                for stmt in &p.body {
                    generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
                }
            }
        }
        PerformLoopType::Times(count) => {
            let count_usize = operand_to_usize_expr(count);
            if let Some(ref target) = p.target {
                w.open_block(&format!(
                    "for _cobol_i in 0..{count_usize} {{"
                ));
                if let Some(ref thru_name) = p.thru {
                    generate_perform_thru_inline(w, &target.name, thru_name, ptable, has_sql);
                } else {
                    let fn_name = cobol_to_rust_name(&target.name, "");
                    if has_sql {
                        w.line(&format!("{fn_name}(ws, ctx, sql);"));
                    } else {
                        w.line(&format!("{fn_name}(ws, ctx);"));
                    }
                }
                w.close_block("}");
            } else {
                w.open_block(&format!(
                    "for _cobol_i in 0..{count_usize} {{"
                ));
                for stmt in &p.body {
                    generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
                }
                w.close_block("}");
            }
        }
        PerformLoopType::Until {
            test_before,
            condition,
        } => {
            let cond = condition_expr(condition, cmap);
            if *test_before {
                generate_perform_until_before(w, &cond, p, cmap, ptable, rfm, sfm, gcm, has_sql);
            } else {
                generate_perform_until_after(w, &cond, p, cmap, ptable, rfm, sfm, gcm, has_sql);
            }
        }
        PerformLoopType::Varying {
            test_before,
            counter,
            from,
            by,
            until,
            ..
        } => {
            let counter_name = data_ref_expr(counter);
            let from_expr = operand_numeric_expr(from);
            let by_expr = operand_numeric_expr(by);
            let until_cond = condition_expr(until, cmap);

            // Use cobol_move_numeric to properly initialize the counter
            w.line(&format!(
                "cobol_move_numeric(&{from_expr}, &mut {counter_name}, &ctx.config);"
            ));
            if *test_before {
                w.open_block(&format!("while !({until_cond}) {{"));
            } else {
                w.open_block("loop {");
            }

            if let Some(ref target) = p.target {
                if let Some(ref thru_name) = p.thru {
                    generate_perform_thru_inline(w, &target.name, thru_name, ptable, has_sql);
                } else {
                    let fn_name = cobol_to_rust_name(&target.name, "");
                    if has_sql {
                        w.line(&format!("{fn_name}(ws, ctx, sql);"));
                    } else {
                        w.line(&format!("{fn_name}(ws, ctx);"));
                    }
                }
            } else {
                for stmt in &p.body {
                    generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
                }
            }

            // Use cobol_add to properly increment the counter
            w.line(&format!(
                "cobol_add(&{by_expr}, &mut {counter_name}, None, &ctx.config);"
            ));

            if !test_before {
                w.open_block(&format!("if {until_cond} {{"));
                w.line("break;");
                w.close_block("}");
            }

            w.close_block("}");
        }
    }
}

/// Generate an inline dispatch loop for PERFORM target THRU end.
fn generate_perform_thru_inline(w: &mut RustWriter, target_name: &str, thru_name: &str, ptable: &[ParagraphIndex], has_sql: bool) {
    let target_upper = target_name.to_uppercase();
    let thru_upper = thru_name.to_uppercase();
    let start_idx = ptable.iter().position(|pi| pi.name == target_upper);
    let end_idx = ptable.iter().position(|pi| pi.name == thru_upper);
    if let (Some(s), Some(e)) = (start_idx, end_idx) {
        w.open_block("{");
        w.line(&format!("let mut _perf_pc: usize = {s};"));
        w.open_block(&format!("while _perf_pc <= {e} {{"));
        w.open_block("match _perf_pc {");
        for pi in &ptable[s..=e] {
            if has_sql {
                w.line(&format!("{} => {}(ws, ctx, sql),", pi.index, pi.rust_name));
            } else {
                w.line(&format!("{} => {}(ws, ctx),", pi.index, pi.rust_name));
            }
        }
        w.line("_ => break,");
        w.close_block("}");
        w.line("if ctx.stopped || ctx.exit_program || ctx.goto_target.is_some() { break; }");
        w.line("_perf_pc += 1;");
        w.close_block("}");
        w.close_block("}");
    } else {
        // Fallback: just call the target
        let fn_name = cobol_to_rust_name(target_name, "");
        if has_sql {
            w.line(&format!("{fn_name}(ws, ctx, sql);"));
        } else {
            w.line(&format!("{fn_name}(ws, ctx);"));
        }
    }
}

fn generate_perform_until_before(w: &mut RustWriter, cond: &str, p: &PerformStatement, cmap: &ConditionMap, ptable: &[ParagraphIndex], rfm: &RecordFileMap, sfm: &SortFieldMap, gcm: &GroupChildMap, has_sql: bool) {
    w.open_block(&format!("while !({cond}) {{"));
    if let Some(ref target) = p.target {
        if let Some(ref thru_name) = p.thru {
            generate_perform_thru_inline(w, &target.name, thru_name, ptable, has_sql);
        } else {
            let fn_name = cobol_to_rust_name(&target.name, "");
            w.line(&format!("{fn_name}(ws, ctx);"));
        }
    } else {
        for stmt in &p.body {
            generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
        }
    }
    w.close_block("}");
}

fn generate_perform_until_after(w: &mut RustWriter, cond: &str, p: &PerformStatement, cmap: &ConditionMap, ptable: &[ParagraphIndex], rfm: &RecordFileMap, sfm: &SortFieldMap, gcm: &GroupChildMap, has_sql: bool) {
    w.open_block("loop {");
    if let Some(ref target) = p.target {
        if let Some(ref thru_name) = p.thru {
            generate_perform_thru_inline(w, &target.name, thru_name, ptable, has_sql);
        } else {
            let fn_name = cobol_to_rust_name(&target.name, "");
            w.line(&format!("{fn_name}(ws, ctx);"));
        }
    } else {
        for stmt in &p.body {
            generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
        }
    }
    w.open_block(&format!("if {cond} {{"));
    w.line("break;");
    w.close_block("}");
    w.close_block("}");
}

fn generate_goto(w: &mut RustWriter, g: &GoToStatement) {
    if let Some(ref dep_ref) = g.depending {
        // GO TO para-1 para-2 ... DEPENDING ON field
        // Index is 1-based (COBOL semantics): value 1 -> first target, etc.
        let dep_expr = data_ref_expr(dep_ref);
        w.line("{");
        w.line(&format!(
            "    let _goto_idx = {dep_expr}.to_decimal().to_string().parse::<i64>().unwrap_or(0) as usize;"
        ));
        w.line("    match _goto_idx {");
        for (i, target) in g.targets.iter().enumerate() {
            let upper = target.to_uppercase();
            w.line(&format!(
                "        {} => ctx.goto_target = Some(\"{upper}\".to_string()),",
                i + 1
            ));
        }
        w.line("        _ => {} // out of range: no transfer (COBOL standard)");
        w.line("    }");
        w.line("    if ctx.goto_target.is_some() { return; }");
        w.line("}");
    } else if let Some(target) = g.targets.first() {
        let target_upper = target.to_uppercase();
        w.line(&format!(
            "ctx.goto_target = Some(\"{target_upper}\".to_string());"
        ));
        w.line("return;");
    }
}

fn generate_initialize(w: &mut RustWriter, init: &InitializeStatement, gcm: &GroupChildMap) {
    for target in &init.targets {
        let target_name = target.name.to_uppercase();
        // If target is a group in the GCM, expand into individual child initializations
        if let Some(children) = gcm.get(&target_name) {
            if init.replacing.is_empty() {
                // Default INITIALIZE: spaces for alpha, zeros for numeric
                emit_initialize_children(w, children, gcm, &InitializeMode::Default);
            } else {
                for repl in &init.replacing {
                    emit_initialize_children(w, children, gcm, &InitializeMode::Replacing(repl));
                }
            }
        } else {
            // Leaf field -- just call cobol_initialize
            let dest = data_ref_expr(target);
            w.line(&format!("cobol_initialize(&mut {dest});"));
        }
    }
}

enum InitializeMode<'a> {
    Default,
    Replacing(&'a InitializeReplacing),
}

/// Recursively emit initialization for group children.
fn emit_initialize_children(w: &mut RustWriter, children: &[(String, bool)], gcm: &GroupChildMap, mode: &InitializeMode<'_>) {
    for (child_name, is_numeric) in children {
        let child_upper = child_name.to_uppercase();
        // If child is itself a group, recurse
        if let Some(sub_children) = gcm.get(&child_upper) {
            emit_initialize_children(w, sub_children, gcm, mode);
            continue;
        }
        let field = cobol_to_rust_name(child_name, "");
        match mode {
            InitializeMode::Default => {
                w.line(&format!("cobol_initialize(&mut ws.{field});"));
            }
            InitializeMode::Replacing(repl) => {
                let matches = match repl.category {
                    InitializeCategory::Numeric | InitializeCategory::NumericEdited => *is_numeric,
                    InitializeCategory::Alphanumeric | InitializeCategory::Alphabetic | InitializeCategory::AlphanumericEdited => !*is_numeric,
                    InitializeCategory::National => false,
                };
                if matches {
                    let val_expr = operand_expr(&repl.value);
                    if *is_numeric {
                        // Numeric replacing: MOVE the value as Decimal
                        w.line(&format!("move_numeric_literal({val_expr}, &mut ws.{field}, &ctx.config);"));
                    } else {
                        // Alphanumeric replacing: MOVE the value as bytes
                        w.line(&format!("move_alphanumeric_literal({val_expr}.as_bytes(), &mut ws.{field}, &ctx.config);"));
                    }
                }
            }
        }
    }
}

fn generate_call(w: &mut RustWriter, call: &CallStatement, cmap: &ConditionMap, ptable: &[ParagraphIndex], rfm: &RecordFileMap, sfm: &SortFieldMap, gcm: &GroupChildMap, has_sql: bool) {
    let program = match &call.program {
        Operand::Literal(Literal::Alphanumeric(s)) => format!("\"{s}\""),
        other => operand_expr(other),
    };

    // Generate parameter marshaling
    let has_params = !call.using.is_empty();
    if has_params {
        w.open_block("{");
        for (i, param) in call.using.iter().enumerate() {
            match param.mode {
                PassingMode::ByReference => {
                    if let Some(ref op) = param.operand {
                        let expr = data_ref_base_expr_from_operand(op);
                        w.line(&format!(
                            "let mut _cp{i} = call_param_by_ref(&mut {expr});"
                        ));
                    }
                }
                PassingMode::ByContent => {
                    if let Some(ref op) = param.operand {
                        let expr = data_ref_base_expr_from_operand(op);
                        // BY CONTENT: create a temporary copy
                        w.line(&format!(
                            "let mut _cp{i}_tmp = {expr}.clone();"
                        ));
                        w.line(&format!(
                            "let mut _cp{i} = call_param_by_content(&mut _cp{i}_tmp);"
                        ));
                    }
                }
                PassingMode::ByValue => {
                    if let Some(ref op) = param.operand {
                        let expr = operand_expr(op);
                        w.line(&format!(
                            "let mut _cp{i} = call_param_by_value({expr}.to_decimal().to_string().parse::<i64>().unwrap_or(0));"
                        ));
                    }
                }
                PassingMode::Omitted => {
                    w.line(&format!("let mut _cp{i} = call_param_omitted();"));
                }
            }
        }

        // Build the params array
        let param_refs: Vec<String> = (0..call.using.len())
            .map(|i| format!("_cp{i}"))
            .collect();
        w.line(&format!(
            "let mut _call_params: [CallParam; {}] = [{}];",
            call.using.len(),
            param_refs.join(", ")
        ));
    }

    let params_arg = if has_params {
        "&mut _call_params"
    } else {
        "&mut []"
    };

    // Generate the call with or without exception handlers
    if !call.on_exception.is_empty() || !call.not_on_exception.is_empty() {
        w.open_block(&format!(
            "match cobol_call(&mut ctx.dispatcher, {program}, {params_arg}, &ctx.config) {{"
        ));

        // Ok path -- NOT ON EXCEPTION
        w.open_block("Ok(rc) => {");
        w.line("ctx.return_code = rc;");
        for stmt in &call.not_on_exception {
            generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
        }
        w.close_block("}");

        // Err path -- ON EXCEPTION
        w.open_block("Err(_e) => {");
        for stmt in &call.on_exception {
            generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
        }
        w.close_block("}");

        w.close_block("}");
    } else {
        w.line(&format!(
            "if let Ok(rc) = cobol_call(&mut ctx.dispatcher, {program}, {params_arg}, &ctx.config) {{"
        ));
        w.line("    ctx.return_code = rc;");
        w.line("}");
    }

    // RETURNING -- store return_code into the specified field
    if let Some(ref ret_ref) = call.returning {
        let ret_expr = data_ref_base_expr(ret_ref);
        w.line(&format!(
            "cobol_move(&{{ let mut _rc = PackedDecimal::new(9, 0, true); _rc.pack(Decimal::from(ctx.return_code)); _rc }}, &mut {ret_expr}, &ctx.config);"
        ));
    }

    if has_params {
        w.close_block("}");
    }
}

/// Helper to get a mutable field expression from an operand (for call params).
fn data_ref_base_expr_from_operand(op: &Operand) -> String {
    match op {
        Operand::DataRef(dr) => data_ref_base_expr(dr),
        Operand::Literal(Literal::Alphanumeric(s)) => format!("PicX::new({}, b\"{s}\")", s.len()),
        Operand::Literal(Literal::Numeric(n)) => format!("\"{n}\".parse::<Decimal>().unwrap()"),
        _ => "/* unsupported operand */".to_string(),
    }
}

fn generate_cancel(w: &mut RustWriter, cancel: &CancelStatement) {
    for prog in &cancel.programs {
        let name = match prog {
            Operand::Literal(Literal::Alphanumeric(s)) => format!("\"{s}\""),
            other => operand_expr(other),
        };
        w.line(&format!(
            "let _ = cobol_cancel(&mut ctx.dispatcher, {name});"
        ));
    }
}

fn generate_accept(w: &mut RustWriter, acc: &AcceptStatement) {
    let target = data_ref_expr(&acc.target);
    let source = match acc.from {
        AcceptSource::Sysin => "accept_from_sysin",
        AcceptSource::Date => "accept_date",
        AcceptSource::Time => "accept_time",
        AcceptSource::DayOfWeek => "accept_day_of_week",
        AcceptSource::Day => "accept_day",
        AcceptSource::DateYyyyMmDd => "accept_date_yyyymmdd",
        AcceptSource::DayYyyyDdd => "accept_day_yyyyddd",
    };
    w.line(&format!("{source}(&mut {target});"));
}

// ---------------------------------------------------------------------------
// File I/O statement generators
// ---------------------------------------------------------------------------

fn generate_open(w: &mut RustWriter, open: &OpenStatement) {
    for file in &open.files {
        let fname = cobol_to_rust_name(&file.file_name, "");
        let mode = match file.mode {
            OpenMode::Input => "FileOpenMode::Input",
            OpenMode::Output => "FileOpenMode::Output",
            OpenMode::IoMode => "FileOpenMode::InputOutput",
            OpenMode::Extend => "FileOpenMode::Extend",
        };
        w.line(&format!(
            "assert!(ws.{fname}.open({mode}).is_success(), \"OPEN {}\");",
            file.file_name
        ));
    }
}

fn generate_close(w: &mut RustWriter, close: &CloseStatement) {
    for file_name in &close.files {
        let fname = cobol_to_rust_name(file_name, "");
        w.line(&format!(
            "assert!(ws.{fname}.close().is_success(), \"CLOSE {file_name}\");"
        ));
    }
}

#[allow(unused_variables)]
fn generate_read(w: &mut RustWriter, read: &ReadStatement, cmap: &ConditionMap, ptable: &[ParagraphIndex], rfm: &RecordFileMap, sfm: &SortFieldMap, gcm: &GroupChildMap, has_sql: bool) {
    let fname = cobol_to_rust_name(&read.file_name, "");

    // read_next() and read_by_key() both return (FileStatusCode, Option<Vec<u8>>)
    let read_call = if let Some(ref key_ref) = read.key {
        let key_expr = data_ref_expr(key_ref);
        format!("ws.{fname}.read_by_key({key_expr}.as_bytes())")
    } else {
        format!("ws.{fname}.read_next()")
    };

    // Generate: let (_status, _data) = <read_call>;
    // Then branch on _status.is_success()
    if !read.at_end.is_empty() || !read.not_at_end.is_empty() {
        w.open_block("{");
        w.line(&format!("let (_status, _data) = {read_call};"));
        w.open_block("if _status.is_success() {");
        if let Some(ref into_ref) = read.into {
            let into_expr = data_ref_expr(into_ref);
            w.open_block("if let Some(ref data) = _data {");
            w.line(&format!(
                "{into_expr}.set_raw_bytes(&data[..{into_expr}.byte_length().min(data.len())]);"
            ));
            w.close_block("}");
        }
        for stmt in &read.not_at_end {
            generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
        }
        w.close_block("}");

        if !read.at_end.is_empty() {
            w.open_block("else {");
            for stmt in &read.at_end {
                generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
            }
            w.close_block("}");
        }
        w.close_block("}");
    } else if !read.invalid_key.is_empty() || !read.not_invalid_key.is_empty() {
        w.open_block("{");
        w.line(&format!("let (_status, _data) = {read_call};"));
        w.open_block("if _status.is_success() {");
        if let Some(ref into_ref) = read.into {
            let into_expr = data_ref_expr(into_ref);
            w.open_block("if let Some(ref data) = _data {");
            w.line(&format!(
                "{into_expr}.set_raw_bytes(&data[..{into_expr}.byte_length().min(data.len())]);"
            ));
            w.close_block("}");
        }
        for stmt in &read.not_invalid_key {
            generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
        }
        w.close_block("}");

        if !read.invalid_key.is_empty() {
            w.open_block("else {");
            for stmt in &read.invalid_key {
                generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
            }
            w.close_block("}");
        }
        w.close_block("}");
    } else {
        // Simple read with no handler
        if let Some(ref into_ref) = read.into {
            let into_expr = data_ref_expr(into_ref);
            w.open_block("{");
            w.line(&format!("let (_status, _data) = {read_call};"));
            w.open_block("if let Some(ref data) = _data {");
            w.line(&format!(
                "{into_expr}.set_raw_bytes(&data[..{into_expr}.byte_length().min(data.len())]);"
            ));
            w.close_block("}");
            w.close_block("}");
        } else {
            w.line(&format!("let _ = {read_call};"));
        }
    }
}

fn generate_write(w: &mut RustWriter, write: &WriteStatement, cmap: &ConditionMap, ptable: &[ParagraphIndex], rfm: &RecordFileMap, sfm: &SortFieldMap, gcm: &GroupChildMap, has_sql: bool) {
    let rec = cobol_to_rust_name(&write.record_name, "");
    // Look up the file handle for this record
    let file_field = rfm
        .get(&write.record_name.to_uppercase())
        .map_or_else(|| format!("{rec}_file"), |f| cobol_to_rust_name(f, ""));

    // FROM clause: copy source into record before writing
    if let Some(ref from_ref) = write.from {
        let from_expr = data_ref_expr(from_ref);
        w.line(&format!(
            "ws.{rec}.set_raw_bytes(&{from_expr}.as_bytes()[..ws.{rec}.byte_length().min({from_expr}.byte_length())]);"
        ));
    }

    // WRITE the record through its file handle
    let write_call = format!("ws.{file_field}.write_record(ws.{rec}.as_bytes())");

    // ADVANCING clause generates print control after write
    if let Some(ref adv) = write.advancing {
        match adv {
            Advancing::Page => {
                w.line(&format!("assert!({write_call}.is_success(), \"WRITE {}\");", write.record_name));
                w.line("print!(\"\\x0C\"); // page eject");
            }
            Advancing::Lines(op) => {
                w.line(&format!("assert!({write_call}.is_success(), \"WRITE {}\");", write.record_name));
                let lines = operand_expr(op);
                w.open_block(&format!("for _ in 0..{lines} {{"));
                w.line("println!();");
                w.close_block("}");
            }
        }
    } else if !write.invalid_key.is_empty() || !write.not_invalid_key.is_empty() {
        // INVALID KEY / NOT INVALID KEY
        w.open_block("{");
        w.line(&format!("let _status = {write_call};"));
        w.open_block("if _status.is_success() {");
        for stmt in &write.not_invalid_key {
            generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
        }
        w.close_block("}");

        if !write.invalid_key.is_empty() {
            w.open_block("else {");
            for stmt in &write.invalid_key {
                generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
            }
            w.close_block("}");
        }
        w.close_block("}");
    } else {
        w.line(&format!("assert!({write_call}.is_success(), \"WRITE {}\");", write.record_name));
    }
}

fn generate_rewrite(w: &mut RustWriter, rw: &RewriteStatement, cmap: &ConditionMap, ptable: &[ParagraphIndex], rfm: &RecordFileMap, sfm: &SortFieldMap, gcm: &GroupChildMap, has_sql: bool) {
    let rec = cobol_to_rust_name(&rw.record_name, "");
    let file_field = rfm
        .get(&rw.record_name.to_uppercase())
        .map_or_else(|| format!("{rec}_file"), |f| cobol_to_rust_name(f, ""));

    // FROM clause
    if let Some(ref from_ref) = rw.from {
        let from_expr = data_ref_expr(from_ref);
        w.line(&format!(
            "ws.{rec}.set_raw_bytes(&{from_expr}.as_bytes()[..ws.{rec}.byte_length().min({from_expr}.byte_length())]);"
        ));
    }

    let rewrite_call = format!("ws.{file_field}.rewrite_record(ws.{rec}.as_bytes())");

    if !rw.invalid_key.is_empty() || !rw.not_invalid_key.is_empty() {
        w.open_block("{");
        w.line(&format!("let _status = {rewrite_call};"));
        w.open_block("if _status.is_success() {");
        for stmt in &rw.not_invalid_key {
            generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
        }
        w.close_block("}");

        if !rw.invalid_key.is_empty() {
            w.open_block("else {");
            for stmt in &rw.invalid_key {
                generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
            }
            w.close_block("}");
        }
        w.close_block("}");
    } else {
        w.line(&format!(
            "assert!({rewrite_call}.is_success(), \"REWRITE {}\");",
            rw.record_name
        ));
    }
}

#[allow(unused_variables)]
fn generate_delete(w: &mut RustWriter, del: &DeleteStatement, cmap: &ConditionMap, ptable: &[ParagraphIndex], rfm: &RecordFileMap, sfm: &SortFieldMap, gcm: &GroupChildMap, has_sql: bool) {
    let fname = cobol_to_rust_name(&del.file_name, "");
    let delete_call = format!("ws.{fname}.delete_record()");

    if !del.invalid_key.is_empty() || !del.not_invalid_key.is_empty() {
        w.open_block("{");
        w.line(&format!("let _status = {delete_call};"));
        w.open_block("if _status.is_success() {");
        for stmt in &del.not_invalid_key {
            generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
        }
        w.close_block("}");

        if !del.invalid_key.is_empty() {
            w.open_block("else {");
            for stmt in &del.invalid_key {
                generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
            }
            w.close_block("}");
        }
        w.close_block("}");
    } else {
        w.line(&format!(
            "assert!({delete_call}.is_success(), \"DELETE {}\");",
            del.file_name
        ));
    }
}

/// Generate Rust code for a START statement.
///
/// START positions a file cursor for subsequent sequential READ operations.
/// Supports KEY IS EQUAL/GREATER/NOT LESS conditions with INVALID KEY handlers.
fn generate_start(w: &mut RustWriter, start: &StartStatement, cmap: &ConditionMap, ptable: &[ParagraphIndex], rfm: &RecordFileMap, sfm: &SortFieldMap, gcm: &GroupChildMap, has_sql: bool) {
    let fname = cobol_to_rust_name(&start.file_name, "");

    let start_call = if let Some(ref cond) = start.key_condition {
        let key_expr = data_ref_expr(&cond.key);
        let op_str = match cond.op {
            ComparisonOp::GreaterThan | ComparisonOp::GreaterOrEqual => "std::cmp::Ordering::Greater",
            _ => "std::cmp::Ordering::Equal",
        };
        format!(
            "ws.{fname}.start({key_expr}.as_bytes(), {op_str})"
        )
    } else {
        format!("ws.{fname}.start(&[], std::cmp::Ordering::Equal)")
    };

    if !start.invalid_key.is_empty() || !start.not_invalid_key.is_empty() {
        w.open_block("{");
        w.line(&format!("let _status = {start_call};"));
        w.open_block("if _status.is_success() {");
        for stmt in &start.not_invalid_key {
            generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
        }
        w.close_block("}");

        if !start.invalid_key.is_empty() {
            w.open_block("else {");
            for stmt in &start.invalid_key {
                generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
            }
            w.close_block("}");
        }
        w.close_block("}");
    } else {
        w.line(&format!(
            "assert!({start_call}.is_success(), \"START {}\");",
            start.file_name
        ));
    }
}

/// Generate Rust code for a SET statement.
///
/// Handles SET...TO TRUE (88-level conditions), SET...TO value, SET...UP BY, SET...DOWN BY.
fn generate_set(w: &mut RustWriter, set: &SetStatement, cmap: &ConditionMap) {
    match &set.action {
        SetAction::ToBool(true) => {
            // SET condition-name TO TRUE
            // MOVE first value of 88-level to its parent field
            for target in &set.targets {
                let key = target.name.to_uppercase();
                if let Some(info) = cmap.get(&key) {
                    if let Some(first_val) = info.values.first() {
                        let parent = &info.parent_field;
                        match first_val {
                            ConditionValue::Single(lit) => {
                                if info.parent_is_numeric {
                                    let val = literal_to_decimal_expr(lit);
                                    w.line(&format!("{parent}.pack({val});"));
                                } else {
                                    let val = literal_to_bytes_expr(lit);
                                    w.line(&format!(
                                        "cobol_move(&PicX::new({val}.len(), {val}), &mut {parent}, &ctx.config);"
                                    ));
                                }
                            }
                            ConditionValue::Range { low, .. } => {
                                // SET TO TRUE on a range: use the low value
                                if info.parent_is_numeric {
                                    let val = literal_to_decimal_expr(low);
                                    w.line(&format!("{parent}.pack({val});"));
                                } else {
                                    let val = literal_to_bytes_expr(low);
                                    w.line(&format!(
                                        "cobol_move(&PicX::new({val}.len(), {val}), &mut {parent}, &ctx.config);"
                                    ));
                                }
                            }
                        }
                    }
                } else {
                    w.line(&format!(
                        "// SET {} TO TRUE -- condition not found in map",
                        target.name
                    ));
                }
            }
        }
        SetAction::ToBool(false) => {
            // SET condition-name TO FALSE (IBM extension)
            // Not commonly used; generate a comment
            for target in &set.targets {
                w.line(&format!(
                    "// SET {} TO FALSE -- not yet implemented",
                    target.name
                ));
            }
        }
        SetAction::To(value) => {
            // SET index/field TO value
            let val_expr = operand_numeric_expr(value);
            for target in &set.targets {
                let tgt = data_ref_expr(target);
                w.line(&format!(
                    "cobol_move_numeric(&{val_expr}, &mut {tgt}, &ctx.config);"
                ));
            }
        }
        SetAction::UpBy(value) => {
            // SET index UP BY value
            let val_expr = operand_numeric_expr(value);
            for target in &set.targets {
                let tgt = data_ref_expr(target);
                w.line(&format!(
                    "cobol_add(&{val_expr}, &mut {tgt}, None, &ctx.config);"
                ));
            }
        }
        SetAction::DownBy(value) => {
            // SET index DOWN BY value
            let val_expr = operand_numeric_expr(value);
            for target in &set.targets {
                let tgt = data_ref_expr(target);
                w.line(&format!(
                    "cobol_subtract(&{val_expr}, &mut {tgt}, None, &ctx.config);"
                ));
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Expression formatting helpers
// ---------------------------------------------------------------------------

/// Format an operand for use in comparisons.
/// Data references use `.to_decimal()` to enable comparison with Decimal values.
/// Check if an operand is a figurative constant.
fn is_figurative(op: &Operand) -> Option<FigurativeConstant> {
    if let Operand::Literal(Literal::Figurative(fig)) = op {
        Some(fig.clone())
    } else {
        None
    }
}

/// Check if an operand is an alphanumeric literal.
fn is_alpha_literal(op: &Operand) -> bool {
    matches!(op, Operand::Literal(Literal::Alphanumeric(_)))
}

fn operand_cmp_expr(op: &Operand) -> String {
    match op {
        Operand::DataRef(dr) => {
            let base = data_ref_expr(dr);
            format!("{base}.to_decimal()")
        }
        _ => operand_expr(op),
    }
}

/// Format an operand as a Rust expression.
fn operand_expr(op: &Operand) -> String {
    match op {
        Operand::Literal(lit) => literal_expr(lit),
        Operand::DataRef(dr) => data_ref_expr(dr),
        Operand::Function(f) => generate_intrinsic_call(f),
    }
}

/// Classify an intrinsic function by its argument and return types.
///
/// Returns: (param_kind, return_kind)
///   param_kind: "decimal" | "bytes" | "variadic_decimal" | "variadic_bytes" | "none" | "special"
///   return_kind: "decimal" | "bytes"
fn classify_intrinsic(name: &str) -> (&'static str, &'static str) {
    match name {
        // Category A: Decimal-in, Decimal-out
        "ABS" | "SQRT" | "LOG" | "LOG10" | "SIN" | "COS" | "TAN" | "ASIN" | "ACOS"
        | "ATAN" | "FACTORIAL" | "INTEGER" | "INTEGER-PART" | "RANDOM" => ("decimal", "decimal"),
        // Category A2: Two Decimal args
        "MOD" | "REM" => ("decimal", "decimal"),
        // Category B: Bytes-in, Decimal-out
        "LENGTH" | "ORD" | "NUMVAL" => ("bytes", "decimal"),
        "NUMVAL-C" => ("special", "decimal"), // 2 byte args
        // Category C: Bytes-in, Bytes-out
        "UPPER-CASE" | "LOWER-CASE" | "REVERSE" => ("bytes", "bytes"),
        "TRIM" => ("special", "bytes"), // bytes + trim_type
        "CONCATENATE" => ("variadic_bytes", "bytes"),
        // Category A3: Decimal-in, Bytes-out
        "CHAR" => ("decimal", "bytes"),
        // Category D: No args, Bytes-out
        "CURRENT-DATE" | "WHEN-COMPILED" => ("none", "bytes"),
        // Category E: Variadic Decimal args
        "MAX" | "MIN" | "ORD-MAX" | "ORD-MIN" => ("variadic_decimal", "decimal"),
        // Default: assume decimal-in, decimal-out
        _ => ("decimal", "decimal"),
    }
}

/// Generate a Rust expression for an intrinsic function call with correct arg types.
fn generate_intrinsic_call(f: &FunctionCall) -> String {
    let func_name = format!("cobol_function_{}", f.name.to_lowercase().replace('-', "_"));
    let (param_kind, _return_kind) = classify_intrinsic(&f.name);

    match param_kind {
        "decimal" => {
            let args: Vec<String> = f.arguments.iter().map(|a| operand_to_decimal_expr(a)).collect();
            format!("{func_name}({})", args.join(", "))
        }
        "bytes" => {
            let args: Vec<String> = f.arguments.iter().map(|a| {
                match a {
                    Operand::Literal(Literal::Alphanumeric(s)) => format!("b\"{s}\""),
                    Operand::DataRef(dr) => format!("{}.as_bytes()", data_ref_expr(dr)),
                    _ => format!("{}.as_bytes()", operand_expr(a)),
                }
            }).collect();
            format!("{func_name}({})", args.join(", "))
        }
        "variadic_decimal" => {
            let args: Vec<String> = f.arguments.iter().map(|a| operand_to_decimal_expr(a)).collect();
            format!("{func_name}(&[{}])", args.join(", "))
        }
        "variadic_bytes" => {
            let args: Vec<String> = f.arguments.iter().map(|a| {
                match a {
                    Operand::Literal(Literal::Alphanumeric(s)) => format!("b\"{s}\".as_slice()"),
                    Operand::DataRef(dr) => format!("{}.as_bytes()", data_ref_expr(dr)),
                    _ => format!("{}.as_bytes()", operand_expr(a)),
                }
            }).collect();
            format!("{func_name}(&[{}])", args.join(", "))
        }
        "none" => format!("{func_name}()"),
        "special" => {
            // Handle special cases
            if f.name == "TRIM" {
                let data_arg = if let Some(a) = f.arguments.first() {
                    match a {
                        Operand::Literal(Literal::Alphanumeric(s)) => format!("b\"{s}\""),
                        Operand::DataRef(dr) => format!("{}.as_bytes()", data_ref_expr(dr)),
                        _ => format!("{}.as_bytes()", operand_expr(a)),
                    }
                } else {
                    "b\"\"".to_string()
                };
                // TRIM type: 0=both, 1=leading, 2=trailing (default: both)
                format!("{func_name}({data_arg}, 0)")
            } else if f.name == "NUMVAL-C" {
                let args: Vec<String> = f.arguments.iter().map(|a| {
                    match a {
                        Operand::Literal(Literal::Alphanumeric(s)) => format!("b\"{s}\""),
                        Operand::DataRef(dr) => format!("{}.as_bytes()", data_ref_expr(dr)),
                        _ => format!("{}.as_bytes()", operand_expr(a)),
                    }
                }).collect();
                format!("{func_name}({})", args.join(", "))
            } else {
                let args: Vec<String> = f.arguments.iter().map(|a| operand_expr(a)).collect();
                format!("{func_name}({})", args.join(", "))
            }
        }
        _ => {
            let args: Vec<String> = f.arguments.iter().map(|a| operand_expr(a)).collect();
            format!("{func_name}({})", args.join(", "))
        }
    }
}

/// Generate a Rust expression for an intrinsic function returning Decimal.
/// For functions that return Vec<u8>, this is invalid in a COMPUTE context,
/// so we handle it gracefully.
fn generate_intrinsic_call_decimal(f: &FunctionCall) -> String {
    let (_param_kind, return_kind) = classify_intrinsic(&f.name);
    let call = generate_intrinsic_call(f);
    if return_kind == "bytes" {
        // String function in numeric context -- try to parse as decimal
        format!("{{ let _v = {call}; cobol_function_numval(&_v) }}")
    } else {
        call
    }
}

/// Generate a Rust expression for an intrinsic function returning bytes.
/// For functions that return Decimal, convert to string bytes.
fn generate_intrinsic_call_bytes(f: &FunctionCall) -> String {
    let (_param_kind, return_kind) = classify_intrinsic(&f.name);
    let call = generate_intrinsic_call(f);
    if return_kind == "decimal" {
        format!("{call}.to_string().into_bytes()")
    } else {
        call
    }
}

/// Format an operand as a Rust usize expression.
/// Numeric literals become plain integers, data refs use `decimal_to_usize()`.
fn operand_to_usize_expr(op: &Operand) -> String {
    match op {
        Operand::Literal(Literal::Numeric(n)) => {
            // Just use the integer part directly
            let int_part = n.split('.').next().unwrap_or(n);
            let int_part = int_part.trim_start_matches('+').trim_start_matches('-');
            format!("{int_part}usize")
        }
        Operand::DataRef(dr) => {
            let base = data_ref_expr(dr);
            format!("decimal_to_usize({base}.to_decimal())")
        }
        _ => {
            let expr = operand_expr(op);
            format!("decimal_to_usize({expr})")
        }
    }
}

/// Format an operand as a Rust expression suitable for arithmetic functions
/// (i.e. implements `CobolNumeric`). Numeric literals are wrapped in a
/// temporary `PackedDecimal` since bare `Decimal` which
/// does not implement `CobolNumeric`.
fn operand_numeric_expr(op: &Operand) -> String {
    match op {
        Operand::Literal(Literal::Numeric(n)) => {
            // Determine precision and scale from the literal text
            let (prec, scale) = numeric_literal_precision(n);
            let signed = n.starts_with('-') || n.starts_with('+');
            format!(
                "{{ let mut _tmp = PackedDecimal::new({prec}, {scale}, {signed}); _tmp.pack(\"{n}\".parse::<Decimal>().unwrap()); _tmp }}"
            )
        }
        _ => operand_expr(op),
    }
}

/// Compute (precision, scale) from a numeric literal string.
fn numeric_literal_precision(n: &str) -> (u8, u8) {
    let s = n.trim_start_matches(['-', '+']);
    if let Some(dot_pos) = s.find('.') {
        let int_part = &s[..dot_pos];
        let frac_part = &s[dot_pos + 1..];
        let int_digits = int_part.len().max(1) as u8;
        let frac_digits = frac_part.len() as u8;
        (int_digits + frac_digits, frac_digits)
    } else {
        let digits = s.len().max(1) as u8;
        (digits, 0)
    }
}

/// Format a literal as a Rust expression.
fn literal_expr(lit: &Literal) -> String {
    match lit {
        Literal::Numeric(n) => format!("\"{n}\".parse::<Decimal>().unwrap()"),
        Literal::Alphanumeric(s) => format!("\"{s}\""),
        Literal::Figurative(fig) => match fig {
            FigurativeConstant::Spaces => "FigurativeConstant::Spaces".to_string(),
            FigurativeConstant::Zeros => "FigurativeConstant::Zeros".to_string(),
            FigurativeConstant::HighValues => "FigurativeConstant::HighValues".to_string(),
            FigurativeConstant::LowValues => "FigurativeConstant::LowValues".to_string(),
            FigurativeConstant::Quotes => "FigurativeConstant::Quotes".to_string(),
            FigurativeConstant::Nulls => "FigurativeConstant::Nulls".to_string(),
            FigurativeConstant::All(s) => format!("b\"{s}\""),
        },
    }
}

/// Format a data reference as a Rust expression (base: field + subscripts only).
///
/// Does NOT apply reference modification. Used for write targets in MOVE
/// and for subscript sub-expressions.
fn data_ref_base_expr(dr: &DataReference) -> String {
    // If qualified (e.g., WS-NAME OF WS-DST-REC), use qualifier as prefix
    // to match the disambiguated field name from data_gen
    let field_name = if !dr.qualifiers.is_empty() {
        cobol_to_rust_name(&dr.name, &dr.qualifiers[0])
    } else {
        cobol_to_rust_name(&dr.name, "")
    };
    let mut expr = format!("ws.{field_name}");

    // Add subscripts
    for sub in &dr.subscripts {
        match sub {
            Subscript::IntLiteral(n) => {
                // COBOL is 1-based, Rust is 0-based
                let idx = (*n).max(1) - 1;
                expr = format!("{expr}[{idx}]");
            }
            Subscript::DataRef(sub_dr) => {
                let sub_expr = data_ref_base_expr(sub_dr);
                // Convert COBOL 1-based subscript to Rust 0-based index
                expr = format!("{expr}[decimal_to_usize({sub_expr}.to_decimal()).saturating_sub(1)]");
            }
            Subscript::Expr(_) => {
                expr = format!("{expr}[0 /* complex subscript */]");
            }
        }
    }

    expr
}

/// Generate a Rust expression evaluating to usize for `ref_mod` offset/length.
fn ref_mod_index_expr(expr: &ArithExpr) -> String {
    match expr {
        ArithExpr::Operand(Operand::Literal(Literal::Numeric(n))) => {
            format!("{n}usize")
        }
        ArithExpr::Operand(Operand::DataRef(dr)) => {
            let base = data_ref_base_expr(dr);
            format!("decimal_to_usize({base}.to_decimal())")
        }
        _ => {
            let e = arith_expr_str(expr);
            format!("decimal_to_usize({e})")
        }
    }
}

/// Format a data reference as a Rust expression for read contexts.
///
/// Applies reference modification when present, wrapping the result
/// in a temporary `PicX` (since ref-mod always produces alphanumeric).
fn data_ref_expr(dr: &DataReference) -> String {
    let base = data_ref_base_expr(dr);

    if let Some(ref rm) = dr.ref_mod {
        let offset = ref_mod_index_expr(&rm.offset);
        if let Some(ref len) = rm.length {
            let length = ref_mod_index_expr(len);
            format!(
                "{{ let _rm = ref_mod_read(&{base}, {offset}, {length}); PicX::new(_rm.len(), &_rm) }}"
            )
        } else {
            format!(
                "{{ let _rm = ref_mod_read_to_end(&{base}, {offset}); PicX::new(_rm.len(), &_rm) }}"
            )
        }
    } else {
        base
    }
}

/// Format an arithmetic expression as a Rust expression evaluating to `Decimal`.
///
/// Converts all operands to `Decimal` via `.to_decimal()` so that standard
/// Rust arithmetic operators work. `Decimal` implements `std::ops` traits,
/// while `PackedDecimal` and other `CobolNumeric` types do not.
fn arith_expr_str(expr: &ArithExpr) -> String {
    match expr {
        ArithExpr::Operand(op) => operand_to_decimal_expr(op),
        ArithExpr::Negate(inner) => format!("-({})", arith_expr_str(inner)),
        ArithExpr::BinaryOp { left, op, right } => {
            let l = arith_expr_str(left);
            let r = arith_expr_str(right);
            match op {
                ArithOp::Add => format!("({l} + {r})"),
                ArithOp::Subtract => format!("({l} - {r})"),
                ArithOp::Multiply => format!("({l} * {r})"),
                ArithOp::Divide => format!("cobol_checked_div({l}, {r})"),
                ArithOp::Power => {
                    // Power via f64 since Decimal has no built-in pow
                    format!(
                        "Decimal::from_f64_retain(({l}).to_f64().unwrap_or(0.0).powf(({r}).to_f64().unwrap_or(0.0))).unwrap_or(Decimal::ZERO)"
                    )
                }
            }
        }
        ArithExpr::Paren(inner) => format!("({})", arith_expr_str(inner)),
    }
}

/// Convert an operand to a `Decimal` expression for use in COMPUTE.
///
/// Field references get `.to_decimal()`, numeric literals get `.parse::<Decimal>()`,
/// and intrinsic function calls are passed through (they already return `Decimal`).
fn operand_to_decimal_expr(op: &Operand) -> String {
    match op {
        Operand::Literal(Literal::Numeric(n)) => format!("\"{n}\".parse::<Decimal>().unwrap()"),
        Operand::Literal(_) => {
            // Non-numeric literals in arithmetic context -- fall back to operand_expr
            operand_expr(op)
        }
        Operand::DataRef(dr) => {
            let base = data_ref_expr(dr);
            format!("{base}.to_decimal()")
        }
        Operand::Function(f) => generate_intrinsic_call_decimal(f),
    }
}

/// Generate a comparison expression handling figurative constants and
/// alphanumeric vs numeric comparison dispatch.
fn generate_comparison_expr(left: &Operand, op: ComparisonOp, right: &Operand) -> String {
    let op_str = match op {
        ComparisonOp::Equal => "==",
        ComparisonOp::NotEqual => "!=",
        ComparisonOp::LessThan => "<",
        ComparisonOp::GreaterThan => ">",
        ComparisonOp::LessOrEqual => "<=",
        ComparisonOp::GreaterOrEqual => ">=",
    };

    // Case 1: field vs figurative constant
    if let (Operand::DataRef(dr), Some(fig)) = (left, is_figurative(right)) {
        return field_vs_figurative_cmp(dr, op, fig);
    }
    if let (Some(fig), Operand::DataRef(dr)) = (is_figurative(left), right) {
        // Reverse the operator for swapped operands
        let rev_op = match op {
            ComparisonOp::Equal => ComparisonOp::Equal,
            ComparisonOp::NotEqual => ComparisonOp::NotEqual,
            ComparisonOp::LessThan => ComparisonOp::GreaterThan,
            ComparisonOp::GreaterThan => ComparisonOp::LessThan,
            ComparisonOp::LessOrEqual => ComparisonOp::GreaterOrEqual,
            ComparisonOp::GreaterOrEqual => ComparisonOp::LessOrEqual,
        };
        return field_vs_figurative_cmp(dr, rev_op, fig);
    }

    // Case 2: field vs alphanumeric literal
    if let (Operand::DataRef(dr), true) = (left, is_alpha_literal(right)) {
        let field = data_ref_expr(dr);
        let rhs = operand_expr(right);
        return format!("{field}.as_bytes() {op_str} {rhs}.as_bytes()");
    }
    if let (true, Operand::DataRef(dr)) = (is_alpha_literal(left), right) {
        let field = data_ref_expr(dr);
        let lhs = operand_expr(left);
        return format!("{lhs}.as_bytes() {op_str} {field}.as_bytes()");
    }

    // Case 3a: DataRef vs DataRef -- use as_bytes() (works for both alpha and numeric display)
    if let (Operand::DataRef(dr_l), Operand::DataRef(dr_r)) = (left, right) {
        let l = data_ref_expr(dr_l);
        let r = data_ref_expr(dr_r);
        return format!("{l}.as_bytes() {op_str} {r}.as_bytes()");
    }

    // Case 3b: DataRef vs numeric literal -- use .to_decimal()
    let l = operand_cmp_expr(left);
    let r = operand_cmp_expr(right);
    format!("{l} {op_str} {r}")
}

/// Generate comparison of a field against a figurative constant.
fn field_vs_figurative_cmp(dr: &DataReference, op: ComparisonOp, fig: FigurativeConstant) -> String {
    let field = data_ref_expr(dr);
    match fig {
        FigurativeConstant::Spaces => {
            let test = format!("{field}.as_bytes().iter().all(|&b| b == b' ')");
            match op {
                ComparisonOp::Equal => test,
                ComparisonOp::NotEqual => format!("!({test})"),
                _ => {
                    // For ordering comparisons, compare against space-filled field
                    let op_str = match op {
                        ComparisonOp::LessThan => "<",
                        ComparisonOp::GreaterThan => ">",
                        ComparisonOp::LessOrEqual => "<=",
                        ComparisonOp::GreaterOrEqual => ">=",
                        _ => unreachable!(),
                    };
                    format!("{field}.as_bytes() {op_str} &vec![b' '; {field}.byte_length()][..]")
                }
            }
        }
        FigurativeConstant::Zeros => {
            // ZEROS: for numeric fields compare decimal, for alpha compare bytes
            let test_numeric = format!("{field}.to_decimal() == Decimal::ZERO");
            match op {
                ComparisonOp::Equal => test_numeric,
                ComparisonOp::NotEqual => format!("!({test_numeric})"),
                _ => {
                    let op_str = match op {
                        ComparisonOp::LessThan => "<",
                        ComparisonOp::GreaterThan => ">",
                        ComparisonOp::LessOrEqual => "<=",
                        ComparisonOp::GreaterOrEqual => ">=",
                        _ => unreachable!(),
                    };
                    format!("{field}.to_decimal() {op_str} Decimal::ZERO")
                }
            }
        }
        FigurativeConstant::HighValues => {
            let test = format!("{field}.as_bytes().iter().all(|&b| b == 0xFF)");
            match op {
                ComparisonOp::Equal => test,
                ComparisonOp::NotEqual => format!("!({test})"),
                _ => "/* HIGH-VALUES ordering not supported */ true".to_string(),
            }
        }
        FigurativeConstant::LowValues => {
            let test = format!("{field}.as_bytes().iter().all(|&b| b == 0x00)");
            match op {
                ComparisonOp::Equal => test,
                ComparisonOp::NotEqual => format!("!({test})"),
                _ => "/* LOW-VALUES ordering not supported */ true".to_string(),
            }
        }
        _ => {
            // Quotes, Nulls -- fallback to byte comparison
            "/* figurative constant comparison */ true".to_string()
        }
    }
}

/// Format a condition as a Rust expression.
/// Uses `.to_decimal()` for numeric comparisons since `PackedDecimal`
/// doesn't implement `PartialOrd` directly.
fn condition_expr(cond: &Condition, cmap: &ConditionMap) -> String {
    match cond {
        Condition::Comparison { left, op, right } => {
            generate_comparison_expr(left, *op, right)
        }
        Condition::ClassTest { field, class } => {
            let f = data_ref_expr(field);
            let method = match class {
                ClassCondition::Numeric => "is_numeric",
                ClassCondition::Alphabetic => "is_alphabetic",
                ClassCondition::AlphabeticLower => "is_alphabetic_lower",
                ClassCondition::AlphabeticUpper => "is_alphabetic_upper",
            };
            format!("{f}.{method}()")
        }
        Condition::SignTest { field, sign } => {
            let f = data_ref_expr(field);
            let method = match sign {
                SignCondition::Positive => "is_positive",
                SignCondition::Negative => "is_negative",
                SignCondition::Zero => "is_zero",
            };
            format!("{f}.{method}()")
        }
        Condition::ConditionName(dr) => {
            condition_name_expr(dr, cmap)
        }
        Condition::Not(inner) => {
            format!("!({})", condition_expr(inner, cmap))
        }
        Condition::And(left, right) => {
            // Note: IBM COBOL evaluates BOTH sides (no short-circuit).
            // We generate standard Rust && but add a comment.
            format!(
                "({} && {})",
                condition_expr(left, cmap),
                condition_expr(right, cmap)
            )
        }
        Condition::Or(left, right) => {
            format!(
                "({} || {})",
                condition_expr(left, cmap),
                condition_expr(right, cmap)
            )
        }
    }
}

/// Generate an expression for an 88-level condition name test.
///
/// Looks up the condition in the `ConditionMap`. If found, generates inline
/// comparisons against the parent field. Falls back to `.is_true()` if not found.
fn condition_name_expr(dr: &DataReference, cmap: &ConditionMap) -> String {
    let key = dr.name.to_uppercase();
    if let Some(info) = cmap.get(&key) {
        let parent = &info.parent_field;
        let parts: Vec<String> = info.values.iter().map(|cv| {
            match cv {
                ConditionValue::Single(lit) => {
                    if info.parent_is_numeric {
                        // Numeric comparison: compare Decimal values
                        let val = literal_to_decimal_expr(lit);
                        format!("{parent}.to_decimal() == {val}")
                    } else {
                        // Alphanumeric comparison: compare bytes
                        let val = literal_to_bytes_expr(lit);
                        format!("{parent}.as_bytes() == {val}")
                    }
                }
                ConditionValue::Range { low, high } => {
                    if info.parent_is_numeric {
                        let lo = literal_to_decimal_expr(low);
                        let hi = literal_to_decimal_expr(high);
                        format!(
                            "({parent}.to_decimal() >= {lo} && {parent}.to_decimal() <= {hi})"
                        )
                    } else {
                        let lo = literal_to_bytes_expr(low);
                        let hi = literal_to_bytes_expr(high);
                        format!(
                            "({parent}.as_bytes() >= {lo} && {parent}.as_bytes() <= {hi})"
                        )
                    }
                }
            }
        }).collect();

        if parts.len() == 1 {
            parts.into_iter().next().unwrap()
        } else {
            format!("({})", parts.join(" || "))
        }
    } else {
        // Fallback: condition not in map, generate as field reference
        let f = data_ref_expr(dr);
        format!("/* 88-level {key} not resolved */ {f}.is_true()")
    }
}

/// Convert a Literal to a Decimal expression string for codegen.
fn literal_to_decimal_expr(lit: &Literal) -> String {
    match lit {
        Literal::Numeric(n) => format!("\"{n}\".parse::<Decimal>().unwrap()"),
        Literal::Alphanumeric(s) => format!("\"{s}\".parse::<Decimal>().unwrap()"),
        Literal::Figurative(_) => "Decimal::ZERO".to_string(),
    }
}

/// Convert a Literal to a byte-slice expression string for codegen.
fn literal_to_bytes_expr(lit: &Literal) -> String {
    match lit {
        Literal::Numeric(n) => format!("b\"{n}\""),
        Literal::Alphanumeric(s) => {
            let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
            format!("b\"{escaped}\"")
        }
        Literal::Figurative(fig) => match fig {
            FigurativeConstant::Spaces => "b\" \"".to_string(),
            FigurativeConstant::Zeros => "b\"0\"".to_string(),
            FigurativeConstant::HighValues => "b\"\\xFF\"".to_string(),
            FigurativeConstant::LowValues | FigurativeConstant::Nulls => "b\"\\x00\"".to_string(),
            FigurativeConstant::Quotes => "b\"\\\"\"".to_string(),
            FigurativeConstant::All(s) => {
                let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
                format!("b\"{escaped}\"")
            }
        },
    }
}

// ---------------------------------------------------------------------------
// Phase 3 statement generators: SORT, MERGE, RELEASE, RETURN, INSPECT, STRING, UNSTRING
// ---------------------------------------------------------------------------

fn generate_sort(w: &mut RustWriter, sort: &SortStatement, _cmap: &ConditionMap, sfm: &SortFieldMap) {
    w.open_block("{");

    // Build key specs from sort field map
    w.line("let sort_keys = vec![");
    for key in &sort.keys {
        let key_upper = key.field.name.to_uppercase();
        let asc = key.ascending;
        if let Some(&(offset, length)) = sfm.get(&key_upper) {
            w.line(&format!(
                "    SortKeySpec::new({offset}, {length}, {asc}, SortKeyType::Alphanumeric),"
            ));
        } else {
            // Fallback: use field byte_length at runtime
            let field_name = cobol_to_rust_name(&key.field.name, "");
            w.line(&format!(
                "    SortKeySpec::new(0, ws.{field_name}.byte_length(), {asc}, SortKeyType::Alphanumeric),"
            ));
        }
    }
    w.line("];");

    // Config -- compute record length from sort field map or use a default
    let reclen_key = format!("__RECLEN_{}", sort.file_name.to_uppercase());
    let reclen = sfm.get(&reclen_key).map_or(80, |&(_, len)| len);
    let stable = sort.duplicates;
    w.line(&format!(
        "let sort_config = SortConfig::new({reclen}).with_stable({stable});"
    ));

    // Determine which combination of INPUT/OUTPUT we have
    let has_input_proc = matches!(&sort.input, SortInput::InputProcedure { .. });
    let has_output_proc = matches!(&sort.output, SortOutput::OutputProcedure { .. });

    match (has_input_proc, has_output_proc) {
        // USING + GIVING: file-to-file sort
        (false, false) => {
            generate_sort_using_giving(w, sort);
        }
        // INPUT PROCEDURE + GIVING: procedural input, file output
        (true, false) => {
            let proc_name = if let SortInput::InputProcedure { name, .. } = &sort.input {
                cobol_to_rust_name(name, "")
            } else { unreachable!() };

            w.line(&format!(
                "let sorted_records = sort_with_input_procedure(&sort_keys, &sort_config, None, |releaser| {{ {proc_name}(ws, ctx, releaser); }}).unwrap_or_default();"
            ));

            // Write sorted records to GIVING file(s)
            if let SortOutput::Giving(files) = &sort.output {
                for f in files {
                    let n = cobol_to_rust_name(f, "");
                    w.line(&format!("assert!(ws.{n}.open(FileOpenMode::Output).is_success(), \"SORT GIVING OPEN\");"));
                    w.open_block("for rec in &sorted_records {");
                    w.line(&format!("ws.{n}.write_record(rec);"));
                    w.close_block("}");
                    w.line(&format!("assert!(ws.{n}.close().is_success(), \"SORT GIVING CLOSE\");"));
                }
            }
        }
        // USING + OUTPUT PROCEDURE: file input, procedural output
        (false, true) => {
            let proc_name = if let SortOutput::OutputProcedure { name, .. } = &sort.output {
                cobol_to_rust_name(name, "")
            } else { unreachable!() };

            // Read records from USING files
            w.line("let mut input_records: Vec<Vec<u8>> = Vec::new();");
            if let SortInput::Using(files) = &sort.input {
                for f in files {
                    let n = cobol_to_rust_name(f, "");
                    w.line(&format!("assert!(ws.{n}.open(FileOpenMode::Input).is_success(), \"SORT USING OPEN\");"));
                    w.open_block("loop {");
                    w.line(&format!("let (_st, _data) = ws.{n}.read_next();"));
                    w.line("if !_st.is_success() { break; }");
                    w.line("if let Some(d) = _data { input_records.push(d); }");
                    w.close_block("}");
                    w.line(&format!("assert!(ws.{n}.close().is_success(), \"SORT USING CLOSE\");"));
                }
            }

            w.line(&format!(
                "let _ = sort_with_output_procedure(&sort_keys, &sort_config, None, input_records, |returner| {{ {proc_name}(ws, ctx, returner); }});"
            ));
        }
        // INPUT PROCEDURE + OUTPUT PROCEDURE: both procedural
        (true, true) => {
            let in_proc = if let SortInput::InputProcedure { name, .. } = &sort.input {
                cobol_to_rust_name(name, "")
            } else { unreachable!() };
            let out_proc = if let SortOutput::OutputProcedure { name, .. } = &sort.output {
                cobol_to_rust_name(name, "")
            } else { unreachable!() };

            w.line(&format!(
                "let _ = sort_with_procedures(&sort_keys, &sort_config, None, |releaser| {{ {in_proc}(ws, ctx, releaser); }}, |returner| {{ {out_proc}(ws, ctx, returner); }});"
            ));
        }
    }

    w.close_block("}");
}

/// Generate SORT USING/GIVING (file-to-file sort with no procedures).
fn generate_sort_using_giving(w: &mut RustWriter, sort: &SortStatement) {
    // Input files
    if let SortInput::Using(files) = &sort.input {
        let input_refs: Vec<String> = files
            .iter()
            .map(|f| {
                let n = cobol_to_rust_name(f, "");
                format!("&mut ws.{n} as &mut dyn CobolFile")
            })
            .collect();
        w.line(&format!(
            "let mut sort_inputs: Vec<&mut dyn CobolFile> = vec![{}];",
            input_refs.join(", ")
        ));
    }

    // Output files
    if let SortOutput::Giving(files) = &sort.output {
        let output_refs: Vec<String> = files
            .iter()
            .map(|f| {
                let n = cobol_to_rust_name(f, "");
                format!("&mut ws.{n} as &mut dyn CobolFile")
            })
            .collect();
        w.line(&format!(
            "let mut sort_outputs: Vec<&mut dyn CobolFile> = vec![{}];",
            output_refs.join(", ")
        ));
    }

    // Execute sort
    w.line("let _sort_result = CobolSortEngine::sort_using_giving(");
    w.line("    &sort_keys, &sort_config, None,");
    w.line("    &mut sort_inputs.iter_mut().map(|f| &mut **f).collect::<Vec<_>>(),");
    w.line("    &mut sort_outputs.iter_mut().map(|f| &mut **f).collect::<Vec<_>>(),");
    w.line(");");
}

fn generate_merge(w: &mut RustWriter, merge: &MergeStatement, _cmap: &ConditionMap, sfm: &SortFieldMap) {
    let fname = cobol_to_rust_name(&merge.file_name, "");
    w.open_block("{");

    // Build key specs from sort field map
    w.line("let merge_keys = vec![");
    for key in &merge.keys {
        let key_upper = key.field.name.to_uppercase();
        let asc = key.ascending;
        if let Some(&(offset, length)) = sfm.get(&key_upper) {
            w.line(&format!(
                "    SortKeySpec::new({offset}, {length}, {asc}, SortKeyType::Alphanumeric),"
            ));
        } else {
            let field_name = cobol_to_rust_name(&key.field.name, "");
            w.line(&format!(
                "    SortKeySpec::new(0, ws.{field_name}.byte_length(), {asc}, SortKeyType::Alphanumeric),"
            ));
        }
    }
    w.line("];");

    w.line("let merge_engine = CobolMergeEngine::new(&merge_keys, None);");

    // Using files
    let input_refs: Vec<String> = merge
        .using
        .iter()
        .map(|f| {
            let n = cobol_to_rust_name(f, "");
            format!("&mut ws.{n} as &mut dyn CobolFile")
        })
        .collect();
    w.line(&format!(
        "let mut merge_inputs: Vec<&mut dyn CobolFile> = vec![{}];",
        input_refs.join(", ")
    ));

    // Output
    match &merge.output {
        SortOutput::Giving(files) => {
            let output_ref = files
                .first()
                .map_or_else(|| fname.clone(), |f| cobol_to_rust_name(f, ""));
            w.line("let _merge_result = merge_engine.merge_files(");
            w.line("    &mut merge_inputs.iter_mut().map(|f| &mut **f).collect::<Vec<_>>(),");
            w.line(&format!("    &mut ws.{output_ref},"));
            w.line(");");
        }
        SortOutput::OutputProcedure { name, .. } => {
            let proc_name = cobol_to_rust_name(name, "para");
            w.line(&format!(
                "// OUTPUT PROCEDURE: {proc_name} would use RETURN verb"
            ));
        }
    }

    w.close_block("}");
}

fn generate_release(w: &mut RustWriter, rel: &ReleaseStatement) {
    let rec = cobol_to_rust_name(&rel.record_name, "");

    if let Some(ref from_ref) = rel.from {
        let from_expr = data_ref_expr(from_ref);
        w.line(&format!(
            "releaser.release({from_expr}.as_bytes());"
        ));
    } else {
        w.line(&format!("releaser.release(ws.{rec}.as_bytes());"));
    }
}

fn generate_return(w: &mut RustWriter, ret: &ReturnStatement, cmap: &ConditionMap, ptable: &[ParagraphIndex], rfm: &RecordFileMap, sfm: &SortFieldMap, gcm: &GroupChildMap, has_sql: bool) {
    let fname = cobol_to_rust_name(&ret.file_name, "");

    if !ret.at_end.is_empty() || !ret.not_at_end.is_empty() {
        w.open_block("match returner.return_record() {");

        w.open_block("Some(record_data) => {");
        if let Some(ref into_ref) = ret.into {
            let into_expr = data_ref_expr(into_ref);
            w.line(&format!(
                "{into_expr}.set_raw_bytes(&record_data[..{into_expr}.byte_length().min(record_data.len())]);"
            ));
        }
        for stmt in &ret.not_at_end {
            generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
        }
        w.close_block("}");

        w.open_block("None => {");
        for stmt in &ret.at_end {
            generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
        }
        w.close_block("}");

        w.close_block("}");
    } else {
        w.line(&format!(
            "let _ = returner.return_record(); // RETURN {fname}"
        ));
    }
}

fn generate_inspect(w: &mut RustWriter, insp: &InspectStatement, _cmap: &ConditionMap) {
    let target = data_ref_expr(&insp.target);

    // TALLYING
    for tally in &insp.tallying {
        let counter = data_ref_expr(&tally.counter);
        match &tally.what {
            InspectWhat::Characters => {
                w.line("{ let specs = vec![TallyingSpec { what: InspectWhat::Characters, bounds: vec![] }];");
                w.line(&format!(
                    "let counts = cobol_inspect_tallying({target}.as_bytes(), &specs);"
                ));
                w.line(&format!(
                    "let val = {counter}.to_decimal() + Decimal::from(counts[0]); {counter}.pack(val); }}"
                ));
            }
            InspectWhat::All(pattern) | InspectWhat::Leading(pattern) | InspectWhat::First(pattern) => {
                let variant = match &tally.what {
                    InspectWhat::All(_) => "All",
                    InspectWhat::Leading(_) => "Leading",
                    InspectWhat::First(_) => "First",
                    InspectWhat::Characters => unreachable!(),
                };
                let pat_expr = operand_to_bytes_expr(pattern);
                w.line(&format!(
                    "{{ let pat = {pat_expr};"
                ));
                w.line(&format!(
                    "let specs = vec![TallyingSpec {{ what: InspectWhat::{variant}(pat.to_vec()), bounds: vec![] }}];"
                ));
                w.line(&format!(
                    "let counts = cobol_inspect_tallying({target}.as_bytes(), &specs);"
                ));
                w.line(&format!(
                    "let val = {counter}.to_decimal() + Decimal::from(counts[0]); {counter}.pack(val); }}"
                ));
            }
        }
    }

    // REPLACING
    if !insp.replacing.is_empty() {
        w.open_block("{");
        w.line(&format!(
            "let mut target_bytes = {target}.as_bytes().to_vec();"
        ));

        w.line("let specs = vec![");
        for rep in &insp.replacing {
            let by_expr = operand_to_bytes_expr(&rep.by);
            match &rep.what {
                InspectWhat::Characters => {
                    w.line(&format!(
                        "    ReplacingSpec {{ what: InspectWhat::Characters, by: {by_expr}.to_vec(), bounds: vec![] }},"
                    ));
                }
                InspectWhat::All(pattern) | InspectWhat::Leading(pattern) | InspectWhat::First(pattern) => {
                    let variant = match &rep.what {
                        InspectWhat::All(_) => "All",
                        InspectWhat::Leading(_) => "Leading",
                        InspectWhat::First(_) => "First",
                        InspectWhat::Characters => unreachable!(),
                    };
                    let pat_expr = operand_to_bytes_expr(pattern);
                    w.line(&format!(
                        "    ReplacingSpec {{ what: InspectWhat::{variant}({pat_expr}.to_vec()), by: {by_expr}.to_vec(), bounds: vec![] }},"
                    ));
                }
            }
        }
        w.line("];");

        w.line("cobol_inspect_replacing(&mut target_bytes, &specs);");
        w.line(&format!("{target}.set_raw_bytes(&target_bytes);"));
        w.close_block("}");
    }

    // CONVERTING
    if let Some(ref conv) = insp.converting {
        let from_expr = operand_to_bytes_expr(&conv.from);
        let to_expr = operand_to_bytes_expr(&conv.to);
        w.open_block("{");
        w.line(&format!(
            "let mut target_bytes = {target}.as_bytes().to_vec();"
        ));
        w.line(&format!(
            "cobol_inspect_converting(&mut target_bytes, {from_expr}, {to_expr}, &[]);"
        ));
        w.line(&format!("{target}.set_raw_bytes(&target_bytes);"));
        w.close_block("}");
    }
}

/// Format an operand as a byte slice expression for INSPECT/STRING/UNSTRING.
fn operand_to_bytes_expr(op: &Operand) -> String {
    match op {
        Operand::Literal(Literal::Alphanumeric(s)) => format!("b\"{s}\""),
        Operand::Literal(Literal::Numeric(n)) => format!("b\"{n}\""),
        Operand::DataRef(dr) => {
            if let Some(rm) = &dr.ref_mod {
                // ref_mod_read returns Vec<u8>, use directly
                let base = data_ref_base_expr(dr);
                let offset = ref_mod_index_expr(&rm.offset);
                if let Some(ref len) = rm.length {
                    let length = ref_mod_index_expr(len);
                    format!("ref_mod_read(&{base}, {offset}, {length})")
                } else {
                    format!("ref_mod_read_to_end(&{base}, {offset})")
                }
            } else {
                let expr = data_ref_base_expr(dr);
                format!("{expr}.as_bytes()")
            }
        }
        _ => "b\"\"".to_string(),
    }
}

fn generate_string(w: &mut RustWriter, s: &StringStatement, cmap: &ConditionMap, ptable: &[ParagraphIndex], rfm: &RecordFileMap, sfm: &SortFieldMap, gcm: &GroupChildMap, has_sql: bool) {
    let into = data_ref_expr(&s.into);

    w.open_block("{");

    // Build source specs
    w.line("let sources = vec![");
    for src in &s.sources {
        let data_expr = operand_to_bytes_expr(&src.operand);
        match &src.delimited_by {
            StringDelimiter::Size => {
                w.line(&format!(
                    "    StringSourceSpec::by_size({data_expr}),"
                ));
            }
            StringDelimiter::Literal(delim) => {
                let delim_expr = operand_to_bytes_expr(delim);
                w.line(&format!(
                    "    StringSourceSpec::by_literal({data_expr}, {delim_expr}),"
                ));
            }
        }
    }
    w.line("];");

    // Target buffer
    w.line(&format!(
        "let mut target_bytes = {into}.as_bytes().to_vec();"
    ));

    // Pointer
    if let Some(ref ptr) = s.pointer {
        let ptr_expr = data_ref_expr(ptr);
        w.line(&format!(
            "let mut ptr = {ptr_expr}.to_decimal().to_u32().unwrap_or(1) as usize;"
        ));
    } else {
        w.line("let mut ptr = 1usize;");
    }

    // Execute
    w.line(
        "let string_result = cobol_string(&sources, &mut target_bytes, &mut ptr);",
    );
    w.line(&format!("{into}.set_raw_bytes(&target_bytes);"));

    // Update pointer field
    if let Some(ref ptr) = s.pointer {
        let ptr_expr = data_ref_expr(ptr);
        w.line(&format!(
            "{ptr_expr}.pack(Decimal::from(ptr as u32));"
        ));
    }

    // Overflow handling
    if !s.on_overflow.is_empty() || !s.not_on_overflow.is_empty() {
        w.open_block("if string_result == StringResult::Overflow {");
        for stmt in &s.on_overflow {
            generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
        }
        w.dedent();
        w.open_block("} else {");
        for stmt in &s.not_on_overflow {
            generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
        }
        w.close_block("}");
    }

    w.close_block("}");
}

fn generate_unstring(w: &mut RustWriter, u: &UnstringStatement, cmap: &ConditionMap, ptable: &[ParagraphIndex], rfm: &RecordFileMap, sfm: &SortFieldMap, gcm: &GroupChildMap, has_sql: bool) {
    let source = data_ref_expr(&u.source);

    w.open_block("{");

    // Build delimiter specs
    w.line("let delimiters = vec![");
    for delim in &u.delimiters {
        let val_expr = operand_to_bytes_expr(&delim.value);
        let all = delim.all;
        w.line(&format!(
            "    UnstringDelimSpec::new({val_expr}, {all}),"
        ));
    }
    w.line("];");

    // Build target buffers
    let target_count = u.into.len();
    for (i, into) in u.into.iter().enumerate() {
        let tgt = data_ref_expr(&into.target);
        w.line(&format!(
            "let mut unstr_buf_{i} = vec![b' '; {tgt}.byte_length()];"
        ));
    }

    // Build into slice
    w.line(&format!(
        "let mut into_slices: Vec<&mut [u8]> = vec![{}];",
        (0..target_count)
            .map(|i| format!("&mut unstr_buf_{i}"))
            .collect::<Vec<_>>()
            .join(", ")
    ));

    // Pointer
    if let Some(ref ptr) = u.pointer {
        let ptr_expr = data_ref_expr(ptr);
        w.line(&format!(
            "let mut ptr = {ptr_expr}.to_decimal().to_u32().unwrap_or(1) as usize;"
        ));
    } else {
        w.line("let mut ptr = 1usize;");
    }

    // Tallying
    if let Some(ref tally) = u.tallying {
        let tally_expr = data_ref_expr(tally);
        w.line(&format!(
            "let mut tally = {tally_expr}.to_decimal().to_u32().unwrap_or(0) as usize;"
        ));
    }

    // Execute
    let tally_arg = if u.tallying.is_some() {
        "Some(&mut tally)"
    } else {
        "None"
    };
    w.line(&format!(
        "let unstring_result = cobol_unstring_simple({source}.as_bytes(), &delimiters, &mut into_slices, &mut ptr, {tally_arg});",
    ));

    // Copy results back to fields
    for (i, into) in u.into.iter().enumerate() {
        let tgt = data_ref_expr(&into.target);
        w.line(&format!("{tgt}.set_raw_bytes(&unstr_buf_{i});"));
    }

    // Update pointer
    if let Some(ref ptr) = u.pointer {
        let ptr_expr = data_ref_expr(ptr);
        w.line(&format!(
            "{ptr_expr}.pack(Decimal::from(ptr as u32));"
        ));
    }

    // Update tallying
    if let Some(ref tally) = u.tallying {
        let tally_expr = data_ref_expr(tally);
        w.line(&format!(
            "{tally_expr}.pack(Decimal::from(tally as u32));"
        ));
    }

    // Overflow handling
    if !u.on_overflow.is_empty() || !u.not_on_overflow.is_empty() {
        w.open_block("if unstring_result == UnstringResult::Overflow {");
        for stmt in &u.on_overflow {
            generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
        }
        w.dedent();
        w.open_block("} else {");
        for stmt in &u.not_on_overflow {
            generate_statement(w, stmt, cmap, ptable, rfm, sfm, gcm, has_sql);
        }
        w.close_block("}");
    }

    w.close_block("}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{
        CallParam, DisplayTarget, InspectConverting, InspectReplacing, InspectTallying,
        OpenFile, PerformTarget, RefMod, SortKey, StartKeyCondition, StringSource,
        UnstringDelimiter, UnstringInto, Usage, WhenBranch,
    };

    fn empty_cmap() -> ConditionMap {
        ConditionMap::new()
    }

    #[test]
    fn operand_formatting() {
        let op = Operand::Literal(Literal::Numeric("42".to_string()));
        assert_eq!(operand_expr(&op), r#""42".parse::<Decimal>().unwrap()"#);

        let op = Operand::Literal(Literal::Alphanumeric("HELLO".to_string()));
        assert_eq!(operand_expr(&op), "\"HELLO\"");

        let op = Operand::DataRef(DataReference {
            name: "WS-COUNT".to_string(),
            qualifiers: Vec::new(),
            subscripts: Vec::new(),
            ref_mod: None,
        });
        assert_eq!(operand_expr(&op), "ws.ws_count");
    }

    #[test]
    fn condition_formatting() {
        let cond = Condition::Comparison {
            left: Operand::DataRef(DataReference {
                name: "WS-X".to_string(),
                qualifiers: Vec::new(),
                subscripts: Vec::new(),
                ref_mod: None,
            }),
            op: ComparisonOp::GreaterThan,
            right: Operand::Literal(Literal::Numeric("0".to_string())),
        };
        assert_eq!(condition_expr(&cond, &empty_cmap()), r#"ws.ws_x.to_decimal() > "0".parse::<Decimal>().unwrap()"#);
    }

    #[test]
    fn arith_expr_formatting() {
        let expr = ArithExpr::BinaryOp {
            left: Box::new(ArithExpr::Operand(Operand::DataRef(DataReference {
                name: "WS-A".to_string(),
                qualifiers: Vec::new(),
                subscripts: Vec::new(),
                ref_mod: None,
            }))),
            op: ArithOp::Add,
            right: Box::new(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
                "1".to_string(),
            )))),
        };
        assert_eq!(arith_expr_str(&expr), r#"(ws.ws_a.to_decimal() + "1".parse::<Decimal>().unwrap())"#);
    }

    #[test]
    fn generate_move_statement() {
        let mut w = RustWriter::new();
        let stmt = MoveStatement {
            corresponding: false,
            source: Operand::DataRef(DataReference {
                name: "WS-A".to_string(),
                qualifiers: Vec::new(),
                subscripts: Vec::new(),
                ref_mod: None,
            }),
            destinations: vec![DataReference {
                name: "WS-B".to_string(),
                qualifiers: Vec::new(),
                subscripts: Vec::new(),
                ref_mod: None,
            }],
        };
        generate_move(&mut w, &stmt, &HashMap::new());
        let output = w.finish();
        assert!(output.contains("cobol_move(&ws.ws_a, &mut ws.ws_b, &ctx.config);"));
    }

    #[test]
    fn generate_open_statement() {
        let mut w = RustWriter::new();
        let stmt = OpenStatement {
            files: vec![
                OpenFile {
                    mode: OpenMode::Input,
                    file_name: "INPUT-FILE".to_string(),
                },
                OpenFile {
                    mode: OpenMode::Output,
                    file_name: "OUTPUT-FILE".to_string(),
                },
            ],
        };
        generate_open(&mut w, &stmt);
        let output = w.finish();
        assert!(output.contains("ws.input_file.open(FileOpenMode::Input)"));
        assert!(output.contains("ws.output_file.open(FileOpenMode::Output)"));
    }

    #[test]
    fn generate_open_io_extend() {
        let mut w = RustWriter::new();
        let stmt = OpenStatement {
            files: vec![
                OpenFile {
                    mode: OpenMode::IoMode,
                    file_name: "MASTER-FILE".to_string(),
                },
                OpenFile {
                    mode: OpenMode::Extend,
                    file_name: "LOG-FILE".to_string(),
                },
            ],
        };
        generate_open(&mut w, &stmt);
        let output = w.finish();
        assert!(output.contains("ws.master_file.open(FileOpenMode::InputOutput)"));
        assert!(output.contains("ws.log_file.open(FileOpenMode::Extend)"));
    }

    #[test]
    fn generate_close_statement() {
        let mut w = RustWriter::new();
        let stmt = CloseStatement {
            files: vec!["INPUT-FILE".to_string(), "OUTPUT-FILE".to_string()],
        };
        generate_close(&mut w, &stmt);
        let output = w.finish();
        assert!(output.contains("ws.input_file.close()"));
        assert!(output.contains("ws.output_file.close()"));
    }

    #[test]
    fn generate_read_simple() {
        let mut w = RustWriter::new();
        let stmt = ReadStatement {
            file_name: "INPUT-FILE".to_string(),
            into: Some(DataReference {
                name: "WS-RECORD".to_string(),
                qualifiers: Vec::new(),
                subscripts: Vec::new(),
                ref_mod: None,
            }),
            key: None,
            at_end: Vec::new(),
            not_at_end: Vec::new(),
            invalid_key: Vec::new(),
            not_invalid_key: Vec::new(),
        };
        generate_read(&mut w, &stmt, &empty_cmap(), &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        assert!(output.contains("ws.input_file.read_next()"));
        assert!(output.contains("ws.ws_record"));
    }

    #[test]
    fn generate_read_with_at_end() {
        let mut w = RustWriter::new();
        let stmt = ReadStatement {
            file_name: "INPUT-FILE".to_string(),
            into: None,
            key: None,
            at_end: vec![Statement::Display(DisplayStatement {
                items: vec![Operand::Literal(Literal::Alphanumeric(
                    "END OF FILE".to_string(),
                ))],
                upon: DisplayTarget::Sysout,
                no_advancing: false,
            })],
            not_at_end: Vec::new(),
            invalid_key: Vec::new(),
            not_invalid_key: Vec::new(),
        };
        generate_read(&mut w, &stmt, &empty_cmap(), &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        assert!(output.contains("ws.input_file.read_next()"), "missing read_next: {output}");
        assert!(output.contains("_status.is_success()"), "missing is_success: {output}");
        assert!(output.contains("END OF FILE"), "missing END OF FILE: {output}");
    }

    #[test]
    fn generate_write_simple() {
        let mut w = RustWriter::new();
        let stmt = WriteStatement {
            record_name: "OUT-RECORD".to_string(),
            from: None,
            advancing: None,
            invalid_key: Vec::new(),
            not_invalid_key: Vec::new(),
            at_eop: Vec::new(),
            not_at_eop: Vec::new(),
        };
        generate_write(&mut w, &stmt, &empty_cmap(), &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        // No RecordFileMap entry -> fallback to {rec}_file pattern
        assert!(output.contains("ws.out_record_file.write_record(ws.out_record.as_bytes())"));
    }

    #[test]
    fn generate_write_with_advancing_page() {
        let mut w = RustWriter::new();
        let stmt = WriteStatement {
            record_name: "PRINT-LINE".to_string(),
            from: None,
            advancing: Some(Advancing::Page),
            invalid_key: Vec::new(),
            not_invalid_key: Vec::new(),
            at_eop: Vec::new(),
            not_at_eop: Vec::new(),
        };
        generate_write(&mut w, &stmt, &empty_cmap(), &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        assert!(output.contains("write_record"));
        assert!(output.contains("\\x0C"));
    }

    #[test]
    fn generate_rewrite_statement() {
        let mut w = RustWriter::new();
        let stmt = RewriteStatement {
            record_name: "MASTER-REC".to_string(),
            from: None,
            invalid_key: Vec::new(),
            not_invalid_key: Vec::new(),
        };
        generate_rewrite(&mut w, &stmt, &empty_cmap(), &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        // No RecordFileMap entry -> fallback to {rec}_file pattern
        assert!(output.contains("ws.master_rec_file.rewrite_record(ws.master_rec.as_bytes())"));
    }

    #[test]
    fn generate_delete_statement() {
        let mut w = RustWriter::new();
        let stmt = DeleteStatement {
            file_name: "INDEXED-FILE".to_string(),
            invalid_key: Vec::new(),
            not_invalid_key: Vec::new(),
        };
        generate_delete(&mut w, &stmt, &empty_cmap(), &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        assert!(output.contains("ws.indexed_file.delete_record()"));
    }

    #[test]
    fn generate_delete_with_invalid_key() {
        let mut w = RustWriter::new();
        let stmt = DeleteStatement {
            file_name: "INDEXED-FILE".to_string(),
            invalid_key: vec![Statement::Display(DisplayStatement {
                items: vec![Operand::Literal(Literal::Alphanumeric(
                    "KEY NOT FOUND".to_string(),
                ))],
                upon: DisplayTarget::Sysout,
                no_advancing: false,
            })],
            not_invalid_key: Vec::new(),
        };
        generate_delete(&mut w, &stmt, &empty_cmap(), &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        assert!(output.contains("ws.indexed_file.delete_record()"), "missing delete_record: {output}");
        assert!(output.contains("_status.is_success()"), "missing is_success: {output}");
        assert!(output.contains("KEY NOT FOUND"), "missing KEY NOT FOUND: {output}");
    }

    // -----------------------------------------------------------------------
    // Phase 3 generator tests
    // -----------------------------------------------------------------------

    fn make_ref(name: &str) -> DataReference {
        DataReference {
            name: name.to_string(),
            qualifiers: Vec::new(),
            subscripts: Vec::new(),
            ref_mod: None,
        }
    }

    #[test]
    fn generate_sort_using_giving() {
        let mut w = RustWriter::new();
        let stmt = SortStatement {
            file_name: "SORT-FILE".to_string(),
            keys: vec![SortKey {
                ascending: true,
                field: make_ref("SORT-KEY"),
            }],
            duplicates: false,
            collating: None,
            input: SortInput::Using(vec!["INPUT-FILE".to_string()]),
            output: SortOutput::Giving(vec!["OUTPUT-FILE".to_string()]),
        };
        generate_sort(&mut w, &stmt, &empty_cmap(), &HashMap::new());
        let output = w.finish();
        assert!(output.contains("sort_keys"));
        assert!(output.contains("SortKeySpec"));
        assert!(output.contains("ws.sort_key"));
        assert!(output.contains("ws.input_file"));
        assert!(output.contains("ws.output_file"));
        assert!(output.contains("sort_using_giving"));
    }

    #[test]
    fn generate_sort_input_procedure() {
        let mut w = RustWriter::new();
        let stmt = SortStatement {
            file_name: "SORT-FILE".to_string(),
            keys: vec![SortKey {
                ascending: false,
                field: make_ref("SORT-KEY"),
            }],
            duplicates: true,
            collating: None,
            input: SortInput::InputProcedure {
                name: "INPUT-PARA".to_string(),
                thru: None,
            },
            output: SortOutput::Giving(vec!["OUTPUT-FILE".to_string()]),
        };
        generate_sort(&mut w, &stmt, &empty_cmap(), &HashMap::new());
        let output = w.finish();
        assert!(output.contains("sort_with_input_procedure"), "should use sort_with_input_procedure API: {output}");
        assert!(output.contains("input_para(ws, ctx, releaser)"), "should call input procedure with releaser: {output}");
        assert!(output.contains("with_stable(true)"));
    }

    #[test]
    fn generate_merge_statement() {
        let mut w = RustWriter::new();
        let stmt = MergeStatement {
            file_name: "MERGE-FILE".to_string(),
            keys: vec![SortKey {
                ascending: true,
                field: make_ref("MERGE-KEY"),
            }],
            collating: None,
            using: vec!["FILE-A".to_string(), "FILE-B".to_string()],
            output: SortOutput::Giving(vec!["OUTPUT-FILE".to_string()]),
        };
        generate_merge(&mut w, &stmt, &empty_cmap(), &HashMap::new());
        let output = w.finish();
        assert!(output.contains("merge_keys"));
        assert!(output.contains("CobolMergeEngine"));
        assert!(output.contains("ws.file_a"));
        assert!(output.contains("ws.file_b"));
        assert!(output.contains("merge_files"));
    }

    #[test]
    fn generate_release_statement() {
        let mut w = RustWriter::new();
        let stmt = ReleaseStatement {
            record_name: "SORT-REC".to_string(),
            from: None,
        };
        generate_release(&mut w, &stmt);
        let output = w.finish();
        assert!(output.contains("releaser.release(ws.sort_rec.as_bytes())"));
    }

    #[test]
    fn generate_release_from_statement() {
        let mut w = RustWriter::new();
        let stmt = ReleaseStatement {
            record_name: "SORT-REC".to_string(),
            from: Some(make_ref("WS-RECORD")),
        };
        generate_release(&mut w, &stmt);
        let output = w.finish();
        assert!(output.contains("releaser.release(ws.ws_record.as_bytes())"));
    }

    #[test]
    fn generate_return_with_at_end() {
        let mut w = RustWriter::new();
        let stmt = ReturnStatement {
            file_name: "SORT-FILE".to_string(),
            into: Some(make_ref("WS-RECORD")),
            at_end: vec![Statement::Display(DisplayStatement {
                items: vec![Operand::Literal(Literal::Alphanumeric(
                    "END OF DATA".to_string(),
                ))],
                upon: DisplayTarget::Sysout,
                no_advancing: false,
            })],
            not_at_end: Vec::new(),
        };
        generate_return(&mut w, &stmt, &empty_cmap(), &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        assert!(output.contains("match returner.return_record()"));
        assert!(output.contains("Some(record_data)"));
        assert!(output.contains("None =>"));
        assert!(output.contains("END OF DATA"));
    }

    #[test]
    fn generate_inspect_tallying_characters() {
        let mut w = RustWriter::new();
        let stmt = InspectStatement {
            target: make_ref("WS-STRING"),
            tallying: vec![InspectTallying {
                counter: make_ref("WS-COUNT"),
                what: InspectWhat::Characters,
            }],
            replacing: Vec::new(),
            converting: None,
        };
        generate_inspect(&mut w, &stmt, &empty_cmap());
        let output = w.finish();
        assert!(output.contains("cobol_inspect_tallying"));
        assert!(output.contains("InspectWhat::Characters"));
        assert!(output.contains("ws.ws_count"));
    }

    #[test]
    fn generate_inspect_replacing_all() {
        let mut w = RustWriter::new();
        let stmt = InspectStatement {
            target: make_ref("WS-STRING"),
            tallying: Vec::new(),
            replacing: vec![InspectReplacing {
                what: InspectWhat::All(Operand::Literal(Literal::Alphanumeric("A".to_string()))),
                by: Operand::Literal(Literal::Alphanumeric("B".to_string())),
            }],
            converting: None,
        };
        generate_inspect(&mut w, &stmt, &empty_cmap());
        let output = w.finish();
        assert!(output.contains("cobol_inspect_replacing"));
        assert!(output.contains("InspectWhat::All"));
    }

    #[test]
    fn generate_inspect_converting() {
        let mut w = RustWriter::new();
        let stmt = InspectStatement {
            target: make_ref("WS-STRING"),
            tallying: Vec::new(),
            replacing: Vec::new(),
            converting: Some(InspectConverting {
                from: Operand::Literal(Literal::Alphanumeric("abc".to_string())),
                to: Operand::Literal(Literal::Alphanumeric("ABC".to_string())),
            }),
        };
        generate_inspect(&mut w, &stmt, &empty_cmap());
        let output = w.finish();
        assert!(output.contains("cobol_inspect_converting"));
        assert!(output.contains("b\"abc\""));
        assert!(output.contains("b\"ABC\""));
    }

    #[test]
    fn generate_string_basic() {
        let mut w = RustWriter::new();
        let stmt = StringStatement {
            sources: vec![
                StringSource {
                    operand: Operand::DataRef(make_ref("WS-FIRST")),
                    delimited_by: StringDelimiter::Size,
                },
                StringSource {
                    operand: Operand::Literal(Literal::Alphanumeric(" ".to_string())),
                    delimited_by: StringDelimiter::Size,
                },
                StringSource {
                    operand: Operand::DataRef(make_ref("WS-LAST")),
                    delimited_by: StringDelimiter::Literal(Operand::Literal(
                        Literal::Alphanumeric(" ".to_string()),
                    )),
                },
            ],
            into: make_ref("WS-RESULT"),
            pointer: Some(make_ref("WS-PTR")),
            on_overflow: Vec::new(),
            not_on_overflow: Vec::new(),
        };
        generate_string(&mut w, &stmt, &empty_cmap(), &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        assert!(output.contains("StringSourceSpec::by_size"));
        assert!(output.contains("StringSourceSpec::by_literal"));
        assert!(output.contains("cobol_string"));
        assert!(output.contains("ws.ws_ptr"));
    }

    #[test]
    fn generate_unstring_basic() {
        let mut w = RustWriter::new();
        let stmt = UnstringStatement {
            source: make_ref("WS-INPUT"),
            delimiters: vec![UnstringDelimiter {
                value: Operand::Literal(Literal::Alphanumeric(",".to_string())),
                all: false,
            }],
            into: vec![
                UnstringInto {
                    target: make_ref("WS-FIELD1"),
                    delimiter_in: None,
                    count_in: None,
                },
                UnstringInto {
                    target: make_ref("WS-FIELD2"),
                    delimiter_in: None,
                    count_in: None,
                },
            ],
            pointer: None,
            tallying: None,
            on_overflow: Vec::new(),
            not_on_overflow: Vec::new(),
        };
        generate_unstring(&mut w, &stmt, &empty_cmap(), &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        assert!(output.contains("UnstringDelimSpec::new"));
        assert!(output.contains("cobol_unstring_simple"));
        assert!(output.contains("ws.ws_field1"));
        assert!(output.contains("ws.ws_field2"));
    }

    #[test]
    fn data_ref_with_ref_mod_literal_offsets() {
        let dr = DataReference {
            name: "WS-FIELD".to_string(),
            qualifiers: Vec::new(),
            subscripts: Vec::new(),
            ref_mod: Some(RefMod {
                offset: Box::new(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
                    "3".to_string(),
                )))),
                length: Some(Box::new(ArithExpr::Operand(Operand::Literal(
                    Literal::Numeric("5".to_string()),
                )))),
            }),
        };
        let expr = data_ref_expr(&dr);
        assert!(expr.contains("ref_mod_read(&ws.ws_field, 3usize, 5usize)"));
        assert!(expr.contains("PicX::new(_rm.len(), &_rm)"));
    }

    #[test]
    fn data_ref_with_ref_mod_no_length() {
        let dr = DataReference {
            name: "WS-NAME".to_string(),
            qualifiers: Vec::new(),
            subscripts: Vec::new(),
            ref_mod: Some(RefMod {
                offset: Box::new(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
                    "4".to_string(),
                )))),
                length: None,
            }),
        };
        let expr = data_ref_expr(&dr);
        assert!(expr.contains("ref_mod_read_to_end(&ws.ws_name, 4usize)"));
        assert!(expr.contains("PicX::new(_rm.len(), &_rm)"));
    }

    #[test]
    fn data_ref_base_ignores_ref_mod() {
        let dr = DataReference {
            name: "WS-FIELD".to_string(),
            qualifiers: Vec::new(),
            subscripts: Vec::new(),
            ref_mod: Some(RefMod {
                offset: Box::new(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
                    "3".to_string(),
                )))),
                length: Some(Box::new(ArithExpr::Operand(Operand::Literal(
                    Literal::Numeric("5".to_string()),
                )))),
            }),
        };
        let expr = data_ref_base_expr(&dr);
        assert_eq!(expr, "ws.ws_field");
    }

    #[test]
    fn data_ref_with_ref_mod_data_ref_offset() {
        let dr = DataReference {
            name: "WS-FIELD".to_string(),
            qualifiers: Vec::new(),
            subscripts: Vec::new(),
            ref_mod: Some(RefMod {
                offset: Box::new(ArithExpr::Operand(Operand::DataRef(DataReference {
                    name: "WS-POS".to_string(),
                    qualifiers: Vec::new(),
                    subscripts: Vec::new(),
                    ref_mod: None,
                }))),
                length: Some(Box::new(ArithExpr::Operand(Operand::Literal(
                    Literal::Numeric("3".to_string()),
                )))),
            }),
        };
        let expr = data_ref_expr(&dr);
        assert!(expr.contains("decimal_to_usize(ws.ws_pos.to_decimal())"));
        assert!(expr.contains("3usize"));
    }

    #[test]
    fn generate_move_with_ref_mod_dest() {
        let mut w = RustWriter::new();
        let stmt = MoveStatement {
            corresponding: false,
            source: Operand::Literal(Literal::Alphanumeric("AB".to_string())),
            destinations: vec![DataReference {
                name: "WS-FIELD".to_string(),
                qualifiers: Vec::new(),
                subscripts: Vec::new(),
                ref_mod: Some(RefMod {
                    offset: Box::new(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
                        "3".to_string(),
                    )))),
                    length: Some(Box::new(ArithExpr::Operand(Operand::Literal(
                        Literal::Numeric("2".to_string()),
                    )))),
                }),
            }],
        };
        generate_move(&mut w, &stmt, &HashMap::new());
        let output = w.finish();
        assert!(output.contains("ref_mod_write"));
        assert!(output.contains("b\"AB\""));
        assert!(output.contains("&mut ws.ws_field"));
        assert!(output.contains("3usize"));
        assert!(output.contains("2usize"));
    }

    #[test]
    fn generate_move_with_ref_mod_source() {
        let mut w = RustWriter::new();
        let stmt = MoveStatement {
            corresponding: false,
            source: Operand::DataRef(DataReference {
                name: "WS-SRC".to_string(),
                qualifiers: Vec::new(),
                subscripts: Vec::new(),
                ref_mod: Some(RefMod {
                    offset: Box::new(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
                        "2".to_string(),
                    )))),
                    length: Some(Box::new(ArithExpr::Operand(Operand::Literal(
                        Literal::Numeric("4".to_string()),
                    )))),
                }),
            }),
            destinations: vec![DataReference {
                name: "WS-DEST".to_string(),
                qualifiers: Vec::new(),
                subscripts: Vec::new(),
                ref_mod: None,
            }],
        };
        generate_move(&mut w, &stmt, &HashMap::new());
        let output = w.finish();
        assert!(output.contains("cobol_move"));
        assert!(output.contains("ref_mod_read"));
        assert!(output.contains("PicX::new"));
    }

    #[test]
    fn generate_move_ref_mod_dest_no_length() {
        let mut w = RustWriter::new();
        let stmt = MoveStatement {
            corresponding: false,
            source: Operand::Literal(Literal::Alphanumeric("XYZ".to_string())),
            destinations: vec![DataReference {
                name: "WS-DATA".to_string(),
                qualifiers: Vec::new(),
                subscripts: Vec::new(),
                ref_mod: Some(RefMod {
                    offset: Box::new(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
                        "5".to_string(),
                    )))),
                    length: None,
                }),
            }],
        };
        generate_move(&mut w, &stmt, &HashMap::new());
        let output = w.finish();
        assert!(output.contains("ref_mod_write_to_end"));
        assert!(output.contains("b\"XYZ\""));
        assert!(output.contains("5usize"));
    }

    #[test]
    fn generate_move_ref_mod_both_src_and_dest() {
        let mut w = RustWriter::new();
        let stmt = MoveStatement {
            corresponding: false,
            source: Operand::DataRef(DataReference {
                name: "WS-SRC".to_string(),
                qualifiers: Vec::new(),
                subscripts: Vec::new(),
                ref_mod: Some(RefMod {
                    offset: Box::new(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
                        "1".to_string(),
                    )))),
                    length: Some(Box::new(ArithExpr::Operand(Operand::Literal(
                        Literal::Numeric("3".to_string()),
                    )))),
                }),
            }),
            destinations: vec![DataReference {
                name: "WS-DEST".to_string(),
                qualifiers: Vec::new(),
                subscripts: Vec::new(),
                ref_mod: Some(RefMod {
                    offset: Box::new(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
                        "2".to_string(),
                    )))),
                    length: Some(Box::new(ArithExpr::Operand(Operand::Literal(
                        Literal::Numeric("3".to_string()),
                    )))),
                }),
            }],
        };
        generate_move(&mut w, &stmt, &HashMap::new());
        let output = w.finish();
        // Source ref_mod_read is the bytes for ref_mod_write
        assert!(output.contains("ref_mod_read(&ws.ws_src, 1usize, 3usize)"));
        assert!(output.contains("ref_mod_write"));
        assert!(output.contains("&mut ws.ws_dest"));
        assert!(output.contains("2usize"));
    }

    #[test]
    fn subscript_with_ref_mod() {
        let dr = DataReference {
            name: "WS-TABLE".to_string(),
            qualifiers: Vec::new(),
            subscripts: vec![Subscript::IntLiteral(2)],
            ref_mod: Some(RefMod {
                offset: Box::new(ArithExpr::Operand(Operand::Literal(Literal::Numeric(
                    "1".to_string(),
                )))),
                length: Some(Box::new(ArithExpr::Operand(Operand::Literal(
                    Literal::Numeric("3".to_string()),
                )))),
            }),
        };
        let expr = data_ref_expr(&dr);
        assert!(expr.contains("ws.ws_table[1]"));
        assert!(expr.contains("ref_mod_read"));
        assert!(expr.contains("1usize"));
        assert!(expr.contains("3usize"));
    }

    #[test]
    fn ref_mod_index_expr_integer() {
        let expr = ArithExpr::Operand(Operand::Literal(Literal::Numeric("7".to_string())));
        assert_eq!(ref_mod_index_expr(&expr), "7usize");
    }

    #[test]
    fn ref_mod_index_expr_data_ref() {
        let expr = ArithExpr::Operand(Operand::DataRef(DataReference {
            name: "WS-POS".to_string(),
            qualifiers: Vec::new(),
            subscripts: Vec::new(),
            ref_mod: None,
        }));
        assert_eq!(
            ref_mod_index_expr(&expr),
            "decimal_to_usize(ws.ws_pos.to_decimal())"
        );
    }

    // -----------------------------------------------------------------------
    // Session 27: 88-level condition name codegen tests
    // -----------------------------------------------------------------------

    fn make_data_entry(name: &str, level: u8, pic: Option<PicClause>) -> DataEntry {
        DataEntry {
            level,
            name: name.to_string(),
            pic,
            usage: Usage::Display,
            value: None,
            redefines: None,
            occurs: None,
            occurs_depending: None,
            sign: None,
            justified_right: false,
            blank_when_zero: false,
            children: Vec::new(),
            condition_values: Vec::new(),
            byte_offset: None,
            byte_length: None,
            renames_target: None,
            renames_thru: None,
            index_names: Vec::new(),
        }
    }

    fn make_88_entry(name: &str, values: Vec<ConditionValue>) -> DataEntry {
        DataEntry {
            level: 88,
            name: name.to_string(),
            pic: None,
            usage: Usage::Display,
            value: None,
            redefines: None,
            occurs: None,
            occurs_depending: None,
            sign: None,
            justified_right: false,
            blank_when_zero: false,
            children: Vec::new(),
            condition_values: values,
            byte_offset: None,
            byte_length: None,
            renames_target: None,
            renames_thru: None,
            index_names: Vec::new(),
        }
    }

    fn numeric_pic() -> PicClause {
        PicClause {
            raw: "9(3)".to_string(),
            category: PicCategory::Numeric,
            total_digits: 3,
            scale: 0,
            signed: false,
            display_length: 3,
            edit_symbols: Vec::new(),
        }
    }

    fn alpha_pic() -> PicClause {
        PicClause {
            raw: "X(10)".to_string(),
            category: PicCategory::Alphanumeric,
            total_digits: 0,
            scale: 0,
            signed: false,
            display_length: 10,
            edit_symbols: Vec::new(),
        }
    }

    #[test]
    fn build_condition_map_numeric_single() {
        let mut parent = make_data_entry("WS-STATUS", 5, Some(numeric_pic()));
        parent.children.push(make_88_entry(
            "WS-ACTIVE",
            vec![ConditionValue::Single(Literal::Numeric("1".to_string()))],
        ));
        // We need to put the parent inside a group
        let mut group = make_data_entry("WS-GROUP", 1, None);
        group.children.push(parent);
        let map = build_condition_map(&[group]);
        assert!(map.contains_key("WS-ACTIVE"));
        let info = map.get("WS-ACTIVE").unwrap();
        assert!(info.parent_is_numeric);
        assert_eq!(info.values.len(), 1);
    }

    #[test]
    fn build_condition_map_alpha_multi_value() {
        let mut parent = make_data_entry("WS-CODE", 5, Some(alpha_pic()));
        parent.children.push(make_88_entry(
            "WS-VALID-CODE",
            vec![
                ConditionValue::Single(Literal::Alphanumeric("YES".to_string())),
                ConditionValue::Single(Literal::Alphanumeric("NO".to_string())),
            ],
        ));
        let mut group = make_data_entry("WS-REC", 1, None);
        group.children.push(parent);
        let map = build_condition_map(&[group]);
        let info = map.get("WS-VALID-CODE").unwrap();
        assert!(!info.parent_is_numeric);
        assert_eq!(info.values.len(), 2);
    }

    #[test]
    fn build_condition_map_range() {
        let mut parent = make_data_entry("WS-SCORE", 5, Some(numeric_pic()));
        parent.children.push(make_88_entry(
            "WS-PASSING",
            vec![ConditionValue::Range {
                low: Literal::Numeric("60".to_string()),
                high: Literal::Numeric("100".to_string()),
            }],
        ));
        let mut group = make_data_entry("WS-STUDENT", 1, None);
        group.children.push(parent);
        let map = build_condition_map(&[group]);
        let info = map.get("WS-PASSING").unwrap();
        assert!(info.parent_is_numeric);
        assert_eq!(info.values.len(), 1);
    }

    #[test]
    fn condition_name_expr_numeric_single() {
        let mut cmap = ConditionMap::new();
        cmap.insert("WS-ACTIVE".to_string(), ConditionInfo {
            parent_field: "ws.ws_status".to_string(),
            parent_is_numeric: true,
            values: vec![ConditionValue::Single(Literal::Numeric("1".to_string()))],
        });
        let dr = DataReference {
            name: "WS-ACTIVE".to_string(),
            qualifiers: Vec::new(),
            subscripts: Vec::new(),
            ref_mod: None,
        };
        let expr = condition_name_expr(&dr, &cmap);
        assert_eq!(expr, r#"ws.ws_status.to_decimal() == "1".parse::<Decimal>().unwrap()"#);
    }

    #[test]
    fn condition_name_expr_alpha_single() {
        let mut cmap = ConditionMap::new();
        cmap.insert("WS-YES".to_string(), ConditionInfo {
            parent_field: "ws.ws_flag".to_string(),
            parent_is_numeric: false,
            values: vec![ConditionValue::Single(Literal::Alphanumeric("Y".to_string()))],
        });
        let dr = DataReference {
            name: "WS-YES".to_string(),
            qualifiers: Vec::new(),
            subscripts: Vec::new(),
            ref_mod: None,
        };
        let expr = condition_name_expr(&dr, &cmap);
        assert_eq!(expr, "ws.ws_flag.as_bytes() == b\"Y\"");
    }

    #[test]
    fn condition_name_expr_multi_value_or() {
        let mut cmap = ConditionMap::new();
        cmap.insert("WS-VALID".to_string(), ConditionInfo {
            parent_field: "ws.ws_code".to_string(),
            parent_is_numeric: false,
            values: vec![
                ConditionValue::Single(Literal::Alphanumeric("A".to_string())),
                ConditionValue::Single(Literal::Alphanumeric("B".to_string())),
                ConditionValue::Single(Literal::Alphanumeric("C".to_string())),
            ],
        });
        let dr = DataReference {
            name: "WS-VALID".to_string(),
            qualifiers: Vec::new(),
            subscripts: Vec::new(),
            ref_mod: None,
        };
        let expr = condition_name_expr(&dr, &cmap);
        assert!(expr.starts_with('('));
        assert!(expr.contains("ws.ws_code.as_bytes() == b\"A\""));
        assert!(expr.contains(" || "));
        assert!(expr.contains("ws.ws_code.as_bytes() == b\"C\""));
    }

    #[test]
    fn condition_name_expr_numeric_range() {
        let mut cmap = ConditionMap::new();
        cmap.insert("WS-PASS".to_string(), ConditionInfo {
            parent_field: "ws.ws_score".to_string(),
            parent_is_numeric: true,
            values: vec![ConditionValue::Range {
                low: Literal::Numeric("60".to_string()),
                high: Literal::Numeric("100".to_string()),
            }],
        });
        let dr = DataReference {
            name: "WS-PASS".to_string(),
            qualifiers: Vec::new(),
            subscripts: Vec::new(),
            ref_mod: None,
        };
        let expr = condition_name_expr(&dr, &cmap);
        assert!(expr.contains(r#"ws.ws_score.to_decimal() >= "60".parse::<Decimal>().unwrap()"#));
        assert!(expr.contains(r#"ws.ws_score.to_decimal() <= "100".parse::<Decimal>().unwrap()"#));
    }

    #[test]
    fn condition_name_expr_not_found_fallback() {
        let cmap = empty_cmap();
        let dr = DataReference {
            name: "UNKNOWN-COND".to_string(),
            qualifiers: Vec::new(),
            subscripts: Vec::new(),
            ref_mod: None,
        };
        let expr = condition_name_expr(&dr, &cmap);
        assert!(expr.contains("not resolved"));
        assert!(expr.contains("is_true()"));
    }

    #[test]
    fn generate_set_to_true_numeric() {
        let mut cmap = ConditionMap::new();
        cmap.insert("WS-ACTIVE".to_string(), ConditionInfo {
            parent_field: "ws.ws_status".to_string(),
            parent_is_numeric: true,
            values: vec![ConditionValue::Single(Literal::Numeric("1".to_string()))],
        });
        let mut w = RustWriter::new();
        let stmt = SetStatement {
            targets: vec![make_ref("WS-ACTIVE")],
            action: SetAction::ToBool(true),
        };
        generate_set(&mut w, &stmt, &cmap);
        let output = w.finish();
        assert!(output.contains(r#"ws.ws_status.pack("1".parse::<Decimal>().unwrap());"#));
    }

    #[test]
    fn generate_set_to_true_alpha() {
        let mut cmap = ConditionMap::new();
        cmap.insert("WS-YES".to_string(), ConditionInfo {
            parent_field: "ws.ws_flag".to_string(),
            parent_is_numeric: false,
            values: vec![ConditionValue::Single(Literal::Alphanumeric("Y".to_string()))],
        });
        let mut w = RustWriter::new();
        let stmt = SetStatement {
            targets: vec![make_ref("WS-YES")],
            action: SetAction::ToBool(true),
        };
        generate_set(&mut w, &stmt, &cmap);
        let output = w.finish();
        assert!(output.contains("cobol_move"));
        assert!(output.contains("PicX::new"));
        assert!(output.contains("b\"Y\""));
        assert!(output.contains("&mut ws.ws_flag"));
    }

    #[test]
    fn generate_set_to_value() {
        let mut w = RustWriter::new();
        let stmt = SetStatement {
            targets: vec![make_ref("WS-INDEX")],
            action: SetAction::To(Operand::Literal(Literal::Numeric("5".to_string()))),
        };
        generate_set(&mut w, &stmt, &empty_cmap());
        let output = w.finish();
        assert!(
            output.contains("cobol_move_numeric") && output.contains("ws.ws_index"),
            "SET TO should use cobol_move_numeric: {output}"
        );
    }

    #[test]
    fn generate_set_up_by() {
        let mut w = RustWriter::new();
        let stmt = SetStatement {
            targets: vec![make_ref("WS-IDX")],
            action: SetAction::UpBy(Operand::Literal(Literal::Numeric("1".to_string()))),
        };
        generate_set(&mut w, &stmt, &empty_cmap());
        let output = w.finish();
        assert!(
            output.contains("cobol_add") && output.contains("ws.ws_idx"),
            "SET UP BY should use cobol_add: {output}"
        );
    }

    #[test]
    fn generate_set_down_by() {
        let mut w = RustWriter::new();
        let stmt = SetStatement {
            targets: vec![make_ref("WS-IDX")],
            action: SetAction::DownBy(Operand::Literal(Literal::Numeric("2".to_string()))),
        };
        generate_set(&mut w, &stmt, &empty_cmap());
        let output = w.finish();
        assert!(
            output.contains("cobol_subtract") && output.contains("ws.ws_idx"),
            "SET DOWN BY should use cobol_subtract: {output}"
        );
    }

    #[test]
    fn condition_expr_with_condition_name_in_if() {
        let mut cmap = ConditionMap::new();
        cmap.insert("WS-DONE".to_string(), ConditionInfo {
            parent_field: "ws.ws_flag".to_string(),
            parent_is_numeric: false,
            values: vec![ConditionValue::Single(Literal::Alphanumeric("Y".to_string()))],
        });
        let cond = Condition::ConditionName(DataReference {
            name: "WS-DONE".to_string(),
            qualifiers: Vec::new(),
            subscripts: Vec::new(),
            ref_mod: None,
        });
        let expr = condition_expr(&cond, &cmap);
        assert_eq!(expr, "ws.ws_flag.as_bytes() == b\"Y\"");
    }

    #[test]
    fn condition_expr_not_condition_name() {
        let mut cmap = ConditionMap::new();
        cmap.insert("WS-ACTIVE".to_string(), ConditionInfo {
            parent_field: "ws.ws_status".to_string(),
            parent_is_numeric: true,
            values: vec![ConditionValue::Single(Literal::Numeric("1".to_string()))],
        });
        let cond = Condition::Not(Box::new(Condition::ConditionName(DataReference {
            name: "WS-ACTIVE".to_string(),
            qualifiers: Vec::new(),
            subscripts: Vec::new(),
            ref_mod: None,
        })));
        let expr = condition_expr(&cond, &cmap);
        assert_eq!(expr, r#"!(ws.ws_status.to_decimal() == "1".parse::<Decimal>().unwrap())"#);
    }

    #[test]
    fn literal_to_decimal_expr_numeric() {
        let lit = Literal::Numeric("42".to_string());
        assert_eq!(literal_to_decimal_expr(&lit), r#""42".parse::<Decimal>().unwrap()"#);
    }

    #[test]
    fn literal_to_bytes_expr_alphanumeric() {
        let lit = Literal::Alphanumeric("HELLO".to_string());
        assert_eq!(literal_to_bytes_expr(&lit), "b\"HELLO\"");
    }

    #[test]
    fn literal_to_bytes_expr_figurative() {
        let lit = Literal::Figurative(FigurativeConstant::Spaces);
        assert_eq!(literal_to_bytes_expr(&lit), "b\" \"");
        let lit = Literal::Figurative(FigurativeConstant::Zeros);
        assert_eq!(literal_to_bytes_expr(&lit), "b\"0\"");
    }

    // -----------------------------------------------------------------------
    // CALL / CANCEL codegen tests
    // -----------------------------------------------------------------------

    #[test]
    fn generate_call_simple() {
        let mut w = RustWriter::new();
        let call = CallStatement {
            program: Operand::Literal(Literal::Alphanumeric("SUBPROG".to_string())),
            using: Vec::new(),
            returning: None,
            on_exception: Vec::new(),
            not_on_exception: Vec::new(),
        };
        generate_call(&mut w, &call, &empty_cmap(), &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        assert!(
            output.contains("cobol_call(&mut ctx.dispatcher, \"SUBPROG\""),
            "output: {output}"
        );
    }

    #[test]
    fn generate_call_with_params() {
        let mut w = RustWriter::new();
        let call = CallStatement {
            program: Operand::Literal(Literal::Alphanumeric("CALC".to_string())),
            using: vec![
                CallParam {
                    mode: PassingMode::ByReference,
                    operand: Some(Operand::DataRef(DataReference {
                        name: "WS-INPUT".to_string(),
                        qualifiers: Vec::new(),
                        subscripts: Vec::new(),
                        ref_mod: None,
                    })),
                },
                CallParam {
                    mode: PassingMode::ByValue,
                    operand: Some(Operand::Literal(Literal::Numeric("42".to_string()))),
                },
            ],
            returning: None,
            on_exception: Vec::new(),
            not_on_exception: Vec::new(),
        };
        generate_call(&mut w, &call, &empty_cmap(), &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        assert!(
            output.contains("call_param_by_ref"),
            "should have BY REF param: {output}"
        );
        assert!(
            output.contains("call_param_by_value"),
            "should have BY VALUE param: {output}"
        );
        assert!(
            output.contains("_call_params"),
            "should build params array: {output}"
        );
    }

    #[test]
    fn generate_call_with_exception() {
        let mut w = RustWriter::new();
        let call = CallStatement {
            program: Operand::Literal(Literal::Alphanumeric("SUBPROG".to_string())),
            using: Vec::new(),
            returning: None,
            on_exception: vec![Statement::Display(DisplayStatement {
                items: vec![Operand::Literal(Literal::Alphanumeric(
                    "CALL FAILED".to_string(),
                ))],
                upon: DisplayTarget::Sysout,
                no_advancing: false,
            })],
            not_on_exception: vec![Statement::Display(DisplayStatement {
                items: vec![Operand::Literal(Literal::Alphanumeric(
                    "CALL OK".to_string(),
                ))],
                upon: DisplayTarget::Sysout,
                no_advancing: false,
            })],
        };
        generate_call(&mut w, &call, &empty_cmap(), &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        assert!(
            output.contains("match cobol_call("),
            "should use match for exception handlers: {output}"
        );
        assert!(output.contains("Ok(rc)"), "should have Ok arm: {output}");
        assert!(output.contains("Err(_e)"), "should have Err arm: {output}");
        assert!(
            output.contains("CALL FAILED"),
            "should contain ON EXCEPTION body: {output}"
        );
        assert!(
            output.contains("CALL OK"),
            "should contain NOT ON EXCEPTION body: {output}"
        );
    }

    #[test]
    fn generate_call_with_returning() {
        let mut w = RustWriter::new();
        let call = CallStatement {
            program: Operand::Literal(Literal::Alphanumeric("GETVAL".to_string())),
            using: Vec::new(),
            returning: Some(DataReference {
                name: "WS-RESULT".to_string(),
                qualifiers: Vec::new(),
                subscripts: Vec::new(),
                ref_mod: None,
            }),
            on_exception: Vec::new(),
            not_on_exception: Vec::new(),
        };
        generate_call(&mut w, &call, &empty_cmap(), &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        assert!(
            output.contains("cobol_call("),
            "should have call: {output}"
        );
        assert!(
            output.contains("ws.ws_result"),
            "should reference returning field: {output}"
        );
        assert!(
            output.contains("return_code"),
            "should store return_code: {output}"
        );
    }

    #[test]
    fn generate_call_by_content() {
        let mut w = RustWriter::new();
        let call = CallStatement {
            program: Operand::Literal(Literal::Alphanumeric("SUB1".to_string())),
            using: vec![CallParam {
                mode: PassingMode::ByContent,
                operand: Some(Operand::DataRef(DataReference {
                    name: "WS-DATA".to_string(),
                    qualifiers: Vec::new(),
                    subscripts: Vec::new(),
                    ref_mod: None,
                })),
            }],
            returning: None,
            on_exception: Vec::new(),
            not_on_exception: Vec::new(),
        };
        generate_call(&mut w, &call, &empty_cmap(), &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        assert!(
            output.contains(".clone()"),
            "BY CONTENT should create a temp copy: {output}"
        );
        assert!(
            output.contains("call_param_by_content"),
            "should use call_param_by_content: {output}"
        );
    }

    #[test]
    fn generate_cancel_statement() {
        let mut w = RustWriter::new();
        let cancel = CancelStatement {
            programs: vec![
                Operand::Literal(Literal::Alphanumeric("SUB1".to_string())),
                Operand::Literal(Literal::Alphanumeric("SUB2".to_string())),
            ],
        };
        generate_cancel(&mut w, &cancel);
        let output = w.finish();
        assert!(
            output.contains("cobol_cancel(&mut ctx.dispatcher, \"SUB1\")"),
            "should cancel SUB1: {output}"
        );
        assert!(
            output.contains("cobol_cancel(&mut ctx.dispatcher, \"SUB2\")"),
            "should cancel SUB2: {output}"
        );
    }

    // -----------------------------------------------------------------------
    // Session 32: Dispatch loop / GO TO / STOP RUN / PERFORM THRU tests
    // -----------------------------------------------------------------------

    fn sample_para_table() -> Vec<ParagraphIndex> {
        vec![
            ParagraphIndex { name: "MAIN-PARA".to_string(), rust_name: "main_para".to_string(), index: 0 },
            ParagraphIndex { name: "SETUP-PARA".to_string(), rust_name: "setup_para".to_string(), index: 1 },
            ParagraphIndex { name: "PROCESS-PARA".to_string(), rust_name: "process_para".to_string(), index: 2 },
        ]
    }

    #[test]
    fn dispatch_loop_basic() {
        let mut w = RustWriter::new();
        let proc_div = ProcedureDivision {
            paragraphs: vec![
                Paragraph { name: "MAIN-PARA".to_string(), sentences: vec![] },
                Paragraph { name: "WORK-PARA".to_string(), sentences: vec![] },
            ],
            sections: vec![],
            using_params: vec![],
            returning: None,
        };
        generate_procedure_division(&mut w, &proc_div, &empty_cmap(), &HashMap::new(), &HashMap::new(), &[], &[], false);
        let output = w.finish();
        assert!(output.contains("let mut _pc: usize = 0;"), "missing _pc decl: {output}");
        assert!(output.contains("0 => main_para(ws, ctx),"), "missing index 0 dispatch: {output}");
        assert!(output.contains("1 => work_para(ws, ctx),"), "missing index 1 dispatch: {output}");
        assert!(output.contains("_ => break,"), "missing default break: {output}");
        assert!(output.contains("_pc += 1;"), "missing _pc increment: {output}");
    }

    #[test]
    fn dispatch_loop_goto_lookup() {
        let mut w = RustWriter::new();
        let proc_div = ProcedureDivision {
            paragraphs: vec![
                Paragraph { name: "A-PARA".to_string(), sentences: vec![] },
                Paragraph { name: "B-PARA".to_string(), sentences: vec![] },
            ],
            sections: vec![],
            using_params: vec![],
            returning: None,
        };
        generate_procedure_division(&mut w, &proc_div, &empty_cmap(), &HashMap::new(), &HashMap::new(), &[], &[], false);
        let output = w.finish();
        assert!(output.contains("ctx.goto_target.take()"), "missing goto_target.take(): {output}");
        assert!(output.contains("\"A-PARA\" => 0,"), "missing A-PARA goto lookup: {output}");
        assert!(output.contains("\"B-PARA\" => 1,"), "missing B-PARA goto lookup: {output}");
    }

    #[test]
    fn goto_sets_target() {
        let mut w = RustWriter::new();
        let goto = GoToStatement {
            targets: vec!["PROCESS-PARA".to_string()],
            depending: None,
        };
        generate_goto(&mut w, &goto);
        let output = w.finish();
        assert!(
            output.contains("ctx.goto_target = Some(\"PROCESS-PARA\".to_string());"),
            "missing goto_target assignment: {output}"
        );
        assert!(output.contains("return;"), "missing return after goto: {output}");
    }

    #[test]
    fn stop_run_returns() {
        let mut w = RustWriter::new();
        generate_statement(&mut w, &Statement::StopRun, &empty_cmap(), &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        assert!(output.contains("ctx.stop_run();"), "missing ctx.stop_run(): {output}");
        assert!(output.contains("return;"), "missing return after stop_run: {output}");
    }

    #[test]
    fn exit_program_sets_flag() {
        let mut w = RustWriter::new();
        generate_statement(&mut w, &Statement::ExitProgram, &empty_cmap(), &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        assert!(output.contains("ctx.exit_program = true;"), "missing exit_program flag: {output}");
        assert!(output.contains("return;"), "missing return after exit_program: {output}");
    }

    #[test]
    fn perform_thru_dispatch() {
        let mut w = RustWriter::new();
        let ptable = sample_para_table();
        let perf = PerformStatement {
            target: Some(PerformTarget { name: "MAIN-PARA".to_string() }),
            thru: Some("PROCESS-PARA".to_string()),
            loop_type: PerformLoopType::Once,
            body: vec![],
        };
        generate_perform(&mut w, &perf, &empty_cmap(), &ptable, &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        assert!(output.contains("let mut _perf_pc: usize = 0;"), "missing _perf_pc: {output}");
        assert!(output.contains("while _perf_pc <= 2"), "missing while loop: {output}");
        assert!(output.contains("0 => main_para(ws, ctx),"), "missing index 0: {output}");
        assert!(output.contains("1 => setup_para(ws, ctx),"), "missing index 1: {output}");
        assert!(output.contains("2 => process_para(ws, ctx),"), "missing index 2: {output}");
    }

    // -----------------------------------------------------------------------
    // Session 33: GO TO DEPENDING ON codegen
    // -----------------------------------------------------------------------

    #[test]
    fn goto_depending_on_codegen() {
        let mut w = RustWriter::new();
        let goto = GoToStatement {
            targets: vec![
                "PARA-A".to_string(),
                "PARA-B".to_string(),
                "PARA-C".to_string(),
            ],
            depending: Some(make_ref("WS-INDEX")),
        };
        generate_goto(&mut w, &goto);
        let output = w.finish();
        assert!(
            output.contains("let _goto_idx = ws.ws_index.to_decimal().to_string().parse::<i64>().unwrap_or(0) as usize;"),
            "missing _goto_idx: {output}"
        );
        assert!(output.contains("1 => ctx.goto_target = Some(\"PARA-A\".to_string()),"), "missing PARA-A: {output}");
        assert!(output.contains("2 => ctx.goto_target = Some(\"PARA-B\".to_string()),"), "missing PARA-B: {output}");
        assert!(output.contains("3 => ctx.goto_target = Some(\"PARA-C\".to_string()),"), "missing PARA-C: {output}");
        assert!(output.contains("_ => {}"), "missing default: {output}");
        assert!(output.contains("if ctx.goto_target.is_some() { return; }"), "missing return guard: {output}");
    }

    #[test]
    fn goto_depending_on_empty_targets() {
        let mut w = RustWriter::new();
        let goto = GoToStatement {
            targets: vec![],
            depending: Some(make_ref("WS-X")),
        };
        generate_goto(&mut w, &goto);
        let output = w.finish();
        // Should still generate the structure but with no match arms (just default)
        assert!(output.contains("match _goto_idx"), "missing match: {output}");
        assert!(output.contains("_ => {}"), "missing default: {output}");
    }

    // -----------------------------------------------------------------------
    // Session 33: START codegen
    // -----------------------------------------------------------------------

    #[test]
    fn generate_start_simple() {
        let mut w = RustWriter::new();
        let start = StartStatement {
            file_name: "INPUT-FILE".to_string(),
            key_condition: None,
            invalid_key: vec![],
            not_invalid_key: vec![],
        };
        generate_start(&mut w, &start, &empty_cmap(), &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        assert!(
            output.contains("ws.input_file.start(&[], std::cmp::Ordering::Equal)"),
            "missing start: {output}"
        );
    }

    #[test]
    fn generate_start_with_key_equal() {
        let mut w = RustWriter::new();
        let start = StartStatement {
            file_name: "MASTER-FILE".to_string(),
            key_condition: Some(StartKeyCondition {
                key: make_ref("WS-KEY"),
                op: ComparisonOp::Equal,
            }),
            invalid_key: vec![],
            not_invalid_key: vec![],
        };
        generate_start(&mut w, &start, &empty_cmap(), &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        assert!(
            output.contains("ws.master_file.start(ws.ws_key.as_bytes(), std::cmp::Ordering::Equal)"),
            "missing start with key: {output}"
        );
    }

    #[test]
    fn generate_start_with_key_greater() {
        let mut w = RustWriter::new();
        let start = StartStatement {
            file_name: "IDX-FILE".to_string(),
            key_condition: Some(StartKeyCondition {
                key: make_ref("WS-KEY"),
                op: ComparisonOp::GreaterThan,
            }),
            invalid_key: vec![],
            not_invalid_key: vec![],
        };
        generate_start(&mut w, &start, &empty_cmap(), &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        assert!(
            output.contains("std::cmp::Ordering::Greater"),
            "missing Greater ordering: {output}"
        );
    }

    #[test]
    fn generate_start_with_invalid_key() {
        let mut w = RustWriter::new();
        let start = StartStatement {
            file_name: "MASTER-FILE".to_string(),
            key_condition: Some(StartKeyCondition {
                key: make_ref("WS-KEY"),
                op: ComparisonOp::GreaterOrEqual,
            }),
            invalid_key: vec![Statement::Display(DisplayStatement {
                items: vec![Operand::Literal(Literal::Alphanumeric("KEY NOT FOUND".to_string()))],
                upon: DisplayTarget::Console,
                no_advancing: false,
            })],
            not_invalid_key: vec![],
        };
        generate_start(&mut w, &start, &empty_cmap(), &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        assert!(output.contains("ws.master_file.start("), "missing start call: {output}");
        assert!(output.contains("_status.is_success()"), "missing is_success: {output}");
        assert!(output.contains("KEY NOT FOUND"), "missing KEY NOT FOUND: {output}");
    }

    // -----------------------------------------------------------------------
    // data_ref_base_expr: qualified name disambiguation
    // -----------------------------------------------------------------------

    #[test]
    fn data_ref_no_qualifier() {
        let dr = DataReference {
            name: "WS-NAME".to_string(),
            qualifiers: Vec::new(),
            subscripts: Vec::new(),
            ref_mod: None,
        };
        assert_eq!(data_ref_base_expr(&dr), "ws.ws_name");
    }

    #[test]
    fn data_ref_with_qualifier() {
        let dr = DataReference {
            name: "WS-NAME".to_string(),
            qualifiers: vec!["WS-SRC-REC".to_string()],
            subscripts: Vec::new(),
            ref_mod: None,
        };
        assert_eq!(data_ref_base_expr(&dr), "ws.ws_src_rec_ws_name");
    }

    #[test]
    fn data_ref_with_subscript() {
        let dr = DataReference {
            name: "WS-ITEM".to_string(),
            qualifiers: Vec::new(),
            subscripts: vec![Subscript::DataRef(DataReference {
                name: "WS-IX".to_string(),
                qualifiers: Vec::new(),
                subscripts: Vec::new(),
                ref_mod: None,
            })],
            ref_mod: None,
        };
        let expr = data_ref_base_expr(&dr);
        assert!(expr.contains("[decimal_to_usize("), "should convert subscript: {expr}");
        assert!(expr.contains("saturating_sub(1)"), "should convert 1-based to 0-based: {expr}");
    }

    // -----------------------------------------------------------------------
    // generate_initialize: default expansion into per-child calls
    // -----------------------------------------------------------------------

    fn make_simple_gcm() -> GroupChildMap {
        let mut gcm = GroupChildMap::new();
        gcm.insert("WS-RECORD".to_string(), vec![
            ("WS-ALPHA".to_string(), false),
            ("WS-NUM".to_string(), true),
        ]);
        gcm
    }

    #[test]
    fn initialize_default_expands_children() {
        let mut w = RustWriter::new();
        let init = InitializeStatement {
            targets: vec![make_ref("WS-RECORD")],
            replacing: Vec::new(),
        };
        let gcm = make_simple_gcm();
        generate_initialize(&mut w, &init, &gcm);
        let output = w.finish();
        assert!(output.contains("cobol_initialize(&mut ws.ws_alpha)"), "should init alpha child: {output}");
        assert!(output.contains("cobol_initialize(&mut ws.ws_num)"), "should init numeric child: {output}");
        // Should NOT call cobol_initialize on the group itself
        assert!(!output.contains("cobol_initialize(&mut ws.ws_record)"), "should not init group directly: {output}");
    }

    #[test]
    fn initialize_leaf_field() {
        let mut w = RustWriter::new();
        let init = InitializeStatement {
            targets: vec![make_ref("WS-STANDALONE")],
            replacing: Vec::new(),
        };
        let gcm = GroupChildMap::new(); // no groups
        generate_initialize(&mut w, &init, &gcm);
        let output = w.finish();
        assert!(output.contains("cobol_initialize(&mut ws.ws_standalone)"), "leaf should use cobol_initialize: {output}");
    }

    #[test]
    fn initialize_nested_group_recursion() {
        let mut gcm = GroupChildMap::new();
        gcm.insert("WS-OUTER".to_string(), vec![
            ("WS-HDR".to_string(), false),
            ("WS-INNER".to_string(), false), // sub-group
        ]);
        gcm.insert("WS-INNER".to_string(), vec![
            ("WS-DETAIL".to_string(), false),
            ("WS-AMOUNT".to_string(), true),
        ]);

        let mut w = RustWriter::new();
        let init = InitializeStatement {
            targets: vec![make_ref("WS-OUTER")],
            replacing: Vec::new(),
        };
        generate_initialize(&mut w, &init, &gcm);
        let output = w.finish();
        assert!(output.contains("cobol_initialize(&mut ws.ws_hdr)"), "should init WS-HDR: {output}");
        assert!(output.contains("cobol_initialize(&mut ws.ws_detail)"), "should recurse into WS-INNER: {output}");
        assert!(output.contains("cobol_initialize(&mut ws.ws_amount)"), "should recurse into WS-INNER: {output}");
        // WS-INNER itself should NOT appear as a standalone init
        assert!(!output.contains("cobol_initialize(&mut ws.ws_inner)"), "sub-group should not be initialized directly: {output}");
    }

    // -----------------------------------------------------------------------
    // generate_initialize: REPLACING clause
    // -----------------------------------------------------------------------

    #[test]
    fn initialize_replacing_numeric() {
        let mut w = RustWriter::new();
        let init = InitializeStatement {
            targets: vec![make_ref("WS-RECORD")],
            replacing: vec![InitializeReplacing {
                category: InitializeCategory::Numeric,
                value: Operand::Literal(Literal::Numeric("42".to_string())),
            }],
        };
        let gcm = make_simple_gcm();
        generate_initialize(&mut w, &init, &gcm);
        let output = w.finish();
        // Numeric child should get the replacement value
        assert!(output.contains("ws.ws_num"), "should target numeric child: {output}");
        assert!(output.contains("move_numeric_literal"), "should use move_numeric_literal: {output}");
        assert!(output.contains("42"), "should contain value 42: {output}");
        // Alpha child should NOT be touched
        assert!(!output.contains("ws_alpha"), "should NOT touch alpha child: {output}");
    }

    #[test]
    fn initialize_replacing_alphanumeric() {
        let mut w = RustWriter::new();
        let init = InitializeStatement {
            targets: vec![make_ref("WS-RECORD")],
            replacing: vec![InitializeReplacing {
                category: InitializeCategory::Alphanumeric,
                value: Operand::Literal(Literal::Alphanumeric("INIT".to_string())),
            }],
        };
        let gcm = make_simple_gcm();
        generate_initialize(&mut w, &init, &gcm);
        let output = w.finish();
        // Alpha child should get the replacement value
        assert!(output.contains("ws.ws_alpha"), "should target alpha child: {output}");
        assert!(output.contains("move_alphanumeric_literal"), "should use move_alphanumeric_literal: {output}");
        assert!(output.contains("INIT"), "should contain value INIT: {output}");
        // Numeric child should NOT be touched
        assert!(!output.contains("ws_num"), "should NOT touch numeric child: {output}");
    }

    // -----------------------------------------------------------------------
    // generate_move: CORRESPONDING expansion
    // -----------------------------------------------------------------------

    #[test]
    fn move_corresponding_expands_matching_children() {
        let mut gcm = GroupChildMap::new();
        gcm.insert("WS-SRC".to_string(), vec![
            ("WS-NAME".to_string(), false),
            ("WS-AMT".to_string(), true),
            ("WS-CODE".to_string(), false),
        ]);
        gcm.insert("WS-DST".to_string(), vec![
            ("WS-NAME".to_string(), false),
            ("WS-AMT".to_string(), true),
            ("WS-EXTRA".to_string(), false),
        ]);

        let mut w = RustWriter::new();
        let stmt = MoveStatement {
            corresponding: true,
            source: Operand::DataRef(make_ref("WS-SRC")),
            destinations: vec![make_ref("WS-DST")],
        };
        generate_move(&mut w, &stmt, &gcm);
        let output = w.finish();
        // WS-NAME and WS-AMT match between src and dst
        assert!(output.contains("ws_name"), "should move matching WS-NAME: {output}");
        assert!(output.contains("ws_amt"), "should move matching WS-AMT: {output}");
        // WS-CODE (src-only) and WS-EXTRA (dst-only) should NOT appear in moves
        assert!(!output.contains("ws_code"), "should NOT move non-matching WS-CODE: {output}");
        assert!(!output.contains("ws_extra"), "should NOT move non-matching WS-EXTRA: {output}");
    }

    #[test]
    fn move_corresponding_no_matches() {
        let mut gcm = GroupChildMap::new();
        gcm.insert("WS-SRC".to_string(), vec![
            ("WS-A".to_string(), false),
        ]);
        gcm.insert("WS-DST".to_string(), vec![
            ("WS-B".to_string(), false),
        ]);

        let mut w = RustWriter::new();
        let stmt = MoveStatement {
            corresponding: true,
            source: Operand::DataRef(make_ref("WS-SRC")),
            destinations: vec![make_ref("WS-DST")],
        };
        generate_move(&mut w, &stmt, &gcm);
        let output = w.finish();
        // No matching children -> no moves generated
        assert!(output.trim().is_empty() || output.contains("// MOVE CORR"), "should generate no moves for non-matching: {output}");
    }

    // -----------------------------------------------------------------------
    // condition_expr: 88-level condition name expansion
    // -----------------------------------------------------------------------

    #[test]
    fn condition_name_88_expansion() {
        let mut cmap = ConditionMap::new();
        cmap.insert("STATUS-ACTIVE".to_string(), ConditionInfo {
            parent_field: "ws.ws_status".to_string(),
            parent_is_numeric: false,
            values: vec![ConditionValue::Single(Literal::Alphanumeric("AC".to_string()))],
        });

        let cond = Condition::ConditionName(make_ref("STATUS-ACTIVE"));
        let expr = condition_expr(&cond, &cmap);
        assert!(expr.contains("ws_status"), "should reference parent field: {expr}");
        assert!(expr.contains("AC"), "should contain condition value: {expr}");
    }

    #[test]
    fn condition_name_88_numeric() {
        let mut cmap = ConditionMap::new();
        cmap.insert("FLAG-ON".to_string(), ConditionInfo {
            parent_field: "ws.ws_flag".to_string(),
            parent_is_numeric: true,
            values: vec![ConditionValue::Single(Literal::Numeric("1".to_string()))],
        });

        let cond = Condition::ConditionName(make_ref("FLAG-ON"));
        let expr = condition_expr(&cond, &cmap);
        assert!(expr.contains("ws_flag"), "should reference parent field: {expr}");
        assert!(expr.contains("to_decimal()"), "should use to_decimal for numeric: {expr}");
    }

    // -----------------------------------------------------------------------
    // EVALUATE TRUE with condition name in WHEN
    // -----------------------------------------------------------------------

    #[test]
    fn evaluate_true_when_condition_name() {
        let mut cmap = ConditionMap::new();
        cmap.insert("STATUS-ACTIVE".to_string(), ConditionInfo {
            parent_field: "ws.ws_status".to_string(),
            parent_is_numeric: false,
            values: vec![ConditionValue::Single(Literal::Alphanumeric("AC".to_string()))],
        });

        let mut w = RustWriter::new();
        let eval = EvaluateStatement {
            subjects: vec![EvaluateSubject::Bool(true)],
            when_branches: vec![WhenBranch {
                values: vec![WhenValue::Value(Operand::DataRef(make_ref("STATUS-ACTIVE")))],
                body: vec![Statement::Display(DisplayStatement {
                    items: vec![Operand::Literal(Literal::Alphanumeric("ACTIVE".to_string()))],
                    upon: DisplayTarget::Sysout,
                    no_advancing: false,
                })],
            }],
            when_other: Vec::new(),
        };
        generate_evaluate(&mut w, &eval, &cmap, &[], &HashMap::new(), &HashMap::new(), &HashMap::new(), false);
        let output = w.finish();
        // Should expand the condition name, not reference it as a data field
        assert!(output.contains("ws_status") && output.contains("AC"),
            "should expand 88-level condition in EVALUATE TRUE: {output}");
    }
}
