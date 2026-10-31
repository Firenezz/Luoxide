//! Parse regression over the official Lua test suite (catalog snapshot in-tree).

use std::path::PathBuf;

use luoxide::lua_suite::{self, Config, DEFAULT_VERSION};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn official_lua_testes() {
    let root = workspace_root();
    let config = Config {
        version: DEFAULT_VERSION.to_owned(),
        workspace_root: root.clone(),
        artifact_root: PathBuf::from("target/lua-suite"),
        verbose: false,
    };

    let outcome = lua_suite::run(&config).unwrap_or_else(|error| {
        panic!("lua suite run failed: {error}");
    });

    assert!(
        outcome.is_success(),
        "lua suite had {} partial and {} failed parses",
        outcome.partial,
        outcome.failed
    );

    let snap_name = format!("lua-{}", outcome.version);
    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(root.join("crates/luoxide-parser/tests/lua-suite"));
    settings.set_prepend_module_to_snapshot(false);
    settings.bind(|| {
        insta::assert_snapshot!(snap_name, outcome.catalog.as_str());
    });
}
