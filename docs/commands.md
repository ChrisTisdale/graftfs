# Commands

All commands run from the workspace root.

## Build

```bash
cargo build --all-features --all-targets            # debug, as CI builds it
cargo build --release --all-features --all-targets
```

## Test

```bash
cargo test                          # unit tests + doctests
cargo test -p graftfs <substring>   # run a single test or subset by name
cargo test --doc                    # doctests only
```

Doctests are a real part of the suite — public API examples are compiled and run. See
[`conventions.md`](conventions.md).

## Lint and format

```bash
cargo fmt --all -- --check
cargo clippy --all -- -W clippy::all -W clippy::pedantic -W clippy::nursery -D warnings
```

Run the clippy line above before proposing changes. `clippy::pedantic` and `clippy::nursery` are denied in
CI, so a plain `cargo clippy` will not reproduce the failures that block a merge.

## Docs

```bash
cargo doc
```

CI builds documentation as a separate job, so broken intra-doc links fail the build.

## Run

```bash
cargo run --package graftfs -- [OPTIONS] [COMMAND]
```

## Distribution

```bash
cargo xtask dist
```

`cargo xtask` is an alias defined in `.cargo/config.toml` (`run --package xtask --`). `dist` builds the
binary and generates man pages into `target/dist`. It **deletes** the output directory before building.
Flags and behavior are documented in [`../xtask/AGENTS.md`](../xtask/AGENTS.md).
