//! Procedure division code generator.
//!
//! Generates Rust functions from COBOL PROCEDURE DIVISION statements.
//! Each paragraph becomes a function. Statements map to runtime API calls.

use crate::ast::*;
use crate::codegen::data_gen::cobol_to_rust_name;
use crate::codegen::rust_writer::RustWriter;

/// Generate all procedure division functions.
pub fn generate_procedure_division(
    w: &mut RustWriter,
    proc_div: &ProcedureDivision,
) {
    // Generate run() method that calls the first paragraph
    w.line("/// Execute the COBOL program.");
    w.open_block("pub fn run(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {");

    // Call the first paragraph or section
    if let Some(first_para) = proc_div.paragraphs.first() {
        let fn_name = cobol_to_rust_name(&first_para.name, "");
        w.line(&format!("{fn_name}(ws, ctx);"));
    } else if let Some(first_section) = proc_div.sections.first() {
        if let Some(first_para) = first_section.paragraphs.first() {
            let fn_name = cobol_to_rust_name(&first_para.name, "");
            w.line(&format!("{fn_name}(ws, ctx);"));
        }
    }

    w.close_block("}");
    w.blank_line();

    // Generate paragraph functions (outside sections)
    for para in &proc_div.paragraphs {
        generate_paragraph_fn(w, para);
    }

    // Generate section paragraphs
    for section in &proc_div.sections {
        w.line(&format!(
            "// --- Section: {} ---",
            section.name
        ));
        w.blank_line();
        for para in &section.paragraphs {
            generate_paragraph_fn(w, para);
        }
    }
}

/// Generate a Rust function for a single paragraph.
fn generate_paragraph_fn(w: &mut RustWriter, para: &Paragraph) {
    let fn_name = cobol_to_rust_name(&para.name, "");
    w.line("#[allow(non_snake_case, unused_variables)]");
    w.open_block(&format!(
        "fn {fn_name}(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {{"
    ));

    for sentence in &para.sentences {
        for stmt in &sentence.statements {
            generate_statement(w, stmt);
        }
    }

    w.close_block("}");
    w.blank_line();
}

/// Generate Rust code for a single statement.
fn generate_statement(w: &mut RustWriter, stmt: &Statement) {
    match stmt {
        Statement::Move(m) => generate_move(w, m),
        Statement::Display(d) => generate_display(w, d),
        Statement::Add(a) => generate_add(w, a),
        Statement::Subtract(s) => generate_subtract(w, s),
        Statement::Multiply(m) => generate_multiply(w, m),
        Statement::Divide(d) => generate_divide(w, d),
        Statement::Compute(c) => generate_compute(w, c),
        Statement::If(i) => generate_if(w, i),
        Statement::Evaluate(e) => generate_evaluate(w, e),
        Statement::Perform(p) => generate_perform(w, p),
        Statement::GoTo(g) => generate_goto(w, g),
        Statement::StopRun => w.line("ctx.stop_run();"),
        Statement::GoBack => w.line("ctx.goback();"),
        Statement::Continue => w.line("// CONTINUE"),
        Statement::NextSentence => w.line("// NEXT SENTENCE"),
        Statement::ExitProgram => w.line("return;"),
        Statement::ExitParagraph => w.line("return;"),
        Statement::ExitSection => w.line("return;"),
        Statement::Initialize(init) => generate_initialize(w, init),
        Statement::Call(call) => generate_call(w, call),
        Statement::Accept(acc) => generate_accept(w, acc),
        Statement::Open(open) => generate_open(w, open),
        Statement::Close(close) => generate_close(w, close),
        Statement::Read(read) => generate_read(w, read),
        Statement::Write(write) => generate_write(w, write),
        Statement::Rewrite(rw) => generate_rewrite(w, rw),
        Statement::Delete(del) => generate_delete(w, del),
        _ => {
            w.line(&format!("// TODO: unsupported statement: {stmt:?}"));
        }
    }
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

fn generate_move(w: &mut RustWriter, m: &MoveStatement) {
    let src = operand_expr(&m.source);
    if m.corresponding {
        for dest in &m.destinations {
            let dest_expr = data_ref_expr(dest);
            w.line(&format!(
                "cobol_move_corresponding(&{src}, &mut {dest_expr}, &ctx.config);"
            ));
        }
    } else {
        for dest in &m.destinations {
            let dest_expr = data_ref_expr(dest);
            w.line(&format!(
                "cobol_move(&{src}, &mut {dest_expr}, &ctx.config);"
            ));
        }
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
    if !a.giving.is_empty() {
        // ADD ... GIVING: sum the operands, store in giving targets
        let operands: Vec<String> = a.operands.iter().map(operand_expr).collect();
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
    } else {
        // ADD ... TO: add each operand to each TO target
        let operands: Vec<String> = a.operands.iter().map(operand_expr).collect();
        for target in &a.to {
            let dest = data_ref_expr(&target.field);
            let r = rounded_str(target.rounded);
            for op in &operands {
                w.line(&format!(
                    "cobol_add(&{op}, &mut {dest}, {r}, &ctx.config);"
                ));
            }
        }
    }
}

fn generate_subtract(w: &mut RustWriter, s: &SubtractStatement) {
    if !s.giving.is_empty() {
        let operands: Vec<String> = s.operands.iter().map(operand_expr).collect();
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
    } else {
        let operands: Vec<String> = s.operands.iter().map(operand_expr).collect();
        for target in &s.from {
            let dest = data_ref_expr(&target.field);
            let r = rounded_str(target.rounded);
            for op in &operands {
                w.line(&format!(
                    "cobol_subtract(&{op}, &mut {dest}, {r}, &ctx.config);"
                ));
            }
        }
    }
}

fn generate_multiply(w: &mut RustWriter, m: &MultiplyStatement) {
    let multiplicand = operand_expr(&m.operand);

    if !m.giving.is_empty() {
        let by_field = m.by.first()
            .map(|t| data_ref_expr(&t.field))
            .unwrap_or_else(|| "0".to_string());
        for target in &m.giving {
            let dest = data_ref_expr(&target.field);
            let r = rounded_str(target.rounded);
            w.line(&format!(
                "cobol_multiply_giving(&{multiplicand}, &{by_field}, &mut {dest}, {r}, &ctx.config);"
            ));
        }
    } else {
        for target in &m.by {
            let dest = data_ref_expr(&target.field);
            let r = rounded_str(target.rounded);
            w.line(&format!(
                "cobol_multiply(&{multiplicand}, &mut {dest}, {r}, &ctx.config);"
            ));
        }
    }
}

fn generate_divide(w: &mut RustWriter, d: &DivideStatement) {
    let operand = operand_expr(&d.operand);
    let remainder_expr = d.remainder.as_ref()
        .map(|rem| format!("Some(&mut {})", data_ref_expr(&rem.field)))
        .unwrap_or_else(|| "None".to_string());

    if !d.giving.is_empty() {
        let into_field = d.into.first()
            .map(|t| data_ref_expr(&t.field))
            .unwrap_or_else(|| "0".to_string());
        match d.direction {
            DivideDirection::Into => {
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
                for target in &d.giving {
                    let dest = data_ref_expr(&target.field);
                    let r = rounded_str(target.rounded);
                    w.line(&format!(
                        "cobol_divide_by_giving(&{operand}, &{into_field}, &mut {dest}, {r}, &ctx.config);"
                    ));
                }
            }
        }
    } else {
        // DIVIDE x INTO y -> cobol_divide(x, y, remainder, rounded, config)
        for target in &d.into {
            let dest = data_ref_expr(&target.field);
            let r = rounded_str(target.rounded);
            w.line(&format!(
                "cobol_divide(&{operand}, &mut {dest}, {remainder_expr}, {r}, &ctx.config);"
            ));
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

fn generate_if(w: &mut RustWriter, i: &IfStatement) {
    let cond = condition_expr(&i.condition);
    w.open_block(&format!("if {cond} {{"));
    for stmt in &i.then_body {
        generate_statement(w, stmt);
    }
    if i.else_body.is_empty() {
        w.close_block("}");
    } else {
        w.dedent();
        w.open_block("} else {");
        for stmt in &i.else_body {
            generate_statement(w, stmt);
        }
        w.close_block("}");
    }
}

fn generate_evaluate(w: &mut RustWriter, e: &EvaluateStatement) {
    let subject = if let Some(subj) = e.subjects.first() {
        match subj {
            EvaluateSubject::Expr(op) => operand_expr(op),
            EvaluateSubject::Bool(b) => b.to_string(),
        }
    } else {
        "true".to_string()
    };

    for (i, branch) in e.when_branches.iter().enumerate() {
        let keyword = if i == 0 { "if" } else { "} else if" };
        let values: Vec<String> = branch.values.iter().map(|v| match v {
            WhenValue::Value(op) => format!("{subject} == {}", operand_expr(op)),
            WhenValue::Range { low, high } => {
                format!(
                    "{subject} >= {} && {subject} <= {}",
                    operand_expr(low),
                    operand_expr(high)
                )
            }
            WhenValue::Condition(c) => condition_expr(c),
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
            generate_statement(w, stmt);
        }
    }

    if !e.when_other.is_empty() {
        w.dedent();
        w.open_block("} else {");
        for stmt in &e.when_other {
            generate_statement(w, stmt);
        }
    }

    if !e.when_branches.is_empty() || !e.when_other.is_empty() {
        w.close_block("}");
    }
}

fn generate_perform(w: &mut RustWriter, p: &PerformStatement) {
    match &p.loop_type {
        PerformLoopType::Once => {
            if let Some(ref target) = p.target {
                let fn_name = cobol_to_rust_name(&target.name, "");
                w.line(&format!("{fn_name}(ws, ctx);"));
            } else {
                // Inline perform (once)
                for stmt in &p.body {
                    generate_statement(w, stmt);
                }
            }
        }
        PerformLoopType::Times(count) => {
            let count_expr = operand_expr(count);
            if let Some(ref target) = p.target {
                let fn_name = cobol_to_rust_name(&target.name, "");
                w.open_block(&format!(
                    "for _cobol_i in 0..{count_expr} as usize {{"
                ));
                w.line(&format!("{fn_name}(ws, ctx);"));
                w.close_block("}");
            } else {
                w.open_block(&format!(
                    "for _cobol_i in 0..{count_expr} as usize {{"
                ));
                for stmt in &p.body {
                    generate_statement(w, stmt);
                }
                w.close_block("}");
            }
        }
        PerformLoopType::Until {
            test_before,
            condition,
        } => {
            let cond = condition_expr(condition);
            if *test_before {
                generate_perform_until_before(w, &cond, p);
            } else {
                generate_perform_until_after(w, &cond, p);
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
            let from_expr = operand_expr(from);
            let by_expr = operand_expr(by);
            let until_cond = condition_expr(until);

            w.line(&format!("{counter_name} = {from_expr};"));
            if *test_before {
                w.open_block(&format!("while !({until_cond}) {{"));
            } else {
                w.open_block("loop {");
            }

            if let Some(ref target) = p.target {
                let fn_name = cobol_to_rust_name(&target.name, "");
                w.line(&format!("{fn_name}(ws, ctx);"));
            } else {
                for stmt in &p.body {
                    generate_statement(w, stmt);
                }
            }

            w.line(&format!("{counter_name} += {by_expr};"));

            if !test_before {
                w.open_block(&format!("if {until_cond} {{"));
                w.line("break;");
                w.close_block("}");
            }

            w.close_block("}");
        }
    }
}

fn generate_perform_until_before(w: &mut RustWriter, cond: &str, p: &PerformStatement) {
    w.open_block(&format!("while !({cond}) {{"));
    if let Some(ref target) = p.target {
        let fn_name = cobol_to_rust_name(&target.name, "");
        w.line(&format!("{fn_name}(ws, ctx);"));
    } else {
        for stmt in &p.body {
            generate_statement(w, stmt);
        }
    }
    w.close_block("}");
}

fn generate_perform_until_after(w: &mut RustWriter, cond: &str, p: &PerformStatement) {
    w.open_block("loop {");
    if let Some(ref target) = p.target {
        let fn_name = cobol_to_rust_name(&target.name, "");
        w.line(&format!("{fn_name}(ws, ctx);"));
    } else {
        for stmt in &p.body {
            generate_statement(w, stmt);
        }
    }
    w.open_block(&format!("if {cond} {{"));
    w.line("break;");
    w.close_block("}");
    w.close_block("}");
}

fn generate_goto(w: &mut RustWriter, g: &GoToStatement) {
    if let Some(target) = g.targets.first() {
        let fn_name = cobol_to_rust_name(target, "");
        w.line(&format!(
            "// GO TO {target} -- requires control flow redesign"
        ));
        w.line(&format!("{fn_name}(ws, ctx);"));
        w.line("return;");
    }
}

fn generate_initialize(w: &mut RustWriter, init: &InitializeStatement) {
    for target in &init.targets {
        let dest = data_ref_expr(target);
        w.line(&format!(
            "cobol_initialize(&mut {dest}, &ctx.config);"
        ));
    }
}

fn generate_call(w: &mut RustWriter, call: &CallStatement) {
    let program = operand_expr(&call.program);
    w.line(&format!(
        "cobol_call({program}, &mut ws, ctx);"
    ));
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
    w.line(&format!("ctx.{source}(&mut {target});"));
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
            "ws.{fname}.open({mode}).expect(\"OPEN {}\");",
            file.file_name
        ));
    }
}

fn generate_close(w: &mut RustWriter, close: &CloseStatement) {
    for file_name in &close.files {
        let fname = cobol_to_rust_name(file_name, "");
        w.line(&format!(
            "ws.{fname}.close().expect(\"CLOSE {file_name}\");"
        ));
    }
}

fn generate_read(w: &mut RustWriter, read: &ReadStatement) {
    let fname = cobol_to_rust_name(&read.file_name, "");

    // Determine read call based on whether a KEY is specified
    let read_call = if let Some(ref key_ref) = read.key {
        let key_expr = data_ref_expr(key_ref);
        format!("ws.{fname}.read_by_key({key_expr}.as_bytes())")
    } else {
        format!("ws.{fname}.read_next()")
    };

    // If there are AT END / NOT AT END handlers, wrap in match
    if !read.at_end.is_empty() || !read.not_at_end.is_empty() {
        w.open_block(&format!("match {read_call} {{"));

        // Ok(data) -> NOT AT END path
        w.open_block("Ok(data) => {");
        if let Some(ref into_ref) = read.into {
            let into_expr = data_ref_expr(into_ref);
            w.line(&format!(
                "{into_expr}.fill_bytes(&data[..{into_expr}.byte_length().min(data.len())]);"
            ));
        }
        for stmt in &read.not_at_end {
            generate_statement(w, stmt);
        }
        w.close_block("}");

        // Err(AT_END) -> AT END path
        w.open_block("Err(_) => {");
        for stmt in &read.at_end {
            generate_statement(w, stmt);
        }
        w.close_block("}");

        w.close_block("}");
    } else if !read.invalid_key.is_empty() || !read.not_invalid_key.is_empty() {
        // INVALID KEY / NOT INVALID KEY (indexed/relative files)
        w.open_block(&format!("match {read_call} {{"));

        w.open_block("Ok(data) => {");
        if let Some(ref into_ref) = read.into {
            let into_expr = data_ref_expr(into_ref);
            w.line(&format!(
                "{into_expr}.fill_bytes(&data[..{into_expr}.byte_length().min(data.len())]);"
            ));
        }
        for stmt in &read.not_invalid_key {
            generate_statement(w, stmt);
        }
        w.close_block("}");

        w.open_block("Err(_) => {");
        for stmt in &read.invalid_key {
            generate_statement(w, stmt);
        }
        w.close_block("}");

        w.close_block("}");
    } else {
        // Simple read with no handler
        if let Some(ref into_ref) = read.into {
            let into_expr = data_ref_expr(into_ref);
            w.open_block(&format!("if let Ok(data) = {read_call} {{"));
            w.line(&format!(
                "{into_expr}.fill_bytes(&data[..{into_expr}.byte_length().min(data.len())]);"
            ));
            w.close_block("}");
        } else {
            w.line(&format!("let _ = {read_call};"));
        }
    }
}

fn generate_write(w: &mut RustWriter, write: &WriteStatement) {
    let rec = cobol_to_rust_name(&write.record_name, "");

    // FROM clause: copy source into record before writing
    if let Some(ref from_ref) = write.from {
        let from_expr = data_ref_expr(from_ref);
        w.line(&format!(
            "ws.{rec}.fill_bytes(&{from_expr}.as_bytes()[..ws.{rec}.byte_length().min({from_expr}.byte_length())]);"
        ));
    }

    // WRITE the record
    let write_call = format!("ws.{rec}_file.write_record(ws.{rec}.as_bytes())");

    // ADVANCING clause generates print control after write
    if let Some(ref adv) = write.advancing {
        match adv {
            Advancing::Page => {
                w.line(&format!("{write_call}.expect(\"WRITE {}\");", write.record_name));
                w.line("print!(\"\\x0C\"); // page eject");
            }
            Advancing::Lines(op) => {
                w.line(&format!("{write_call}.expect(\"WRITE {}\");", write.record_name));
                let lines = operand_expr(op);
                w.open_block(&format!("for _ in 0..{lines} {{"));
                w.line("println!();");
                w.close_block("}");
            }
        }
    } else if !write.invalid_key.is_empty() || !write.not_invalid_key.is_empty() {
        // INVALID KEY / NOT INVALID KEY
        w.open_block(&format!("match {write_call} {{"));

        w.open_block("Ok(()) => {");
        for stmt in &write.not_invalid_key {
            generate_statement(w, stmt);
        }
        w.close_block("}");

        w.open_block("Err(_) => {");
        for stmt in &write.invalid_key {
            generate_statement(w, stmt);
        }
        w.close_block("}");

        w.close_block("}");
    } else {
        w.line(&format!("{write_call}.expect(\"WRITE {}\");", write.record_name));
    }
}

fn generate_rewrite(w: &mut RustWriter, rw: &RewriteStatement) {
    let rec = cobol_to_rust_name(&rw.record_name, "");

    // FROM clause
    if let Some(ref from_ref) = rw.from {
        let from_expr = data_ref_expr(from_ref);
        w.line(&format!(
            "ws.{rec}.fill_bytes(&{from_expr}.as_bytes()[..ws.{rec}.byte_length().min({from_expr}.byte_length())]);"
        ));
    }

    let rewrite_call = format!("ws.{rec}_file.rewrite_record(ws.{rec}.as_bytes())");

    if !rw.invalid_key.is_empty() || !rw.not_invalid_key.is_empty() {
        w.open_block(&format!("match {rewrite_call} {{"));

        w.open_block("Ok(()) => {");
        for stmt in &rw.not_invalid_key {
            generate_statement(w, stmt);
        }
        w.close_block("}");

        w.open_block("Err(_) => {");
        for stmt in &rw.invalid_key {
            generate_statement(w, stmt);
        }
        w.close_block("}");

        w.close_block("}");
    } else {
        w.line(&format!(
            "{rewrite_call}.expect(\"REWRITE {}\");",
            rw.record_name
        ));
    }
}

fn generate_delete(w: &mut RustWriter, del: &DeleteStatement) {
    let fname = cobol_to_rust_name(&del.file_name, "");
    let delete_call = format!("ws.{fname}.delete_record()");

    if !del.invalid_key.is_empty() || !del.not_invalid_key.is_empty() {
        w.open_block(&format!("match {delete_call} {{"));

        w.open_block("Ok(()) => {");
        for stmt in &del.not_invalid_key {
            generate_statement(w, stmt);
        }
        w.close_block("}");

        w.open_block("Err(_) => {");
        for stmt in &del.invalid_key {
            generate_statement(w, stmt);
        }
        w.close_block("}");

        w.close_block("}");
    } else {
        w.line(&format!(
            "{delete_call}.expect(\"DELETE {}\");",
            del.file_name
        ));
    }
}

// ---------------------------------------------------------------------------
// Expression formatting helpers
// ---------------------------------------------------------------------------

/// Format an operand for use in comparisons.
/// Data references use `.to_decimal()` to enable comparison with Decimal values.
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
        Operand::Function(f) => {
            let args: Vec<String> = f.arguments.iter().map(operand_expr).collect();
            format!(
                "cobol_function_{}({})",
                f.name.to_lowercase().replace('-', "_"),
                args.join(", ")
            )
        }
    }
}

/// Format a literal as a Rust expression.
fn literal_expr(lit: &Literal) -> String {
    match lit {
        Literal::Numeric(n) => format!("dec!({n})"),
        Literal::Alphanumeric(s) => format!("\"{s}\""),
        Literal::Figurative(fig) => match fig {
            FigurativeConstant::Spaces => "SPACES".to_string(),
            FigurativeConstant::Zeros => "ZEROS".to_string(),
            FigurativeConstant::HighValues => "HIGH_VALUES".to_string(),
            FigurativeConstant::LowValues => "LOW_VALUES".to_string(),
            FigurativeConstant::Quotes => "QUOTES".to_string(),
            FigurativeConstant::Nulls => "NULLS".to_string(),
        },
    }
}

/// Format a data reference as a Rust expression.
fn data_ref_expr(dr: &DataReference) -> String {
    let field_name = cobol_to_rust_name(&dr.name, "");
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
                let sub_expr = data_ref_expr(sub_dr);
                expr = format!("{expr}[({sub_expr} - 1) as usize]");
            }
            Subscript::Expr(_) => {
                expr = format!("{expr}[0 /* complex subscript */]");
            }
        }
    }

    expr
}

/// Format an arithmetic expression as a Rust expression.
fn arith_expr_str(expr: &ArithExpr) -> String {
    match expr {
        ArithExpr::Operand(op) => operand_expr(op),
        ArithExpr::Negate(inner) => format!("-({})", arith_expr_str(inner)),
        ArithExpr::BinaryOp { left, op, right } => {
            let l = arith_expr_str(left);
            let r = arith_expr_str(right);
            let op_str = match op {
                ArithOp::Add => "+",
                ArithOp::Subtract => "-",
                ArithOp::Multiply => "*",
                ArithOp::Divide => "/",
                ArithOp::Power => ".pow",
            };
            if *op == ArithOp::Power {
                format!("({l}){op_str}({r})")
            } else {
                format!("({l} {op_str} {r})")
            }
        }
        ArithExpr::Paren(inner) => format!("({})", arith_expr_str(inner)),
    }
}

/// Format a condition as a Rust expression.
/// Uses `.to_decimal()` for numeric comparisons since PackedDecimal
/// doesn't implement PartialOrd directly.
fn condition_expr(cond: &Condition) -> String {
    match cond {
        Condition::Comparison { left, op, right } => {
            let l = operand_cmp_expr(left);
            let r = operand_cmp_expr(right);
            let op_str = match op {
                ComparisonOp::Equal => "==",
                ComparisonOp::NotEqual => "!=",
                ComparisonOp::LessThan => "<",
                ComparisonOp::GreaterThan => ">",
                ComparisonOp::LessOrEqual => "<=",
                ComparisonOp::GreaterOrEqual => ">=",
            };
            format!("{l} {op_str} {r}")
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
            let f = data_ref_expr(dr);
            format!("{f}.is_true()")
        }
        Condition::Not(inner) => {
            format!("!({})", condition_expr(inner))
        }
        Condition::And(left, right) => {
            // Note: IBM COBOL evaluates BOTH sides (no short-circuit).
            // We generate standard Rust && but add a comment.
            format!(
                "({} && {})",
                condition_expr(left),
                condition_expr(right)
            )
        }
        Condition::Or(left, right) => {
            format!(
                "({} || {})",
                condition_expr(left),
                condition_expr(right)
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operand_formatting() {
        let op = Operand::Literal(Literal::Numeric("42".to_string()));
        assert_eq!(operand_expr(&op), "dec!(42)");

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
        assert_eq!(condition_expr(&cond), "ws.ws_x.to_decimal() > dec!(0)");
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
        assert_eq!(arith_expr_str(&expr), "(ws.ws_a + dec!(1))");
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
        generate_move(&mut w, &stmt);
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
        generate_read(&mut w, &stmt);
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
        generate_read(&mut w, &stmt);
        let output = w.finish();
        assert!(output.contains("match ws.input_file.read_next()"));
        assert!(output.contains("Ok(data)"));
        assert!(output.contains("Err(_)"));
        assert!(output.contains("END OF FILE"));
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
        generate_write(&mut w, &stmt);
        let output = w.finish();
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
        generate_write(&mut w, &stmt);
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
        generate_rewrite(&mut w, &stmt);
        let output = w.finish();
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
        generate_delete(&mut w, &stmt);
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
        generate_delete(&mut w, &stmt);
        let output = w.finish();
        assert!(output.contains("match ws.indexed_file.delete_record()"));
        assert!(output.contains("Err(_)"));
        assert!(output.contains("KEY NOT FOUND"));
    }
}
