# Conventions

## File headers

Every `.rs` file in the workspace starts with the GPL-3.0-or-later header block. Copy it verbatim into new
files — see the top of any existing source file.

## Formatting

Formatting is governed by `.rustfmt.toml`. Notable settings that differ from rustfmt defaults:

- `max_width = 120`
- `fn_params_layout = "Tall"`
- `edition = "2024"`
- `use_field_init_shorthand = true`, `use_try_shorthand = true`

Do not hand-format against these; run `cargo fmt --all`.

## Tests

Tests live in inline `#[cfg(test)] mod tests` blocks at the bottom of the file they cover. There is no
`tests/` directory and no integration-test harness.

Cases use `rstest` (`#[rstest]`, `#[case]`) rather than plain `#[test]`. Follow the surrounding style when
adding tests.

## Doc examples

Public API doc comments carry runnable examples, and `cargo test` compiles and runs them. Keep them valid:

- Use plain ```` ```rust ```` only when the example can safely execute in a test sandbox.
- Use ```` ```no_run ```` for anything that would mutate the filesystem.

An example that creates symlinks or deletes files without `no_run` will make `cargo test` destructive.

## Platform-specific code

Symlink creation on Windows requires developer mode or elevation. Platform differences are gated with
`#[cfg(unix)]`, `#[cfg(target_os = "windows")]`, and `#[cfg(target_os = "macos")]` rather than runtime
branching. CI builds on Linux, Windows, and macOS, so a `cfg` block that fails to compile on one platform
fails the whole matrix.

## Commits

Commits follow [Conventional Commits](https://www.conventionalcommits.org) (`feat:`, `fix:`, …). The commit
subject feeds changelog generation — see [`ci-and-release.md`](ci-and-release.md).
