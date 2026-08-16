use std::fmt;

use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFiles,
    term::termcolor::{ColorChoice, StandardStream},
};
use luoxide_parser::ast::DisplayLua;
use luoxide_parser::{error::ParseError, outcome::Outcome, parser::compile_expression};

const DEFAULT_SOURCE: &str = r#"a.b.c { "a" = 5 , "b" = function() { return 1 } }"#;

struct Options {
    display: bool,
    debug: bool,
    source: String,
}

fn main() {
    tracing_subscriber::fmt::init();

    let options = parse_args();
    let source = options.source.as_str();

    let mut files = SimpleFiles::new();
    let file_id = files.add("<string>", source);

    match compile_expression(source) {
        Outcome::Ok(ast) => dump_tree(&ast, source, &options),
        Outcome::PartialFailure(ast, errors) => {
            dump_recovery(&ast, source, &errors);
            emit(&files, into_diagnostics(&errors, source, file_id));
        }
        Outcome::TotalFailure(errors) => {
            dump_parse_errors(&errors);
            emit(&files, into_diagnostics(&errors, source, file_id));
        }
    }
}

fn parse_args() -> Options {
    let mut display = false;
    let mut debug = false;
    let mut source_parts = Vec::new();

    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--display" | "-d" => display = true,
            "--debug" => debug = true,
            "--help" | "-h" => {
                eprintln!(
                    "Usage: cargo run -p luoxide-parser --example main -- [--display] [--debug] [source]"
                );
                std::process::exit(0);
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
        codespan_reporting::term::emit(&mut writer.lock(), &config, files, &diagnostic)
            .expect("emit diagnostic");
    }
}
