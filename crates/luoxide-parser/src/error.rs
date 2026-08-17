#[cfg(feature = "debug")]
use std::panic::Location;
use std::{fmt, num::ParseIntError, result};

use luoxide_text::range::TextSpan;
#[cfg(feature = "serde")]
use serde::Serialize;
use thiserror::Error;

use crate::token::TokenKind;

pub type Result<T> = result::Result<T, ParseError>;

#[derive(Debug, Error)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[non_exhaustive]
pub enum ParseErrorKind {
    /// A token the lexer already flagged as invalid.
    #[error("the lexer encountered an error")]
    Lexer,
    #[error("source file ended unexpectedly")]
    UnexpectedEof,
    #[error("found unexpected token")]
    UnexpectedToken {
        expected: Box<[TokenKind]>,
        found: TokenKind,
    },
    #[error("conversion of number returned an error")]
    InvalidNumber {
        #[from]
        #[cfg_attr(feature = "serde", serde(skip_serializing))]
        inner_error: ParseIntError,
    },
    #[error("number literal is malformed")]
    MalformedNumber,
    #[error("string literal contains an invalid escape sequence")]
    InvalidEscape,
    #[error("expressions are nested too deeply")]
    NestingTooDeep,
    #[error("expression cannot be used as a statement")]
    NonCallExpressionStatement,
    #[error("expression cannot be assigned to")]
    InvalidAssignmentTarget,
    #[error("usage of a reserved keyword")]
    ReservedKeyword,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub at: Option<TextSpan>,
    /// Parser call site that constructed this error. Present for
    /// `unexpected_token` when the `debug` feature is enabled.
    #[cfg(feature = "debug")]
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub reported_at: Option<&'static Location<'static>>,
    #[cfg(feature = "debug")]
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub backtrace: Option<backtrace::Backtrace>,
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("ParseError");
        debug.field("kind", &self.kind).field("at", &self.at);
        #[cfg(feature = "debug")]
        {
            debug.field("reported_at", &self.reported_at.map(format_reported_at));
            if let Some(backtrace) = &self.backtrace {
                debug.field("parser_stack", &parser_stack_frames(backtrace));
            }
        }
        debug.finish()
    }
}

impl ParseError {
    pub(crate) fn new(kind: ParseErrorKind, at: Option<TextSpan>) -> Self {
        Self {
            kind,
            at,
            #[cfg(feature = "debug")]
            reported_at: None,
            #[cfg(feature = "debug")]
            backtrace: None,
        }
    }

    #[track_caller]
    pub(crate) fn capturing(kind: ParseErrorKind, at: Option<TextSpan>) -> Self {
        Self {
            kind,
            at,
            #[cfg(feature = "debug")]
            reported_at: Some(Location::caller()),
            #[cfg(feature = "debug")]
            backtrace: Some(backtrace::Backtrace::new()),
        }
    }

    pub fn is_nesting_too_deep(&self) -> bool {
        matches!(self.kind, ParseErrorKind::NestingTooDeep)
    }

    pub fn details(&self) -> (&'static str, Vec<String>) {
        match &self.kind {
            ParseErrorKind::Lexer => ("The lexer encountered an error", vec![]),
            ParseErrorKind::UnexpectedEof => ("The source file ended unexpectedly", vec![]),
            ParseErrorKind::UnexpectedToken { expected, found } => {
                let mut messages = vec![format!("found {}", found.describe())];
                if expected.len() == 1 {
                    messages.push(format!("expected {}", expected[0].describe()));
                } else {
                    messages.push("expected one of:".to_string());
                    messages.extend(expected.iter().map(|kind| format!("- {}", kind.describe())));
                }
                ("unexpected token", messages)
            }
            ParseErrorKind::InvalidNumber { inner_error } => (
                match inner_error.kind() {
                    std::num::IntErrorKind::InvalidDigit => "Number is invalid",
                    std::num::IntErrorKind::PosOverflow => "Number is too big",
                    std::num::IntErrorKind::NegOverflow => "Number is too small",
                    // Zero/Empty cannot be produced by the parser's int
                    // paths; report generically instead of panicking.
                    _ => "Number could not be parsed",
                },
                vec![],
            ),
            ParseErrorKind::MalformedNumber => ("Number literal is malformed", vec![]),
            ParseErrorKind::InvalidEscape => {
                ("String literal contains an invalid escape sequence", vec![])
            }
            ParseErrorKind::NestingTooDeep => (
                "Expressions or blocks are nested too deeply",
                vec!["Reduce the nesting depth of the code".to_string()],
            ),
            ParseErrorKind::NonCallExpressionStatement => (
                "Only function calls can be used as statements",
                vec!["Assign the value or call a function instead".to_string()],
            ),
            ParseErrorKind::InvalidAssignmentTarget => (
                "This expression cannot be assigned to",
                vec!["Only names, fields (a.b) and indexes (a[b]) are assignable".to_string()],
            ),
            ParseErrorKind::ReservedKeyword => (
                "Found a reserved keyword",
                std::iter::once("Found a reserved keyword, reserved keywords are: ".to_string())
                    .chain(token!(reserved_set).iter().map(|s| format!("- {s}")))
                    .collect(),
            ),
        }
    }
}

#[cfg(feature = "debug")]
fn format_reported_at(location: &Location<'_>) -> String {
    format_source_location(location.file(), location.line(), Some(location.column()))
}

/// `crates/luoxide-parser/src/parser/common.rs:25:34`
#[cfg(feature = "debug")]
fn format_source_location(file: &str, line: u32, column: Option<u32>) -> String {
    let file = workspace_relative_path(file);
    match column {
        Some(column) => format!("{file}:{line}:{column}"),
        None => format!("{file}:{line}"),
    }
}

#[cfg(feature = "debug")]
fn workspace_relative_path(file: &str) -> String {
    let file = file.replace('\\', "/");
    let manifest = env!("CARGO_MANIFEST_DIR").replace('\\', "/");
    if let Some(relative) = file.strip_prefix(&format!("{manifest}/")) {
        return format!("crates/luoxide-parser/{relative}");
    }
    if let Some(crates) = file.find("crates/") {
        return file[crates..].to_string();
    }
    if file.starts_with("src/") {
        return format!("crates/luoxide-parser/{file}");
    }
    file
}

/// Parser frames from a captured backtrace, skipping the error constructors.
#[cfg(feature = "debug")]
fn parser_stack_frames(backtrace: &backtrace::Backtrace) -> Vec<String> {
    let mut frames = Vec::new();
    for frame in backtrace.frames() {
        for symbol in frame.symbols() {
            let name = match symbol.name() {
                Some(name) => name.to_string(),
                None => continue,
            };
            if !name.contains("luoxide_parser") {
                continue;
            }
            if name.contains("unexpected_token")
                || name.contains("ParseError::capturing")
                || name.contains("ParseError::new")
            {
                continue;
            }
            let Some(file) = symbol.filename() else {
                continue;
            };
            let Some(line) = symbol.lineno() else {
                continue;
            };
            frames.push(format_source_location(
                &file.display().to_string(),
                line,
                symbol.colno(),
            ));
        }
    }
    frames
}
