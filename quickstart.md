# Quickstart

Luoxidant is a Lua **frontend** (lexer + parser + AST). It does not run Lua or emit bytecode yet.

## Setup

**Recommended:** [devenv](https://devenv.sh) + this repo's `rust-toolchain.toml`.

```bash
devenv allow    # first time, if the shell hook asks
devenv shell
```

With [direnv](https://direnv.net/), `direnv allow` in the repo root loads the same env via `.envrc`.

**Without devenv:** any recent stable Rust (edition 2024, e.g. 1.85+) is enough.

```bash
cargo test --workspace
```

## CLI: `luoxide-cli`

The binary lives in `bin/luoxide-cli`. Subcommands: `parse` and `repl`.

```bash
cargo run -p luoxide-cli -- --help
cargo run -p luoxide-cli -- parse --help
cargo run -p luoxide-cli -- repl --help
```

Inside `devenv shell`, `parse` is on `PATH` (same as `cargo run -p luoxide-cli -- parse …`).

### Parse a file

Successful parses print reconstructed Lua on stdout. Failures still print a partial tree when recovery produced one, plus diagnostics on stderr (exit code `1`).

```bash
# chunk (whole script)
cargo run -p luoxide-cli -- parse lua_scripts/parser/simple.lua

# same via devenv (inside the shell; scripts are PATH commands)
parse lua_scripts/parser/simple.lua
# from outside: devenv shell -- parse lua_scripts/parser/simple.lua

# debug AST instead of reconstructed Lua
cargo run -p luoxide-cli -- parse --debug lua_scripts/parser/simple.lua

# write result to a file
cargo run -p luoxide-cli -- parse -o /tmp/out.lua lua_scripts/parser/simple.lua
```

More sample sources: `crates/luoxide-parser/tests/inputs/` (`statements.lua`, `tables.lua`, `functions.lua`, `errors.lua`, …).

Those sources are only for testing parser and syntax isn't being processed yet.

### Parse stdin or a single expression

```bash
echo 'return 1 + 2' | cargo run -p luoxide-cli -- parse -
echo '1 + 2' | cargo run -p luoxide-cli -- parse --expr -
```

`-` means stdin. `--expr` parses one expression instead of a chunk.

### REPL

Line-oriented parse loop (each line is a **chunk**). Empty line or Ctrl-D exits.

```bash
cargo run -p luoxide-cli -- repl
cargo run -p luoxide-cli -- repl --debug
```

```text
luoxide-cli repl — empty line or Ctrl-D to exit
> return 1 + 2
return 1 + 2
>
```

### Parser tracing (`RUST_LOG`)

The CLI installs a compact `tracing` subscriber when `RUST_LOG` is set. Two independent targets (see `crates/luoxide-parser/docs/tracing.md`):

| Target | Level | What you see |
| --- | --- | --- |
| `luoxide_parser::parse::shallow` | `DEBUG` | production enter/leave, mismatch, recover, sync |
| `luoxide_parser::parse::deep` | `TRACE` | every consumed token |

```bash
RUST_LOG=luoxide_parser::parse::shallow=debug \
  cargo run -p luoxide-cli -- parse lua_scripts/parser/simple.lua

RUST_LOG=luoxide_parser::parse::shallow=debug,luoxide_parser::parse::deep=trace \
  cargo run -p luoxide-cli -- parse --debug lua_scripts/parser/simple.lua
```

Trace lines go to stderr; reconstructed Lua / debug AST stay on stdout.

## Parser example (richer dump / `--trace`)

`crates/luoxide-parser/examples/main.rs` dumps reconstructed Lua plus a Debug AST to `target/parser-ast.debug` (override with `--dump-ast FILE`). `--trace` maps to the filters above unless `RUST_LOG` is already set.

```bash
cargo run -p luoxide-parser --example main -- --trace shallow
cargo run -p luoxide-parser --example main -- --trace deep
cargo run -p luoxide-parser --example main -- --trace both
```

Lexer smoke example:

```bash
cargo run -p luoxide-parser --example lexdebug
```

## Tests and snapshots

```bash
# devenv
devenv test

# cargo
cargo test --workspace
cargo xtask test          # tests + crate examples
cargo xtask ci            # same, with INSTA_UPDATE=new
cargo xtask snap          # insta review for snapshot failures
cargo xtask examples      # run all crate examples
```

`cargo xtask` with no args prints the task list.

## What to expect

- This is a **parser frontend**, not a Lua runtime.
- Grammar is Lua 5.5-oriented; the official Lua test suite is not wired in yet.
- Design notes: `crates/luoxide-parser/docs/names-and-session.md`.
