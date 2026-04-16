# CLAUDE.md — aoc_solver

## Project

A Rust CLI binary (`aoc`) that scaffolds and runs Advent of Code solutions. Built to practice Rust.

Binary + lib layout: `src/main.rs` (thin), `src/lib.rs` (re-exports everything for integration tests).

## Architecture

```
cli/        # clap subcommand handlers (new, run, test)
solver/     # Solution struct, SolverFn, registry, input reader
scaffold/   # file generation and mod.rs patching for `aoc new`
solutions/  # compiled-in solutions, organized as y<year>/d<DD>.rs
error.rs    # AocError enum, Result<T> alias
```

### Solution registration

Solutions are compiled into the binary. Each year has `src/solutions/y<year>/mod.rs` which
exposes `pub fn solutions() -> &'static [Solution]`. `src/solutions/mod.rs` collects all
years into `pub fn all_years()`. `solver/registry.rs` flattens into a `OnceLock<Vec<Solution>>`
for O(N) lookup by `(year, day)`.

The `solution!` macro (in `solver/mod.rs`) constructs a `Solution` from year/day/function refs:

```rust
solution!(2024, 1)
// expands to Solution { year: 2024, day: 1, part1: y2024::d01::part1, part2: y2024::d01::part2 }
```

### Marker-based mod.rs patching

`aoc new` inserts `pub mod` lines into mod.rs files using comment markers:

```
// @@REGISTRY_YEARS_START@@
pub mod y2024;
// @@REGISTRY_YEARS_END@@
```

```
// @@SOLUTIONS_START@@
pub mod d01;
// @@SOLUTIONS_END@@
```

**Rules**:

- Both markers must be present; if missing, return `AocError::ModPatch`.
- Insertions are sorted alphabetically within the marker block.
- Writes are atomic: write to `path.tmp` then `fs::rename`.
- Operation is idempotent: re-running `new` for an existing day returns `AocError::AlreadyExists`.

## Key Types

```rust
pub type SolverFn = fn(&str) -> Result<String>;

pub struct Solution {
    pub year: u16,
    pub day: u8,
    pub part1: SolverFn,
    pub part2: SolverFn,
}
```

```rust
pub enum AocError {
    Io(std::io::Error),
    SolutionNotFound { year: u16, day: u8 },
    InvalidDay(u8),
    InvalidYear(u16),
    InputMissing(PathBuf),
    AlreadyExists(PathBuf),
    ModPatch(String),
    Parse(String),
}
```

## File Conventions

| Concept           | Pattern                          | Example                      |
| ----------------- | -------------------------------- | ---------------------------- |
| Solution source   | `src/solutions/y<year>/d<DD>.rs` | `src/solutions/y2024/d01.rs` |
| Year mod file     | `src/solutions/y<year>/mod.rs`   | `src/solutions/y2024/mod.rs` |
| Root mod file     | `src/solutions/mod.rs`           | —                            |
| Puzzle input      | `inputs/<year>/day<DD>.txt`      | `inputs/2024/day01.txt`      |
| Rust module ident | `d<DD>` (zero-padded)            | `d01`, `d25`                 |
| Solution template | `templates/solution.rs.tmpl`     | loaded via `include_str!`    |

Zero-padding is always two digits (`01`–`25`). Format: `format!("d{day:02}")`.

## Path Resolution

`solver/input.rs` resolves `inputs/<year>/day<DD>.txt` relative to `env::current_dir()`.
In debug builds only, fall back to `env!("CARGO_MANIFEST_DIR")` if current dir lacks `inputs/`.
Document that users must run `aoc` from the project root.

`scaffold/paths.rs` accepts an explicit `root: &Path` parameter — never hardcoded — to keep all
path logic unit-testable without touching the real filesystem.

`templates/solution.rs.tmpl` is embedded at compile time via `include_str!("../../templates/solution.rs.tmpl")`
from `src/scaffold/template.rs`.

## `aoc test` Implementation

Delegates to cargo:

```rust
std::process::Command::new("cargo")
    .args(["test", "--", &format!("solutions::y{year}::d{day:02}")])
    .status()?
```

Safe at runtime (not during build). Must be invoked after `cargo build` completes.

## Gotchas

1. **Marker corruption**: if a user edits between markers, patcher must fail loudly, not silently corrupt.
2. **Atomic writes**: always write `.tmp` then rename to avoid half-written mod.rs.
3. **Sort order**: keep `d01..d25` and `y2015..y20xx` sorted within marker blocks.
4. **Day zero-padding**: file = `d01.rs`, module ident = `d01`, input = `day01.txt`. Never `d1` or `day1`.
5. **`include_str!` path**: relative to the _source file_, not the crate root.
6. **Integration test isolation**: pass `root: &Path` explicitly; never call `env::current_dir()` inside tests.
7. **No `anyhow`/`thiserror`**: hand-roll `Display`, `Error`, `From` impls in `error.rs`.
8. **No mutation**: avoid `&mut` outside IO buffers and clap derive-generated code.
9. **File size limit**: 800 lines max per file; 400 lines is the target. Split if growing.
10. **Scaffold ordering**: write solution file first, then patch year mod.rs, then root mod.rs.
    On failure after file creation, best-effort cleanup (delete created file).

## Testing Requirements

- 80%+ coverage on `scaffold/*` and `solver/registry`.
- Unit tests co-located in `#[cfg(test)] mod tests`.
- Integration tests in `tests/` use a temp dir (hand-rolled, no `tempfile` crate).
- `scaffold_integration.rs`: call `scaffold::execute` against temp dir, assert output, run twice for idempotency.
- All new features start with a failing test (TDD).

## Dependencies

Only `clap = { version = "4", features = ["derive"] }`. No `anyhow`, `thiserror`, `tempfile`, `regex`, etc.

## Nix

- `nix develop` enters the devshell with `cargo`, `rustc`, `rust-analyzer`.
- `lint` script runs `nix flake check` (treefmt + pre-commit).
- Pre-commit runs: `rustfmt`, `nixfmt-rfc-style`, `prettier`, `trufflehog`.
