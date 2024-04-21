pub mod clippy;
pub mod common;
pub mod examples;
pub mod snap;
pub mod test;

use crate::Result;

const HELP: &str = "
Usage:
  xtask <task> <args>

Tasks:
  examples : run all examples
  snap     : run snapshot tests in review mode
  test     : run tests and examples
  clippy   : run clippy
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
        "clippy" => clippy::run(args),
        _ => print_help(),
    }
}
