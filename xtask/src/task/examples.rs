use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use super::common::{cargo, project_root, CheckStatus};
use crate::Result;

pub fn run(args: &[String]) -> Result<()> {
    let mut examples = discover_examples()?;
    examples.sort();

    if examples.is_empty() {
        return Err("no crate examples found".into());
    }

    for (package, example) in examples {
        cargo("run")
            .args(["-p", &package, "--example", &example, "--all-features"])
            .args(args.iter())
            .spawn()?
            .wait()?
            .check()?;
    }

    Ok(())
}

/// `(package, example name)` for each `examples/*.rs` under a workspace crate.
///
/// The workspace is virtual, so the top-level `examples/` directory is not a
/// Cargo example target. Those files are ignored here.
fn discover_examples() -> Result<Vec<(String, String)>> {
    let mut found = Vec::new();
    let root = project_root();

    for dir in crate_dirs(&root)? {
        let examples_dir = dir.join("examples");
        if !examples_dir.is_dir() {
            continue;
        }
        let package = package_name(&dir)?;
        for example in fs::read_dir(&examples_dir)? {
            let example = example?;
            let path = example.path();
            if !example.metadata()?.is_file() || path.extension() != Some(OsStr::new("rs")) {
                continue;
            }
            let name = path
                .file_stem()
                .and_then(OsStr::to_str)
                .ok_or_else(|| format!("invalid example path {}", path.display()))?;
            found.push((package.clone(), name.to_owned()));
        }
    }

    Ok(found)
}

fn crate_dirs(root: &Path) -> Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();
    for group in ["crates", "bin"] {
        let group_dir = root.join(group);
        if !group_dir.is_dir() {
            continue;
        }
        for entry in fs::read_dir(group_dir)? {
            let path = entry?.path();
            if path.is_dir() && path.join("Cargo.toml").is_file() {
                dirs.push(path);
            }
        }
    }
    Ok(dirs)
}

fn package_name(crate_dir: &Path) -> Result<String> {
    let manifest = fs::read_to_string(crate_dir.join("Cargo.toml"))?;
    for line in manifest.lines() {
        let line = line.trim();
        if let Some(name) = line
            .strip_prefix("name")
            .and_then(|rest| rest.trim().strip_prefix('='))
            .map(str::trim)
        {
            let name = name.trim_matches('"');
            if !name.is_empty() {
                return Ok(name.to_owned());
            }
        }
    }
    Err(format!("no package name in {}", crate_dir.display()).into())
}
