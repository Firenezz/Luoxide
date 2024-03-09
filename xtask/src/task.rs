
pub mod examples;
pub mod common;
pub mod test;
pub mod snap;

use crate::Result;

const HELP: &str = "
Usage:
  xtask <task> <args>

Tasks:
  examples : run all examples
  snap     : run snapshot tests in review mode
  test     : run tests and examples
";

pub fn print_help() -> Result<()> {
    eprintln!("{HELP}");
    Ok(())
}

pub fn run(which: &str, args: &[String]) -> Result<()> {
    match which {
        "examples" => examples::run(args),
        "snap" => snap::run(args),
        "test" => test::run(args),
        _ => print_help(),
    }
}