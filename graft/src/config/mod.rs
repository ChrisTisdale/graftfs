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

mod app_configuration;
mod app_directories;
mod color_config;
mod config_error;
mod config_file_version;
mod console_logging_stream_error;
mod format_error;
mod ignored;
mod level_error;
mod linking_strategy_error;
mod logging_config;
mod logging_error;
mod matching_strategy_error;
mod overrides;
pub mod path_resolver;
mod regex_strategy_error;
mod resolve_error;
mod rotation_error;
mod stow_config;
mod version_error;

use crate::config::color_config::V1ColorConfig;
use crate::config::config_error::{
    FileReadSnafu, MissingConfigFileSnafu, ResolveSnafu, TomlSnafu, TomlWriteSnafu, WriteSnafu,
};
pub use app_configuration::{AppConfiguration, DEFAULT_CONFIG_FILE, GLOBAL_CONFIG_FILE};
pub use app_directories::AppDirectories;
pub use color_config::{ColorConfig, ColorSettings};
pub use config_error::ConfigError;
pub use config_file_version::ConfigFileVersion;
pub use ignored::Ignored;
pub use level_error::LevelError;
pub use linking_strategy_error::LinkingStrategyError;
pub use logging_config::{ConsoleLoggingStream, LoggingConfig, LoggingFormat, LoggingLevel};
pub use logging_error::LoggingError;
pub use overrides::Overrides;
pub use regex_strategy_error::RegexStrategyError;
pub use resolve_error::ResolveError;
use serde::de::IntoDeserializer;
use serde::{Deserialize, Serialize};
use snafu::{OptionExt, ResultExt};
use std::fmt::Display;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};
pub use stow_config::{LinkingStrategy, MatchingStrategy, RegexStrategy, StowConfig};
use toml::de::DeTable;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Config {
    #[serde(default)]
    pub version: ConfigFileVersion,
    #[serde(default)]
    pub ignored: Ignored,
    #[serde(default)]
    pub logging: LoggingConfig,
    #[serde(default)]
    pub overrides: Overrides,
    #[serde(default)]
    pub color: ColorConfig,
    #[serde(default)]
    pub stow: StowConfig,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
struct V1Config {
    #[serde(default)]
    pub version: ConfigFileVersion,
    #[serde(default)]
    pub ignored: Ignored,
    #[serde(default)]
    pub logging: LoggingConfig,
    #[serde(default)]
    pub overrides: Overrides,
    #[serde(default)]
    pub color: V1ColorConfig,
    #[serde(default)]
    pub stow: StowConfig,
}

impl From<V1Config> for Config {
    fn from(value: V1Config) -> Self {
        Self {
            version: ConfigFileVersion::V2,
            ignored: value.ignored,
            logging: value.logging,
            overrides: value.overrides,
            color: value.color.into(),
            stow: value.stow,
        }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Config {{ version: {}, ignored: {}, logging: {}, overrides: {}, color: {} }}",
            self.version, self.ignored, self.logging, self.overrides, self.color
        )
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: ConfigFileVersion::V2,
            ignored: Ignored::default(),
            logging: LoggingConfig::default(),
            overrides: Overrides::default(),
            color: ColorConfig::default(),
            stow: StowConfig::default(),
        }
    }
}

impl Default for V1Config {
    fn default() -> Self {
        Self {
            version: ConfigFileVersion::V1,
            ignored: Ignored::default(),
            logging: LoggingConfig::default(),
            overrides: Overrides::default(),
            color: V1ColorConfig::default(),
            stow: StowConfig::default(),
        }
    }
}

trait ConfigWriter {
    fn write_config<T: Write>(&self, writer: &mut T) -> Result<(), ConfigError>
    where
        Self: Serialize,
    {
        let content = toml::to_string_pretty(self).context(TomlWriteSnafu)?;
        writer.write_all(content.as_bytes()).context(WriteSnafu)?;
        Ok(())
    }
}

impl ConfigWriter for V1Config {}

impl ConfigWriter for Config {}

impl Config {
    /// Creates a new `Config` object from a TOML file.
    ///
    /// # Arguments
    ///
    /// * `file_path`: The path to the configuration file. If not provided, the default configuration file will be used.
    ///
    /// returns: Result<Config, `ConfigError`> - The configuration object.
    ///
    /// # Errors
    ///
    /// - `ConfigError::UnableToFindHomeDirectory`: This indicates that the home directory could not be found.
    /// - `ConfigError::UnableToResolvePath`: This indicates that the provided path could not be resolved.
    /// - `ConfigError::InvalidLoggingPath`: This indicates that the logging path is invalid.
    /// - `ConfigError::InvalidLoggingFormat`: This indicates that the logging format is invalid.
    /// - `ConfigError::InvalidLoggingRotation`: This indicates that the logging rotation is invalid.
    /// - `ConfigError::InvalidLoggingLevel`: This indicates that the logging level is invalid.
    /// - `ConfigError::InvalidColorSetting`: This indicates that the color setting is invalid.
    /// - `ConfigError::InvalidColorSupport`: This indicates that the color support is invalid.
    /// - `ConfigError::InvalidMaxLogFiles`: This indicates that the maximum number of log files is invalid.
    /// - `ConfigError::InvalidIgnoredFile`: This indicates that the ignored file is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use graft::config::Config;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = Config::from_file(None)?;
    ///
    ///     println!("Config: {}", config);
    ///     Ok(())
    /// }
    /// ```
    pub fn from_file(file_path: Option<&Path>) -> Result<Self, ConfigError> {
        let app_directories = AppDirectories::load_directories();
        if let Some(file_path) = file_path {
            return Self::read_config_file(file_path, &app_directories);
        }

        let config_file = Self::get_global_config_file(&app_directories);
        if let Some(config_file) = config_file
            && let Ok(config_file) = path_resolver::resolve_path(&config_file)
            && fs::exists(&config_file).unwrap_or(false)
        {
            return Self::read_config_file(&config_file, &app_directories);
        }

        Ok(Self::default())
    }

    /// Prints the configuration to the provided writer. It can print configuration
    /// from a specified file or the default configuration if no file is provided.
    ///
    /// # Generic Parameters
    /// - `T`: A type that implements the `Write` trait, used to write the
    ///   configuration output.
    ///
    /// # Parameters
    /// - `file_path`: An optional file path to the configuration file that should be printed.
    ///   If `None`, the global configuration file or the default configuration is used.
    /// - `upgrade`: A flag indicating whether to apply upgrades or changes to the configuration
    ///   before printing.
    /// - `writer`: A mutable reference to an object implementing the `Write` trait,
    ///   where the configuration output will be written.
    ///
    /// # Returns
    /// - `Ok(())`: If the configuration is successfully printed.
    /// - `Err(ConfigError)`: If an error occurs while attempting to load or write the configuration.
    ///
    /// # Behavior
    /// - If `file_path` is `Some`, it attempts to resolve and print the configuration
    ///   from the specified path.
    /// - If `file_path` is `None`, it tries to locate and resolve the global configuration file:
    ///     - If the global configuration file exists, it is printed.
    ///     - If not, the default configuration is used and printed.
    /// - The function outputs the configuration using the `writer`.
    ///
    /// # Errors
    /// - Returns an error if there is a failure in resolving the configuration path,
    ///   reading the configuration file, or writing to the provided writer.
    ///
    /// # Example
    /// ```rust
    /// use std::io::stdout;
    /// use graft::config::Config;
    ///
    /// let mut output = stdout();
    /// match Config::print_config(None, false, &mut output) {
    ///     Ok(()) => println!("Configuration printed successfully"),
    ///     Err(e) => eprintln!("Failed to print configuration: {:?}", e),
    /// }
    /// ```
    pub fn print_config<T: Write>(file_path: Option<&Path>, upgrade: bool, writer: &mut T) -> Result<(), ConfigError> {
        if let Some(file_path) = file_path {
            Self::print_config_file(file_path, upgrade, writer)?;
            return Ok(());
        }

        let app_directories = AppDirectories::load_directories();
        let config_file = Self::get_global_config_file(&app_directories);
        if let Some(config_file) = config_file
            && let Ok(config_file) = path_resolver::resolve_path(&config_file)
            && fs::exists(&config_file).unwrap_or(false)
        {
            Self::print_config_file(&config_file, upgrade, writer)?;
            return Ok(());
        }

        let config = Self::default();
        config.write_config(writer)?;
        Ok(())
    }

    /// Upgrades a configuration file to the latest version, if needed, and writes the updated
    /// configuration back to the same file.
    ///
    /// # Arguments
    /// * `file_path` - A reference to the path of the configuration file to be upgraded.
    /// * `output` - A reference to the path where the upgraded configuration should be written.
    ///
    /// # Returns
    /// * `Result<(), ConfigError>` - Returns `Ok(())` on successful upgrade or if the file is
    ///   already at the desired version. Returns a `ConfigError` if any error occurs during the process.
    ///
    /// # Behavior
    /// 1. Reads the content of the configuration file at the given `file_path`.
    /// 2. Determines the version of the configuration file using the `get_version` method.
    /// 3. If the configuration file is already at `ConfigFileVersion::V2`, the function exits early with `Ok(())`.
    /// 4. If the version is `ConfigFileVersion::V1`, parses the file content into a `V1Config` struct.
    /// 5. Converts the `V1Config` instance to the updated format.
    /// 6. Serializes the updated configuration into TOML format using `toml::to_string_pretty`.
    /// 7. Writes the upgraded TOML content back to the original file path.
    ///
    /// # Errors
    /// This function may return the following errors:
    /// * `FileReadSnafu` - If there is an error reading the file from the specified `file_path`.
    /// * `TomlSnafu` - If there is an error parsing the configuration into or out of the TOML format.
    /// * `WriteSnafu` - If there is an error writing the upgraded configuration back to the file.
    pub fn upgrade_config(file_path: Option<PathBuf>, output: Option<PathBuf>) -> Result<(), ConfigError> {
        let app_directories = AppDirectories::load_directories();
        let file_path = file_path
            .map_or_else(|| Self::get_global_config_file(&app_directories), Some)
            .context(MissingConfigFileSnafu)?;

        let content = fs::read_to_string(&file_path).with_context(|_| FileReadSnafu {
            file: file_path.display().to_string(),
        })?;

        let table = DeTable::parse(&content).context(TomlSnafu {
            file: file_path.display().to_string(),
        })?;

        let version = Self::get_version(table.as_ref());
        if version == ConfigFileVersion::V2 {
            return Ok(());
        }

        let config = V1Config::deserialize(table.into_deserializer()).with_context(|_| TomlSnafu {
            file: file_path.display().to_string(),
        })?;

        let config: Self = config.into();
        let content = toml::to_string_pretty(&config).context(TomlWriteSnafu)?;
        fs::write(output.unwrap_or(file_path), content).context(WriteSnafu)?;

        Ok(())
    }

    fn get_global_config_file(app_directories: &AppDirectories) -> Option<PathBuf> {
        let resolved = path_resolver::resolve_path(&app_directories.config_dir);
        if let Ok(resolved) = resolved
            && fs::exists(&resolved).unwrap_or(false)
        {
            let config_file = resolved.join(GLOBAL_CONFIG_FILE);
            if fs::exists(&config_file).unwrap_or(false) {
                return Some(config_file);
            }

            let config_file = resolved.join(DEFAULT_CONFIG_FILE);
            if fs::exists(&config_file).unwrap_or(false) {
                return Some(config_file);
            }
        }

        None
    }

    fn get_version(content: &DeTable) -> ConfigFileVersion {
        content
            .get_key_value("version")
            .and_then(|v| ConfigFileVersion::deserialize(v.1.to_owned().into_deserializer()).ok())
            .unwrap_or(ConfigFileVersion::V2)
    }

    fn print_config_file<T: Write>(file_path: &Path, upgrade: bool, writer: &mut T) -> Result<(), ConfigError> {
        let content = fs::read_to_string(file_path).with_context(|_| FileReadSnafu {
            file: file_path.display().to_string(),
        })?;

        let table = DeTable::parse(&content).context(TomlSnafu {
            file: file_path.display().to_string(),
        })?;

        let version = Self::get_version(table.as_ref());
        match version {
            ConfigFileVersion::V1 => {
                let config = V1Config::deserialize(table.into_deserializer()).with_context(|_| TomlSnafu {
                    file: file_path.display().to_string(),
                })?;

                if upgrade {
                    let config: Self = config.into();
                    config.write_config(writer)
                } else {
                    config.write_config(writer)
                }
            }
            ConfigFileVersion::V2 => {
                let config = Self::deserialize(table.into_deserializer()).with_context(|_| TomlSnafu {
                    file: file_path.display().to_string(),
                })?;

                config.write_config(writer)
            }
        }
    }

    fn read_config_file(file_path: &Path, app_directories: &AppDirectories) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(file_path).with_context(|_| FileReadSnafu {
            file: file_path.display().to_string(),
        })?;

        let table = DeTable::parse(&content).context(TomlSnafu {
            file: file_path.display().to_string(),
        })?;

        let version = Self::get_version(table.as_ref());
        let mut config = match version {
            ConfigFileVersion::V1 => {
                let config = V1Config::deserialize(table.into_deserializer()).with_context(|_| TomlSnafu {
                    file: file_path.display().to_string(),
                })?;

                Ok(config.into())
            }
            ConfigFileVersion::V2 => Self::deserialize(table.into_deserializer()).with_context(|_| TomlSnafu {
                file: file_path.display().to_string(),
            }),
        }?;

        let home_dir = env::home_dir().ok_or(ConfigError::UnableToFindHomeDirectory)?;
        config.ignored.file =
            path_resolver::resolve_home_path(&config.ignored.file).with_context(|_| ResolveSnafu {
                file: file_path.display().to_string(),
            })?;

        if config.ignored.file.is_relative() {
            let parent_dir = file_path.parent().unwrap_or(home_dir.as_path());
            config.ignored.file = parent_dir.join(config.ignored.file);
        }

        if let Some(file) = config.ignored.file.to_str() {
            let path = Path::new(file);
            config.ignored.file = path_resolver::resolve_home_path(path).with_context(|_| ResolveSnafu {
                file: path.display().to_string(),
            })?;
        }

        config.logging.logging_path = if let Some(path) = config.logging.logging_path {
            Some(
                path_resolver::resolve_home_path(&path).with_context(|_| ResolveSnafu {
                    file: path.display().to_string(),
                })?,
            )
        } else {
            Some(
                path_resolver::resolve_home_path(&app_directories.log_dir).with_context(|_| ResolveSnafu {
                    file: app_directories.log_dir.display().to_string(),
                })?,
            )
        };

        if let Some(path) = &config.logging.logging_path
            && path.is_relative()
        {
            let parent_dir = file_path.parent().unwrap_or(home_dir.as_path());
            config.logging.logging_path = Some(parent_dir.join(path));
        }

        if let Some(file) = config.overrides.file.to_str() {
            let path = Path::new(file);
            config.overrides.file = path_resolver::resolve_home_path(path).with_context(|_| ResolveSnafu {
                file: path.display().to_string(),
            })?;
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::color_config::{
        CreateConfig, LinkConfig, ListConfig, RemoveConfig, SerializeColorSettings, SerializeCreateConfig,
        SerializeLinkConfig, SerializeListConfig, SerializeRemoveConfig, SerializeSimulationConfig,
        SerializeUnlinkConfig, SimulationConfig, UnlinkConfig, V1SerializeColorSettings,
    };
    use crate::config::logging_config::{ConsoleLoggingStream, LoggingLevel, RotationType};
    use crate::config::stow_config::MatchingStrategy;
    use crossterm::style::Color;
    use rstest::rstest;
    use std::path::PathBuf;

    const COLOR_V1_TEST: V1SerializeColorSettings = V1SerializeColorSettings {
        link: Some(Color::Green),
        unlink: Some(Color::Red),
        list: Some(Color::Blue),
        remove: Some(Color::Magenta),
        create: Some(Color::Yellow),
        arrow: Some(Color::Cyan),
        source: Some(Color::Black),
        target: Some(Color::Grey),
    };

    const COLOR_V2_TEST: SerializeColorSettings = SerializeColorSettings {
        link: SerializeLinkConfig {
            link: Some(Color::Green),
            colon: Some(Color::White),
            source: Some(Color::Black),
            arrow: Some(Color::Cyan),
            target: Some(Color::Grey),
        },
        unlink: SerializeUnlinkConfig {
            unlink: Some(Color::Red),
            colon: Some(Color::White),
            target: Some(Color::Grey),
        },
        list: SerializeListConfig {
            link: Some(Color::Blue),
            colon: Some(Color::White),
            source: Some(Color::Black),
            arrow: Some(Color::Cyan),
            target: Some(Color::Grey),
        },
        remove: SerializeRemoveConfig {
            remove: Some(Color::Magenta),
            colon: Some(Color::White),
            target: Some(Color::Grey),
        },
        create: SerializeCreateConfig {
            create: Some(Color::Yellow),
            colon: Some(Color::White),
            target: Some(Color::Grey),
        },
        simulation: SerializeSimulationConfig {
            note: Some(Color::DarkGreen),
            colon: Some(Color::White),
            text: Some(Color::DarkYellow),
        },
    };

    fn create_logging_config() -> LoggingConfig {
        LoggingConfig {
            level: LoggingLevel::Info,
            stream: ConsoleLoggingStream::Stderr,
            file: Some(PathBuf::from("temp.log")),
            logging_path: Some(PathBuf::from("log_dir")),
            rotation: RotationType::Daily,
            format: LoggingFormat::Pretty,
            max_log_files: 10,
            color_support: true,
        }
    }

    #[rstest]
    fn toml_version_1_deserialization() {
        let config_content = r#"
        version = 1

        [ignored]
        file = "ignored_files.txt"
        comment = 'c'

        [overrides]
        file = "override_files.txt"
        comment = 'q'

        [logging]
        level = "info"
        stream = "stderr"
        file = "temp.log"
        logging_path = "log_dir"
        rotation = "daily"
        format = "pretty"
        max_log_files = 10
        color_support = true

        [color]
        enabled = true
        link = "green"
        unlink = "red"
        list = "blue"
        remove = "magenta"
        create = "yellow"
        arrow = "cyan"
        source = "black"
        target = "grey"

        [stow]
        linking_strategy = "short"
        regex_strategy = "rust"
        matching_strategy = "individual"
        printing_enabled = false
        "#;

        let config: V1Config = toml::from_str(config_content).expect("Failed to parse TOML");
        assert_eq!(config.version, ConfigFileVersion::V1);
        let expected_ignore = Ignored {
            file: PathBuf::from("ignored_files.txt"),
            comment: 'c',
        };

        assert_eq!(config.ignored, expected_ignore);
        let expected_override = Overrides {
            file: PathBuf::from("override_files.txt"),
            comment: 'q',
        };

        assert_eq!(config.overrides, expected_override);
        let expected_logging = create_logging_config();
        assert_eq!(config.logging, expected_logging);

        let settings = COLOR_V1_TEST;
        let expected_color = V1ColorConfig {
            enabled: true,
            settings,
        };

        assert_eq!(config.color, expected_color);

        let expected_stow = StowConfig {
            linking_strategy: LinkingStrategy::Short,
            regex_strategy: RegexStrategy::Rust,
            matching_strategy: MatchingStrategy::Individual,
            printing_enabled: false,
        };

        assert_eq!(config.stow, expected_stow);
    }

    #[rstest]
    fn toml_version_2_deserialization() {
        let config_content = r#"
        version = 2

        [ignored]
        file = "ignored_files.txt"
        comment = 'c'

        [overrides]
        file = "override_files.txt"
        comment = 'q'

        [logging]
        level = "info"
        stream = "stderr"
        file = "temp.log"
        logging_path = "log_dir"
        rotation = "daily"
        format = "pretty"
        max_log_files = 10
        color_support = true

        [color]
        enabled = true

        [color.link]
        link = "green"
        arrow = "cyan"
        source = "black"
        target = "grey"
        colon = "white"

        [color.unlink]
        unlink = "red"
        target = "grey"
        colon = "white"

        [color.list]
        link = "blue"
        arrow = "cyan"
        source = "black"
        target = "grey"
        colon = "white"

        [color.remove]
        remove = "magenta"
        target = "grey"
        colon = "white"

        [color.create]
        create = "yellow"
        target = "grey"
        colon = "white"

        [color.simulation]
        note = "dark_green"
        colon = "white"
        text = "dark_yellow"

        [stow]
        linking_strategy = "short"
        regex_strategy = "rust"
        matching_strategy = "individual"
        printing_enabled = false
        "#;

        let config: Config = toml::from_str(config_content).expect("Failed to parse TOML");
        assert_eq!(config.version, ConfigFileVersion::V2);
        let expected_ignore = Ignored {
            file: PathBuf::from("ignored_files.txt"),
            comment: 'c',
        };

        assert_eq!(config.ignored, expected_ignore);
        let expected_override = Overrides {
            file: PathBuf::from("override_files.txt"),
            comment: 'q',
        };

        assert_eq!(config.overrides, expected_override);
        let expected_logging = create_logging_config();
        assert_eq!(config.logging, expected_logging);

        let settings = COLOR_V2_TEST;
        let expected_color = ColorConfig {
            enabled: true,
            settings,
        };

        assert_eq!(config.color, expected_color);

        let expected_stow = StowConfig {
            linking_strategy: LinkingStrategy::Short,
            regex_strategy: RegexStrategy::Rust,
            matching_strategy: MatchingStrategy::Individual,
            printing_enabled: false,
        };

        assert_eq!(config.stow, expected_stow);
    }

    #[rstest]
    #[case("1", ConfigFileVersion::V1)]
    #[case("v1", ConfigFileVersion::V1)]
    #[case("V1", ConfigFileVersion::V1)]
    #[case("2", ConfigFileVersion::V2)]
    #[case("v2", ConfigFileVersion::V2)]
    #[case("V2", ConfigFileVersion::V2)]
    fn toml_version_deserialization_string_version(#[case] version: &str, #[case] expected: ConfigFileVersion) {
        let config_content = format!(
            r#"
            version = "{version}"
            "#
        );

        let config: V1Config = toml::from_str(&config_content).expect("Failed to parse TOML");
        assert_eq!(config.version, expected);
        assert_eq!(config.stow, StowConfig::default());
    }

    #[rstest]
    fn toml_version_1_everything_is_optional() {
        let config_content = "
        version = 1
        ";

        let default_config = V1Config::default();
        let config: V1Config = toml::from_str(config_content).expect("Failed to parse TOML");
        assert_eq!(config.version, ConfigFileVersion::V1);
        assert_eq!(config.logging, default_config.logging);
        assert_eq!(config.ignored, default_config.ignored);
        assert_eq!(config.overrides, default_config.overrides);
        assert_eq!(config.color.enabled, default_config.color.enabled);
        assert_eq!(config.color, default_config.color);
        assert_eq!(config.stow, default_config.stow);
    }

    #[rstest]
    fn toml_version_2_everything_is_optional() {
        let config_content = "
        version = 2
        ";

        let default_config = Config::default();
        let config: Config = toml::from_str(config_content).expect("Failed to parse TOML");
        assert_eq!(config.version, ConfigFileVersion::V2);
        assert_eq!(config.logging, default_config.logging);
        assert_eq!(config.ignored, default_config.ignored);
        assert_eq!(config.overrides, default_config.overrides);
        assert_eq!(config.color.enabled, default_config.color.enabled);
        assert_eq!(config.color, default_config.color);
        assert_eq!(config.stow, default_config.stow);
    }

    #[rstest]
    #[case(ConfigFileVersion::V1, "off", LoggingLevel::Off)]
    #[case(ConfigFileVersion::V1, "trace", LoggingLevel::Trace)]
    #[case(ConfigFileVersion::V1, "debug", LoggingLevel::Debug)]
    #[case(ConfigFileVersion::V1, "info", LoggingLevel::Info)]
    #[case(ConfigFileVersion::V1, "warn", LoggingLevel::Warn)]
    #[case(ConfigFileVersion::V1, "error", LoggingLevel::Error)]
    #[case(ConfigFileVersion::V1, "Off", LoggingLevel::Off)]
    #[case(ConfigFileVersion::V1, "Trace", LoggingLevel::Trace)]
    #[case(ConfigFileVersion::V1, "Debug", LoggingLevel::Debug)]
    #[case(ConfigFileVersion::V1, "Info", LoggingLevel::Info)]
    #[case(ConfigFileVersion::V1, "Warn", LoggingLevel::Warn)]
    #[case(ConfigFileVersion::V1, "Error", LoggingLevel::Error)]
    #[case(ConfigFileVersion::V1, "OFF", LoggingLevel::Off)]
    #[case(ConfigFileVersion::V1, "TRACE", LoggingLevel::Trace)]
    #[case(ConfigFileVersion::V1, "DEBUG", LoggingLevel::Debug)]
    #[case(ConfigFileVersion::V1, "INFO", LoggingLevel::Info)]
    #[case(ConfigFileVersion::V1, "WARN", LoggingLevel::Warn)]
    #[case(ConfigFileVersion::V1, "ERROR", LoggingLevel::Error)]
    #[case(ConfigFileVersion::V2, "off", LoggingLevel::Off)]
    #[case(ConfigFileVersion::V2, "trace", LoggingLevel::Trace)]
    #[case(ConfigFileVersion::V2, "debug", LoggingLevel::Debug)]
    #[case(ConfigFileVersion::V2, "info", LoggingLevel::Info)]
    #[case(ConfigFileVersion::V2, "warn", LoggingLevel::Warn)]
    #[case(ConfigFileVersion::V2, "error", LoggingLevel::Error)]
    #[case(ConfigFileVersion::V2, "Off", LoggingLevel::Off)]
    #[case(ConfigFileVersion::V2, "Trace", LoggingLevel::Trace)]
    #[case(ConfigFileVersion::V2, "Debug", LoggingLevel::Debug)]
    #[case(ConfigFileVersion::V2, "Info", LoggingLevel::Info)]
    #[case(ConfigFileVersion::V2, "Warn", LoggingLevel::Warn)]
    #[case(ConfigFileVersion::V2, "Error", LoggingLevel::Error)]
    #[case(ConfigFileVersion::V2, "OFF", LoggingLevel::Off)]
    #[case(ConfigFileVersion::V2, "TRACE", LoggingLevel::Trace)]
    #[case(ConfigFileVersion::V2, "DEBUG", LoggingLevel::Debug)]
    #[case(ConfigFileVersion::V2, "INFO", LoggingLevel::Info)]
    #[case(ConfigFileVersion::V2, "WARN", LoggingLevel::Warn)]
    #[case(ConfigFileVersion::V2, "ERROR", LoggingLevel::Error)]
    fn toml_ignores_logging_level_case(
        #[case] version: ConfigFileVersion,
        #[case] level: &str,
        #[case] expected_level: LoggingLevel,
    ) {
        let config_content = format!(
            r#"
            version = "{version}"

            [logging]
            level = "{level}"
            "#
        );

        let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
        assert_eq!(config.logging.level, expected_level);
    }

    #[rstest]
    #[case(ConfigFileVersion::V1, LoggingLevel::Off)]
    #[case(ConfigFileVersion::V1, LoggingLevel::Trace)]
    #[case(ConfigFileVersion::V1, LoggingLevel::Debug)]
    #[case(ConfigFileVersion::V1, LoggingLevel::Info)]
    #[case(ConfigFileVersion::V1, LoggingLevel::Warn)]
    #[case(ConfigFileVersion::V1, LoggingLevel::Error)]
    #[case(ConfigFileVersion::V2, LoggingLevel::Off)]
    #[case(ConfigFileVersion::V2, LoggingLevel::Trace)]
    #[case(ConfigFileVersion::V2, LoggingLevel::Debug)]
    #[case(ConfigFileVersion::V2, LoggingLevel::Info)]
    #[case(ConfigFileVersion::V2, LoggingLevel::Warn)]
    #[case(ConfigFileVersion::V2, LoggingLevel::Error)]
    fn toml_logging_level_can_use_numeric_value(#[case] version: ConfigFileVersion, #[case] level: LoggingLevel) {
        let config_content = format!(
            r#"
            version = "{version}"

            [logging]
            level = {}
            "#,
            level as i64
        );

        let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
        assert_eq!(config.logging.level, level);
    }

    #[rstest]
    #[case(ConfigFileVersion::V1, "Daily", RotationType::Daily)]
    #[case(ConfigFileVersion::V1, "DAILY", RotationType::Daily)]
    #[case(ConfigFileVersion::V1, "daily", RotationType::Daily)]
    #[case(ConfigFileVersion::V1, "Hourly", RotationType::Hourly)]
    #[case(ConfigFileVersion::V1, "HOURLY", RotationType::Hourly)]
    #[case(ConfigFileVersion::V1, "hourly", RotationType::Hourly)]
    #[case(ConfigFileVersion::V2, "Daily", RotationType::Daily)]
    #[case(ConfigFileVersion::V2, "DAILY", RotationType::Daily)]
    #[case(ConfigFileVersion::V2, "daily", RotationType::Daily)]
    #[case(ConfigFileVersion::V2, "Hourly", RotationType::Hourly)]
    #[case(ConfigFileVersion::V2, "HOURLY", RotationType::Hourly)]
    #[case(ConfigFileVersion::V2, "hourly", RotationType::Hourly)]
    fn toml_ignores_rotation_case(
        #[case] version: ConfigFileVersion,
        #[case] rotation: &str,
        #[case] expected_rotation: RotationType,
    ) {
        let config_content = format!(
            r#"
            version = "{version}"

            [logging]
            rotation = "{rotation}"
            "#
        );

        let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
        assert_eq!(config.logging.rotation, expected_rotation);
    }

    #[rstest]
    #[case(ConfigFileVersion::V1, RotationType::Hourly)]
    #[case(ConfigFileVersion::V1, RotationType::Daily)]
    #[case(ConfigFileVersion::V2, RotationType::Hourly)]
    #[case(ConfigFileVersion::V2, RotationType::Daily)]
    fn toml_rotation_can_use_numeric_value(#[case] version: ConfigFileVersion, #[case] rotation: RotationType) {
        let config_content = format!(
            r#"
            version = "{version}"

            [logging]
            rotation = {}
            "#,
            rotation as i64
        );

        let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
        assert_eq!(config.logging.rotation, rotation);
    }

    #[rstest]
    #[case(r#""1""#, ConfigFileVersion::V1)]
    #[case(r#""v1""#, ConfigFileVersion::V1)]
    #[case(r#""V1""#, ConfigFileVersion::V1)]
    #[case("1", ConfigFileVersion::V1)]
    #[case(r#""2""#, ConfigFileVersion::V2)]
    #[case(r#""v2""#, ConfigFileVersion::V2)]
    #[case(r#""V2""#, ConfigFileVersion::V2)]
    #[case("2", ConfigFileVersion::V2)]
    fn toml_supported_version_strings(#[case] version: &str, #[case] expected_version: ConfigFileVersion) {
        let config_content = format!(
            r"
            version = {version}
            "
        );

        let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
        assert_eq!(config.version, expected_version);
    }

    #[rstest]
    #[case(ConfigFileVersion::V1, "Compact", LoggingFormat::Compact)]
    #[case(ConfigFileVersion::V1, "compact", LoggingFormat::Compact)]
    #[case(ConfigFileVersion::V1, "COMPACT", LoggingFormat::Compact)]
    #[case(ConfigFileVersion::V1, "Json", LoggingFormat::Json)]
    #[case(ConfigFileVersion::V1, "json", LoggingFormat::Json)]
    #[case(ConfigFileVersion::V1, "JSON", LoggingFormat::Json)]
    #[case(ConfigFileVersion::V1, "Pretty", LoggingFormat::Pretty)]
    #[case(ConfigFileVersion::V1, "pretty", LoggingFormat::Pretty)]
    #[case(ConfigFileVersion::V1, "PRETTY", LoggingFormat::Pretty)]
    #[case(ConfigFileVersion::V2, "Compact", LoggingFormat::Compact)]
    #[case(ConfigFileVersion::V2, "compact", LoggingFormat::Compact)]
    #[case(ConfigFileVersion::V2, "COMPACT", LoggingFormat::Compact)]
    #[case(ConfigFileVersion::V2, "Json", LoggingFormat::Json)]
    #[case(ConfigFileVersion::V2, "json", LoggingFormat::Json)]
    #[case(ConfigFileVersion::V2, "JSON", LoggingFormat::Json)]
    #[case(ConfigFileVersion::V2, "Pretty", LoggingFormat::Pretty)]
    #[case(ConfigFileVersion::V2, "pretty", LoggingFormat::Pretty)]
    #[case(ConfigFileVersion::V2, "PRETTY", LoggingFormat::Pretty)]
    fn toml_ignores_logging_format_case(
        #[case] version: ConfigFileVersion,
        #[case] format: &str,
        #[case] expected_format: LoggingFormat,
    ) {
        let config_content = format!(
            r#"
            version = "{version}"

            [logging]
            format = "{format}"
            "#,
        );

        let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
        assert_eq!(config.logging.format, expected_format);
    }

    #[rstest]
    #[case(ConfigFileVersion::V1, LoggingFormat::Compact)]
    #[case(ConfigFileVersion::V1, LoggingFormat::Json)]
    #[case(ConfigFileVersion::V1, LoggingFormat::Pretty)]
    #[case(ConfigFileVersion::V2, LoggingFormat::Compact)]
    #[case(ConfigFileVersion::V2, LoggingFormat::Json)]
    #[case(ConfigFileVersion::V2, LoggingFormat::Pretty)]
    fn toml_logging_format_can_use_numeric_value(#[case] version: ConfigFileVersion, #[case] format: LoggingFormat) {
        let config_content = format!(
            r#"
            version = "{version}"

            [logging]
            format = {}
            "#,
            format as i64
        );

        let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
        assert_eq!(config.logging.format, format);
    }

    #[rstest]
    #[case(ConfigFileVersion::V1, "stdout", ConsoleLoggingStream::Stdout)]
    #[case(ConfigFileVersion::V1, "Stdout", ConsoleLoggingStream::Stdout)]
    #[case(ConfigFileVersion::V1, "STDOUT", ConsoleLoggingStream::Stdout)]
    #[case(ConfigFileVersion::V1, "stderr", ConsoleLoggingStream::Stderr)]
    #[case(ConfigFileVersion::V1, "Stderr", ConsoleLoggingStream::Stderr)]
    #[case(ConfigFileVersion::V1, "STDERR", ConsoleLoggingStream::Stderr)]
    #[case(ConfigFileVersion::V2, "stdout", ConsoleLoggingStream::Stdout)]
    #[case(ConfigFileVersion::V2, "Stdout", ConsoleLoggingStream::Stdout)]
    #[case(ConfigFileVersion::V2, "STDOUT", ConsoleLoggingStream::Stdout)]
    #[case(ConfigFileVersion::V2, "stderr", ConsoleLoggingStream::Stderr)]
    #[case(ConfigFileVersion::V2, "Stderr", ConsoleLoggingStream::Stderr)]
    #[case(ConfigFileVersion::V2, "STDERR", ConsoleLoggingStream::Stderr)]
    fn toml_ignores_console_logging_stream_case(
        #[case] version: ConfigFileVersion,
        #[case] stream: &str,
        #[case] expected_stream: ConsoleLoggingStream,
    ) {
        let config_content = format!(
            r#"
            version = "{version}"

            [logging]
            stream = "{stream}"
            "#
        );

        let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
        assert_eq!(config.logging.stream, expected_stream);
    }

    #[rstest]
    #[case(ConfigFileVersion::V1, ConsoleLoggingStream::Stdout)]
    #[case(ConfigFileVersion::V1, ConsoleLoggingStream::Stderr)]
    #[case(ConfigFileVersion::V2, ConsoleLoggingStream::Stdout)]
    #[case(ConfigFileVersion::V2, ConsoleLoggingStream::Stderr)]
    fn toml_console_logging_stream_can_use_numeric_value(
        #[case] version: ConfigFileVersion,
        #[case] stream: ConsoleLoggingStream,
    ) {
        let config_content = format!(
            r#"
            version = "{version}"

            [logging]
            stream = {}
            "#,
            stream as i64
        );

        let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
        assert_eq!(config.logging.stream, stream);
    }

    #[rstest]
    fn toml_version_1_color_settings_can_be_color_names() {
        let config_content = r#"
        version = 1

        [color]
        enabled = true
        link = "green"
        unlink = "red"
        list = "blue"
        remove = "magenta"
        create = "yellow"
        arrow = "cyan"
        source = "black"
        target = "white"
        "#;

        let config: V1Config = toml::from_str(config_content).expect("Failed to parse TOML");
        assert!(config.color.enabled);
        assert_eq!(config.color.settings.link, Some(Color::Green));
        assert_eq!(config.color.settings.unlink, Some(Color::Red));
        assert_eq!(config.color.settings.list, Some(Color::Blue));
        assert_eq!(config.color.settings.remove, Some(Color::Magenta));
        assert_eq!(config.color.settings.create, Some(Color::Yellow));
        assert_eq!(config.color.settings.arrow, Some(Color::Cyan));
        assert_eq!(config.color.settings.source, Some(Color::Black));
        assert_eq!(config.color.settings.target, Some(Color::White));
    }

    #[rstest]
    fn toml_version_1_color_settings_color_names_case_insensitive() {
        let config_content = r#"
        version = 1

        [color]
        enabled = true
        link = "Green"
        unlink = "Red"
        list = "Blue"
        remove = "Magenta"
        create = "Yellow"
        arrow = "Cyan"
        source = "Black"
        target = "Grey"
        "#;

        let config: V1Config = toml::from_str(config_content).expect("Failed to parse TOML");
        let expected_color = V1ColorConfig {
            enabled: true,
            settings: COLOR_V1_TEST,
        };

        assert_eq!(config.color, expected_color);
    }

    #[rstest]
    fn toml_version_2_color_settings_color_names_case_insensitive() {
        let config_content = r#"
        version = 2

        [color]
        enabled = true

        [color.link]
        link = "Green"
        arrow = "Cyan"
        source = "Black"
        target = "Grey"
        colon = "White"

        [color.unlink]
        unlink = "Red"
        target = "Grey"
        colon = "White"

        [color.list]
        link = "Blue"
        arrow = "Cyan"
        source = "Black"
        target = "Grey"
        colon = "White"

        [color.remove]
        remove = "Magenta"
        target = "Grey"
        colon = "White"

        [color.create]
        create = "Yellow"
        target = "Grey"
        colon = "White"

        [color.simulation]
        note = "Dark_Green"
        colon = "White"
        text = "Dark_Yellow"
        "#;

        let config: Config = toml::from_str(config_content).expect("Failed to parse TOML");
        let expected_color = ColorConfig {
            enabled: true,
            settings: COLOR_V2_TEST,
        };

        assert_eq!(config.color, expected_color);
    }

    #[rstest]
    fn toml_version_1_color_settings_can_be_hex_values() {
        let config_content = r##"
        version = 1

       [color]
       enabled = true
       link = "#27F54D"
       unlink = "#F54927"
       list = "#27F54D"
       remove = "#F54927"
       create = "#F54927"
       arrow = "#F54927"
       source = "#F54927"
       target = "#F54927"
       "##;

        let config: V1Config = toml::from_str(config_content).expect("Failed to parse TOML");
        let expected_color = V1ColorConfig {
            enabled: true,
            settings: V1SerializeColorSettings {
                link: Some(Color::Rgb {
                    r: 0x27,
                    g: 0xF5,
                    b: 0x4D,
                }),
                unlink: Some(Color::Rgb {
                    r: 0xF5,
                    g: 0x49,
                    b: 0x27,
                }),
                list: Some(Color::Rgb {
                    r: 0x27,
                    g: 0xF5,
                    b: 0x4D,
                }),
                remove: Some(Color::Rgb {
                    r: 0xF5,
                    g: 0x49,
                    b: 0x27,
                }),
                create: Some(Color::Rgb {
                    r: 0xF5,
                    g: 0x49,
                    b: 0x27,
                }),
                arrow: Some(Color::Rgb {
                    r: 0xF5,
                    g: 0x49,
                    b: 0x27,
                }),
                source: Some(Color::Rgb {
                    r: 0xF5,
                    g: 0x49,
                    b: 0x27,
                }),
                target: Some(Color::Rgb {
                    r: 0xF5,
                    g: 0x49,
                    b: 0x27,
                }),
            },
        };

        assert_eq!(config.color, expected_color);
    }

    #[rstest]
    fn toml_version_1_color_settings_allows_partial_colors() {
        let config_content = r#"
        version = 1

       [color]
       enabled = true
       link = "Red"
       "#;

        let config: V1Config = toml::from_str(config_content).expect("Failed to parse TOML");
        let expected_color = V1ColorConfig {
            enabled: true,
            settings: V1SerializeColorSettings {
                link: Some(Color::Red),
                unlink: None,
                list: None,
                remove: None,
                create: None,
                arrow: None,
                source: None,
                target: None,
            },
        };

        assert_eq!(config.color, expected_color);

        let default_link = LinkConfig::default();
        let expected_color_settings = ColorSettings {
            link: LinkConfig {
                link: Color::Red,
                colon: default_link.colon,
                source: default_link.source,
                arrow: default_link.arrow,
                target: default_link.target,
            },
            unlink: UnlinkConfig::default(),
            list: ListConfig::default(),
            remove: RemoveConfig::default(),
            create: CreateConfig::default(),
            simulation: SimulationConfig::default(),
        };

        let config: Config = config.into();
        assert_eq!(config.color.color_settings(), expected_color_settings);
    }

    #[rstest]
    #[case(ConfigFileVersion::V1, "short", LinkingStrategy::Short)]
    #[case(ConfigFileVersion::V1, "Short", LinkingStrategy::Short)]
    #[case(ConfigFileVersion::V1, "SHORT", LinkingStrategy::Short)]
    #[case(ConfigFileVersion::V1, "full", LinkingStrategy::Full)]
    #[case(ConfigFileVersion::V1, "Full", LinkingStrategy::Full)]
    #[case(ConfigFileVersion::V1, "FULL", LinkingStrategy::Full)]
    #[case(ConfigFileVersion::V2, "short", LinkingStrategy::Short)]
    #[case(ConfigFileVersion::V2, "Short", LinkingStrategy::Short)]
    #[case(ConfigFileVersion::V2, "SHORT", LinkingStrategy::Short)]
    #[case(ConfigFileVersion::V2, "full", LinkingStrategy::Full)]
    #[case(ConfigFileVersion::V2, "Full", LinkingStrategy::Full)]
    #[case(ConfigFileVersion::V2, "FULL", LinkingStrategy::Full)]
    fn toml_ignores_linking_strategy_case(
        #[case] version: ConfigFileVersion,
        #[case] strategy: &str,
        #[case] expected_strategy: LinkingStrategy,
    ) {
        let config_content = format!(
            r#"
            version = "{version}"

            [stow]
            linking_strategy = "{strategy}"
            "#
        );

        let config: Config = toml::from_str(&config_content).expect("Failed to parse TOML");
        assert_eq!(config.stow.linking_strategy, expected_strategy);
    }

    #[rstest]
    #[case(ConfigFileVersion::V1, LinkingStrategy::Short)]
    #[case(ConfigFileVersion::V1, LinkingStrategy::Full)]
    #[case(ConfigFileVersion::V2, LinkingStrategy::Short)]
    #[case(ConfigFileVersion::V2, LinkingStrategy::Full)]
    fn toml_linking_strategy_can_use_numeric_value(
        #[case] version: ConfigFileVersion,
        #[case] strategy: LinkingStrategy,
    ) {
        let config_content = format!(
            r#"
            version = "{version}"

            [stow]
            linking_strategy = {}
            "#,
            strategy as i64
        );

        let config: Config = toml::from_str(&config_content).expect("Failed to parse TOML");
        assert_eq!(config.stow.linking_strategy, strategy);
    }

    #[rstest]
    #[case(ConfigFileVersion::V1, "rust", RegexStrategy::Rust)]
    #[case(ConfigFileVersion::V1, "Rust", RegexStrategy::Rust)]
    #[case(ConfigFileVersion::V1, "RUST", RegexStrategy::Rust)]
    #[case(ConfigFileVersion::V1, "pcre2", RegexStrategy::Pcre2)]
    #[case(ConfigFileVersion::V1, "Pcre2", RegexStrategy::Pcre2)]
    #[case(ConfigFileVersion::V1, "PCRE2", RegexStrategy::Pcre2)]
    #[case(ConfigFileVersion::V2, "rust", RegexStrategy::Rust)]
    #[case(ConfigFileVersion::V2, "Rust", RegexStrategy::Rust)]
    #[case(ConfigFileVersion::V2, "RUST", RegexStrategy::Rust)]
    #[case(ConfigFileVersion::V2, "pcre2", RegexStrategy::Pcre2)]
    #[case(ConfigFileVersion::V2, "Pcre2", RegexStrategy::Pcre2)]
    #[case(ConfigFileVersion::V2, "PCRE2", RegexStrategy::Pcre2)]
    fn toml_ignores_regex_strategy_case(
        #[case] version: ConfigFileVersion,
        #[case] strategy: &str,
        #[case] expected_strategy: RegexStrategy,
    ) {
        let config_content = format!(
            r#"
            version = "{version}"

            [stow]
            regex_strategy = "{strategy}"
            "#
        );

        let config: Config = toml::from_str(&config_content).expect("Failed to parse TOML");
        assert_eq!(config.stow.regex_strategy, expected_strategy);
    }

    #[rstest]
    #[case(ConfigFileVersion::V1, RegexStrategy::Rust)]
    #[case(ConfigFileVersion::V1, RegexStrategy::Pcre2)]
    #[case(ConfigFileVersion::V2, RegexStrategy::Rust)]
    #[case(ConfigFileVersion::V2, RegexStrategy::Pcre2)]
    fn toml_regex_strategy_can_use_numeric_value(#[case] version: ConfigFileVersion, #[case] strategy: RegexStrategy) {
        let config_content = format!(
            r#"
            version = "{version}"

            [stow]
            regex_strategy = {}
            "#,
            strategy as i64
        );

        let config: Config = toml::from_str(&config_content).expect("Failed to parse TOML");
        assert_eq!(config.stow.regex_strategy, strategy);
    }

    #[rstest]
    #[case(ConfigFileVersion::V1, "individual", MatchingStrategy::Individual)]
    #[case(ConfigFileVersion::V1, "Individual", MatchingStrategy::Individual)]
    #[case(ConfigFileVersion::V1, "INDIVIDUAL", MatchingStrategy::Individual)]
    #[case(ConfigFileVersion::V1, "combined", MatchingStrategy::Combined)]
    #[case(ConfigFileVersion::V1, "Combined", MatchingStrategy::Combined)]
    #[case(ConfigFileVersion::V1, "COMBINED", MatchingStrategy::Combined)]
    #[case(ConfigFileVersion::V2, "individual", MatchingStrategy::Individual)]
    #[case(ConfigFileVersion::V2, "Individual", MatchingStrategy::Individual)]
    #[case(ConfigFileVersion::V2, "INDIVIDUAL", MatchingStrategy::Individual)]
    #[case(ConfigFileVersion::V2, "combined", MatchingStrategy::Combined)]
    #[case(ConfigFileVersion::V2, "Combined", MatchingStrategy::Combined)]
    #[case(ConfigFileVersion::V2, "COMBINED", MatchingStrategy::Combined)]
    fn toml_ignores_matching_strategy_case(
        #[case] version: ConfigFileVersion,
        #[case] strategy: &str,
        #[case] expected_strategy: MatchingStrategy,
    ) {
        let config_content = format!(
            r#"
            version = "{version}"

            [stow]
            matching_strategy = "{strategy}"
            "#
        );

        let config: Config = toml::from_str(&config_content).expect("Failed to parse TOML");
        assert_eq!(config.stow.matching_strategy, expected_strategy);
    }

    #[rstest]
    #[case(ConfigFileVersion::V1, MatchingStrategy::Individual)]
    #[case(ConfigFileVersion::V1, MatchingStrategy::Combined)]
    #[case(ConfigFileVersion::V2, MatchingStrategy::Individual)]
    #[case(ConfigFileVersion::V2, MatchingStrategy::Combined)]
    fn toml_matching_strategy_can_use_numeric_value(
        #[case] version: ConfigFileVersion,
        #[case] strategy: MatchingStrategy,
    ) {
        let config_content = format!(
            r#"
            version = "{version}"

            [stow]
            matching_strategy = {}
            "#,
            strategy as i64
        );

        let config: Config = toml::from_str(&config_content).expect("Failed to parse TOML");
        assert_eq!(config.stow.matching_strategy, strategy);
    }
}
