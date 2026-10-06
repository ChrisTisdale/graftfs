# Builders

`src/commands/command_builder.rs`

A `Command` is never constructed directly. `CommandBuilder<T>` is a two-stage chain: shared setup,
then a per-operation builder.

```rust
let command = CommandBuilder::<CommandOperationImpl>::new()
    .with_packages(vec![directory])
    .with_target(parent)
    .stow()          // -> StowCommandBuilder
    .build()?;
```

## Stage 1 — `CommandBuilder<T>`

Shared setup, all `#[must_use]` and consuming `self`:

- `with_target(PathBuf)`
- `with_packages(Vec<PathBuf>)`
- `with_dot_file_prefix(Option<String>)`

## Stage 2 — pick the operation

`.stow()`, `.unstow()`, `.restow()`, `.list()` convert into `StowCommandBuilder`, `UnstowCommandBuilder`,
`RestowCommandBuilder`, or `ListCommandBuilder`. `RestowCommandBuilder` simply wraps a
`StowCommandBuilder`, since restow needs the same inputs.

Each per-operation builder exposes its own options (ignore and override sets, folding, stow strategy, color
support) and a `build()` that returns `Result<Command<T>, CommandBuildError>`.

## Choosing the execution mode

`.simulate(color_support)` and `.command(Option<ColorSupport>)` both return a builder retyped to
`CommandBuilder<CommandOperationImpl>`, selecting the `Simulated` or `Default` variant described in
[`command-operation.md`](command-operation.md).

These exist on the shared `CommandBuilder` **and** on each per-operation builder (`StowCommandBuilder`,
`UnstowCommandBuilder`, `RestowCommandBuilder`, `ListCommandBuilder`), so the mode can be chosen before or after
picking the operation. When adding a new per-operation builder, provide `simulate` there too or the mode becomes
unreachable for that operation.

`CommandLineProcessor::create_command` in `src/command_line_args.rs` is the single place the CLI makes
this choice.

## Generic bound

The builders are implemented for `T: CommandOperation + Default`, which is what lets tests substitute a
different operation type without going through `CommandOperationImpl`.
