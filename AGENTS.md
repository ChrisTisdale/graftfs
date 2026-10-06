# AGENTS.md

Guidance for coding agents working in this repository.

## Project

`graftfs` is a Rust reimplementation of GNU stow: a symlink farm manager for dotfiles. It supports
stow / delete / restow / list, simulation mode, directory folding, and regex-based ignore/override patterns.

`README.md` is the user-facing CLI and configuration reference. This file and `docs/` describe how to work *on* the
repository.

## Workspace layout

Cargo workspace, edition 2024 (`rust-version` in the root `Cargo.toml`).

| Path     | Crate     | Contents                                                              |
|----------|-----------|-----------------------------------------------------------------------|
| `graft/` | `graftfs` | lib `graft` + bin `graft` — all application code                      |
| `xtask/` | `xtask`   | build automation, `publish = false`, not part of the shipped artifact |

> **Note**: The package name is `graftfs` (`cargo test -p graftfs`), but the binary and library target name is `graft`.

Crate-specific guidance lives with the crate, not here:

- [`graft/AGENTS.md`](graft/AGENTS.md) — application architecture
- [`xtask/AGENTS.md`](xtask/AGENTS.md) — build automation

## Workspace documentation

- [`docs/commands.md`](docs/commands.md) — build, test, lint, and dist commands
- [`docs/conventions.md`](docs/conventions.md) — file headers, formatting, test and commit style
- [`docs/ci-and-release.md`](docs/ci-and-release.md) — CI gates and the release process

## Before submitting a change

Run the three CI gates locally (see [`docs/commands.md`](docs/commands.md)). The clippy gate is stricter
than a bare `cargo clippy` and is the one most likely to surprise you:

```bash
cargo fmt --all -- --check
cargo clippy --all -- -W clippy::all -W clippy::pedantic -W clippy::nursery -D warnings
cargo test
```
