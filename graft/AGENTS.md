# graft

Package `graftfs`, producing:

- lib `graft` — `src/lib.rs`, internal to workspace (consumed by `xtask` and doctests; not shipped in crate tarball)
- bin `graft` — `src/main.rs`

For workspace-wide commands, conventions, and CI, see [`../AGENTS.md`](../AGENTS.md) and [`../docs/commands.md`](../docs/commands.md).

## Module tree is declared twice

`src/lib.rs` and `src/main.rs` each declare the same eight modules (`cli_args`, `cli_errors`,
`command_line_args`, `commands`, `config`, `executor`, `shell`, `shell_converter_error`). A new top-level module must be
added to **both** files or the binary and the library will diverge. `lib.rs` marks each with `#[allow(unused)]`
because the library exposes surface the binary does not use.

`src/lib.rs` is listed in the package's `exclude` list, so it is not shipped in the published crate tarball.

## Feature flags

The only feature is `nushell`, which pulls in `clap_complete_nushell` and adds a `Shell::Nushell` variant.
`default = []`.

> The README's completions section says `cargo install graftfs --locked --features completions`. There is no
> `completions` feature; that command fails. The correct flag is `nushell`.

## Architecture

Read these in order if you are new to the crate:

1. [`docs/execution-pipeline.md`](docs/execution-pipeline.md) — startup sequence from `main` through the
   `Executor` dispatch pattern to command execution
2. [`docs/command-operation.md`](docs/command-operation.md) — the single filesystem seam and how
   `--simulate` works
3. [`docs/builders.md`](docs/builders.md) — how a `Command` is constructed
4. [`docs/configuration.md`](docs/configuration.md) — config layering, versioning, and platform paths
5. [`docs/regex-matching.md`](docs/regex-matching.md) — ignore/override pattern evaluation
6. [`docs/errors-and-logging.md`](docs/errors-and-logging.md) — snafu and tracing patterns
7. [`docs/shells.md`](docs/shells.md) — completion generation and adding a shell

## Testing

[`docs/testing.md`](docs/testing.md) — the `StowSetup` harness, committed fixtures under `test_data/`, and
why the command tests are serialized. Read it before adding a test that touches the filesystem.
