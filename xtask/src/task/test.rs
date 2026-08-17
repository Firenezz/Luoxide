use super::common::{cargo, CheckStatus};
use crate::Result;

pub fn run(args: &[String]) -> Result<()> {
    // `--all-targets` also builds benches; those use Criterion and are not
    // part of the default test pass.
    cargo("test")
        .args(["--lib", "--bins", "--tests", "--examples", "--all-features"])
        .args(args.iter())
        .spawn()?
        .wait()?
        .check()?;

    super::examples::run(args)?;

    Ok(())
}
