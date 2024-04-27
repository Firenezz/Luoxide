use super::common::{cargo, CheckStatus};
use crate::Result;

pub fn run(args: &[String]) -> Result<()> {
    cargo("test")
        .args(["--all-targets", "--all-features"])
        .args(args.iter())
        .spawn()?
        .wait()?
        .check()?;

    super::examples::run(args)?;

    Ok(())
}
