use core::error;

use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFiles,
};
use luoxide_parser::parser::compile_expression;
use tracing::instrument::WithSubscriber;

#[allow(unused_must_use)]
fn main() {
    tracing_subscriber::fmt::init();

    let mut files = SimpleFiles::new();

    let fileid = files.add("<string>", "x = 5 + 8 * ");

    let result = compile_expression(files.get(file_id).unwrap().source().chars().as_str());
    match result {
        Ok(ast) => dbg!(ast),
        Err(errs) => into_diagnostics(errs, fileid),
    }
}

fn into_diagnostics(errors: Vec<ParseError>, file_id: usize) {
    for error in errors {
        Diagnostic::error()
            .with_message(format!("{}", error))
            .with_labels(vec![
                Label::primary(file_id, error.)
            ])
    }
}
