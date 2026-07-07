# Graft

A Rust implementation of the GNU stow utility for managing dotfiles.

## Overview

`graft` is a symlink farm manager which takes separate packages of software and/or data located in separate directories
on the filesystem and makes them appear to be installed in the same place. It is primarily used for managing dotfiles,
allowing you to keep your configuration files in a central repository while symlinking them to their expected
locations (e.g., in your home directory).

### Features

- **Stow**: Create symlinks for a package.
- **Delete**: Remove symlinks for a package.
- **Restow**: Refresh symlinks (delete followed by stow).
- **List**: List all available packages and their stow status.
- **Simulation Mode**: See what would happen without making any changes.
- **Folding/Unfolding**: Supports directory folding similar to GNU Stow (can be disabled).
- **Ignore/Override**: Regex-based ignore and override patterns.
- **Customizable**: Configurable via TOML and ignore files.
- **Cross-Platform**: Supports Windows, macOS, and Linux.
- **Color Support**: Supports ANSI color codes for better readability.

## Setup

### Requirements

- **Rust**: Latest stable release

### From Crates IO

1. Install this package from creates IO by running the following command:

   ```bash
   cargo install graftfs --locked
   ```

2. The binary will now be in your path and you can run the command as follows:

   ```bash
   graft [OPTIONS] [COMMAND]
   ```

### From Source

1. **Clone the repository**:

   ```bash
   git clone https://github.com/ChrisTisdale/graftfs.git
   cd graftfs
   ```

2. **Build the project**:

   ```bash
   cargo build --release
   ```

3. The binary will be available at `target/release/graft`.

### Running with Cargo

You can also run the application directly using Cargo:

```bash
cargo run --package graftfs -- [OPTIONS] [COMMAND]
```

## Usage

```bash
graft [OPTIONS] [COMMAND]
```

## Tests

Run the test suite using Cargo:

```bash
cargo test
```

## Completions

Completions are available as an optional feature. To enable completions, run:

```bash
cargo install graftfs --locked --features completions
```

### Configuring completions

#### Fish

Configure fish completions by adding the following to your `~/.config/fish/config.fish` file:

```bash
graft completions fish | source
```

#### Zsh

Configure zsh completions by adding the following to your `~/.zshrc` file:

```bash
eval "$(graft completions zsh)"
```

#### Bash

Configure bash completions by adding the following to your `~/.bashrc` file:

```bash
eval "$(graft completions bash)"
```

#### PowerShell

Configure PowerShell completions by adding the following to your `$PROFILE` file:

```powershell
graft completions powershell | Out-String | Invoke-Expression
```

#### Nushell

Nushell support can be enabled by enabling the nushell feature flag. To enable nushell support you need to install graft
with the following command:

```bash
cargo install graftfs --feature nushell --locked
```

Configure Nushell completions by adding the following to your `~/.config/nushell/config.nu` file:

```nu
graft completions nu | save -f ($nu.default-config-dir | path join graft.nu)
```

Then in your config.nu

```nu
source graft.nu
```

## Configuration

`graft` can be configured using a `.graft.toml` file. It looks for this file in the current working directory or in the
user's configuration directory. The file is in TOML format.

Example `.graft.toml`:

```toml
version = 1

[ignored]
file = ".graft-ignore"
comment = '#'

[overrides]
file = ".graft-override"
comment = '#'

[logging]
level = "Info"
logging_path = "path/to/logs"
rotation = "Daily"
max_log_files = 30
color_support = true

[color]
enabled = true
link = "Green"
unlink = "Red"
list = "Cyan"
remove = "Red"
create = "Green"
arrow = "Blue"
source = "Yellow"
target = "Magenta"
```

### Version 1 Configuration Options

#### Ignored

- `file`: The name of the ignored file (default: `.graft-ignore`)
- `comment`: The comment character used in the ignored file (default: '#')

#### Overrides

- `file`: The name of the override file (default: `.graft-override`)
- `comment`: The comment character used in the override file (default: '#')

#### Logging

- `level`: The logging level (default: 'Warning')
- `logging_path`: The path to the log file. When no file is provided, the logging will output to sterr (default: None)
- `rotation`: The log rotation mode (default: None)
- `format`: The log format (default: 'Compact')
- `max_log_files`: The maximum number of log files to keep (default: None)
- `color_support`: Whether to enable color support in logs (default: True)

#### Color

- `enabled`: Whether to enable color support in logs (default: True)
- `link`: The color of the link text (default: 'Green').
    - Colors can be specified as either a string (e.g., 'Green') or a hex code (e.g., '#00FF00').
- `unlink`: The color of the unlink text (default: 'Red').
    - Colors can be specified as either a string (e.g., 'Red') or a hex code (e.g., '#FF0000').
- `list`: The color of the list text (default: 'Cyan')
    - Colors can be specified as either a string (e.g., 'Cyan') or a hex code (e.g., '#00FFFF').
- `remove`: The color of the remove text (default: 'Red')
    - Colors can be specified as either a string (e.g., 'Red') or a hex code (e.g., '#FF0000').
- `create`: The color of the create text (default: 'Green')
    - Colors can be specified as either a string (e.g., 'Green') or a hex code (e.g., '#00FF00').
- `arrow`: The color of the arrow text (default: 'Blue')
    - Colors can be specified as either a string (e.g., 'Blue') or a hex code (e.g., '#0000FF').
- `source`: The color of the source text (default: 'Yellow')
    - Colors can be specified as either a string (e.g., 'Yellow') or a hex code (e.g., '#FFFF00').
- `target`: The color of the target text (default: 'Magenta')
    - Colors can be specified as either a string (e.g., 'Magenta') or a hex code (e.g., '#FF00FF').

### Configuration Location

#### MacOS

On MacOS, the configuration files will be located in the following locations:

- IF GRAFT_CONFIG_DIR is set, `$GRAFT_CONFIG_DIR/.graft.toml`
- If XDG_CONFIG_HOME is set, `$XDG_CONFIG_HOME/graft/.graft.toml`
- If XDG_CONFIG_HOME is not set, `~/Library/Application Support/graft/.graft.toml`

#### Linux

On Linux, the configuration files will be located in the following locations:

- IF GRAFT_CONFIG_DIR is set, `$GRAFT_CONFIG_DIR/.graft.toml`
- If XDG_CONFIG_HOME is set, `$XDG_CONFIG_HOME/graft/.graft.toml`
- If XDG_CONFIG_HOME is not set, `~/.config/graft/.graft.toml`

#### Windows

On Windows, the configuration files will be located in the following locations:

- IF GRAFT_CONFIG_DIR is set, `%GRAFT_CONFIG_DIR%\.graft.toml`
- `%APPDATA%\graft\.graft.toml`

### Ignore Files

By default, `graft` looks for a `.graft-ignore` file to determine which files should be skipped during stowing. This
file supports regex patterns and comments starting with `#`.

## Notes

### Windows Support

For Windows, graftfs will require either enabling developer mode or running as an administrator. This is because graftfs
use symlinks which on Windows require admin privileges. For more information or how to not need admin access,
see [create-symbolic-links](https://learn.microsoft.com/en-us/previous-versions/windows/it-pro/windows-10/security/threat-protection/security-policy-settings/create-symbolic-links).
If you would like to enable developer mode,
see [enable-developer-mode](https://docs.microsoft.com/en-us/windows/apps/get-started/enable-your-device-for-development)
