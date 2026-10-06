# Testing

General test conventions (inline `#[cfg(test)]` modules, `rstest`, doctests) are in
[`../../docs/conventions.md`](../../docs/conventions.md). This covers what is specific to this crate.

## Where the tests are

Five inline test modules:

| File                                | Covers                             |
|-------------------------------------|------------------------------------|
| `src/commands/command.rs`           | end-to-end stow/unstow/restow/list |
| `src/config/mod.rs`                 | config parsing, V1→V2 upgrade      |
| `src/config/logging_config.rs`      | logging settings                   |
| `src/config/stow_config.rs`         | stow settings                      |
| `src/config/config_file_version.rs` | version parsing                    |

Config tests are pure and use `const` fixtures declared at the top of the module. Command tests touch the
real filesystem.

## `StowSetup` — the command test harness

`src/commands/command.rs` tests run against real directories, not a mock. `StowSetup` manages that:

```rust
let setup = StowSetup::new("existing_directory_test").unwrap();
let command = setup.default_builder().stow().build().unwrap();
```

- **Fixtures** live in `test_data/stow_tests/<name>/` and are **committed to git**. Adding a test that needs
  new input means adding files there.
- **Scratch targets** are created at `test_data/scratch/stow/<name>/` and removed in `StowSetup::drop`. This
  tree is not tracked; a panicking test can leave directories behind.
- Both paths are built from `CARGO_MANIFEST_DIR`, so tests work regardless of the invoking directory.

`StowSetup::new(name)` uses the same string for the scratch directory and the fixture directory — by
convention, the test function's own name. Use `new_with_data(scratch_name, fixture_name)` when several
tests share one fixture, which is how the unstow tests reuse the stow fixtures:

```rust
StowSetup::new_with_data("basic_unstow_test", "basic_stow_test")
```

Scratch names must stay unique per test even when the fixture is shared.

## Tests are serialized

`StowSetup` acquires a static `TEST_SYNC: LazyLock<Mutex<()>>` and holds the guard in `_guard`
for its whole lifetime. Command tests share on-disk state and read the process-global working directory, so
they cannot run concurrently under cargo's default threaded runner.

Consequences:

- A test that does filesystem work **must** go through `StowSetup`, or it will race the ones that do.
- A panic while the guard is held poisons the mutex, so one genuine failure can cascade into
  `PoisonError` failures in the rest of the module. Diagnose the first failure, not the cascade.

## Assertions

`validate_stow_result(path, expected_files)` walks the target tree recursively and asserts that every leaf
is a symlink present in `expected_files`, descending into real (non-symlink) directories. It verifies link *structure*,
not link targets or contents.
