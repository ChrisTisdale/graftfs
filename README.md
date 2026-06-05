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

## Setup

### Requirements

- **Rust**: Latest stable release

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
```

### Version 1 Configuration Options

#### Ignored

- `file`: The name of the ignore file (default: `.graft-ignore`)
- `comment`: The comment character used in the ignore file (default: '#')

#### Overrides

- `file`: The name of the override file (default: `.graft-override`)
- `comment`: The comment character used in the override file (default: '#')

#### Logging

- `level`: The logging level (default: 'Warning')
- `logging_path`: The path to the log file. When no file is provided the logging will output to sterr (default: None)
- `rotation`: The log rotation mode (default: None)
- `format`: The log format (default: 'Compact')
- `max_log_files`: The maximum number of log files to keep (default: None)
- `color_support`: Whether to enable color support in logs (default: True)

### Configuration Location

#### MacOS

On MacOS, the configuration files will be located in the following locations:

- If XDG_CONFIG_HOME is set, `$XDG_CONFIG_HOME/graft/.graft.toml`
- If XDG_CONFIG_HOME is not set, `~/Library/Preferences/graft/.graft.toml`

#### Linux

On Linux, the configuration files will be located in the following locations:

- If XDG_CONFIG_HOME is set, `$XDG_CONFIG_HOME/graft/.graft.toml`
- If XDG_CONFIG_HOME is not set, `~/.config/graft/.graft.toml`

#### Windows

On Windows, the configuration files will be located in the following locations:

- `%APPDATA%\graft\.graft.toml`

### Ignore Files

By default, `graft` looks for a `.graft-ignore` file to determine which files should be skipped during stowing. This
file supports regex patterns and comments starting with `#`.
