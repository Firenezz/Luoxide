{ pkgs, ... }:
{
  name = "luoxidant";

  # Pins rustc/cargo/clippy/rustfmt/rust-analyzer from this file (stable, edition 2024).
  languages.rust = {
    enable = true;
    toolchainFile = ./rust-toolchain.toml;
  };

  packages = [
    pkgs.git
    pkgs.cargo-insta # `cargo xtask snap` → `cargo insta test --review`
    pkgs.lldb # CodeLLDB / native debugging
  ];

  env = {
    CARGO_TERM_COLOR = "always";
    RUST_BACKTRACE = "1";
  };

  # `devenv test` runs enterTest (not scripts.test).
  enterTest = ''
    cargo xtask test
  '';

  git-hooks.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };

  # xtask: test, ci (INSTA_UPDATE=new), snap, examples
  scripts.ci = {
    description = "Workspace tests + examples with INSTA_UPDATE=new (CI)";
    exec = ''cargo --locked xtask ci "$@"'';
  };
  scripts.snap = {
    description = "Review insta snapshot failures";
    exec = ''cargo --locked xtask snap "$@"'';
  };
  scripts.examples = {
    description = "Run all crate examples (parser dump, lexdebug, …)";
    exec = ''cargo --locked xtask examples "$@"'';
  };

  # luoxide-cli (bin/luoxide-cli): parse | repl
  scripts.parse = {
    description = "Parse Lua (chunk or --expr); reconstructed source on stdout";
    exec = ''cargo run -p luoxide-cli -- parse "$@"'';
  };
  scripts.repl = {
    description = "Line-oriented parse REPL";
    exec = ''cargo run -p luoxide-cli -- repl "$@"'';
  };


  # cargo tasks
  scripts.docs = {
    description = "Generate crate documentation";
    exec = ''cargo --locked doc --all-features "$@"'';
  };
  scripts.clean = {
    description = "Clean all targets";
    exec = ''cargo clean "$@"'';
  };
  scripts.fmt = {
    description = "cargo fmt --all";
    exec = ''cargo fmt --all "$@"'';
  };
  scripts.clippy = {
    description = "Workspace clippy (CI: all-targets, -D warnings)";
    exec = ''cargo clippy --all-targets --all-features "$@" -- -D warnings'';
  };
}
