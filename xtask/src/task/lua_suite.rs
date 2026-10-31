use std::path::{Path, PathBuf};

use luoxide::lua_suite::{self, Config, DEFAULT_VERSION, PINS};

use super::common::project_root;
use crate::Result;

pub fn run(args: &[String]) -> Result<()> {
    let root = project_root();
    let options = parse_args(args, &root)?;
    let config = Config {
        version: options.version,
        workspace_root: root.clone(),
        artifact_root: options.output_dir,
        verbose: true,
    };

    let outcome = lua_suite::run(&config).map_err(|error| error.to_string())?;
    if !outcome.is_success() {
        return Err(format!(
            "lua suite: {} partial, {} failed",
            outcome.partial, outcome.failed
        )
        .into());
    }

    let snap_name = format!("lua-{}", outcome.version);
    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(root.join("crates/luoxide-parser/tests/lua-suite"));
    settings.set_prepend_module_to_snapshot(false);
    settings.bind(|| {
        insta::assert_snapshot!(snap_name, outcome.catalog);
    });

    Ok(())
}

struct Options {
    version: String,
    output_dir: PathBuf,
}

fn parse_args(args: &[String], root: &Path) -> Result<Options> {
    let mut version = None;
    let mut output_dir = None;
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == "--output-dir" {
            let Some(path) = iter.next() else {
                return Err("--output-dir requires a path".into());
            };
            output_dir = Some(PathBuf::from(path));
        } else if let Some(path) = arg.strip_prefix("--output-dir=") {
            if path.is_empty() {
                return Err("--output-dir requires a path".into());
            }
            output_dir = Some(PathBuf::from(path));
        } else if arg.starts_with('-') {
            return Err(format!("unknown flag {arg}").into());
        } else if version.is_some() {
            return Err(usage().into());
        } else {
            version = Some(arg.clone());
        }
    }

    Ok(Options {
        version: version.unwrap_or_else(|| DEFAULT_VERSION.to_owned()),
        output_dir: resolve_output_dir(root, output_dir),
    })
}

fn resolve_output_dir(root: &Path, output_dir: Option<PathBuf>) -> PathBuf {
    match output_dir {
        Some(path) if path.is_absolute() => path,
        Some(path) => root.join(path),
        None => PathBuf::from("target/lua-suite"),
    }
}

fn usage() -> String {
    format!(
        "usage: cargo xtask lua-suite [VERSION] [--output-dir DIR]\n\
         \n\
         VERSION defaults to {DEFAULT_VERSION}. Known: {}",
        PINS.iter()
            .map(|(version, _)| *version)
            .collect::<Vec<_>>()
            .join(", ")
    )
}
