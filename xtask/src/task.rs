pub mod ci;
pub mod common;
pub mod examples;
pub mod lua_suite;
pub mod snap;
pub mod test;

use crate::Result;

const HELP: &str = "
Usage:
    xtask <task> <args>

Tasks:
  ci        : tests, snapshots, and examples with INSTA_UPDATE=new
  examples  : run all examples
  lua-suite [ver] : parse official Lua testes (default 5.5.1; 5.5.0, 5.4.8)
                    dumps under target/lua-suite/; catalog snap in-tree
  snap      : review snapshot failures (insta --review)
  test      : run tests, snapshots, and examples
";

pub fn print_help() -> Result<()> {
    eprintln!("{HELP}");
    Ok(())
}

pub fn run(which: &str, args: &[String]) -> Result<()> {
    match which {
        "ci" => ci::run(args),
        "examples" => examples::run(args),
        "lua-suite" => lua_suite::run(args),
        "snap" => snap::run(args),
        "test" => test::run(args),
        _ => print_help(),
    }
}
