use std::process::ExitCode;

use clap::Command;

mod commands;

fn main() -> ExitCode {
    init_tracing();
    match run() {
        Ok(code) => code,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<ExitCode, commands::Error> {
    match cli().try_get_matches() {
        Ok(matches) => commands::dispatch(&matches),
        Err(error) => {
            error.print()?;
            let code = u8::try_from(error.exit_code()).unwrap_or(2);
            Ok(ExitCode::from(code))
        }
    }
}

fn cli() -> Command {
    Command::new("luoxide-cli")
        .about("Parse and inspect Lua with Luoxide")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(commands::parse_command())
        .subcommand(commands::repl_command())
}

fn init_tracing() {
    let Ok(filter) = tracing_subscriber::EnvFilter::try_from_default_env() else {
        return;
    };
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .compact()
        .try_init();
}
