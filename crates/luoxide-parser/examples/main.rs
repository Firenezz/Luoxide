use std::fmt::{self, Write};
use std::fs;
use std::path::{Path, PathBuf};

use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFiles,
    term::termcolor::{ColorChoice, StandardStream},
};
use luoxide_parser::{ast::DebugAst, ast::DisplayLua, parser::compile_chunk};
use luoxide_parser::{error::ParseError, outcome::Outcome, parser::compile_expression};
use luoxide_text::Interner;

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
    dump_ast: Option<PathBuf>,
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
    if options.dump_ast.is_none() {
        options.dump_ast = Some(PathBuf::from("target/parser-ast.debug"));
    }

    let mut files = SimpleFiles::new();
    let file_id = files.add("<string>", source);

    let mut intern = Interner::new();
    if options.expression {
        let outcome = compile_expression(&mut intern, source);
        handle_outcome(outcome, source, &intern, &options, &files, file_id);
    } else {
        let outcome = compile_chunk(&mut intern, source);
        handle_outcome(outcome, source, &intern, &options, &files, file_id);
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
    intern: &Interner,
    options: &Options,
    files: &SimpleFiles<&str, &str>,
    file_id: usize,
) where
    T: fmt::Debug,
    for<'a> DisplayLua<'a, T>: fmt::Display,
{
    match outcome {
        Outcome::Ok(ast) => dump_tree(&ast, source, intern, options),
        Outcome::PartialFailure(ast, errors) => {
            dump_recovery(&ast, source, intern, &errors, options);
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
    let mut dump_ast = None;
    let mut source_parts = Vec::new();

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--display" | "-d" => display = true,
            "--debug" => debug = true,
            "--dump-ast" => {
                let Some(value) = args.next() else {
                    eprintln!("--dump-ast requires a path");
                    std::process::exit(2);
                };
                dump_ast = Some(PathBuf::from(value));
            }
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
                    "Usage: cargo run -p luoxide-parser --example main -- [--display] [--debug] [--dump-ast FILE] [--expr] [--trace shallow|deep|both] [source]"
                );
                eprintln!("  --trace shallow  productions, mismatch, error, recover (default)");
                eprintln!("  --trace deep     every eat, no enter/leave");
                eprintln!("  --trace both     both layers");
                eprintln!(
                    "  --dump-ast FILE  write reconstructed Lua and Debug AST (default: target/parser-ast.debug)"
                );
                eprintln!(
                    "RUST_LOG overrides this, e.g. luoxide_parser::parse::shallow=debug,luoxide_parser::parse::deep=trace"
                );
                std::process::exit(0);
            }
            _ if let Some(value) = arg.strip_prefix("--trace=") => {
                trace = parse_trace_mode(value);
            }
            _ if let Some(value) = arg.strip_prefix("--dump-ast=") => {
                dump_ast = Some(PathBuf::from(value));
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
        dump_ast,
        source,
    }
}

fn dump_tree<T>(ast: &T, source: &str, intern: &Interner, options: &Options)
where
    T: fmt::Debug,
    for<'a> DisplayLua<'a, T>: fmt::Display,
{
    if options.display {
        println!("{}", DisplayLua::with_source(ast, intern, source));
    }
    if options.debug {
        println!("{:#?}", DebugAst::new(ast, intern));
    }
    dump_ast_file(options.dump_ast.as_deref(), ast, source, intern);
}

/// Prints recovered Lua, the tree, and the parse errors.
fn dump_recovery<T>(
    ast: &T,
    source: &str,
    intern: &Interner,
    errors: &[ParseError],
    options: &Options,
) where
    T: fmt::Debug,
    for<'a> DisplayLua<'a, T>: fmt::Display,
{
    eprintln!("--- recovered Lua ---");
    eprintln!("{}", DisplayLua::with_source(ast, intern, source));
    eprintln!("--- recovered AST ---");
    eprintln!("{:#?}", DebugAst::new(ast, intern));
    dump_parse_errors(errors);
    dump_ast_file(options.dump_ast.as_deref(), ast, source, intern);
}

fn dump_ast_file<T>(path: Option<&Path>, ast: &T, source: &str, intern: &Interner)
where
    T: fmt::Debug,
    for<'a> DisplayLua<'a, T>: fmt::Display,
{
    let Some(path) = path else {
        return;
    };
    let mut out = String::new();
    let _ = writeln!(out, "--- Lua ---");
    let _ = writeln!(out, "{}", DisplayLua::with_source(ast, intern, source));
    let _ = writeln!(out, "--- AST ---");
    let _ = writeln!(out, "{:#?}", DebugAst::new(ast, intern));
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
        && let Err(error) = fs::create_dir_all(parent)
    {
        eprintln!("failed to create {}: {error}", parent.display());
        return;
    }
    if let Err(error) = fs::write(path, out) {
        eprintln!("failed to write {}: {error}", path.display());
        return;
    }
    eprintln!("wrote AST dump to {}", path.display());
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
