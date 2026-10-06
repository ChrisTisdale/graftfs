# Configuration

`src/config/`

`README.md` documents every setting from a user's perspective. This describes how loading works.

## Sources, in precedence order

1. CLI arguments
2. Project-local `.graft.toml` (`DEFAULT_CONFIG_FILE`) — looked up in the **source** directory, not the
   current working directory, despite what the README implies
3. Global `config.toml` (`GLOBAL_CONFIG_FILE`) in the platform config directory
4. Built-in defaults

A missing config file at any level is not an error.

## Platform paths

`AppDirectories::load_directories()` in `src/config/app_directories.rs` resolves two directories:

**Config** — `GRAFT_CONFIG_DIR` wins outright; otherwise `XDG_CONFIG_HOME` (unix) or `APPDATA` (Windows);
otherwise a platform default (`~/Library/Application Support/`, `~/.config/`, `~\AppData\Roaming`). The
app name `graft` is appended to all but the `GRAFT_CONFIG_DIR` case.

**Logs** — `XDG_DATA_HOME` (unix) or `LOCALAPPDATA` (Windows), else `~/.local/share/`,
`~/Library/Application Support/`, or `~\AppData\Local`.

Path expansion for `~` goes through `config::path_resolver`, which canonicalizes and therefore requires the
path to exist.

## The file is versioned

`ConfigFileVersion` is `V1 = 1` or `V2 = 2`, with **V2 as the default**. It deserializes from either an
integer or a string (`"1"`, `"v1"`, `"V1"`, …) via a hand-written `Visitor`.

`Config::read_config_file` in `src/config/mod.rs` does a two-pass read:

1. Parse the TOML into a `toml::de::DeTable`.
2. Peek the version with `Config::get_version`.
3. Deserialize into `V1Config` or `Config` accordingly, then convert `V1Config -> Config` via `From`.

Only `color` differs between the versions (`V1ColorConfig` vs `ColorConfig`); every other section is shared.

**When adding a config field:** decide whether it is V2-only, and keep `impl From<V1Config> for Config`
total — it must produce a sensible value for a field that did not exist in V1. `graft upgrade-config`
(`Config::upgrade_config`) rewrites a V1 file to V2 and depends on that conversion being correct.

## `AppConfiguration`

`AppConfiguration::load_configuration` in `src/config/app_configuration.rs` is the merge point. It:

- Resolves relative `ignored.file`, `overrides.file`, and `logging.logging_path` against the search path
- Reads pattern files (`.graft-ignore`, `.graft-override`) honoring their configured comment character
- Unions those with the CLI-supplied sets and with `DEFAULT_IGNORE` — VCS metadata (`.git`, `.jj`,
  `.hg`, `.svn`, CVS, RCS, darcs), editor backups (`.+~`, `#...#`), and `README*`, `LICENSE*`, `COPYING`,
  `.DS_Store`

Patterns are `HashSet<String>` and are compiled later — see [`regex-matching.md`](regex-matching.md).

## Behavioral settings

`StowConfig` (`src/config/stow_config.rs`) carries the knobs that change what stowing does:

| Setting            | Values                   |
| ------------------ | ------------------------ |
| `linking_strategy` | `short`, `full`          |
| `regex_strategy`   | `rust`, `pcre2`          |
| `matching_strategy`| `combined`, `individual` |
| `printing_enable`  | bool                     |

Each has a dedicated error type for parse failures (`linking_strategy_error.rs`, etc.).
