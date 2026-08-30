# Luoxidant

A Lua **frontend** in Rust: lexer, parser, compact AST, and session-scoped string interning.

It parses Lua into a tree you can print, snapshot-test, and (later) lower. It does **not** emit bytecode or run a VM yet.

```text
source  →  lexer  →  parser  →  AST  (+ interned names)
              12-byte tokens     spans on every node
```

## Status

| Implemented | Not yet |
| --- | --- |
| Logos lexer | Codegen / Lua bytecode |
| Recursive-descent parser with error recovery | VM / execution |
| AST with interned identifiers (`Atom` / `Name`) | **Official Lua test suite** (planned; coverage today is `crates/luoxide-parser/tests/inputs/`) |
| `global`, attributes (`<const>`), named varargs | |
| CLI `parse` / `repl`, insta snapshots | |
| Clippy-strict workspace (no `unwrap` / `panic` in lib code) | |

Lua 5.5-oriented syntax is the goal. Until the [official Lua test suite](https://www.lua.org/tests/) is wired in, do not claim 100% grammar coverage.

## Developer environment

[devenv](https://devenv.sh) pins a stable Rust toolchain (`rust-toolchain.toml`: rustc, cargo, clippy, rustfmt, rust-analyzer).

```bash
# once: https://devenv.sh/getting-started/
devenv allow    # if you use devenv's shell hook
devenv shell
```

Inside the shell, custom scripts (`parse`, `repl`, `ci`, …) are on `PATH`. They are not `devenv` subcommands. `devenv test` is the exception: it runs `enterTest`.

```bash
devenv test
parse lua_scripts/parser/simple.lua
repl --debug
```

From outside the shell: `devenv shell -- parse lua_scripts/parser/simple.lua`.

With [direnv](https://direnv.net/), `direnv allow` in this directory loads the same env via `.envrc`.

Without devenv, any recent stable Rust (edition 2024, e.g. 1.85+) is enough:

```bash
cargo test --workspace
cargo xtask test
cargo xtask ci
```

## Parse a file

```bash
cargo run -p luoxide-cli -- parse lua_scripts/parser/simple.lua
cargo run -p luoxide-cli -- parse --debug path/to/script.lua
echo 'return 1 + 2' | cargo run -p luoxide-cli -- parse --expr -
```

Successful parses print reconstructed Lua. Failures still print a partial tree when recovery produced one, plus diagnostics.

## Crates

| Crate | Role |
| --- | --- |
| `luoxide` | Compile session (`Session` owns the intern table) |
| `luoxide-parser` | Lexer, parser, AST |
| `luoxide-text` | Source spans + string intern (`ahash` bucket backend) |
| `luoxide-cli` | `parse` and `repl` |
| `luoxide-bench` | Criterion benches |
| `xtask` | `cargo xtask {test,ci,snap,examples}` |

## License

MIT OR Apache-2.0. See `LICENSE-MIT` and `LICENSE-APACHE`.
