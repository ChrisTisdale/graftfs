/*
 * graftfs
 * Copyright (C) 2026 Chris Tisdale
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::CliError;
use crate::cli_args::CliArgs;
use crate::cli_errors::{
    CommandBuildSnafu, CommandLineParsingSnafu, GenerateCompletionsSnafu, InvalidConfigFileSnafu, InvalidPathSnafu,
    LoggingSnafu, OutputFileCreationSnafu, ResolveSnafu,
};
use crate::commands::{CommandBuilder, CommandOperationImpl};
use crate::config::{AppConfiguration, DEFAULT_CONFIG_FILE, LinkingStrategy, LoggingFormat, path_resolver};
use crate::shell::Shell;
use clap::builder::Styles;
use clap::error::ErrorKind;
use clap::{Args, CommandFactory, Parser, Subcommand, ValueHint};
use clap_complete::Generator;
use snafu::ResultExt;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};
use tracing::level_filters::LevelFilter;

const APP_NAME: &str = "graft";
const STYLES: Styles = Styles::styled();
const MISSING_DIRECTORY_ERROR: &str = r"Either the source directory or package name is required.  Please provide either of the following:
  --directory <DIRECTORY>
  --package <PACKAGE>";

#[derive(Args, Default, Debug, Clone, PartialEq, Eq)]
struct GlobalArgs {
    #[arg(
        long = "no-color",
        help = "Disable color output. This will prevent the application from using ANSI escape codes to format text and output.",
        action = clap::ArgAction::SetTrue,
        hide = true,
    )]
    no_color: bool,
    #[arg(
        short = 'c',
        long = "config",
        help = "Path to a custom configuration file. If not specified, graft looks for a '.graft.toml' file in the current working directory.",
        value_name = "FILE",
        value_hint = ValueHint::FilePath
    )]
    config_file: Option<PathBuf>,
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq)]
struct DirectoryArgs {
    #[arg(
        short = 'd',
        long = "directory",
        alias = "dir",
        visible_alias = "dir",
        help = "Specify the source directory (stow directory) containing the packages to be managed.",
        value_name = "DIRECTORY",
        value_hint = ValueHint::DirPath
    )]
    source: Option<PathBuf>,
    #[arg(
        short = 't',
        long = "target",
        help = "Specify the target directory where symbolic links will be created. By default, this is the parent of the current directory.",
        value_name = "DIRECTORY",
        value_hint = ValueHint::DirPath
    )]
    target: Option<PathBuf>,
    #[arg(
        long = "dotfiles",
        help = "Enable special handling for dotfiles by automatically renaming files with a specific prefix. For example, using the default 'dot-' prefix, a file named 'dot-bashrc' will be stowed as '.bashrc' in the target directory.",
        value_name = "PREFIX",
        default_missing_value = "dot-",
        num_args = 0..=1
    )]
    dotfiles: Option<String>,
    #[arg(
        short = 'p',
        long = "package",
        value_name = "PACKAGE",
        help = "Specify a package name to unstow. This can be used multiple times to specify multiple packages."
    )]
    packages: Vec<String>,
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq)]
struct LoggingArgs {
    #[arg(
        short = 'l',
        long = "log-level",
        help = "Set the application logging level. Supported levels are: Trace, Debug, Info, Warn, Error, or Off. This is primarily used for troubleshooting and debugging.",
        value_name = "LEVEL"
    )]
    log_level: Option<LevelFilter>,
    #[arg(
        long = "log-format",
        help = "Set the logging format. Supported formats are: Text, JSON, or Combined.",
        value_name = "FORMAT"
    )]
    log_format: Option<LoggingFormat>,
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq)]
struct StowArgs {
    #[arg(
        long = "no-folding",
        help = "Disable directory folding during stowing and refolding during deletion. Folding is a technique where a single symbolic link to a directory is used instead of individual links for each file within that directory."
    )]
    no_folding: bool,
    #[arg(
        short = 'i',
        long = "ignore",
        help = "Specify a file path or a regular expression pattern to exclude specific files or directories from being processed.",
        value_name = "PATTERN"
    )]
    ignored: Vec<String>,
    #[arg(
        short = 'o',
        long = "override",
        help = "Specify a file path or a regular expression pattern for files or directories that should be forcefully stowed, even if they would otherwise be ignored or causing conflicts.",
        value_name = "PATTERN"
    )]
    overrides: Vec<String>,
    #[clap(flatten)]
    logging: LoggingArgs,
    #[clap(flatten)]
    directory: DirectoryArgs,
    #[clap(flatten)]
    global: GlobalArgs,
    #[arg(
        short = 'n',
        long = "simulate",
        alias = "no",
        visible_alias = "no",
        help = "Perform a dry run of the operation. This will display the actions that would be taken without making any actual changes to the filesystem."
    )]
    simulate: bool,
    #[clap(
        short = 's',
        long = "linking-strategy",
        help = "Specify the linking strategy for stowing files."
    )]
    linking_strategy: Option<LinkingStrategy>,
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq)]
struct UnstowArgs {
    #[clap(flatten)]
    logging: LoggingArgs,
    #[clap(flatten)]
    directory: DirectoryArgs,
    #[clap(flatten)]
    global: GlobalArgs,
    #[arg(
        short = 'n',
        long = "simulate",
        alias = "no",
        visible_alias = "no",
        help = "Perform a dry run of the operation. This will display the actions that would be taken without making any actual changes to the filesystem."
    )]
    simulate: bool,
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq)]
struct CompletionArgs {
    #[arg(
        value_enum,
        help = "The shell for which completions should be generated.",
        value_name = "SHELL",
        required = false
    )]
    shell: Option<Shell>,
    #[arg(
        short = 'o',
        long = "output",
        help = "Specify the output file for the completion script. If not provided, the script will be printed to stdout.",
        value_name = "FILE",
        required = false
    )]
    output: Option<PathBuf>,
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq)]
struct ListArgs {
    #[clap(flatten)]
    logging: LoggingArgs,
    #[clap(flatten)]
    directory: DirectoryArgs,
    #[clap(flatten)]
    global: GlobalArgs,
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
enum ProcessCommands {
    #[command(
        short_flag = 'S',
        name = "stow",
        long_flag = "stow",
        flatten_help = true,
        about = "Stow packages into the target directory, creating symbolic links for each file."
    )]
    Stow(#[clap(flatten)] StowArgs),
    #[command(
        short_flag = 'D',
        name = "delete",
        long_flag = "delete",
        visible_long_flag_alias = "unstow",
        visible_alias = "unstow",
        long_flag_alias = "unstow",
        flatten_help = true,
        about = "Remove symbolic links from the target directory that belong to the specified packages. This is useful for cleaning up after a package is no longer needed."
    )]
    Delete(#[clap(flatten)] UnstowArgs),
    #[command(
        short_flag = 'R',
        name = "restow",
        long_flag = "restow",
        flatten_help = true,
        about = "Restow packages by first removing their existing symbolic links and then re-stowing them. This is equivalent to running 'delete' followed by 'stow', and is useful for updating links after package contents change."
    )]
    Restow(#[clap(flatten)] StowArgs),
    #[command(
        short_flag = 'L',
        name = "list",
        long_flag = "list",
        flatten_help = true,
        about = "List all packages in the source directory along with their stow status (stowed, unstowed, or partially stowed). This provides an overview of which packages are currently active in the target directory."
    )]
    List(#[clap(flatten)] ListArgs),
    #[command(
        name = "completions",
        long_flag = "completions",
        flatten_help = true,
        about = "Generate shell completions for the stow command. This is useful for enhancing the user experience by providing auto-completion suggestions for command-line arguments."
    )]
    Completions(#[clap(flatten)] CompletionArgs),
}

impl Display for ProcessCommands {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stow { .. } => f.write_str("Stow"),
            Self::Delete { .. } => f.write_str("Delete"),
            Self::Restow { .. } => f.write_str("Restow"),
            Self::List { .. } => f.write_str("List"),
            Self::Completions { .. } => f.write_str("Completions"),
        }
    }
}

#[derive(Parser)]
#[command(version, name = APP_NAME, about, author, propagate_version = true, styles = STYLES, help_template = "\
{before-help}{name} {version}: {author-with-newline}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
")]
#[clap(rename_all = "snake_case")]
pub struct CommandLineProcessor {
    #[clap(subcommand)]
    process_command: ProcessCommands,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct CompletionPrinter {
    shell: Shell,
    output: Option<PathBuf>,
}

impl Display for CompletionPrinter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.shell.to_string())
    }
}

impl CompletionPrinter {
    const fn new(shell: Shell, output: Option<PathBuf>) -> Self {
        Self { shell, output }
    }

    /// Prints completions for the given shell.
    ///
    /// This function generates completions for the specified shell and exits the program and will exit with a zero status code.
    ///
    /// # Errors
    /// - Returns a `CliError` if the output file cannot be created.
    ///
    /// Returns an error if the output file cannot be created.
    pub fn print_completions(&self) -> Result<(), CliError> {
        if let Some(path) = &self.output {
            let mut file = File::create(path).with_context(|_| OutputFileCreationSnafu {
                path: path.display().to_string(),
            })?;

            self.generate_completions(&mut file)?;
        } else {
            self.generate_completions(&mut std::io::stdout())?;
        }

        std::process::exit(0);
    }

    fn generate_completions<T: Write>(&self, writer: &mut T) -> Result<(), CliError> {
        let mut command = CommandLineProcessor::command();
        command.set_bin_name(APP_NAME);
        command.build();
        self.shell
            .try_generate(&command, writer)
            .context(GenerateCompletionsSnafu)
    }
}

impl CommandLineProcessor {
    /// Parses and processes command-line arguments to configure the CLI application.
    ///
    /// This function performs the following steps:
    /// 1. Parses the command-line arguments into a `CliArgs` structure using `try_parse()`.
    /// 2. Resolves the `directory` argument, which is mandatory, and throws an error if it is missing.
    /// 3. Determines the configuration file path from the provided `config_file` argument, or defaults to `DEFAULT_CONFIG_FILE` under the resolved directory.
    /// 4. Checks if the configuration file exists and sets it up accordingly. If it doesn't exist, an optional `None` is used.
    /// 5. Loads the application configuration using `AppConfiguration::load_configuration`, which also incorporates ignored items.
    /// 6. Sets up the logger using the logging level specified by the global arguments.
    /// 7. Resolves the `target` argument, falling back to a default target if none is provided.
    /// 8. Configures the command builder with the parsed `directory` and `target`.
    ///     - If the `simulate` mode is enabled in the global arguments, simulation behavior is applied to the builder.
    ///     - Otherwise, the real command execution mode is applied.
    /// 9. Constructs the specific command to execute based on the parsed subcommand:
    ///     - `ProcessCommands::Stow`: Constructs a "stow" command with ignored items and folding settings.
    ///     - `ProcessCommands::Delete`: Constructs an "unstow" command.
    ///     - `ProcessCommands::Restow`: Constructs a "restow" command with ignored items and folding settings.
    /// 10. Wraps the constructed command along with the logger guard into a `CliArgs` structure and returns it.
    ///
    /// # Errors
    /// - Returns a `CliError` if any mandatory argument is missing or if there are issues resolving paths, loading configuration, or building commands.
    ///
    /// # Returns
    /// On successful processing of the CLI arguments, a `Result<CliArgs<CommandOperationImpl>, CliError>` structure is returned, containing:
    /// - The prepared command to execute.
    /// - A guard for resource cleanup (e.g., the logger setup).
    ///
    /// # Example
    /// ```
    /// use graft::command_line_args::{CommandLineProcessor};
    /// use graft::cli_args::CliArgs;
    ///
    /// match CommandLineProcessor::get_cli_args() {
    ///     Ok(cli_args) => {
    ///         println!("Command setup successfully: {}", cli_args.command);
    ///     }
    ///     Err(e) => {
    ///         eprintln!("Failed to parse command-line arguments: {e}");
    ///     }
    /// }
    /// ```
    pub fn get_cli_args() -> Result<CliArgs<CommandOperationImpl>, CliError> {
        let cli_args = Self::try_parse().context(CommandLineParsingSnafu)?;
        match cli_args.process_command {
            ProcessCommands::Stow(stow_args) => Self::stow(stow_args),
            ProcessCommands::Delete(unstow_args) => Self::delete(unstow_args),
            ProcessCommands::Restow(stow_args) => Self::restow(stow_args),
            ProcessCommands::List(list_args) => Self::list(list_args),
            ProcessCommands::Completions(completion_args) => Self::completions(completion_args),
        }
    }

    fn load_app_config(
        directory: &Path,
        global: &GlobalArgs,
        ignored: HashSet<String>,
        overrides: HashSet<String>,
    ) -> Result<AppConfiguration, CliError> {
        let config_file = match &global.config_file {
            Some(c) => Self::resolve_config_file(c)?,
            None => directory.join(DEFAULT_CONFIG_FILE),
        };

        let config_file_path = if fs::exists(&config_file).unwrap_or(false) {
            Some(config_file.as_path())
        } else {
            None
        };

        let app_config = AppConfiguration::load_configuration(
            config_file_path,
            directory,
            ignored,
            overrides,
            global.no_color,
        )
        .with_context(|_| InvalidConfigFileSnafu {
            file: config_file_path.map_or_else(
                || DEFAULT_CONFIG_FILE.to_string(),
                |p| p.display().to_string(),
            ),
        })?;

        Ok(app_config)
    }

    fn create_command(simulated: bool, app_config: &AppConfiguration) -> CommandBuilder<CommandOperationImpl> {
        if simulated {
            CommandBuilder::<CommandOperationImpl>::new().simulate(app_config.color_support())
        } else {
            CommandBuilder::new()
        }
    }

    fn get_source(directory_args: &DirectoryArgs) -> Result<PathBuf, CliError> {
        if directory_args.packages.is_empty() {
            directory_args.source.as_ref().map_or_else(
                || {
                    Err(Self::command().error(ErrorKind::MissingRequiredArgument, MISSING_DIRECTORY_ERROR))
                        .context(CommandLineParsingSnafu)?
                },
                |d| {
                    path_resolver::resolve_path(d).with_context(|_| ResolveSnafu {
                        file: d.display().to_string(),
                    })
                },
            )
        } else {
            directory_args
                .source
                .as_ref()
                .map_or_else(Self::get_default_source, |d| {
                    path_resolver::resolve_path(d).with_context(|_| ResolveSnafu {
                        file: d.display().to_string(),
                    })
                })
        }
    }

    fn get_target(directory_args: &DirectoryArgs) -> Result<PathBuf, CliError> {
        let target = directory_args
            .target
            .as_deref()
            .map_or_else(Self::get_default_target, |p| {
                path_resolver::resolve_path(p).with_context(|_| ResolveSnafu {
                    file: p.display().to_string(),
                })
            })?;

        Ok(target)
    }

    fn get_package_directories(source: &Path, packages: &[String]) -> Result<Vec<PathBuf>, CliError> {
        if packages.is_empty() {
            return Ok(vec![source.to_path_buf()]);
        }

        let mut package_directories = Vec::with_capacity(packages.len());
        for package in packages {
            let package_directory = source
                .join(package)
                .canonicalize()
                .with_context(|_| InvalidPathSnafu { path: package })?;
            package_directories.push(package_directory);
        }

        Ok(package_directories)
    }

    fn stow(stow_args: StowArgs) -> Result<CliArgs<CommandOperationImpl>, CliError> {
        let directory = Self::get_source(&stow_args.directory)?;
        let target = Self::get_target(&stow_args.directory)?;
        let app_config = Self::load_app_config(
            &directory,
            &stow_args.global,
            stow_args.ignored.into_iter().collect(),
            stow_args.overrides.into_iter().collect(),
        )?;

        let guard = app_config
            .setup_logger(stow_args.logging.log_level, stow_args.logging.log_format)
            .context(LoggingSnafu)?;

        let linking_strategy = stow_args
            .linking_strategy
            .unwrap_or_else(|| app_config.linking_strategy());

        let packages = Self::get_package_directories(&directory, &stow_args.directory.packages)?;
        let command = Self::create_command(stow_args.simulate, &app_config)
            .stow()
            .with_dot_file_prefix(stow_args.directory.dotfiles)
            .with_ignored(app_config.ignored)
            .with_no_folding(stow_args.no_folding)
            .with_overrides(app_config.overrides)
            .with_target(target)
            .with_packages(packages)
            .with_linking_strategy(linking_strategy)
            .build()
            .with_context(|_| CommandBuildSnafu { command: "Stow" })?;

        Ok(CliArgs::new(command, guard))
    }

    fn delete(unstow_args: UnstowArgs) -> Result<CliArgs<CommandOperationImpl>, CliError> {
        let directory = Self::get_source(&unstow_args.directory)?;
        let target = Self::get_target(&unstow_args.directory)?;
        let app_config = Self::load_app_config(
            &directory,
            &unstow_args.global,
            HashSet::new(),
            HashSet::new(),
        )?;

        let guard = app_config
            .setup_logger(
                unstow_args.logging.log_level,
                unstow_args.logging.log_format,
            )
            .context(LoggingSnafu)?;

        let packages = Self::get_package_directories(&directory, &unstow_args.directory.packages)?;
        let command = Self::create_command(unstow_args.simulate, &app_config)
            .unstow()
            .with_dot_file_prefix(unstow_args.directory.dotfiles)
            .with_target(target)
            .with_packages(packages)
            .build()
            .with_context(|_| CommandBuildSnafu { command: "Unstow" })?;

        Ok(CliArgs::new(command, guard))
    }

    fn restow(stow_args: StowArgs) -> Result<CliArgs<CommandOperationImpl>, CliError> {
        let directory = Self::get_source(&stow_args.directory)?;
        let target = Self::get_target(&stow_args.directory)?;
        let app_config = Self::load_app_config(
            &directory,
            &stow_args.global,
            stow_args.ignored.into_iter().collect(),
            stow_args.overrides.into_iter().collect(),
        )?;

        let guard = app_config
            .setup_logger(stow_args.logging.log_level, stow_args.logging.log_format)
            .context(LoggingSnafu)?;

        let linking_strategy = stow_args
            .linking_strategy
            .unwrap_or_else(|| app_config.linking_strategy());

        let packages = Self::get_package_directories(&directory, &stow_args.directory.packages)?;
        let command = Self::create_command(stow_args.simulate, &app_config)
            .restow()
            .with_dot_file_prefix(stow_args.directory.dotfiles)
            .with_ignored(app_config.ignored)
            .with_no_folding(stow_args.no_folding)
            .with_overrides(app_config.overrides)
            .with_target(target)
            .with_packages(packages)
            .with_linking_strategy(linking_strategy)
            .build()
            .with_context(|_| CommandBuildSnafu { command: "Restow" })?;

        Ok(CliArgs::new(command, guard))
    }

    fn list(list_args: ListArgs) -> Result<CliArgs<CommandOperationImpl>, CliError> {
        let directory = Self::get_source(&list_args.directory)?;
        let target = Self::get_target(&list_args.directory)?;
        let app_config = Self::load_app_config(
            &directory,
            &list_args.global,
            HashSet::new(),
            HashSet::new(),
        )?;

        let guard = app_config
            .setup_logger(list_args.logging.log_level, list_args.logging.log_format)
            .context(LoggingSnafu)?;

        let packages = Self::get_package_directories(&directory, &list_args.directory.packages)?;
        let command = CommandBuilder::new()
            .list()
            .with_target(target)
            .with_packages(packages)
            .with_color_support(app_config.color_support())
            .with_dot_file_prefix(list_args.directory.dotfiles)
            .build()
            .with_context(|_| CommandBuildSnafu { command: "List" })?;
        Ok(CliArgs::new(command, guard))
    }

    fn completions(completion_args: CompletionArgs) -> Result<CliArgs<CommandOperationImpl>, CliError> {
        let shell = completion_args
            .shell
            .map_or_else(|| Shell::from_env().ok_or(CliError::InvalidShell), Ok)?;

        Err(CliError::PrintCompletions {
            printer: CompletionPrinter::new(shell, completion_args.output),
        })
    }

    fn resolve_config_file(path: &Path) -> Result<PathBuf, CliError> {
        fs::metadata(path)
            .map(|m| m.is_file())
            .map_err(|_| CliError::InvalidConfigurationFile {
                file: path.display().to_string(),
            })
            .and_then(|is_file| {
                if is_file {
                    Ok(path.to_path_buf())
                } else {
                    Err(CliError::InvalidConfigurationFile {
                        file: path.display().to_string(),
                    })
                }
            })
    }

    fn get_default_source() -> Result<PathBuf, CliError> {
        let current_dir = env::current_dir().with_context(|_| InvalidPathSnafu {
            path: String::new(),
        })?;

        let current_dir = current_dir
            .canonicalize()
            .with_context(|_| InvalidPathSnafu {
                path: current_dir.display().to_string(),
            })?;

        Ok(current_dir)
    }

    fn get_default_target() -> Result<PathBuf, CliError> {
        let current_dir = env::current_dir().with_context(|_| InvalidPathSnafu {
            path: String::new(),
        })?;

        current_dir.parent().map_or_else(
            || Err(CliError::InvalidTargetDirectory),
            |p| {
                p.canonicalize().with_context(|_| InvalidPathSnafu {
                    path: p.display().to_string(),
                })
            },
        )
    }
}
