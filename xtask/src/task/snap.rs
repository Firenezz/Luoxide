use std::ffi::OsStr;
use std::fs;

use super::common::{cargo, project_root, CheckStatus};
use crate::Result;

pub fn run(args: &[String]) -> Result<()> {
    cargo("insta")
        .args([
            "test",
            "--all-features",
            "--review",
            "--delete-unreferenced-snapshots",
            "--no-ignore",
            "--",
        ])
        .args(args.iter())
        .spawn()?
        .wait()?
        .check()
}
