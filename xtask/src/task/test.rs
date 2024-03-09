use std::ffi::OsStr;
use std::fs;

use super::common::{cargo, project_root, CheckStatus};
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