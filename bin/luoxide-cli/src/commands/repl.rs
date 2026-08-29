use std::io::{self, BufRead, Write};
use std::process::ExitCode;

use clap::{Arg, ArgAction, ArgMatches, Command};
use luoxide::prelude::*;

use super::{Error, render, write_output};

pub(super) fn command() -> Command {
    Command::new("repl").about("Start a parse REPL").arg(
        Arg::new("debug")
            .long("debug")
            .action(ArgAction::SetTrue)
            .help("Print the debug AST instead of reconstructed Lua"),
    )
}

pub(super) fn run(args: &ArgMatches) -> Result<ExitCode, Error> {
    let debug = args.get_flag("debug");
    let mut session = Session::new();
    let mut line = String::new();

    eprintln!("luoxide-cli repl — empty line or Ctrl-D to exit");

    loop {
        line.clear();
        eprint!("> ");
        io::stderr().flush()?;
        let n = io::stdin().lock().read_line(&mut line)?;
        if n == 0 {
            eprintln!();
            break;
        }
        if line.trim().is_empty() {
            break;
        }

        let outcome = session.parse_chunk(&line);
        let (text, _) = render(&session, "<repl>", &line, debug, outcome);
        if let Some(text) = text {
            write_output(None, &text)?;
        }
    }

    Ok(ExitCode::SUCCESS)
}
