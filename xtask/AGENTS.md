# xtask

Build automation, invoked as `cargo xtask <command>` via the alias in `.cargo/config.toml`
(`run --package xtask --`). `publish = false` — this crate is never released and is not part of the shipped
artifact.

For workspace-wide commands and conventions, see [`../AGENTS.md`](../AGENTS.md) and [`../docs/commands.md`](../docs/commands.md).

## `cargo xtask dist`

Builds the `graft` binary and generates man pages into one output directory.

| Flag                          | Effect                         |
|-------------------------------|--------------------------------|
| `-c`, `--configuration`       | `release` (default) or `debug` |
| `-a`, `--all-features`        | conflicts with `--feature`     |
| `-f`, `--feature <NAME>`      | repeatable                     |
| `-n`, `--no-default-features` |                                |
| `-o`, `--output <DIR>`        | defaults to `target/dist`      |

**The output directory is deleted before the build** (`fs::remove_dir_all`, ignoring failure). Never point
`-o` at a directory holding anything you want to keep.

The build shells out to `cargo` — honoring the `CARGO` environment variable when set — then copies
`target/<configuration>/graft` to `<out_dir>/graft`. The binary name is hardcoded without a platform
extension, so the copy step is Unix-shaped.

`project_root()` resolves via `CARGO_MANIFEST_DIR` and `.ancestors().nth(1)`, i.e. it assumes this crate sits
exactly one level below the workspace root. Moving the crate breaks it.

## Man page generation

`dist_manpage` reflects over `graft`'s clap command tree rather than duplicating it: it calls
`CommandLineProcessor::command()` through the `graft` library dependency, renders the root with
`clap_mangen`, and walks `get_subcommands()` to emit `graft-<name>.1` alongside `graft.1`.

This is the only reason `xtask` depends on `graftfs` (imported in Rust as `graft`). CLI changes in
`graft/src/command_line_args.rs` propagate to the man pages automatically.

## The duplicated `Shell` enums

`create_subcommand` special-cases the `completions` subcommand. Because the man page must list the shells
available in *the build being distributed*, and the `nushell` variant is `#[cfg(feature = "nushell")]` in
the `graft` library, `xtask` cannot reuse that enum — the feature is resolved at `graft`'s compile time,
not at man-page-generation time. Instead it declares two local `ValueEnum`s and picks one at runtime based
on whether `nushell` or `--all-features` appears in the requested feature list:

- `Shell` — Bash, Fish, Elvish, PowerShell, Zsh
- `ShelWithNusehll` — the same plus Nushell (name is misspelled in the source)

These are **not** checked against `graft`'s `Shell` by the compiler. A shell added in
[`../graft/docs/shells.md`](../graft/docs/shells.md) but not here yields man pages that omit it, with no
build error.
