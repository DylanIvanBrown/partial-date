# AGENTS.md — partial-date

Guidelines for AI coding agents working in this repository.

---

## Repository layout

```
partial-date/           # library crate (published)
  src/
    lib.rs              # crate root, module declarations, top-level doc examples
    models.rs           # all types: Extracted<T>, PartialDate, Config, …
    extract.rs          # public extract() function and extraction logic
  spec.md               # design specification — read before making structural changes

partial-date-tests/     # integration test crate (publish = false)
  src/
    lib.rs              # module declarations + shared helpers module
    day.rs / month.rs / year.rs / full_date.rs / partial_date.rs / config.rs
```

The two crates form a Cargo workspace. Test dependencies (rstest, etc.) live only in `partial-date-tests` so they are never imposed on library consumers.

---

## Build, lint, and test commands

```bash
# Build the whole workspace
cargo build

# Check without producing artefacts (fast)
cargo check

# Run clippy — must pass with zero warnings
cargo clippy --workspace -- -D warnings

# Format all code (apply in-place)
cargo fmt

# Check formatting without modifying files
cargo fmt -- --check

# Run all tests (library doctests + integration tests)
cargo test

# Run only integration tests (partial-date-tests crate)
cargo test -p partial-date-tests

# Run all tests whose path contains a substring — e.g. the whole year module
cargo test -p partial-date-tests -- year::

# Run one specific test function
cargo test -p partial-date-tests -- config::config_custom_window_range

# Run one specific rstest case (cases are numbered case_1, case_2, …)
cargo test -p partial-date-tests -- config::config_custom_format::case_2

# Run all library doctests only
cargo test -p partial-date
```

Always run `cargo fmt` and `cargo clippy --workspace -- -D warnings` before considering a change complete. No `rustfmt.toml` or `clippy.toml` — default settings only.

---

## Code style

### Edition and toolchain

- Rust **edition 2024** for both crates (set in each `Cargo.toml`).
- Target stable Rust; do not use nightly-only features.

### Formatting

- Use `cargo fmt` (rustfmt defaults).
- 4-space indentation; no tabs.
- **Always use trailing commas** in multi-line struct literals, enum variants,
  function arguments, match arms, and `vec!` literals.

### Imports

Import order — internal first, then workspace crates, then external, then std.
No blank lines between groups:

```rust
use crate::models::{Config, Extracted};   // crate-internal
use crate::extract::extract;              // crate-internal
use some_external_crate::Foo;             // external crates
use std::collections::HashMap;            // stdlib
```

- Group multiple items from the same path:
  `use crate::models::{Config, Extracted, Input};`
- In library code (`src/`), import individual items — no glob imports.
- `use crate::models::*` is acceptable inside test modules only, to avoid
  repetitive re-imports of every type.

### Naming conventions

| Item | Convention | Example |
|------|-----------|---------|
| Types / Enums / Traits | `UpperCamelCase` | `PartialDate`, `MonthName` |
| Enum variants | `UpperCamelCase` | `Extracted::Found`, `Format::DDMMYYYY` |
| Functions / methods | `snake_case` | `extract()`, `is_not_found()` |
| Variables / fields | `snake_case` | `primary_format`, `two_digit_expansion` |
| Test functions | `snake_case`, descriptive | `year_two_digit_sliding_window` |
| Constants | `SCREAMING_SNAKE_CASE` | (none yet) |

Format variant names (`DDMMYYYY`, `YYYYMMDD`, etc.) are intentional all-caps
abbreviations matching the spec.

### Variable name clarity

**Do not abbreviate variable names outside of closures.** Full words are
required; contractions that save a few characters at the cost of readability
are not acceptable.

```rust
// Bad — abbreviated
let sep_chars  = ...;
let multi_seps = ...;
let dc         = 2_u8;
let v          = 42_i16;

// Good — full words
let separator_chars        = ...;
let multi_char_separators  = ...;
let digit_count            = 2_u8;
let value                  = 42_i16;
```

The exception is **closure parameters**, where the brevity is idiomatic Rust
and the scope is so small that the meaning is obvious from context:

```rust
// Fine — closure parameter, scope is one expression
ranges.sort_by_key(|r| r.start);
bytes.iter().all(|b| b.is_ascii_digit());
```

Common abbreviations to avoid:

| Avoid | Use instead |
|-------|-------------|
| `sep` / `seps` | `separator` / `separators` |
| `dc` | `digit_count` |
| `v` / `v0` / `v1` | `value` / `first_value` / `second_value` |
| `d` / `d0` / `d1` | `digit_count` / `first_digit_count` / `second_digit_count` |
| `ch` | `character` |
| `abs_` prefix | spell out the full intent, e.g. `absolute_start` |
| `m` / `n` | `month` / `number` (or whatever the domain meaning is) |

### Types and type safety

- **Newtypes over primitives** — wrap values in domain types (`Day`, `Month`,
  `Year`) rather than passing raw `u8`/`i32` across boundaries.
- Use `Extracted<T>` (not `Option<T>`) for all extraction results — it
  distinguishes `Found` / `Defaulted` / `NotFound`, which `Option` cannot.
- Use `i32` for year values (not `u32`) to allow the full `0–3000` range and
  historical dates. Prefer `u8` for day and month values.
- **Fallible constructors** — new validation-capable types (e.g. `WindowRange`)
  must expose a `new()` returning `Result<Self, SomeError>`, with validation
  logic self-contained there. Implement `Default` for the canonical safe value.
- **Make invalid states unrepresentable** — use enums and type structure to
  prevent illegal combinations at compile time rather than runtime checks.

### Error handling

- The library always returns a `PartialDate` — even when nothing is found,
  each field is `Extracted::NotFound` rather than bubbling an error.
- **Never use `unwrap()` or `expect()` in library code** (`src/`). Use the `?`
  operator and `Result` types. Test code may use `.unwrap()` only where the
  value is guaranteed to be `Ok`/`Some`.
- Internal validation errors (e.g. `WindowRangeError`) use dedicated
  `#[derive(Debug, Clone, PartialEq, Eq)]` error enums with descriptive
  variants — not stringly-typed messages.

### Enums and match arms

- **Avoid `_ =>` match arms.** Exhaustive matches force every new variant to be
  handled explicitly — the compiler catches omissions. Use nested enum
  structures to express sub-groups rather than defaulting to a catch-all.
- **Never `_ => panic!(...)` in library code.** Panics create unstable code and
  are especially dangerous in a library that callers cannot recover from.

### Derives

Every public type must derive `Debug`. Add further derives based on the type's
role:

- `Clone` — add when callers are likely to need to clone the value (e.g.
  config types, input types). Omit for result types (`PartialDate`, `Day`,
  `Month`, `Year`) that are consumed rather than cloned.
- `PartialEq, Eq` — add for enums and any type used in assertions or
  comparisons.
- `Copy` — add when the type is small and cheap to copy (e.g. `IsExpected`,
  `TwoDigitYearExpansion`, `WindowRange`, `Range`).
- `Default` — add whenever a sensible zero-configuration value exists.

```rust
#[derive(Debug)]                           // result structs (PartialDate, Day, …)
#[derive(Debug, Clone)]                    // config / input structs
#[derive(Debug, Clone, PartialEq, Eq)]    // enums used in assertions / comparisons
```

### Documentation

- Every public item in `src/` must have a `///` doc comment.
- Doc comments on `Config` fields should state the default value explicitly,
  e.g. `/// Default: \`31\``.
- Use `` [`backtick links`] `` to cross-reference related types.
- The `lib.rs` quick-start example in the module doc must compile (it is run
  as a doctest).

---

## Testing conventions

- **All tests live in `partial-date-tests/`**, not in `src/` — this keeps test
  dependencies out of the library's dependency tree.
- Use **`rstest`** for any test that exercises the same logic with multiple
  inputs/outputs. A plain `#[test]` is only appropriate when a case is
  genuinely unique and cannot be parameterised.
- Prefer more `#[case]` entries over separate `#[test]` functions when the
  assertion structure is identical.
- Shared setup helpers (config builders, input constructors) live in the
  `helpers` module in `partial-date-tests/src/lib.rs`. Add helpers there
  rather than duplicating boilerplate inside individual test files.
- Test function names follow the pattern `<component>_<scenario>`, e.g.
  `year_two_digit_sliding_window`, `day_ordinal_in_context`.
- rstest case groups are named `<what_is_being_tested>` (no `_cases` suffix).
- `unwrap()` is acceptable in test code — the no-unwrap rule applies to library
  code only.

---

## Architecture notes

- `models.rs` owns **all** type definitions. Do not define domain types
  elsewhere.
- `extract.rs` owns the extraction entry point (`pub fn extract(input: Input)
  -> PartialDate`) and all extraction logic. Keep it free of type definitions.
- The library is intentionally **zero external dependencies** in `partial-date`
  itself. Do not add entries to `[dependencies]` in the root `Cargo.toml`
  without strong justification.
- Extraction must remain **fully deterministic** — no randomness, no ML/AI,
  no network calls.
- Consult `spec.md` before making structural decisions (new fields, new config
  options, changed semantics). The spec is the source of truth.

---

## Key principles

1. **Leverage the type system** — make invalid states unrepresentable.
2. **Validate once** — in type constructors, not scattered across functions.
3. **Explicit error handling** — `Result` with `?`, never `unwrap()` in library code.
4. **Work with the compiler** — exhaustive match arms, strong types, avoid
   unnecessary `clone()` and `Arc<Mutex<>>`.
5. **Keep it simple** — don't over-engineer; only add what is needed now.
