use core::error;

use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFiles,
};
use luoxide_parser::{error::ParseError, parser::compile_expression};
use tracing::instrument::WithSubscriber;

#[allow(unused_must_use)]
fn main() {
    tracing_subscriber::fmt::init();

    let mut files = SimpleFiles::new();

    let file_id = files.add("<string>", "a.b.c() + 5");

    let result = compile_expression(files.get(file_id).unwrap().source().chars().as_str());
    todo!();
    /*let diagnostics = match result {
        Ok(ast) => dbg!(ast),
        Err(errs) => into_diagnostics(errs, fileid),
    }*/
}

fn into_diagnostics(errors: Vec<ParseError>, file_id: usize) -> Vec<Diagnostic<usize>> {
    //let file = files.get(file_id).unwrap();
    for error in errors {
        Diagnostic::<usize>::error()
            .with_message(format!("{}", "error"))
            .with_labels(vec![]);
    }
    todo!()
}
