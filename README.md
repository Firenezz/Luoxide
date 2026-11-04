# Luoxidant

A Lua frontend in Rust: lexer, parser, compact AST, and session-scoped string interning.

```text
source  →  lexer  →  parser  →  AST  (+ interned names)
              12-byte tokens     spans on every node
```

No bytecode emission or VM yet. Day-to-day usage is in [`quickstart.md`](quickstart.md).

## Status

Target grammar: Lua 5.5-oriented.

| Layer | State |
| --- | --- |
| Lexer (Logos) | Done |
| Recursive-descent parser (error recovery) | Done |
| AST + interned identifiers (`Atom` / `Name`) | Done |
| Lua 5.5 syntax (`global`, `<const>`, named varargs, …) | Done |
| CLI (`parse`, `repl`) and insta snapshots | Done |
| [Official Lua testes](https://www.lua.org/tests/) | Parsed (`cargo xtask lua-suite`; default 5.5.1). Not executed — no runtime yet. |
| Codegen / bytecode / VM | In design |

In-tree fixtures live under `crates/luoxide-parser/tests/inputs/`. The suite catalog snapshot is `crates/luoxide-parser/tests/lua-suite/`.

## Build

[devenv](https://devenv.sh) pins the toolchain (`rust-toolchain.toml`). Inside the shell, `parse`, `repl`, and `ci` are on `PATH`.

```bash
devenv shell
devenv test
parse lua_scripts/parser/simple.lua
```

Without devenv, use a recent stable Rust (edition 2024):

```bash
cargo test --workspace
cargo xtask test
cargo xtask ci
cargo xtask lua-suite          # download + parse official testes
```

## Parse

```bash
cargo run -p luoxide-cli -- parse lua_scripts/parser/simple.lua
cargo run -p luoxide-cli -- parse --debug path/to/script.lua
echo 'return 1 + 2' | cargo run -p luoxide-cli -- parse --expr -
```

Success prints reconstructed source. Recovered failures still print a partial tree plus diagnostics.

## Crates

| Path | Role |
| --- | --- |
| `crates/luoxide` | Compile session (`Session` owns the intern table) |
| `crates/luoxide-parser` | Lexer, parser, AST |
| `crates/luoxide-text` | Source spans + string intern |
| `crates/luoxide-macros` | Internal macros |
| `bin/luoxide-cli` | `parse` and `repl` |
| `crates/luoxide-bench` | Criterion benches |
| `xtask` | `cargo xtask {test,ci,snap,examples,lua-suite}` |

## License

MIT OR Apache-2.0. See `LICENSE-MIT` and `LICENSE-APACHE`.
