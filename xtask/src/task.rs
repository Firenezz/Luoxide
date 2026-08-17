pub mod ci;
pub mod common;
pub mod examples;
pub mod snap;
pub mod test;

use crate::Result;

const HELP: &str = "
Usage:
    xtask <task> <args>

Tasks:
  ci       : tests, snapshots, and examples with INSTA_UPDATE=new
  examples : run all examples
  snap     : review snapshot failures (insta --review)
  test     : run tests, snapshots, and examples
";

pub fn print_help() -> Result<()> {
    eprintln!("{HELP}");
    Ok(())
}

pub fn run(which: &str, args: &[String]) -> Result<()> {
    match which {
        "ci" => ci::run(args),
        "examples" => examples::run(args),
        "snap" => snap::run(args),
        "test" => test::run(args),
        _ => print_help(),
    }
}
