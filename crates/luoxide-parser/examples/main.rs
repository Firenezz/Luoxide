use std::fmt;

use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFiles,
    term::termcolor::{ColorChoice, StandardStream},
};
use luoxide_parser::{ast::DisplayLua, parser::compile_chunk};
use luoxide_parser::{error::ParseError, outcome::Outcome, parser::compile_expression};

const DEFAULT_SOURCE: &str = r#"

local constvar <const> = 1
local closevar <close> = resource()
-- a.b.c { a, ["b"] = function(...) return 1 end }
"#;

const SIMPLE_SOURCE: &str = include_str!("../../../lua_scripts/parser/simple.lua");

struct Options {
    display: bool,
    debug: bool,
    expression: bool,
    trace: TraceMode,
    source: String,
}

#[derive(Clone, Copy)]
enum TraceMode {
    /// Production stack, mismatches, recovery.
    Shallow,
    /// Every consumed token; no enter/leave.
    Deep,
    /// Both layers.
    Both,
}

fn parse_trace_mode(value: &str) -> TraceMode {
    match value {
        "shallow" => TraceMode::Shallow,
        "deep" => TraceMode::Deep,
        "both" => TraceMode::Both,
        other => {
            eprintln!("unknown --trace {other}, expected shallow|deep|both");
            std::process::exit(2);
        }
    }
}

fn main() {
    let mut options = parse_args();
    init_tracing(options.trace);
    options.source = SIMPLE_SOURCE.to_string();
    let source = options.source.as_str();

    options.display = true;
    options.debug = false;

    let mut files = SimpleFiles::new();
    let file_id = files.add("<string>", source);

    if options.expression {
        handle_outcome(
            compile_expression(source),
            source,
            &options,
            &files,
            file_id,
        );
    } else {
        handle_outcome(compile_chunk(source), source, &options, &files, file_id);
    }
}

fn init_tracing(mode: TraceMode) {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        tracing_subscriber::EnvFilter::new(match mode {
            TraceMode::Shallow => "luoxide_parser::parse::shallow=debug",
            TraceMode::Deep => "luoxide_parser::parse::deep=trace",
            TraceMode::Both => {
                "luoxide_parser::parse::shallow=debug,luoxide_parser::parse::deep=trace"
            }
        })
    });
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .compact()
        .init();
}

fn handle_outcome<T>(
    outcome: Outcome<T, Vec<ParseError>>,
    source: &str,
    options: &Options,
    files: &SimpleFiles<&str, &str>,
    file_id: usize,
) where
    T: fmt::Debug,
    for<'a> DisplayLua<'a, T>: fmt::Display,
{
    match outcome {
        Outcome::Ok(ast) => dump_tree(&ast, source, options),
        Outcome::PartialFailure(ast, errors) => {
            dump_recovery(&ast, source, &errors);
            emit(files, into_diagnostics(&errors, source, file_id));
        }
        Outcome::TotalFailure(errors) => {
            dump_parse_errors(&errors);
            emit(files, into_diagnostics(&errors, source, file_id));
        }
    }
}

fn parse_args() -> Options {
    let mut display = false;
    let mut debug = false;
    let mut expression = false;
    let mut trace = TraceMode::Shallow;
    let mut source_parts = Vec::new();

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--display" | "-d" => display = true,
            "--debug" => debug = true,
            "--expr" => expression = true,
            "--trace" => {
                let Some(value) = args.next() else {
                    eprintln!("--trace requires shallow, deep, or both");
                    std::process::exit(2);
                };
                trace = parse_trace_mode(&value);
            }
            "--help" | "-h" => {
                eprintln!(
                    "Usage: cargo run -p luoxide-parser --example main -- [--display] [--debug] [--expr] [--trace shallow|deep|both] [source]"
                );
                eprintln!("  --trace shallow  productions, mismatch, error, recover (default)");
                eprintln!("  --trace deep     every eat, no enter/leave");
                eprintln!("  --trace both     both layers");
                eprintln!(
                    "RUST_LOG overrides this, e.g. luoxide_parser::parse::shallow=debug,luoxide_parser::parse::deep=trace"
                );
                std::process::exit(0);
            }
            _ if let Some(value) = arg.strip_prefix("--trace=") => {
                trace = parse_trace_mode(value);
            }
            _ => source_parts.push(arg),
        }
    }

    if !display && !debug {
        debug = true;
    }

    let source = if source_parts.is_empty() {
        DEFAULT_SOURCE.to_string()
    } else {
        source_parts.join(" ")
    };

    Options {
        display,
        debug,
        expression,
        trace,
        source,
    }
}

fn dump_tree<T>(ast: &T, source: &str, options: &Options)
where
    T: fmt::Debug,
    for<'a> DisplayLua<'a, T>: fmt::Display,
{
    if options.display {
        println!("{}", DisplayLua::with_source(ast, source));
    }
    if options.debug {
        println!("{ast:#?}");
    }
}

/// On a partial parse, print recovered Lua, the tree, and the raw error values
/// so recovery bugs are visible instead of looking like valid source.
fn dump_recovery<T>(ast: &T, source: &str, errors: &[ParseError])
where
    T: fmt::Debug,
    for<'a> DisplayLua<'a, T>: fmt::Display,
{
    eprintln!("--- recovered Lua ---");
    eprintln!("{}", DisplayLua::with_source(ast, source));
    eprintln!("--- recovered AST ---");
    eprintln!("{ast:#?}");
    dump_parse_errors(errors);
}

fn dump_parse_errors(errors: &[ParseError]) {
    eprintln!("--- parse errors ({}) ---", errors.len());
    for (i, error) in errors.iter().enumerate() {
        eprintln!("[{i}] {error:#?}");
    }
}

fn into_diagnostics(errors: &[ParseError], source: &str, file_id: usize) -> Vec<Diagnostic<usize>> {
    errors
        .iter()
        .map(|error| {
            let (title, notes) = error.details();
            let labels = error
                .at
                .map(|span| {
                    let range = span.start.to_usize()..span.end.to_usize();
                    let lexeme = source.get(range.clone()).unwrap_or("");
                    vec![Label::primary(file_id, range).with_message(format!("found {lexeme:?}"))]
                })
                .unwrap_or_default();

            Diagnostic::error()
                .with_message(title)
                .with_labels(labels)
                .with_notes(notes)
        })
        .collect()
}

fn emit(files: &SimpleFiles<&str, &str>, diagnostics: Vec<Diagnostic<usize>>) {
    let writer = StandardStream::stderr(ColorChoice::Auto);
    let config = codespan_reporting::term::Config::default();
    for diagnostic in diagnostics {
        codespan_reporting::term::emit_to_write_style(
            &mut writer.lock(),
            &config,
            files,
            &diagnostic,
        )
        .expect("emit diagnostic");
    }
}
