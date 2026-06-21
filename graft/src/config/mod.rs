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
mod config_error;
mod config_file_version;
mod ignored;
mod logging_config;
mod logging_error;

mod color_config;
mod format_error;
mod level_error;
mod overrides;
pub mod path_resolver;
mod resolve_error;
mod rotation_error;
mod version_error;

pub use app_configuration::{AppConfiguration, DEFAULT_CONFIG_FILE};
pub use app_directories::AppDirectories;
pub use color_config::{ColorConfig, ColorSettings};
pub use config_error::ConfigError;
pub use config_file_version::ConfigFileVersion;
pub use ignored::Ignored;
pub use level_error::LevelError;
pub use logging_config::{LoggingConfig, LoggingFormat};
pub use logging_error::LoggingError;
pub use overrides::Overrides;
pub use resolve_error::ResolveError;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::path::Path;
use std::{env, fs};

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
            version: ConfigFileVersion::V1,
            ignored: Ignored::default(),
            logging: LoggingConfig::default(),
            overrides: Overrides::default(),
            color: ColorConfig::default(),
        }
    }
}

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

        let config_file = app_directories.config_dir.join(DEFAULT_CONFIG_FILE);
        if let Ok(config_file) = path_resolver::resolve_path(&config_file)
            && fs::exists(&config_file).unwrap_or(false)
        {
            return Self::read_config_file(&config_file, &app_directories);
        }

        Ok(Self::default())
    }

    fn read_config_file(file_path: &Path, app_directories: &AppDirectories) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(file_path)?;
        let mut config = toml::from_str::<Self>(&content)?;
        let home_dir = env::home_dir().ok_or(ConfigError::UnableToFindHomeDirectory)?;
        config.ignored.file = path_resolver::resolve_home_path(&config.ignored.file)?;
        if config.ignored.file.is_relative() {
            let parent_dir = file_path.parent().unwrap_or(home_dir.as_path());
            config.ignored.file = parent_dir.join(config.ignored.file);
        }

        if let Some(file) = config.ignored.file.to_str() {
            let path = Path::new(file);
            config.ignored.file = path_resolver::resolve_home_path(path)?;
        }

        config.logging.logging_path = if let Some(path) = config.logging.logging_path {
            Some(path_resolver::resolve_home_path(&path)?)
        } else {
            Some(path_resolver::resolve_home_path(&app_directories.log_dir)?)
        };

        if let Some(path) = &config.logging.logging_path
            && path.is_relative()
        {
            let parent_dir = file_path.parent().unwrap_or(home_dir.as_path());
            config.logging.logging_path = Some(parent_dir.join(path));
        }

        if let Some(file) = config.overrides.file.to_str() {
            let path = Path::new(file);
            config.overrides.file = path_resolver::resolve_home_path(path)?;
        }

        Ok(config)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::logging_config::{LoggingLevel, RotationType};
    use crossterm::style::Color;
    use std::path::PathBuf;

    #[test]
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
        remove = "yellow"
        create = "cyan"
        arrow = "magenta"
        source = "white"
        target = "black"
        "#;

        let config: Config = toml::from_str(config_content).expect("Failed to parse TOML");
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
        let expected_logging = LoggingConfig {
            level: LoggingLevel::Info,
            file: Some(PathBuf::from("temp.log")),
            logging_path: Some(PathBuf::from("log_dir")),
            rotation: RotationType::Daily,
            format: LoggingFormat::Pretty,
            max_log_files: 10,
            color_support: true,
        };

        assert_eq!(config.logging, expected_logging);

        let settings = ColorSettings {
            link: Color::Green,
            unlink: Color::Red,
            list: Color::Blue,
            remove: Color::Yellow,
            create: Color::Cyan,
            arrow: Color::Magenta,
            source: Color::White,
            target: Color::Black,
        };

        let expected_color = ColorConfig {
            enabled: true,
            settings,
        };

        assert_eq!(config.color, expected_color);
    }

    #[test]
    fn toml_version_1_deserialization_string_version() {
        let allowed_versions = vec!["1", "v1", "V1"];
        for version in allowed_versions {
            let config_content = format!(
                r#"
            version = "{version}"
            "#
            );

            let config: Config = toml::from_str(&config_content).expect("Failed to parse TOML");
            assert_eq!(config.version, ConfigFileVersion::V1);
        }
    }

    #[test]
    fn toml_version_1_everything_is_optional() {
        let config_content = "
        version = 1
        ";

        let default_config = Config::default();
        let config: Config = toml::from_str(config_content).expect("Failed to parse TOML");
        assert_eq!(config.version, ConfigFileVersion::V1);
        assert_eq!(config.logging, default_config.logging);
        assert_eq!(config.ignored, default_config.ignored);
        assert_eq!(config.overrides, default_config.overrides);
        assert_eq!(config.color.enabled, default_config.color.enabled);
        assert_eq!(config.color, default_config.color);
    }

    #[test]
    fn toml_version_1_ignores_logging_level_case() {
        let allowed_levels = vec![
            LoggingLevel::Off,
            LoggingLevel::Trace,
            LoggingLevel::Debug,
            LoggingLevel::Info,
            LoggingLevel::Warn,
            LoggingLevel::Error,
        ];

        for level in allowed_levels {
            let config_content = format!(
                r#"
            version = 1

            [logging]
            level = "{level}"
            "#
            );

            let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
            assert_eq!(config.logging.level, level);

            let config_content = format!(
                r#"
            version = 1

            [logging]
            level = "{}"
            "#,
                level.to_string().to_uppercase()
            );

            let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
            assert_eq!(config.logging.level, level);

            let config_content = format!(
                r#"
            version = 1

            [logging]
            level = "{}"
            "#,
                level.to_string().to_lowercase()
            );

            let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
            assert_eq!(config.logging.level, level);
        }
    }

    #[test]
    fn toml_version_1_logging_level_can_use_numeric_value() {
        let allowed_levels = vec![
            LoggingLevel::Off,
            LoggingLevel::Trace,
            LoggingLevel::Debug,
            LoggingLevel::Info,
            LoggingLevel::Warn,
            LoggingLevel::Error,
        ];

        for level in allowed_levels {
            let config_content = format!(
                r"
            version = 1

            [logging]
            level = {}
            ",
                level as i64
            );

            let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
            assert_eq!(config.logging.level, level);
        }
    }

    #[test]
    fn toml_version_1_ignores_rotation_case() {
        let rotations_types = vec![RotationType::Hourly, RotationType::Daily];
        for rotation in rotations_types {
            let config_content = format!(
                r#"
            version = 1

            [logging]
            rotation = "{rotation}"
            "#
            );

            let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
            assert_eq!(config.logging.rotation, rotation);

            let config_content = format!(
                r#"
            version = 1

            [logging]
            rotation = "{}"
            "#,
                rotation.to_string().to_uppercase()
            );

            let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
            assert_eq!(config.logging.rotation, rotation);

            let config_content = format!(
                r#"
            version = 1

            [logging]
            rotation = "{}"
            "#,
                rotation.to_string().to_lowercase()
            );

            let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
            assert_eq!(config.logging.rotation, rotation);
        }
    }

    #[test]
    fn toml_version_1_rotation_can_use_numeric_value() {
        let rotations_types = vec![RotationType::Hourly, RotationType::Daily];
        for rotation in rotations_types {
            let config_content = format!(
                r"
            version = 1

            [logging]
            rotation = {}
            ",
                rotation as i64
            );

            let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
            assert_eq!(config.logging.rotation, rotation);
        }
    }

    #[test]
    fn toml_version_1_supported_version_strings() {
        let version_text = vec![r#""1""#, r#""v1""#, r#""V1""#, "1"];
        for version in version_text {
            let config_content = format!(
                r"
            version = {version}
            "
            );

            let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
            assert_eq!(config.version, ConfigFileVersion::V1);
        }
    }

    #[test]
    fn toml_version_1_ignores_logging_format_case() {
        let format_types = vec![LoggingFormat::Compact, LoggingFormat::Json, LoggingFormat::Pretty];
        for format in format_types {
            let config_content = format!(
                r#"
            version = 1

            [logging]
            format = "{format}"
            "#,
            );

            let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
            assert_eq!(config.logging.format, format);

            let config_content = format!(
                r#"
            version = 1

            [logging]
            format = "{}"
            "#,
                format.to_string().to_uppercase()
            );

            let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
            assert_eq!(config.logging.format, format);

            let config_content = format!(
                r#"
            version = 1

            [logging]
            format = "{}"
            "#,
                format.to_string().to_lowercase()
            );

            let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
            assert_eq!(config.logging.format, format);
        }
    }

    #[test]
    fn toml_version_1_logging_format_can_use_numeric_value() {
        let format_types = vec![LoggingFormat::Compact, LoggingFormat::Json, LoggingFormat::Pretty];
        for format in format_types {
            let config_content = format!(
                r"
            version = 1

            [logging]
            format = {}
            ",
                format as i64
            );

            let config: Config = toml::from_str(config_content.as_str()).expect("Failed to parse TOML");
            assert_eq!(config.logging.format, format);
        }
    }

    #[test]
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

        let config: Config = toml::from_str(config_content).expect("Failed to parse TOML");
        assert!(config.color.enabled);
        assert_eq!(config.color.settings.link, Color::Green);
        assert_eq!(config.color.settings.unlink, Color::Red);
        assert_eq!(config.color.settings.list, Color::Blue);
        assert_eq!(config.color.settings.remove, Color::Magenta);
        assert_eq!(config.color.settings.create, Color::Yellow);
        assert_eq!(config.color.settings.arrow, Color::Cyan);
        assert_eq!(config.color.settings.source, Color::Black);
        assert_eq!(config.color.settings.target, Color::White);
    }

    #[test]
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

        let config: Config = toml::from_str(config_content).expect("Failed to parse TOML");
        let expected_color = ColorConfig {
            enabled: true,
            settings: ColorSettings {
                link: Color::Green,
                unlink: Color::Red,
                list: Color::Blue,
                remove: Color::Magenta,
                create: Color::Yellow,
                arrow: Color::Cyan,
                source: Color::Black,
                target: Color::Grey,
            },
        };

        assert_eq!(config.color, expected_color);
    }

    #[test]
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

        let config: Config = toml::from_str(config_content).expect("Failed to parse TOML");
        let expected_color = ColorConfig {
            enabled: true,
            settings: ColorSettings {
                link: Color::Rgb {
                    r: 0x27,
                    g: 0xF5,
                    b: 0x4D,
                },
                unlink: Color::Rgb {
                    r: 0xF5,
                    g: 0x49,
                    b: 0x27,
                },
                list: Color::Rgb {
                    r: 0x27,
                    g: 0xF5,
                    b: 0x4D,
                },
                remove: Color::Rgb {
                    r: 0xF5,
                    g: 0x49,
                    b: 0x27,
                },
                create: Color::Rgb {
                    r: 0xF5,
                    g: 0x49,
                    b: 0x27,
                },
                arrow: Color::Rgb {
                    r: 0xF5,
                    g: 0x49,
                    b: 0x27,
                },
                source: Color::Rgb {
                    r: 0xF5,
                    g: 0x49,
                    b: 0x27,
                },
                target: Color::Rgb {
                    r: 0xF5,
                    g: 0x49,
                    b: 0x27,
                },
            },
        };

        assert_eq!(config.color, expected_color);
    }
}
