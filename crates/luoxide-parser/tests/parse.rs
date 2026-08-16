//! Snapshot tests: every `tests/inputs/*.lua` file is parsed as a chunk and
//! the resulting tree (plus any recovered errors) is snapshotted.

use luoxide_parser::error::{ErrorKind, ParseErrorKind};
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
        insta::assert_debug_snapshot!(outcome);
    });
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
