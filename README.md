# aoc_solver

A CLI tool for scaffolding and running [Advent of Code](https://adventofcode.com) solutions in Rust.

## Usage

```
aoc new <year> <day>     # scaffold a new solution
aoc run <year> <day>     # run both parts
aoc run <year> <day> --part <1|2>  # run a specific part
aoc test <year> <day>    # run tests for a day
```

### Examples

```sh
# scaffold day 1 of 2024
aoc new 2024 1

# add your puzzle input
echo "..." > inputs/2024/day01.txt

# run it
aoc run 2024 1

# run only part 2
aoc run 2024 1 --part 2

# run tests
aoc test 2024 1
```

## Project Layout

```
.
├── inputs/              # puzzle inputs (gitignored)
│   └── <year>/
│       └── day<DD>.txt
├── templates/
│   └── solution.rs.tmpl # template for new solution files
└── src/
    ├── main.rs          # CLI entrypoint
    ├── lib.rs
    ├── error.rs         # AocError, Result alias
    ├── cli/             # subcommand handlers
    │   ├── mod.rs
    │   ├── new.rs
    │   ├── run.rs
    │   └── test.rs
    ├── solver/          # solution dispatch
    │   ├── mod.rs       # Solution struct, SolverFn, solution! macro
    │   ├── registry.rs  # runtime lookup by (year, day)
    │   └── input.rs     # reads inputs/<year>/day<DD>.txt
    └── solutions/       # compiled-in solutions
        ├── mod.rs       # @@REGISTRY_YEARS_START@@ ... @@REGISTRY_YEARS_END@@
        └── y<year>/
            ├── mod.rs   # @@SOLUTIONS_START@@ ... @@SOLUTIONS_END@@
            └── d<DD>.rs # individual solution
```

## Solution Format

Each solution file (`src/solutions/y<year>/d<DD>.rs`) exposes:

```rust
use crate::error::Result;

pub fn part1(input: &str) -> Result<String> {
    todo!()
}

pub fn part2(input: &str) -> Result<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).unwrap(), "");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE).unwrap(), "");
    }
}
```

## Module Registry

Solutions are registered via marker comments in `mod.rs` files. The `aoc new` command
patches these automatically in sorted order:

```rust
// src/solutions/mod.rs
// @@REGISTRY_YEARS_START@@
pub mod y2023;
pub mod y2024;
// @@REGISTRY_YEARS_END@@
```

```rust
// src/solutions/y2024/mod.rs
// @@SOLUTIONS_START@@
pub mod d01;
pub mod d02;
// @@SOLUTIONS_END@@
```

Never manually edit lines inside the marker blocks — instead let `aoc new` manage them.
Lines outside the markers are yours to edit freely.

## Inputs

Puzzle inputs are stored in `inputs/<year>/day<DD>.txt` relative to the working directory.
They are gitignored. `aoc new` creates an empty placeholder file.

Running `aoc run` from anywhere other than the project root requires the `inputs/` tree
to be present in the current directory.

## Building

```sh
cargo build --release
# or via nix devshell:
nix develop
cargo build
```
