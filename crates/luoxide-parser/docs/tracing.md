# Parser tracing

How to watch the recursive-descent walk: which production is on the
stack, which token is being consumed, and where recovery skipped.

The subscriber is not the parser. The parser only emits `tracing` events.
The example binary installs a subscriber; tests and the library do not.

## Two independent targets

Pick a layer without the other. They do not imply each other.

| Target | Level | What it is |
|---|---|---|
| `luoxide_parser::parse::shallow` | `DEBUG` | Production enter/leave, mismatch, error, recover, sync |
| `luoxide_parser::parse::deep` | `TRACE` | Every consumed token (`eat`) |

`deep` does **not** log enter/leave. `shallow` does **not** log `eat`.
Turn both on when you need the production path *and* the cursor.

```text
RUST_LOG=luoxide_parser::parse::shallow=debug
RUST_LOG=luoxide_parser::parse::deep=trace
RUST_LOG=luoxide_parser::parse::shallow=debug,luoxide_parser::parse::deep=trace
```

The example maps `--trace` onto those filters, and `RUST_LOG` wins if set:

```text
cargo run -p luoxide-parser --example main -- --trace shallow
cargo run -p luoxide-parser --example main -- --trace deep
cargo run -p luoxide-parser --example main -- --trace both
```

Snapshot tests should leave `RUST_LOG` unset. A live subscriber would
spam the runner and can make insta output noisy.

## Frames vs depth

Two stacks, different jobs.

```text
frames  ["chunk", "block", "statement", "local", "function_body"]   names
depth   1                                                            nesting
```

- **`frames`** is the production path. `with_frame(name, …)` pushes a
  static name, logs `enter`, runs the closure, logs `leave`, pops.
  Frames do **not** count toward [`MAX_NESTING_DEPTH`](src/../src/parser.rs).
- **`depth`** is Lua-style nesting (`LUAI_MAXCCALLS`). `with_depth`
  increments it, then calls `with_frame`. Only **statement** and
  **expression** use `with_depth`. Nested `a + b` therefore nests two
  `"expression"` frames and bumps `depth` twice.

A `local function` body can sit at `depth=1` with five frames: `local`
and `function_body` are labels, not extra nesting.

`with_frame` also enters an anonymous `parse` span so a subscriber can
indent the walk. Cursor and production name live on the **event**, not
the span, so the parent stays quiet.

## Shallow actions

Emitted at `DEBUG` on `luoxide_parser::parse::shallow`.

| `action` | When | Extra fields |
|---|---|---|
| `enter` / `leave` | `with_frame` | full `frames` |
| `mismatch` | `unexpected_token` | `expected` when one token was required |
| `error` | `record_error` | `error` (`ParseErrorKind` / `"lexer error"`) |
| `recover` | statement/expression recovery starts | `error` = recover kind |
| `sync` | skip-until / forced bump finished | `skipped`, `from` (span before skip) |

`mismatch` is the *look*: current token is not what this site wanted.
`error` is the *record*: the diagnostic is now in the error context.
`expect` emits both in that order. Call sites that return `Err` without
`record_error` emit `mismatch` only; recovery later emits `error`.

Recover kinds:

```text
statement            recover_statement (usual)
statement-nesting    NestingTooDeep in a statement
expression           recover_expression (usual)
expression-nesting   NestingTooDeep in an expression
```

## Deep `eat`

Emitted at `TRACE` on `luoxide_parser::parse::deep` from `bump()`.
Fields are the cursor plus `name` (top of `frames`) and `depth`. **No**
`frames` list — that belongs to shallow.

Recovery skip uses `bump_untraced()` so skipped junk does not look like
accepted syntax. `sync` is the shallow counterpart: how many tokens
moved, and from where.

## Event fields

Every action carries the lookahead at emit time (`token` / `lexeme` /
`at`). That is **`current`**, not the token just eaten. `prev` is the
lexeme of the last accepted token.

| Field | Meaning |
|---|---|
| `name` | Top production (`frames.last()`) |
| `depth` | `with_depth` counter |
| `frames` | Full production stack (shallow only) |
| `action` | Shallow: `enter`, `leave`, `mismatch`, `error`, `recover`, `sync` |
| `token` | `TokenKind` of lookahead |
| `lexeme` | Source slice of lookahead |
| `at` | Span of lookahead (`start..end` byte offsets) |
| `prev` | Lexeme of the last consumed token |
| `expected` | `mismatch` with a single required kind (`as_lua`) |
| `skipped` | Tokens advanced by this `sync` |
| `error` | Diagnostic label, or recover kind |
| `from` | Span where skip started (`sync` only) |

On `leave`, `token` is whatever comes *next*, and `prev` is whatever the
production just finished. That is how you see `leave` of `table` with
`prev="}"` and lookahead already on the next `local`.

## Production names

Named by `with_frame` / `with_depth` today:

| Name | Kind | Site |
|---|---|---|
| `chunk` | frame | whole file |
| `block` | frame | `{statement}` including nested bodies |
| `statement` | depth | one statement |
| `local` | frame | `local` name list / `local function` |
| `global` | frame | assignment / expression-statement |
| `function` | frame | `function name …` declaration |
| `function_body` | frame | `(parlist) block end` |
| `table` | frame | `{ … }` |
| `expression` | depth | precedence-climbing operand |

Missing names (if/while/for, suffix, call, …) still run; they just
inherit the enclosing `statement` / `expression` frame. Add a frame when
a walk is hard to read, not on every helper.

## Reading a log

`local simple = {}` from the example source:

```text
DEBUG shallow: enter  depth=0 name="chunk"      frames=["chunk"]
DEBUG shallow: enter  depth=0 name="block"      frames=["chunk","block"]
DEBUG shallow: enter  depth=1 name="statement"  frames=[…,"statement"]
DEBUG shallow: enter  depth=1 name="local"      frames=[…,"local"]
TRACE deep:    eat    depth=1 name="local"      token=Local lexeme="local"
TRACE deep:    eat    depth=1 name="local"      token=Lit_Identifier lexeme="simple"
TRACE deep:    eat    depth=1 name="local"      token=Assign lexeme="="
DEBUG shallow: enter  depth=2 name="expression" frames=[…,"expression"]
DEBUG shallow: enter  depth=2 name="table"      frames=[…,"table"]
TRACE deep:    eat    depth=2 name="table"      token=LeftCurly lexeme="{"
TRACE deep:    eat    depth=2 name="table"      token=RightCurly lexeme="}"
DEBUG shallow: leave  depth=2 name="table"      prev="}"
DEBUG shallow: leave  depth=2 name="expression"
DEBUG shallow: leave  depth=1 name="local"
DEBUG shallow: leave  depth=1 name="statement"
```

How to use that:

- **Wrong production** — `name` / `frames` at `mismatch` or `error`.
  `expected` is what that site asked for.
- **Cursor stall** — successive events with the same `at`. Recovery
  should then `sync` (and `bump_untraced` so `deep` stays quiet).
- **Too much skip** — `sync` `from`…`at` and `skipped`.
- **Nesting** — `depth` climbing toward 200 is `with_depth`, not extra
  frames. `NestingTooDeep` recover kinds skip to a terminator instead of
  retrying the same opener.

Compact `tracing-subscriber` lines look like:

```text
DEBUG luoxide_parser::parse::shallow: enter depth=1 frames=["chunk", "block", "statement", "local"] action="enter" token=Local lexeme="local" at=2..7 prev="" name="local"
TRACE luoxide_parser::parse::deep: eat depth=1 name="local" token=Lit_Identifier lexeme="simple" at=8..14 prev="local"
```

Filter in the subscriber (or `rg 'action="(mismatch|error|recover|sync)"'`)
when the walk is long.

## Compile-level events (not these targets)

`compile_chunk` / `compile_expression` also emit:

```text
info_span!("compile_chunk") / info_span!("compile_expression")
INFO  "starting chunk compilation"
ERROR "parsing produced N error(s)"
```

Those use the **module path** (`luoxide_parser::parser`), not
`parse::shallow` / `parse::deep`. `--trace` will not show them. Enable
them separately if you want a parse start/stop bookmark:

```text
RUST_LOG=luoxide_parser::parser=info,luoxide_parser::parse::shallow=debug
```

## Wiring

`tracing` is pulled in by the `parse` feature. Library users who want
logs install their own subscriber and set the filters above. The example
does:

```text
EnvFilter from RUST_LOG, else:
  --trace shallow  →  luoxide_parser::parse::shallow=debug
  --trace deep     →  luoxide_parser::parse::deep=trace
  --trace both     →  both
fmt().compact()
```
