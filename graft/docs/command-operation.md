# The `CommandOperation` seam

`src/commands/command_operation.rs`

Every filesystem effect in the crate goes through the `CommandOperation` trait:

| Method             | Kind     |
| ------------------ | -------- |
| `link_item`        | mutating |
| `remove_link`      | mutating |
| `remove_item`      | mutating |
| `create_directory` | mutating |
| `is_directory`     | query    |
| `is_file`          | query    |
| `is_symlink`       | query    |
| `read_link`        | query    |
| `exists`           | query    |
| `read_directory`   | query    |

`Command<TCommand: CommandOperation>` is generic over it, and the associated type `Item` is the iterator
`read_directory` returns (`Iterator<Item = Result<PathBuf, CommandError>>`).

**Adding a filesystem call anywhere other than through this trait breaks simulation mode and makes the code
untestable.** If you need a new effect, add a trait method and implement it in both variants below.

## The two implementations

`CommandOperationImpl` is a two-variant enum:

### `Default(Option<Box<ColorSupport>>)`

Real filesystem calls. The `ColorSupport` is `Some` only when `printing_enable` is set in config or the
relevant CLI flag is passed — that is how "do the work quietly" and "do the work and narrate it" are
distinguished.

`link_item` and `remove_link` are written twice, whole, under `#[cfg(unix)]` and
`#[cfg(target_os = "windows")]` — Windows needs `symlink_dir` vs `symlink_file` depending on the source,
and link removal differs. Changing the behavior of either means editing both copies; only one is compiled
per platform, so CI on the other platform is what catches a missed edit.

### `Simulated(Box<SimulatedData>)`

Backs `--simulate`. `SimulatedData` accumulates four vectors — `created_directories`,
`created_links`, `removed_links`, `removed_items` — instead of touching disk, and answers queries against
that accumulated state layered over the real filesystem. Two details that matter:

- Paths are resolved to absolute form against the current working directory as they are recorded
  (`SimulatedData::resolve_path`), so the summary is stable regardless of where the process was invoked.
- Recording a link cancels a matching pending removal. This is what makes a simulated `restow` report a
  no-op instead of "remove everything, then create everything".

**The summary is printed in `Drop`** (`impl Drop for CommandOperationImpl`), not at the end of
`execute()`. A `CommandOperationImpl::Simulated` that is leaked, forgotten, or dropped during unwinding
changes user-visible output.
