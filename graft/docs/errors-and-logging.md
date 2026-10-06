# Errors and logging

## Errors: `snafu`

One error enum per file, named `*_error.rs`, exporting a single type through the module's `mod.rs`:

```
cli_errors.rs                  -> CliError
shell_converter_error.rs       -> ShellConverterError
commands/command_error.rs      -> CommandError
commands/command_build_error.rs-> CommandBuildError
commands/matcher_error.rs      -> MatchError
config/config_error.rs         -> ConfigError
config/version_error.rs        -> VersionError
config/resolve_error.rs        -> ResolveError
config/logging_error.rs        -> LoggingError
config/level_error.rs          -> LevelError
config/format_error.rs         -> FormatError
config/rotation_error.rs       -> RotationError
config/linking_strategy_error.rs, matching_strategy_error.rs, regex_strategy_error.rs
config/console_logging_stream_error.rs
```

Follow that pattern for new errors: new file, one enum, re-export from the parent `mod.rs`.

Snafu generates a context selector per variant, suffixed `Snafu`, applied at the call site:

```rust
fs::read_to_string(path).with_context(|_| FileReadSnafu { file: path.display().to_string() })?;
```

Use `.context(..)` when the context struct needs no computation and `.with_context(|_| ..)` when it does —
the latter avoids formatting a path on the success path. Both forms are used throughout; match the
surrounding code.

`main` in `src/main.rs` is annotated `#[snafu::report]`, which renders the full error chain on exit.
`CliError::CommandLineParsingError` is intercepted in `main` to let clap handle help rendering and exit codes (`source.exit()`). All other errors propagate to `#[snafu::report]`. See
[`execution-pipeline.md`](execution-pipeline.md).

## Logging: `tracing`

`tracing` + `tracing-subscriber` with the `json` feature. Command-processing functions carry
`#[instrument(level = "debug", skip_all)]` — `skip_all` because the arguments are large path collections.

`LoggingConfig` (`src/config/logging_config.rs`) resolves four independent settings, each with its own
error type: `level`, `stream` (`stdout`/`stderr`), `format` (`compact`/`pretty`/`json`), and rotation
(`daily`/`hourly`, plus `max_log_files`).

File logging uses `tracing-appender`'s non-blocking writer. Its `WorkerGuard` is owned by `CliArgs` and
must outlive execution or buffered output is dropped — see
[`execution-pipeline.md`](execution-pipeline.md).

## Logging vs. user output

They are separate channels. `tracing` is diagnostics. User-facing output — the `link`/`unlink`/`create`
lines and the simulation summary — goes through `ColorSupport` (`src/commands/color_support.rs`), driven by
`printing_enable` and the color config. Do not use `println!` directly, and do not use `info!` to tell the
user something.
