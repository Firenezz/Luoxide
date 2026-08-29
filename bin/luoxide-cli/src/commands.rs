use std::borrow::Cow;
use std::fmt::{self, Write as _};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::ExitCode;

use clap::ArgMatches;
use luoxide::prelude::*;
use thiserror::Error;

mod parse;
mod repl;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown subcommand `{0}`")]
    UnknownCommand(String),
    #[error("a subcommand is required")]
    MissingCommand,
    #[error("missing required argument `{0}`")]
    MissingArg(&'static str),
    #[error("failed to read {path}: {source}")]
    Read {
        path: String,
        #[source]
        source: io::Error,
    },
    #[error("failed to write {path}: {source}")]
    Write {
        path: String,
        #[source]
        source: io::Error,
    },
    #[error("{0}")]
    Io(#[from] io::Error),
}

pub fn parse_command() -> clap::Command {
    parse::command()
}

pub fn repl_command() -> clap::Command {
    repl::command()
}

pub fn dispatch(matches: &ArgMatches) -> Result<ExitCode, Error> {
    match matches.subcommand() {
        Some(("parse", args)) => parse::run(args),
        Some(("repl", args)) => repl::run(args),
        Some((name, _)) => Err(Error::UnknownCommand(name.to_owned())),
        None => Err(Error::MissingCommand),
    }
}

fn render<T>(
    session: &Session,
    file: &str,
    source: &str,
    debug: bool,
    outcome: Outcome<T, Vec<ParseError>>,
) -> (Option<String>, bool)
where
    T: fmt::Debug,
    for<'a> DisplayLua<'a, T>: fmt::Display,
{
    match outcome {
        Outcome::Ok(node) => (Some(format_ast(session, &node, source, debug)), false),
        Outcome::PartialFailure(node, errors) => {
            report_errors(file, source, &errors);
            (Some(format_ast(session, &node, source, debug)), true)
        }
        Outcome::TotalFailure(errors) => {
            report_errors(file, source, &errors);
            (None, true)
        }
    }
}

fn write_output(path: Option<&str>, text: &str) -> Result<(), Error> {
    let text = with_newline(text);
    match path {
        None | Some("-") => {
            let mut out = io::stdout().lock();
            out.write_all(text.as_bytes())?;
            out.flush()?;
            Ok(())
        }
        Some(path) => {
            if let Some(parent) = Path::new(path).parent()
                && !parent.as_os_str().is_empty()
            {
                fs::create_dir_all(parent).map_err(|source| Error::Write {
                    path: path.to_owned(),
                    source,
                })?;
            }
            fs::write(path, text.as_bytes()).map_err(|source| Error::Write {
                path: path.to_owned(),
                source,
            })
        }
    }
}

fn format_ast<T>(session: &Session, node: &T, source: &str, debug: bool) -> String
where
    T: fmt::Debug,
    for<'a> DisplayLua<'a, T>: fmt::Display,
{
    if debug {
        format!("{:#?}", session.debug_ast(node))
    } else {
        session.display(node, source).to_string()
    }
}

fn report_errors(file: &str, source: &str, errors: &[ParseError]) {
    for error in errors {
        report_error(file, source, error);
    }
}

fn report_error(file: &str, source: &str, error: &ParseError) {
    let (title, notes) = error.details();
    let mut message = String::new();
    let _ = writeln!(message, "error: {title}");
    if let Some(span) = error.at {
        let (line, column) = byte_location(source, span.start.to_usize());
        let _ = writeln!(message, " --> {file}:{line}:{column}");
        if let Some(lexeme) = source.get(span.start.to_usize()..span.end.to_usize())
            && !lexeme.is_empty()
        {
            let _ = writeln!(message, "  found {lexeme:?}");
        }
    }
    for note in notes {
        let _ = writeln!(message, "  {note}");
    }
    eprint!("{message}");
}

fn byte_location(source: &str, offset: usize) -> (usize, usize) {
    let offset = offset.min(source.len());
    let mut line = 1_usize;
    let mut column = 1_usize;
    for (index, ch) in source.char_indices() {
        if index >= offset {
            break;
        }
        if ch == '\n' {
            line = line.saturating_add(1);
            column = 1;
        } else {
            column = column.saturating_add(1);
        }
    }
    (line, column)
}

fn with_newline(text: &str) -> Cow<'_, str> {
    if text.ends_with('\n') {
        Cow::Borrowed(text)
    } else {
        Cow::Owned(format!("{text}\n"))
    }
}
