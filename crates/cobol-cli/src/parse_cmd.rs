//! `cobol2rust parse` -- parse COBOL source to AST (tree or JSON).

use std::fmt::Write as _;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, ValueEnum};
use miette::{miette, Context, IntoDiagnostic, Result};

use cobol_transpiler::ast::{
    ArithExpr, CobolProgram, Condition, DataEntry, DataReference, Literal, Operand,
    PicCategory, ProcedureDivision, Statement, Usage,
};
use cobol_transpiler::parser::parse_cobol;

use crate::Cli;

/// Arguments for `cobol2rust parse`.
#[derive(Debug, Args)]
pub struct ParseArgs {
    /// COBOL source file to parse.
    pub input: PathBuf,

    /// Output format.
    #[arg(long, default_value = "tree")]
    pub format: ParseFormat,

    /// Section filter.
    #[arg(long, default_value = "all")]
    pub section: SectionFilter,

    /// Output file (default: stdout).
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

/// Output format for parse results.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ParseFormat {
    Tree,
    Json,
}

/// Section filter for parse output.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SectionFilter {
    All,
    Data,
    Procedure,
    Identification,
}

/// Run the parse subcommand.
pub fn run(cli: &Cli, args: &ParseArgs) -> Result<ExitCode> {
    let source = fs::read_to_string(&args.input)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read {}", args.input.display()))?;

    if !cli.quiet {
        eprintln!("Parsing {}...", args.input.display());
    }

    let program = parse_cobol(&source).map_err(|e| miette!("{e}"))?;

    let output = match args.format {
        ParseFormat::Json => render_json(&program, args.section)?,
        ParseFormat::Tree => render_tree(&program, args.section),
    };

    match &args.output {
        Some(path) => {
            fs::write(path, &output)
                .into_diagnostic()
                .wrap_err_with(|| format!("failed to write {}", path.display()))?;
            if !cli.quiet {
                eprintln!("Wrote {}", path.display());
            }
        }
        None => {
            std::io::stdout()
                .write_all(output.as_bytes())
                .into_diagnostic()
                .wrap_err("failed to write to stdout")?;
        }
    }

    Ok(ExitCode::SUCCESS)
}

// ---------------------------------------------------------------------------
// JSON output
// ---------------------------------------------------------------------------

fn render_json(program: &CobolProgram, section: SectionFilter) -> Result<String> {
    let json = match section {
        SectionFilter::All => serde_json::to_string_pretty(program),
        SectionFilter::Data => {
            serde_json::to_string_pretty(&program.data_division)
        }
        SectionFilter::Procedure => {
            serde_json::to_string_pretty(&program.procedure_division)
        }
        SectionFilter::Identification => {
            serde_json::to_string_pretty(&serde_json::json!({
                "program_id": program.program_id,
                "source_path": program.source_path,
            }))
        }
    };
    json.into_diagnostic().wrap_err("failed to serialize AST to JSON")
}

// ---------------------------------------------------------------------------
// Tree output
// ---------------------------------------------------------------------------

fn render_tree(program: &CobolProgram, section: SectionFilter) -> String {
    let mut buf = String::new();
    buf.push_str(&program.program_id);
    buf.push('\n');

    let show_id = matches!(section, SectionFilter::All | SectionFilter::Identification);
    let show_data = matches!(section, SectionFilter::All | SectionFilter::Data);
    let show_proc = matches!(section, SectionFilter::All | SectionFilter::Procedure);

    // Determine which sections are present for connector logic.
    let has_data = show_data && program.data_division.is_some();
    let has_proc = show_proc && program.procedure_division.is_some();

    if show_id {
        let is_last = !has_data && !has_proc;
        let connector = if is_last { "`" } else { "+" };
        let _ = writeln!(buf,"{connector}-- IDENTIFICATION DIVISION");
        let prefix = if is_last { "    " } else { "|   " };
        let _ = writeln!(buf,"{prefix}Program-ID: {}", program.program_id);
    }

    if has_data {
        let is_last = !has_proc;
        let connector = if is_last { "`" } else { "+" };
        let prefix = if is_last { "    " } else { "|   " };
        let _ = writeln!(buf,"{connector}-- DATA DIVISION");
        render_data_division(&mut buf, program.data_division.as_ref().unwrap(), prefix);
    }

    if has_proc {
        buf.push_str("`-- PROCEDURE DIVISION\n");
        render_procedure_division(&mut buf, program.procedure_division.as_ref().unwrap(), "    ");
    }

    buf
}

fn render_data_division(buf: &mut String, dd: &cobol_transpiler::ast::DataDivision, prefix: &str) {
    let sections: Vec<(&str, &[DataEntry])> = [
        ("WORKING-STORAGE SECTION", dd.working_storage.as_slice()),
        ("LOCAL-STORAGE SECTION", dd.local_storage.as_slice()),
        ("LINKAGE SECTION", dd.linkage.as_slice()),
    ]
    .into_iter()
    .filter(|(_, entries)| !entries.is_empty())
    .collect();

    let has_file_section = !dd.file_section.is_empty();
    let total = sections.len() + usize::from(has_file_section);

    for (i, (name, entries)) in sections.iter().enumerate() {
        let is_last = i == total - 1;
        let connector = if is_last { "`" } else { "+" };
        let child_prefix = if is_last {
            format!("{prefix}    ")
        } else {
            format!("{prefix}|   ")
        };
        let _ = writeln!(buf,"{prefix}{connector}-- {name}");
        render_data_entries(buf, entries, &child_prefix);
    }

    if has_file_section {
        let _ = writeln!(buf,"{prefix}`-- FILE SECTION");
        let child_prefix = format!("{prefix}    ");
        for fd in &dd.file_section {
            let _ = writeln!(buf,"{child_prefix}FD {}", fd.file_name);
            render_data_entries(buf, &fd.records, &format!("{child_prefix}    "));
        }
    }

    if total == 0 {
        // Show empty sections that were filtered.
        let ws_empty = dd.working_storage.is_empty();
        let link_empty = dd.linkage.is_empty();
        if ws_empty {
            let _ = writeln!(buf,"{prefix}+-- WORKING-STORAGE SECTION");
            let _ = writeln!(buf,"{prefix}|   (empty)");
        }
        if link_empty {
            let _ = writeln!(buf,"{prefix}`-- LINKAGE SECTION");
            let _ = writeln!(buf,"{prefix}    (empty)");
        }
    }
}

fn render_data_entries(buf: &mut String, entries: &[DataEntry], prefix: &str) {
    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let connector = if is_last { "`" } else { "+" };
        let child_prefix = if is_last {
            format!("{prefix}    ")
        } else {
            format!("{prefix}|   ")
        };

        let desc = format_data_entry(entry);
        let _ = writeln!(buf,"{prefix}{connector}-- {desc}");

        // Render children (88-level items show values inline, skip sub-rendering).
        if !entry.children.is_empty() {
            render_data_entries(buf, &entry.children, &child_prefix);
        }
    }
}

fn format_data_entry(entry: &DataEntry) -> String {
    let level = format!("{:02}", entry.level);

    if let Some(ref pic) = entry.pic {
        let cat_str = match pic.category {
            PicCategory::Alphabetic => "Alphabetic",
            PicCategory::Alphanumeric => "Alphanumeric",
            PicCategory::Numeric => "Numeric",
            PicCategory::NumericEdited => "NumericEdited",
            PicCategory::AlphanumericEdited => "AlphaEdited",
        };

        let usage_str = match entry.usage {
            Usage::Display => String::new(),
            Usage::Comp | Usage::Comp5 => String::from(" COMP"),
            Usage::Comp3 => String::from(" COMP-3"),
            Usage::Comp1 => String::from(" COMP-1"),
            Usage::Comp2 => String::from(" COMP-2"),
            Usage::Index => String::from(" INDEX"),
            Usage::Pointer => String::from(" POINTER"),
        };

        let size = entry.byte_length.map_or(String::new(), |s| {
            if s == 1 {
                String::from(", 1 byte")
            } else {
                format!(", {s} bytes")
            }
        });

        format!(
            "{level} {} (PIC {}{}, {cat_str}{size})",
            entry.name, pic.raw, usage_str,
        )
    } else if entry.level == 88 {
        // 88-level -- show condition values inline.
        let vals: Vec<String> = entry.condition_values.iter().map(|cv| match cv {
            cobol_transpiler::ast::ConditionValue::Single(lit) => format_literal(lit),
            cobol_transpiler::ast::ConditionValue::Range { low, high } => {
                format!("{} THRU {}", format_literal(low), format_literal(high))
            }
        }).collect();
        if vals.is_empty() {
            format!("{level} {}", entry.name)
        } else {
            format!("{level} {} = {}", entry.name, vals.join(", "))
        }
    } else {
        // Group item.
        let size = entry.byte_length.map_or(String::new(), |s| {
            format!(", {s} bytes")
        });
        format!("{level} {} (Group{size})", entry.name)
    }
}

fn format_literal(lit: &Literal) -> String {
    match lit {
        Literal::Numeric(s) => s.clone(),
        Literal::Alphanumeric(s) => format!("'{s}'"),
        Literal::Figurative(f) => format!("{f:?}").to_uppercase(),
    }
}

// ---------------------------------------------------------------------------
// Procedure division tree
// ---------------------------------------------------------------------------

fn render_procedure_division(buf: &mut String, pd: &ProcedureDivision, prefix: &str) {
    if !pd.using_params.is_empty() {
        let params: Vec<&str> = pd.using_params.iter().map(|p| p.name.as_str()).collect();
        let _ = writeln!(buf,"{prefix}USING {}", params.join(", "));
    }

    let total = pd.sections.len() + pd.paragraphs.len();

    // Render sections.
    for (i, section) in pd.sections.iter().enumerate() {
        let is_last = i == total - 1;
        let connector = if is_last { "`" } else { "+" };
        let child_prefix = if is_last {
            format!("{prefix}    ")
        } else {
            format!("{prefix}|   ")
        };
        let _ = writeln!(buf,"{prefix}{connector}-- SECTION {}", section.name);
        render_paragraphs(buf, &section.paragraphs, &child_prefix);
    }

    // Render standalone paragraphs.
    let para_offset = pd.sections.len();
    render_paragraphs_with_offset(buf, &pd.paragraphs, prefix, para_offset, total);
}

fn render_paragraphs(buf: &mut String, paragraphs: &[cobol_transpiler::ast::Paragraph], prefix: &str) {
    let total = paragraphs.len();
    render_paragraphs_with_offset(buf, paragraphs, prefix, 0, total);
}

fn render_paragraphs_with_offset(
    buf: &mut String,
    paragraphs: &[cobol_transpiler::ast::Paragraph],
    prefix: &str,
    offset: usize,
    total: usize,
) {
    for (i, para) in paragraphs.iter().enumerate() {
        let global_idx = offset + i;
        let is_last = global_idx == total - 1;
        let connector = if is_last { "`" } else { "+" };
        let child_prefix = if is_last {
            format!("{prefix}    ")
        } else {
            format!("{prefix}|   ")
        };
        let _ = writeln!(buf,"{prefix}{connector}-- {}", para.name);

        // Render statement summaries.
        for sentence in &para.sentences {
            for stmt in &sentence.statements {
                let summary = summarize_statement(stmt);
                let _ = writeln!(buf,"{child_prefix}{summary}");
            }
        }
    }
}

fn summarize_statement(stmt: &Statement) -> String {
    match stmt {
        Statement::Move(m) => {
            let src = summarize_operand(&m.source);
            let dests: Vec<String> = m.destinations.iter().map(format_data_ref).collect();
            if m.corresponding {
                format!("MOVE CORRESPONDING {src} TO {}", dests.join(" "))
            } else {
                format!("MOVE {src} TO {}", dests.join(" "))
            }
        }
        Statement::Add(a) => {
            let ops: Vec<String> = a.operands.iter().map(summarize_operand).collect();
            let targets: Vec<String> = a.to.iter().map(|t| format_data_ref(&t.field)).collect();
            format!("ADD {} TO {}", ops.join(" "), targets.join(" "))
        }
        Statement::Subtract(s) => {
            let ops: Vec<String> = s.operands.iter().map(summarize_operand).collect();
            let targets: Vec<String> = s.from.iter().map(|t| format_data_ref(&t.field)).collect();
            format!("SUBTRACT {} FROM {}", ops.join(" "), targets.join(" "))
        }
        Statement::Multiply(m) => {
            let op = summarize_operand(&m.operand);
            let targets: Vec<String> = m.by.iter().map(|t| format_data_ref(&t.field)).collect();
            format!("MULTIPLY {op} BY {}", targets.join(" "))
        }
        Statement::Divide(d) => {
            let op = summarize_operand(&d.operand);
            let targets: Vec<String> = d.into.iter().map(|t| format_data_ref(&t.field)).collect();
            format!("DIVIDE {op} INTO {}", targets.join(" "))
        }
        Statement::Compute(c) => {
            let targets: Vec<String> = c.targets.iter().map(|t| format_data_ref(&t.field)).collect();
            let expr = summarize_arith_expr(&c.expression);
            format!("COMPUTE {} = {expr}", targets.join(" "))
        }
        Statement::Display(d) => {
            let items: Vec<String> = d.items.iter().map(summarize_operand).collect();
            format!("DISPLAY {}", items.join(" "))
        }
        Statement::Accept(a) => {
            format!("ACCEPT {}", format_data_ref(&a.target))
        }
        Statement::If(i) => {
            let cond = summarize_condition(&i.condition);
            format!("IF {cond}")
        }
        Statement::Evaluate(e) => {
            let subjs: Vec<String> = e.subjects.iter().map(|s| match s {
                cobol_transpiler::ast::EvaluateSubject::Expr(op) => summarize_operand(op),
                cobol_transpiler::ast::EvaluateSubject::Bool(b) => {
                    if *b { String::from("TRUE") } else { String::from("FALSE") }
                }
            }).collect();
            format!("EVALUATE {}", subjs.join(" ALSO "))
        }
        Statement::Perform(p) => {
            if let Some(ref target) = p.target {
                let thru = p.thru.as_ref().map_or(String::new(), |t| format!(" THRU {t}"));
                let loop_info = summarize_loop_type(&p.loop_type);
                format!("PERFORM {}{thru}{loop_info}", target.name)
            } else {
                let loop_info = summarize_loop_type(&p.loop_type);
                format!("PERFORM INLINE{loop_info}")
            }
        }
        Statement::GoTo(g) => {
            format!("GO TO {}", g.targets.join(" "))
        }
        Statement::StopRun => String::from("STOP RUN"),
        Statement::GoBack => String::from("GOBACK"),
        Statement::Continue => String::from("CONTINUE"),
        Statement::NextSentence => String::from("NEXT SENTENCE"),
        Statement::ExitProgram => String::from("EXIT PROGRAM"),
        Statement::ExitParagraph => String::from("EXIT PARAGRAPH"),
        Statement::ExitSection => String::from("EXIT SECTION"),
        Statement::Open(o) => {
            let files: Vec<String> = o.files.iter().map(|f| {
                format!("{:?} {}", f.mode, f.file_name)
            }).collect();
            format!("OPEN {}", files.join(", "))
        }
        Statement::Close(c) => {
            format!("CLOSE {}", c.files.join(" "))
        }
        Statement::Read(r) => {
            let clause = if r.at_end.is_empty() {
                ""
            } else {
                " AT END ..."
            };
            format!("READ {}{clause}", r.file_name)
        }
        Statement::Write(w) => {
            format!("WRITE {}", w.record_name)
        }
        Statement::Rewrite(r) => {
            format!("REWRITE {}", r.record_name)
        }
        Statement::Delete(d) => {
            format!("DELETE {}", d.file_name)
        }
        Statement::Start(s) => {
            format!("START {}", s.file_name)
        }
        Statement::Call(c) => {
            let prog = summarize_operand(&c.program);
            let params: Vec<String> = c.using.iter().filter_map(|p| {
                p.operand.as_ref().map(summarize_operand)
            }).collect();
            if params.is_empty() {
                format!("CALL {prog}")
            } else {
                format!("CALL {prog} USING {}", params.join(" "))
            }
        }
        Statement::Cancel(c) => {
            let progs: Vec<String> = c.programs.iter().map(summarize_operand).collect();
            format!("CANCEL {}", progs.join(" "))
        }
        Statement::String(_) => String::from("STRING ..."),
        Statement::Unstring(_) => String::from("UNSTRING ..."),
        Statement::Inspect(_) => String::from("INSPECT ..."),
        Statement::Sort(s) => format!("SORT {}", s.file_name),
        Statement::Merge(m) => format!("MERGE {}", m.file_name),
        Statement::Release(r) => format!("RELEASE {}", r.record_name),
        Statement::Return(r) => format!("RETURN {}", r.file_name),
        Statement::Set(s) => {
            let targets: Vec<String> = s.targets.iter().map(format_data_ref).collect();
            format!("SET {}", targets.join(" "))
        }
        Statement::Alter(a) => {
            format!("ALTER {} TO PROCEED TO {}", a.source, a.target)
        }
        Statement::Initialize(init) => {
            let targets: Vec<String> = init.targets.iter().map(format_data_ref).collect();
            format!("INITIALIZE {}", targets.join(" "))
        }
        Statement::ExecSql(sql) => {
            format!("EXEC SQL {:?}: {}", sql.sql_type, sql.raw_sql)
        }
    }
}

fn summarize_operand(op: &Operand) -> String {
    match op {
        Operand::Literal(lit) => format_literal(lit),
        Operand::DataRef(dr) => format_data_ref(dr),
        Operand::Function(f) => {
            let args: Vec<String> = f.arguments.iter().map(summarize_operand).collect();
            format!("FUNCTION {}({})", f.name, args.join(", "))
        }
    }
}

fn format_data_ref(dr: &DataReference) -> String {
    dr.to_string()
}

fn summarize_condition(cond: &Condition) -> String {
    match cond {
        Condition::Comparison { left, op, right } => {
            format!("{} {op} {}", summarize_operand(left), summarize_operand(right))
        }
        Condition::ClassTest { field, class } => {
            format!("{} IS {class:?}", format_data_ref(field))
        }
        Condition::SignTest { field, sign } => {
            format!("{} IS {sign:?}", format_data_ref(field))
        }
        Condition::ConditionName(dr) => format_data_ref(dr),
        Condition::Not(inner) => format!("NOT {}", summarize_condition(inner)),
        Condition::And(a, b) => {
            format!("{} AND {}", summarize_condition(a), summarize_condition(b))
        }
        Condition::Or(a, b) => {
            format!("{} OR {}", summarize_condition(a), summarize_condition(b))
        }
    }
}

fn summarize_arith_expr(expr: &ArithExpr) -> String {
    match expr {
        ArithExpr::Operand(op) => summarize_operand(op),
        ArithExpr::Negate(inner) => format!("-{}", summarize_arith_expr(inner)),
        ArithExpr::BinaryOp { left, op, right } => {
            format!(
                "{} {op} {}",
                summarize_arith_expr(left),
                summarize_arith_expr(right),
            )
        }
        ArithExpr::Paren(inner) => format!("({})", summarize_arith_expr(inner)),
    }
}

fn summarize_loop_type(lt: &cobol_transpiler::ast::PerformLoopType) -> String {
    match lt {
        cobol_transpiler::ast::PerformLoopType::Once => String::new(),
        cobol_transpiler::ast::PerformLoopType::Times(op) => {
            format!(" {} TIMES", summarize_operand(op))
        }
        cobol_transpiler::ast::PerformLoopType::Until { condition, .. } => {
            format!(" UNTIL {}", summarize_condition(condition))
        }
        cobol_transpiler::ast::PerformLoopType::Varying { counter, from, by, until, .. } => {
            format!(
                " VARYING {} FROM {} BY {} UNTIL {}",
                format_data_ref(counter),
                summarize_operand(from),
                summarize_operand(by),
                summarize_condition(until),
            )
        }
    }
}
