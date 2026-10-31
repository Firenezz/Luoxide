//! Parse a chunk with [`Session`], then lower it to a [`FunctionPrototype`].

use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFiles,
    term::termcolor::{ColorChoice, StandardStream},
};
use luoxide::prelude::*;
use luoxide_parser::error::ParseError;

const DEFAULT_SOURCE: &str = include_str!("../../../lua_scripts/parser/simple.lua");

struct Options {
    display: bool,
    debug_ast: bool,
    proto: bool,
    json: bool,
    trace: Option<TraceMode>,
    dump: Option<PathBuf>,
    source: String,
}

#[derive(Clone, Copy)]
enum TraceMode {
    Shallow,
    Deep,
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
    let options = parse_args();
    init_tracing(options.trace);
    let source = options.source.as_str();

    let mut files = SimpleFiles::new();
    let file_id = files.add("<string>", source);

    let mut session = Session::new();
    match session.parse_chunk(source) {
        Outcome::Ok(chunk) => {
            let proto = session.compile(&chunk);
            dump_success(&session, &chunk, &proto, source, &options);
        }
        Outcome::PartialFailure(chunk, errors) => {
            eprintln!("--- recovered Lua ---");
            eprintln!("{}", session.display(&chunk, source));
            dump_parse_errors(&errors);
            emit(&files, into_diagnostics(&errors, source, file_id));
            let proto = session.compile(&chunk);
            dump_success(&session, &chunk, &proto, source, &options);
        }
        Outcome::TotalFailure(errors) => {
            dump_parse_errors(&errors);
            emit(&files, into_diagnostics(&errors, source, file_id));
            std::process::exit(1);
        }
    }
}

fn dump_success(
    session: &Session,
    chunk: &Chunk,
    proto: &FunctionPrototype,
    source: &str,
    options: &Options,
) {
    if options.display {
        println!("{}", session.display(chunk, source));
    }
    if options.debug_ast {
        println!("{:#?}", session.debug_ast(chunk));
    }
    if options.proto {
        println!("{proto:#?}");
    }
    if options.json {
        match serde_json::to_string_pretty(&session.json_ast(chunk)) {
            Ok(json) => println!("{json}"),
            Err(error) => eprintln!("failed to serialize AST as JSON: {error}"),
        }
    }
    dump_file(options.dump.as_deref(), session, chunk, proto, source);
}

fn dump_file(
    path: Option<&Path>,
    session: &Session,
    chunk: &Chunk,
    proto: &FunctionPrototype,
    source: &str,
) {
    let Some(path) = path else {
        return;
    };
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
        && let Err(error) = fs::create_dir_all(parent)
    {
        eprintln!("failed to create {}: {error}", parent.display());
        return;
    }

    let mut out = String::new();
    let _ = writeln!(out, "--- Lua ---");
    let _ = writeln!(out, "{}", session.display(chunk, source));
    let _ = writeln!(out, "--- AST ---");
    let _ = writeln!(out, "{:#?}", session.debug_ast(chunk));
    let _ = writeln!(out, "--- proto ---");
    let _ = writeln!(out, "{proto:#?}");
    if let Err(error) = fs::write(path, out) {
        eprintln!("failed to write {}: {error}", path.display());
        return;
    }
    eprintln!("wrote compile dump to {}", path.display());

    let json_path = path.with_extension("json");
    match serde_json::to_string_pretty(&session.json_ast(chunk)) {
        Ok(json) => {
            if let Err(error) = fs::write(&json_path, json) {
                eprintln!("failed to write {}: {error}", json_path.display());
                return;
            }
            eprintln!("wrote AST JSON to {}", json_path.display());
        }
        Err(error) => eprintln!("failed to serialize AST JSON: {error}"),
    }
}

fn init_tracing(mode: Option<TraceMode>) {
    if mode.is_none() && std::env::var_os("RUST_LOG").is_none() {
        return;
    }
    let default = match mode.unwrap_or(TraceMode::Shallow) {
        TraceMode::Shallow => "luoxide_parser::parse::shallow=debug",
        TraceMode::Deep => "luoxide_parser::parse::deep=trace",
        TraceMode::Both => "luoxide_parser::parse::shallow=debug,luoxide_parser::parse::deep=trace",
    };
    let filter =
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| default.into());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .compact()
        .init();
}

fn parse_args() -> Options {
    let mut display = false;
    let mut debug_ast = false;
    let mut proto = false;
    let mut json = false;
    let mut trace = None;
    let mut dump = None;
    let mut source_parts = Vec::new();

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--display" | "-d" => display = true,
            "--ast" => debug_ast = true,
            "--proto" => proto = true,
            "--json" => json = true,
            "--dump" => {
                let Some(value) = args.next() else {
                    eprintln!("--dump requires a path");
                    std::process::exit(2);
                };
                dump = Some(PathBuf::from(value));
            }
            "--trace" => {
                let Some(value) = args.next() else {
                    eprintln!("--trace requires shallow, deep, or both");
                    std::process::exit(2);
                };
                trace = Some(parse_trace_mode(&value));
            }
            "--help" | "-h" => {
                eprintln!(
                    "Usage: cargo run -p luoxide --example compiler -- [--display] [--ast] [--proto] [--json] [--dump FILE] [--trace shallow|deep|both] [source]"
                );
                eprintln!("  --display        reconstructed Lua");
                eprintln!("  --ast            Debug AST");
                eprintln!("  --proto          FunctionPrototype (default if nothing else is set)");
                eprintln!("  --json           AST as JSON (string table + atom ids)");
                eprintln!(
                    "  --dump FILE      write .debug and AST .json (default: target/compiler-proto.debug)"
                );
                std::process::exit(0);
            }
            _ if let Some(value) = arg.strip_prefix("--trace=") => {
                trace = Some(parse_trace_mode(value));
            }
            _ if let Some(value) = arg.strip_prefix("--dump=") => {
                dump = Some(PathBuf::from(value));
            }
            _ => source_parts.push(arg),
        }
    }

    if !display && !debug_ast && !proto && !json {
        proto = true;
    }
    if dump.is_none() {
        dump = Some(PathBuf::from("target/compiler-proto.debug"));
    }

    let source = if source_parts.is_empty() {
        DEFAULT_SOURCE.to_string()
    } else {
        source_parts.join(" ")
    };

    Options {
        display,
        debug_ast,
        proto,
        json,
        trace,
        dump,
        source,
    }
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
