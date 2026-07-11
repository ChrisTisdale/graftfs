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

use crate::commands::ColorSupport;
use crate::config::config_error::FileReadSnafu;
use crate::config::logging_config::LoggingFormat;
use crate::config::logging_error::LoggingSnafu;
use crate::config::{Config, ConfigError, LinkingStrategy, LoggingError};
use snafu::ResultExt;
use std::collections::HashSet;
use std::fmt::Display;
use std::io::stderr;
use std::ops::BitOr;
use std::path::{Path, PathBuf};
use std::{env, fs};
use supports_color::Stream;
use tracing::subscriber;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_appender::rolling::RollingFileAppender;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::format::{FmtSpan, Format};
use tracing_subscriber::fmt::{FormatFields, SubscriberBuilder};

pub const DEFAULT_CONFIG_FILE: &str = ".graft.toml";

const DEFAULT_LOG_FILE: &str = "graft.log";

const DEFAULT_IGNORE: &[&str] = &[
    "RCS",
    ".+,v",
    "CVS",
    r"\.\#.+",
    r"\.cvsignore",
    r"\.svn",
    "_darcs",
    r"\.hg",
    r"\.git",
    r"\.gitignore",
    r"\.gitmodules",
    r"\.jj",
    ".+~",
    r"\#.*\#",
    "^/README.*",
    "^/LICENSE.*",
    "^/COPYING",
    "^/.DS_Store",
];

#[derive(Debug)]
pub struct AppConfiguration {
    config: Config,
    pub ignored: HashSet<String>,
    pub overrides: HashSet<String>,
}

impl Display for AppConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AppConfiguration {{ config: {}, ignored: {:?} }}",
            self.config, self.ignored
        )
    }
}

impl AppConfiguration {
    /// Load the configuration from the provided configuration file
    ///
    /// # Arguments
    ///
    /// * `config_file`: The path to the configuration file
    /// * `ignored`: The set of ignored patterns
    ///
    /// returns: Result<`AppConfiguration`, `ConfigError`>
    ///
    /// # Errors
    /// * `ConfigError::ConfigError` - Returned when the configuration file cannot be read
    /// * `ConfigError::TomlError` - Returned when the configuration file is not a valid Toml File
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use std::error::Error;
    /// use graft::config::{AppConfiguration, ConfigError};
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     use std::env;
    ///
    ///     let configuration = AppConfiguration::load_configuration(None, &env::current_dir()?, HashSet::new(), HashSet::new(), false)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn load_configuration(
        config_file: Option<&Path>,
        search_path: &Path,
        mut ignored: HashSet<String>,
        mut overrides: HashSet<String>,
        no_color: bool,
    ) -> Result<Self, ConfigError> {
        let mut config = Config::from_file(config_file)?;
        if config.ignored.file.is_relative() {
            config.ignored.file = search_path.join(config.ignored.file);
        }

        if let Some(logging_path) = &config.logging.logging_path
            && logging_path.is_relative()
        {
            config.logging.logging_path = Some(search_path.join(logging_path));
        }

        config.logging.color_support = !no_color && config.logging.color_support;
        config.color.enabled = !no_color && config.color.enabled;
        ignored.extend(Self::read_ignore_file(&config, config_file)?);
        overrides.extend(Self::read_override_file(&config)?);
        Ok(Self {
            config,
            ignored,
            overrides,
        })
    }

    /// Setting up logging for the application using the provided configuration
    ///
    /// # Arguments
    ///
    /// * `override_level`: The level to override the configuration level with
    /// * `override_format`: The format to override the configuration format with
    ///
    /// returns: `Result<Option<WorkerGuard>`, `LoggingError`>
    /// The guard for the log file, if any, is returned
    ///
    /// # Errors
    /// * `LoggingError::LoggingError` - Returned when the logger cannot be set up
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use std::error::Error;
    /// use graft::config::{AppConfiguration, LoggingError};
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     use std::env;
    ///
    ///     let configuration = AppConfiguration::load_configuration(None, &env::current_dir()?, HashSet::new(), HashSet::new(), false)?;
    ///     configuration.setup_logger(None, None)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn setup_logger(
        &self,
        override_level: Option<LevelFilter>,
        override_format: Option<LoggingFormat>,
    ) -> Result<Option<WorkerGuard>, LoggingError> {
        let config_level = override_level.unwrap_or_else(|| self.config.logging.level.into());
        let config_format = override_format.unwrap_or(self.config.logging.format);
        if config_level == LevelFilter::OFF {
            return Ok(None);
        }

        self.config
            .logging
            .file
            .as_ref()
            .and_then(|d| self.get_rolling_appender(d))
            .map(tracing_appender::non_blocking)
            .map_or_else(
                || self.set_console_logger(config_level, config_format),
                |(appender, guard)| Self::set_file_logger(config_level, config_format, appender, guard),
            )
    }

    #[must_use]
    pub fn color_support(&self) -> ColorSupport {
        if self.config.color.enabled && supports_color::on(Stream::Stdout).is_some() {
            ColorSupport::Colored(self.config.color.settings.clone())
        } else {
            ColorSupport::None
        }
    }

    #[must_use]
    pub const fn linking_strategy(&self) -> LinkingStrategy {
        self.config.stow.linking_strategy
    }

    fn set_file_logger(
        config_level: LevelFilter,
        logging_format: LoggingFormat,
        appender: NonBlocking,
        guard: WorkerGuard,
    ) -> Result<Option<WorkerGuard>, LoggingError> {
        match logging_format {
            LoggingFormat::Compact => {
                let subscriber = Self::setup_subscriber_builder(tracing_subscriber::fmt().compact(), config_level)
                    .with_ansi(false)
                    .with_writer(appender)
                    .finish();
                subscriber::set_global_default(subscriber).context(LoggingSnafu)?;
            }
            LoggingFormat::Pretty => {
                let subscriber = Self::setup_subscriber_builder(tracing_subscriber::fmt().pretty(), config_level)
                    .with_ansi(false)
                    .with_writer(appender)
                    .finish();
                subscriber::set_global_default(subscriber).context(LoggingSnafu)?;
            }
            LoggingFormat::Json => {
                let subscriber = Self::setup_subscriber_builder(tracing_subscriber::fmt().json(), config_level)
                    .with_ansi(false)
                    .with_writer(appender)
                    .finish();
                subscriber::set_global_default(subscriber).context(LoggingSnafu)?;
            }
        }

        Ok(Some(guard))
    }

    fn set_console_logger(
        &self,
        config_level: LevelFilter,
        logging_format: LoggingFormat,
    ) -> Result<Option<WorkerGuard>, LoggingError> {
        let color_support = self.config.logging.color_support && supports_color::on(Stream::Stderr).is_some();
        match logging_format {
            LoggingFormat::Compact => {
                let subscriber = Self::setup_subscriber_builder(tracing_subscriber::fmt().compact(), config_level)
                    .with_ansi(color_support)
                    .with_writer(stderr)
                    .finish();
                subscriber::set_global_default(subscriber).context(LoggingSnafu)?;
            }
            LoggingFormat::Pretty => {
                let subscriber = Self::setup_subscriber_builder(tracing_subscriber::fmt().pretty(), config_level)
                    .with_ansi(color_support)
                    .with_writer(stderr)
                    .finish();
                subscriber::set_global_default(subscriber).context(LoggingSnafu)?;
            }
            LoggingFormat::Json => {
                let subscriber = Self::setup_subscriber_builder(tracing_subscriber::fmt().json(), config_level)
                    .with_ansi(color_support)
                    .with_writer(stderr)
                    .finish();
                subscriber::set_global_default(subscriber).context(LoggingSnafu)?;
            }
        }

        Ok(None)
    }

    fn build_file_pattern(path: Option<&Path>) -> Option<String> {
        path.and_then(|p| p.file_name())
            .and_then(|p| p.to_str())
            .map(|p| format!("^/{p}"))
    }

    fn read_ignore_file(config: &Config, config_file: Option<&Path>) -> Result<HashSet<String>, ConfigError> {
        let mut files = HashSet::new();
        if let Some(file_string) = Self::build_file_pattern(Some(config.ignored.file.as_path())) {
            files.insert(file_string);
        }

        if let Some(file_string) = Self::build_file_pattern(config_file) {
            files.insert(file_string);
        }

        if !fs::exists(config.ignored.file.as_path()).unwrap_or(false) {
            files.extend(DEFAULT_IGNORE.iter().map(ToString::to_string));
            return Ok(files);
        }

        Self::read_ignore_or_override_file(config.ignored.file.as_path(), config.ignored.comment, files)
    }

    fn read_override_file(config: &Config) -> Result<HashSet<String>, ConfigError> {
        let files = HashSet::new();
        if !fs::exists(config.overrides.file.as_path()).unwrap_or(false) {
            return Ok(files);
        }

        Self::read_ignore_or_override_file(
            config.overrides.file.as_path(),
            config.overrides.comment,
            files,
        )
    }

    fn read_ignore_or_override_file(
        file: &Path,
        comment: char,
        mut files: HashSet<String>,
    ) -> Result<HashSet<String>, ConfigError> {
        let content = fs::read_to_string(file).with_context(|_| FileReadSnafu {
            file: file.display().to_string(),
        })?;
        let items = content
            .lines()
            .filter(|line| !line.is_empty() && !line.starts_with(comment))
            .map(|line| Self::parse_line(comment, line))
            .filter(|line| !line.is_empty())
            .map(ToString::to_string);

        files.extend(items);
        Ok(files)
    }

    fn parse_line(comment: char, line: &str) -> &str {
        let mut has_escaped_backslash = false;
        for (i, c) in line.char_indices() {
            if !has_escaped_backslash && c == comment {
                return line[..i].trim();
            }

            has_escaped_backslash = c == '\\' && !has_escaped_backslash;
        }

        line.trim()
    }

    fn get_log_path(root: &Path, dir: &Path) -> PathBuf {
        if dir.is_absolute() {
            dir.to_owned()
        } else {
            root.join(dir)
        }
    }

    fn create_directory_if_necessary(dir: &Path) -> Result<(), std::io::Error> {
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }

        Ok(())
    }

    fn get_rolling_appender(&self, path: &Path) -> Option<RollingFileAppender> {
        self.config
            .logging
            .logging_path
            .as_ref()
            .and_then(Self::get_file_path)
            .and_then(|dir| Self::try_make_log_path(path, &dir))
            .map_or_else(|| None, |root| self.setup_rolling_appender(path, root))
    }

    fn setup_rolling_appender(&self, path: &Path, root: String) -> Option<RollingFileAppender> {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(DEFAULT_LOG_FILE)
            .to_string();
        self.map_file_appender(file_name, root)
    }

    fn try_make_log_path(path: &Path, dir: &Path) -> Option<String> {
        path.parent()
            .map(|d| Self::get_log_path(dir, d))
            .and_then(|d| Self::create_directory_if_necessary(&d).ok().map(|()| d))
            .and_then(|n| n.to_str().map(ToString::to_string))
            .or_else(|| dir.to_str().map(ToString::to_string))
    }

    fn get_file_path(p: &PathBuf) -> Option<PathBuf> {
        if p.is_absolute() {
            Some(p.to_owned())
        } else {
            env::current_dir().map_or(None, |c| Some(c.join(p)))
        }
    }

    fn map_file_appender(&self, file_name: String, root: String) -> Option<RollingFileAppender> {
        RollingFileAppender::builder()
            .rotation(self.config.logging.rotation.into())
            .max_log_files(self.config.logging.max_log_files)
            .filename_prefix(file_name)
            .build(root)
            .ok()
    }

    fn setup_subscriber_builder<TFields, TFormat>(
        subscriber_builder: SubscriberBuilder<TFields, Format<TFormat>>,
        log_level: LevelFilter,
    ) -> SubscriberBuilder<TFields, Format<TFormat>>
    where
        TFields: for<'writer> FormatFields<'writer> + 'static,
    {
        subscriber_builder
            .with_level(true)
            .with_max_level(log_level)
            .with_file(true)
            .log_internal_errors(true)
            .with_span_events(FmtSpan::ENTER.bitor(FmtSpan::CLOSE).bitor(FmtSpan::EXIT))
            .with_line_number(false)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_target(true)
    }
}
