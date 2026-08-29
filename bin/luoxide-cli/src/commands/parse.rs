use std::fs;
use std::io::{self, Read};
use std::process::ExitCode;

use clap::{Arg, ArgAction, ArgMatches, Command};
use luoxide::prelude::*;

use super::{Error, render, write_output};

pub(super) fn command() -> Command {
    Command::new("parse")
        .about("Parse a Lua script")
        .arg(
            Arg::new("script")
                .required(true)
                .value_name("FILE")
                .help("Lua source file. Use - to read stdin."),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Write the parse result to FILE instead of stdout"),
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .action(ArgAction::SetTrue)
                .help("Print the debug AST instead of reconstructed Lua"),
        )
        .arg(
            Arg::new("expr")
                .long("expr")
                .action(ArgAction::SetTrue)
                .help("Parse a single expression instead of a chunk"),
        )
}

pub(super) fn run(args: &ArgMatches) -> Result<ExitCode, Error> {
    let script = args
        .get_one::<String>("script")
        .ok_or(Error::MissingArg("script"))?;
    let output = args.get_one::<String>("output").map(String::as_str);
    let debug = args.get_flag("debug");
    let expression = args.get_flag("expr");

    let source = read_source(script)?;
    let name = if script == "-" {
        "<stdin>"
    } else {
        script.as_str()
    };

    let mut session = Session::new();
    let (text, failed) = if expression {
        let outcome = session.parse_expression(&source);
        render(&session, name, &source, debug, outcome)
    } else {
        let outcome = session.parse_chunk(&source);
        render(&session, name, &source, debug, outcome)
    };

    if let Some(text) = text {
        write_output(output, &text)?;
    }

    Ok(if failed {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    })
}

fn read_source(path: &str) -> Result<String, Error> {
    if path == "-" {
        let mut source = String::new();
        io::stdin().read_to_string(&mut source)?;
        return Ok(source);
    }
    fs::read_to_string(path).map_err(|source| Error::Read {
        path: path.to_owned(),
        source,
    })
}
