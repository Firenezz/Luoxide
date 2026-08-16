//! Snapshot tests: every `tests/inputs/*.lua` file is parsed as a chunk and
//! the reconstructed Lua (what the parser understood) is snapshotted, plus
//! compact diagnostics on recovered errors.

use std::fmt::Write;

use insta::assert_snapshot;
use luoxide_parser::ast::DisplayLua;
use luoxide_parser::error::{ErrorKind, ParseError, ParseErrorKind};
use luoxide_parser::outcome::Outcome;
use luoxide_parser::parser::{compile_chunk, compile_expression};

/// Expression-level snapshot for the focused case `a.b.c()`.
#[test]
fn expression_suffixed_call() {
    insta::assert_debug_snapshot!(compile_expression("a.b.c()"));
}

#[test]
fn parse_snapshots() {
    insta::glob!("inputs/*.lua", |path| {
        let source = std::fs::read_to_string(path).expect("read test input");
        let outcome = compile_chunk(&source);
        assert_snapshot!(snapshot_parse(&source, &outcome));
    });
}

/// Human review artifact: Lua the tree round-trips to, then any errors.
fn snapshot_parse(source: &str, outcome: &Outcome<luoxide_parser::ast::Chunk, Vec<ParseError>>) -> String {
    let mut out = String::new();
    match outcome {
        Outcome::Ok(chunk) => {
            writeln!(&mut out, "ok").unwrap();
            write!(&mut out, "{}", DisplayLua::with_source(chunk, source)).unwrap();
        }
        Outcome::PartialFailure(chunk, errors) => {
            writeln!(&mut out, "partial").unwrap();
            write!(&mut out, "{}", DisplayLua::with_source(chunk, source)).unwrap();
            write_errors(&mut out, errors, source);
        }
        Outcome::TotalFailure(errors) => {
            writeln!(&mut out, "total failure").unwrap();
            write_errors(&mut out, errors, source);
        }
    }
    out
}

fn write_errors(out: &mut String, errors: &[ParseError], source: &str) {
    writeln!(out, "\n--- errors ({}) ---", errors.len()).unwrap();
    for (i, error) in errors.iter().enumerate() {
        let (title, notes) = error.details();
        write!(out, "[{i}] {title}").unwrap();
        if let Some(span) = error.at {
            let range = span.start.to_usize()..span.end.to_usize();
            let lexeme = source.get(range).unwrap_or("");
            write!(out, " at {span} {lexeme:?}").unwrap();
        }
        writeln!(out).unwrap();
        for note in notes {
            writeln!(out, "    {note}").unwrap();
        }
    }
}

/// Deeply nested input must produce a `NestingTooDeep` error, not a stack
/// overflow or an unbounded recovery loop.
#[test]
fn deep_nesting_is_an_error_not_a_crash() {
    let source = format!("{}x{}", "(".repeat(1000), ")".repeat(1000));
    let outcome = compile_expression(&source);
    let errors = expect_errors(outcome);
    assert_nesting_reported_once(&errors);
}

/// Same guard for statements (nested `do ... end`). Removing `do` from the
/// statement sync set hid the hang for this input; nested `if` still used to
/// retry the same opener until the process ran out of memory.
#[test]
fn deep_block_nesting_is_an_error_not_a_crash() {
    let source = format!("{}{}", "do ".repeat(1000), "end ".repeat(1000));
    let outcome = compile_chunk(&source);
    let errors = expect_errors(outcome);
    assert_nesting_reported_once(&errors);
}

#[test]
fn deep_if_nesting_does_not_duplicate_errors() {
    let source = format!(
        "{}{}",
        "if true then ".repeat(1000),
        "end ".repeat(1000)
    );
    let outcome = compile_chunk(&source);
    let errors = expect_errors(outcome);
    assert_nesting_reported_once(&errors);
}

fn expect_errors<T: std::fmt::Debug>(outcome: Outcome<T, Vec<luoxide_parser::error::ParseError>>) -> Vec<luoxide_parser::error::ParseError> {
    match outcome {
        Outcome::Ok(value) => panic!("expected a nesting error, got Ok({value:?})"),
        Outcome::PartialFailure(_, errors) | Outcome::TotalFailure(errors) => errors,
    }
}

fn assert_nesting_reported_once(errors: &[luoxide_parser::error::ParseError]) {
    let nesting = errors
        .iter()
        .filter(|error| {
            matches!(
                error.error,
                ErrorKind::ParserError {
                    error_kind: ParseErrorKind::NestingTooDeep
                }
            )
        })
        .count();
    assert_eq!(
        nesting, 1,
        "expected exactly one NestingTooDeep, got {} in {errors:?}",
        nesting
    );
    assert!(
        errors.len() < 32,
        "recovery produced too many diagnostics ({})",
        errors.len()
    );
}

/// Malformed input never panics and always yields a tree.
#[test]
fn recovery_always_yields_a_tree() {
    let sources = [
        "",
        ";;;",
        "local",
        "f(",
        "a.b.",
        "a[",
        "return return",
        "end",
        "= = =",
        "\"unterminated",
        "0x",
        "local x = 99999999999999999999999999",
    ];

    for source in sources {
        // A tree (possibly with Error nodes) must come back for every input.
        match compile_chunk(source) {
            Outcome::Ok(..) | Outcome::PartialFailure(..) => {}
            Outcome::TotalFailure(errors) => {
                panic!("no tree produced for {source:?}: {errors:?}")
            }
        }
    }
}
