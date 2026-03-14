//! cobol2rust CLI: transpile COBOL to Rust, validate, preprocess.

mod analyze;
mod check;
mod compile_cmd;
mod corpus_cmd;
mod diff_cmd;
mod init_cmd;
mod parse_cmd;
mod pipeline;
mod preprocess;
mod project_cmd;
mod scan;
mod transpile_cmd;
mod workspace;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};

/// cobol2rust -- transpile COBOL source code to Rust.
#[derive(Debug, Parser)]
#[command(name = "cobol2rust", version, about)]
pub struct Cli {
    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Command,

    /// Copybook search directory (repeatable).
    #[arg(short = 'C', long = "copybook-path", global = true)]
    pub copybook_paths: Vec<PathBuf>,

    /// Override source format detection.
    #[arg(short = 'f', long = "source-format", global = true, default_value = "auto")]
    pub source_format: SourceFormatArg,

    /// Increase output verbosity (repeatable: -v, -vv, -vvv).
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Suppress non-error output.
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Color output control.
    #[arg(long, global = true, default_value = "auto")]
    pub color: ColorArg,

    /// Configuration file path (TOML).
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,
}

/// Source format override.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SourceFormatArg {
    Auto,
    Fixed,
    Free,
}

/// Color output control.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ColorArg {
    Auto,
    Always,
    Never,
}

/// CLI subcommands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Transpile COBOL source to Rust.
    Transpile(transpile_cmd::TranspileArgs),
    /// Validate COBOL source without transpiling.
    Check(check::CheckArgs),
    /// Expand COPY statements and clean source format.
    Preprocess(preprocess::PreprocessArgs),
    /// Parse COBOL source to AST (tree or JSON).
    Parse(parse_cmd::ParseArgs),
    /// Scaffold a Cargo workspace from COBOL sources.
    Init(init_cmd::InitArgs),
    /// Transpile and build in one step.
    Compile(compile_cmd::CompileArgs),
    /// Compare transpilation outputs between two COBOL files.
    Diff(diff_cmd::DiffArgs),
    /// Run the full pipeline for a single COBOL project.
    Project(project_cmd::ProjectArgs),
    /// Run the pipeline across a multi-repo COBOL corpus.
    Corpus(corpus_cmd::CorpusArgs),
    /// Scan an enterprise COBOL codebase with DuckDB persistence.
    Scan(scan::args::ScanArgs),
    /// Show scan progress and history from DuckDB.
    Status(scan::args::StatusArgs),
    /// Generate reports from a completed scan.
    Report(scan::args::ReportArgs),
    /// Internal: worker process for parallel scanning (do not call directly).
    #[command(hide = true)]
    ScanWorker(scan::args::ScanWorkerArgs),
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    // Configure miette color based on --color flag.
    match cli.color {
        ColorArg::Never => {
            miette::set_hook(Box::new(|_| {
                Box::new(
                    miette::MietteHandlerOpts::new()
                        .color(false)
                        .build(),
                )
            }))
            .ok();
        }
        ColorArg::Always => {
            miette::set_hook(Box::new(|_| {
                Box::new(
                    miette::MietteHandlerOpts::new()
                        .color(true)
                        .build(),
                )
            }))
            .ok();
        }
        ColorArg::Auto => {}
    }

    let result = match cli.command {
        Command::Project(ref args) => project_cmd::run(&cli, args),
        Command::Corpus(ref args) => corpus_cmd::run(&cli, args),
        Command::Transpile(ref args) => transpile_cmd::run(&cli, args),
        Command::Check(ref args) => check::run(&cli, args),
        Command::Preprocess(ref args) => preprocess::run(&cli, args),
        Command::Parse(ref args) => parse_cmd::run(&cli, args),
        Command::Init(ref args) => init_cmd::run(&cli, args),
        Command::Compile(ref args) => compile_cmd::run(&cli, args),
        Command::Diff(ref args) => diff_cmd::run(&cli, args),
        Command::Scan(ref args) => scan::run(&cli, args),
        Command::Status(ref args) => scan::run_status(args),
        Command::Report(ref args) => scan::run_report(args),
        Command::ScanWorker(ref args) => scan::run_scan_worker(args),
    };

    match result {
        Ok(code) => code,
        Err(e) => {
            eprintln!("{e:?}");
            ExitCode::from(1)
        }
    }
}
