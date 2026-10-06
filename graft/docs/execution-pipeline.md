# Execution pipeline

`main` → `CommandLineProcessor::get_cli_args()` → `Executor` dispatch (`Command::execute()` or helper action).

## Startup sequence

`src/command_line_args.rs` is the largest single concern in the crate and owns the whole startup sequence.
`CommandLineProcessor::get_cli_args()` in `src/command_line_args.rs` dispatches to a private constructor
per subcommand — `stow`, `delete`, `restow`, `list`, `completions`, `export_config`, `upgrade_config` — and
each one walks roughly the same path:

1. Parse arguments with clap (`try_parse`).
2. Resolve the source directory (required) and the target directory (defaulted if absent).
3. Locate the config file: `--config-file` if given, otherwise `.graft.toml` in the **source** directory.
   A missing file is not an error; `None` is passed through and defaults apply.
4. `AppConfiguration::load_configuration` merges config, ignore patterns, and override patterns.
5. Initialize `tracing` from the resolved logging config.
6. Build the command through `CommandBuilder`, choosing simulated or real execution.
7. Wrap the command and the logging guard in `CliArgs`.

`CommandLineProcessor::load_app_config` and `CommandLineProcessor::create_command` are the shared helpers.

## Command vs non-command execution via `Executor`

`completions`, `export-config`, and `upgrade-config` do not produce a filesystem `Command`. Instead,
`CommandLineProcessor::get_cli_args()` returns `CliArgs` wrapping an `Executor<T>` enum (`src/executor.rs`):

```rust
pub enum Executor<T: CommandOperation> {
    Command(Command<T>),
    Completion(CompletionPrinter),
    Export(ConfigPrinter),
    Upgrade(ConfigUpgrader),
}
```

`process_command_line_args` in `src/main.rs` matches on `executor`:

```rust
match executor {
    Executor::Command(cmd) => cmd.execute().context(CommandSnafu)?,
    Executor::Completion(cmd) => cmd.print_completions()?,
    Executor::Export(cmd) => cmd.print_config()?,
    Executor::Upgrade(cmd) => cmd.upgrade_config()?,
}
```

In `main()`, `CliError::CommandLineParsingError` is special-cased: it calls `source.exit()` so clap renders its
own help/usage and sets the exit code.

## The logging guard

`CliArgs` (`src/cli_args.rs`) holds the `tracing_appender::non_blocking::WorkerGuard` in a `_guard` field.
It must stay alive for the duration of execution — dropping `CliArgs` early silently discards buffered file
log output.

## Command dispatch

`Command<TCommand: CommandOperation>` in `src/commands/command.rs` is a four-variant enum — `Stow`,
`Unstow`, `Restow`, `List` — each wrapping a `CommandData<TData, TOperation>` pairing operation-specific data
with the operation implementation. `execute()` matches and delegates to `process_stow`, `process_unstow`,
`process_restow`, or `process_list`.

`Command::process_restow` is literally `Command::process_unstow` followed by `Command::process_stow`;
`RestowData` implements `AsRef` for both `UnstowData` and `StowData` so the same value feeds each phase.
