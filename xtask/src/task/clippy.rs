use super::common::{cargo, CheckStatus};
use crate::Result;

pub fn run(args: &[String]) -> Result<()> {
    cargo("clippy")
        .args(["--all-targets", "--all-features"])
        .args(["--", "-D", "warnings"])
        .args(args.iter())
        .spawn()?
        .wait()?
        .check()?;

    Ok(())
}
