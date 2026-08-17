use super::common::{cargo, CheckStatus};
use crate::Result;

/// Installed snapshot tests (run by `xtask snap` / the CI snapshot step).
///
/// `cargo test -- --skip NAME` matches any test whose name contains `NAME`.
const SKIP_INSTA: &[&str] = &[
    "--skip",
    "parse_snapshots",
    "--skip",
    "expression_suffixed_call",
    "--skip",
    "lex_design_file",
    "--skip",
    "lex_string_file",
];

pub fn run(args: &[String]) -> Result<()> {
    cargo("test")
        .args(["--all-targets", "--all-features"])
        .args(args.iter())
        .arg("--")
        .args(SKIP_INSTA)
        .spawn()?
        .wait()?
        .check()?;

    super::examples::run(args)?;

    Ok(())
}
