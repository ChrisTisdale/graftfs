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

use crate::config::console_logging_stream_error::ConsoleLoggingStreamError;
use crate::config::format_error::FormatError;
use crate::config::logging_error::LoggingSnafu;
use crate::config::rotation_error::RotationError;
use crate::config::{LevelError, LoggingError};
use clap::ValueEnum;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, de};
use snafu::ResultExt;
use std::fmt::Display;
use std::io::{stderr, stdout};
use std::ops::BitOr;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{env, fs};
use supports_color::Stream;
use tracing::level_filters::LevelFilter;
use tracing::subscriber;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::format::{FmtSpan, Format};
use tracing_subscriber::fmt::{FormatFields, MakeWriter, SubscriberBuilder};

const DEFAULT_LOG_FILE: &str = "graft.log";

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default, ValueEnum)]
#[repr(i64)]
pub enum LoggingLevel {
    Off = 0,
    Trace,
    Debug,
    Info,
    #[default]
    Warn,
    Error,
}

impl FromStr for LoggingLevel {
    type Err = LevelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.eq_ignore_ascii_case("off") => Ok(Self::Off),
            s if s.eq_ignore_ascii_case("trace") => Ok(Self::Trace),
            s if s.eq_ignore_ascii_case("debug") => Ok(Self::Debug),
            s if s.eq_ignore_ascii_case("info") => Ok(Self::Info),
            s if s.eq_ignore_ascii_case("warn") => Ok(Self::Warn),
            s if s.eq_ignore_ascii_case("error") => Ok(Self::Error),
            _ => Err(LevelError::InvalidLevelString {
                level: s.to_string(),
            }),
        }
    }
}

impl TryFrom<i64> for LoggingLevel {
    type Error = LevelError;

    fn try_from(value: i64) -> Result<Self, LevelError> {
        match value {
            0 => Ok(Self::Off),
            1 => Ok(Self::Trace),
            2 => Ok(Self::Debug),
            3 => Ok(Self::Info),
            4 => Ok(Self::Warn),
            5 => Ok(Self::Error),
            _ => Err(LevelError::InvalidLevel { level: value }),
        }
    }
}

impl Serialize for LoggingLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Self::Off => "off",
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        })
    }
}

impl<'de> Deserialize<'de> for LoggingLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LoggingLevelVisitor;

        impl Visitor<'_> for LoggingLevelVisitor {
            type Value = LoggingLevel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("off or trace or debug or info or warn or error")
            }

            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
                v.try_into().map_err(de::Error::custom)
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                v.parse().map_err(de::Error::custom)
            }

            fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
                self.visit_str(&v)
            }
        }

        deserializer.deserialize_any(LoggingLevelVisitor)
    }
}

impl Display for LoggingLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::Trace => write!(f, "Trace"),
            Self::Debug => write!(f, "Debug"),
            Self::Info => write!(f, "Info"),
            Self::Warn => write!(f, "Warn"),
            Self::Error => write!(f, "Error"),
        }
    }
}

impl From<LevelFilter> for LoggingLevel {
    fn from(value: LevelFilter) -> Self {
        match value {
            LevelFilter::OFF => Self::Off,
            LevelFilter::TRACE => Self::Trace,
            LevelFilter::DEBUG => Self::Debug,
            LevelFilter::INFO => Self::Info,
            LevelFilter::WARN => Self::Warn,
            LevelFilter::ERROR => Self::Error,
        }
    }
}

impl From<LoggingLevel> for LevelFilter {
    fn from(value: LoggingLevel) -> Self {
        match value {
            LoggingLevel::Off => Self::OFF,
            LoggingLevel::Trace => Self::TRACE,
            LoggingLevel::Debug => Self::DEBUG,
            LoggingLevel::Info => Self::INFO,
            LoggingLevel::Warn => Self::WARN,
            LoggingLevel::Error => Self::ERROR,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default, ValueEnum)]
#[repr(i64)]
pub enum ConsoleLoggingStream {
    Stdout = 1,
    #[default]
    Stderr,
}

impl ConsoleLoggingStream {
    pub(crate) fn setup_logger(
        self,
        level_filter: LevelFilter,
        logging_format: LoggingFormat,
        color_support: bool,
    ) -> Result<(), LoggingError> {
        match self {
            Self::Stdout => logging_format.setup_logger(level_filter, stdout, color_support),
            Self::Stderr => logging_format.setup_logger(level_filter, stderr, color_support),
        }
    }
}

impl FromStr for ConsoleLoggingStream {
    type Err = ConsoleLoggingStreamError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.eq_ignore_ascii_case("stdout") => Ok(Self::Stdout),
            s if s.eq_ignore_ascii_case("stderr") => Ok(Self::Stderr),
            _ => Err(ConsoleLoggingStreamError::InvalidStreamString {
                stream: s.to_string(),
            }),
        }
    }
}

impl TryFrom<i64> for ConsoleLoggingStream {
    type Error = ConsoleLoggingStreamError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Stdout),
            2 => Ok(Self::Stderr),
            _ => Err(ConsoleLoggingStreamError::InvalidStream { stream: value }),
        }
    }
}

impl Serialize for ConsoleLoggingStream {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Self::Stdout => "stdout",
            Self::Stderr => "stderr",
        })
    }
}

impl<'de> Deserialize<'de> for ConsoleLoggingStream {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ConsoleLoggingStreamVisitor;

        impl Visitor<'_> for ConsoleLoggingStreamVisitor {
            type Value = ConsoleLoggingStream;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("stdout or stderr")
            }

            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
                v.try_into().map_err(de::Error::custom)
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                v.parse().map_err(de::Error::custom)
            }

            fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
                self.visit_str(&v)
            }
        }

        deserializer.deserialize_any(ConsoleLoggingStreamVisitor)
    }
}

impl Display for ConsoleLoggingStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stdout => write!(f, "stdout"),
            Self::Stderr => write!(f, "stderr"),
        }
    }
}

impl From<Stream> for ConsoleLoggingStream {
    fn from(value: Stream) -> Self {
        match value {
            Stream::Stdout => Self::Stdout,
            Stream::Stderr => Self::Stderr,
        }
    }
}

impl From<ConsoleLoggingStream> for Stream {
    fn from(value: ConsoleLoggingStream) -> Self {
        match value {
            ConsoleLoggingStream::Stdout => Self::Stdout,
            ConsoleLoggingStream::Stderr => Self::Stderr,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(i64)]
pub enum RotationType {
    Hourly = 1,
    #[default]
    Daily,
}

impl From<RotationType> for Rotation {
    fn from(value: RotationType) -> Self {
        match value {
            RotationType::Hourly => Self::HOURLY,
            RotationType::Daily => Self::DAILY,
        }
    }
}

impl FromStr for RotationType {
    type Err = RotationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.eq_ignore_ascii_case("hourly") => Ok(Self::Hourly),
            s if s.eq_ignore_ascii_case("daily") => Ok(Self::Daily),
            _ => Err(RotationError::InvalidRotationTypeString {
                rotation_type: s.to_string(),
            }),
        }
    }
}

impl TryFrom<i64> for RotationType {
    type Error = RotationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Hourly),
            2 => Ok(Self::Daily),
            _ => Err(RotationError::InvalidRotationType {
                rotation_type: value,
            }),
        }
    }
}

impl Serialize for RotationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Self::Hourly => "hourly",
            Self::Daily => "daily",
        })
    }
}

impl<'de> Deserialize<'de> for RotationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RotationTypeVisitor;

        impl Visitor<'_> for RotationTypeVisitor {
            type Value = RotationType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("hourly or daily")
            }

            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
                v.try_into().map_err(de::Error::custom)
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                v.parse().map_err(de::Error::custom)
            }

            fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
                self.visit_str(&v)
            }
        }

        deserializer.deserialize_any(RotationTypeVisitor)
    }
}

impl Display for RotationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hourly => write!(f, "Hourly"),
            Self::Daily => write!(f, "Daily"),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, ValueEnum)]
#[repr(i64)]
pub enum LoggingFormat {
    #[default]
    Compact = 1,
    Pretty,
    Json,
}

impl LoggingFormat {
    pub(crate) fn setup_logger<T>(
        self,
        level_filter: LevelFilter,
        stream: T,
        color_support: bool,
    ) -> Result<(), LoggingError>
    where
        T: for<'writer> MakeWriter<'writer> + Send + Sync + 'static,
    {
        match self {
            Self::Compact => {
                let logger = Self::setup_subscriber_builder(tracing_subscriber::fmt().compact(), level_filter)
                    .with_ansi(color_support)
                    .with_writer(stream)
                    .finish();

                subscriber::set_global_default(logger).context(LoggingSnafu)?;
            }
            Self::Pretty => {
                let logger = Self::setup_subscriber_builder(tracing_subscriber::fmt().pretty(), level_filter)
                    .with_ansi(color_support)
                    .with_writer(stream)
                    .finish();

                subscriber::set_global_default(logger).context(LoggingSnafu)?;
            }
            Self::Json => {
                let logger = Self::setup_subscriber_builder(tracing_subscriber::fmt().json(), level_filter)
                    .with_ansi(color_support)
                    .with_writer(stream)
                    .finish();

                subscriber::set_global_default(logger).context(LoggingSnafu)?;
            }
        }

        Ok(())
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

impl FromStr for LoggingFormat {
    type Err = FormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.eq_ignore_ascii_case("compact") => Ok(Self::Compact),
            s if s.eq_ignore_ascii_case("pretty") => Ok(Self::Pretty),
            s if s.eq_ignore_ascii_case("json") => Ok(Self::Json),
            _ => Err(FormatError::InvalidFormatTypeString {
                format: s.to_string(),
            }),
        }
    }
}

impl TryFrom<i64> for LoggingFormat {
    type Error = FormatError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Compact),
            2 => Ok(Self::Pretty),
            3 => Ok(Self::Json),
            _ => Err(FormatError::InvalidFormatType { format: value }),
        }
    }
}

impl Serialize for LoggingFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Self::Compact => "compact",
            Self::Pretty => "pretty",
            Self::Json => "json",
        })
    }
}

impl<'de> Deserialize<'de> for LoggingFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LoggingFormatVisitor;

        impl Visitor<'_> for LoggingFormatVisitor {
            type Value = LoggingFormat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("compact or pretty or json")
            }

            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
                v.try_into().map_err(de::Error::custom)
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                v.parse().map_err(de::Error::custom)
            }

            fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
                self.visit_str(&v)
            }
        }

        deserializer.deserialize_any(LoggingFormatVisitor)
    }
}

impl Display for LoggingFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compact => write!(f, "Compact"),
            Self::Pretty => write!(f, "Pretty"),
            Self::Json => write!(f, "Json"),
        }
    }
}

const fn default_color_support() -> bool {
    true
}

const fn default_max_log_files() -> usize {
    5
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct LoggingConfig {
    #[serde(default)]
    pub level: LoggingLevel,
    #[serde(default)]
    pub stream: ConsoleLoggingStream,
    #[serde(default)]
    pub format: LoggingFormat,
    #[serde(default)]
    pub rotation: RotationType,
    pub file: Option<PathBuf>,
    pub logging_path: Option<PathBuf>,
    #[serde(default = "default_max_log_files")]
    pub max_log_files: usize,
    #[serde(default = "default_color_support")]
    pub color_support: bool,
}

impl LoggingConfig {
    /// Setting up logging for the application using the provided configuration
    ///
    /// # Arguments
    ///
    /// * `override_level`: The level to override the configuration level with
    /// * `override_format`: The format to override the configuration format with
    /// * `override_stream`: The stream to override the configuration stream with
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
    /// use std::error::Error;
    /// use graft::config::{LoggingConfig, LoggingError};
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     use std::env;
    ///
    ///     let config = LoggingConfig::default();
    ///     config.setup_logger(None, None, None)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn setup_logger(
        &self,
        override_level: Option<LoggingLevel>,
        override_format: Option<LoggingFormat>,
        override_stream: Option<ConsoleLoggingStream>,
    ) -> Result<Option<WorkerGuard>, LoggingError> {
        let config_level = override_level.unwrap_or(self.level);
        let config_format = override_format.unwrap_or(self.format);
        if config_level == LoggingLevel::Off {
            return Ok(None);
        }

        let stream = override_stream.unwrap_or(self.stream);
        self.file
            .as_ref()
            .and_then(|d| self.get_rolling_appender(d))
            .map(tracing_appender::non_blocking)
            .map_or_else(
                || self.set_console_logger(config_level.into(), config_format, stream),
                |(appender, guard)| Self::set_file_logger(config_level.into(), config_format, appender, guard),
            )
    }

    fn set_console_logger(
        &self,
        config_level: LevelFilter,
        logging_format: LoggingFormat,
        stream: ConsoleLoggingStream,
    ) -> Result<Option<WorkerGuard>, LoggingError> {
        let color_support = self.color_support && supports_color::on(stream.into()).is_some();
        stream.setup_logger(config_level, logging_format, color_support)?;
        Ok(None)
    }

    fn set_file_logger(
        config_level: LevelFilter,
        logging_format: LoggingFormat,
        appender: NonBlocking,
        guard: WorkerGuard,
    ) -> Result<Option<WorkerGuard>, LoggingError> {
        logging_format.setup_logger(config_level, appender, false)?;
        Ok(Some(guard))
    }

    fn get_rolling_appender(&self, path: &Path) -> Option<RollingFileAppender> {
        self.logging_path
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

    fn get_file_path(p: &PathBuf) -> Option<PathBuf> {
        if p.is_absolute() {
            Some(p.to_owned())
        } else {
            env::current_dir().map_or(None, |c| Some(c.join(p)))
        }
    }

    fn map_file_appender(&self, file_name: String, root: String) -> Option<RollingFileAppender> {
        RollingFileAppender::builder()
            .rotation(self.rotation.into())
            .max_log_files(self.max_log_files)
            .filename_prefix(file_name)
            .build(root)
            .ok()
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: LoggingLevel::default(),
            stream: ConsoleLoggingStream::default(),
            format: LoggingFormat::default(),
            rotation: RotationType::default(),
            file: None,
            logging_path: None,
            max_log_files: default_max_log_files(),
            color_support: default_color_support(),
        }
    }
}

impl Display for LoggingConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LoggingConfig {{ level: {}, stream: {}, file: {:?}, logging_path: {:?}, rotation: {}, logging_format: {}, max_log_files: {}, color_support: {} }}",
            self.level,
            self.stream,
            self.file,
            self.logging_path,
            self.rotation,
            self.format,
            self.max_log_files,
            self.color_support
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("off", LoggingLevel::Off)]
    #[case("trace", LoggingLevel::Trace)]
    #[case("debug", LoggingLevel::Debug)]
    #[case("info", LoggingLevel::Info)]
    #[case("warn", LoggingLevel::Warn)]
    #[case("error", LoggingLevel::Error)]
    fn logging_level_from_str(#[case] level: &str, #[case] expected_level: LoggingLevel) {
        let logging_level = <LoggingLevel as FromStr>::from_str(level).unwrap();
        assert_eq!(logging_level, expected_level);
    }

    #[rstest]
    fn logging_level_from_str_invalid() {
        let result = <LoggingLevel as FromStr>::from_str("invalid");
        match result {
            Err(LevelError::InvalidLevelString { level }) => assert_eq!(level, "invalid"),
            _ => panic!("Unexpected error type"),
        }
    }

    #[rstest]
    #[case(0, LoggingLevel::Off)]
    #[case(1, LoggingLevel::Trace)]
    #[case(2, LoggingLevel::Debug)]
    #[case(3, LoggingLevel::Info)]
    #[case(4, LoggingLevel::Warn)]
    #[case(5, LoggingLevel::Error)]
    fn logging_level_from_i64(#[case] level: i64, #[case] expected_level: LoggingLevel) {
        let logging_level = LoggingLevel::try_from(level).unwrap();
        assert_eq!(logging_level, expected_level);
    }

    #[rstest]
    fn logging_level_from_i64_invalid() {
        let result = LoggingLevel::try_from(-1);
        match result {
            Err(LevelError::InvalidLevel { level }) => assert_eq!(level, -1),
            _ => panic!("Unexpected error type"),
        }
    }

    #[rstest]
    #[case("hourly", RotationType::Hourly)]
    #[case("daily", RotationType::Daily)]
    fn rotation_type_from_str(#[case] rotation: &str, #[case] expected_rotation: RotationType) {
        let rotation_type = RotationType::from_str(rotation).unwrap();
        assert_eq!(rotation_type, expected_rotation);
    }

    #[rstest]
    fn rotation_type_from_str_invalid() {
        let result = RotationType::from_str("invalid");
        match result {
            Err(RotationError::InvalidRotationTypeString { rotation_type: s }) => assert_eq!(s, "invalid"),
            _ => panic!("Unexpected error type"),
        }
    }

    #[rstest]
    #[case(1, RotationType::Hourly)]
    #[case(2, RotationType::Daily)]
    fn rotation_type_from_i64(#[case] rotation: i64, #[case] expected_rotation: RotationType) {
        let rotation_type = RotationType::try_from(rotation).unwrap();
        assert_eq!(rotation_type, expected_rotation);
    }

    #[rstest]
    fn invalid_rotation_type_from_i64() {
        let result = RotationType::try_from(-1);
        match result {
            Err(RotationError::InvalidRotationType { rotation_type: i }) => assert_eq!(i, -1),
            _ => panic!("Unexpected error type"),
        }
    }

    #[rstest]
    #[case(LoggingLevel::Off, LevelFilter::OFF)]
    #[case(LoggingLevel::Trace, LevelFilter::TRACE)]
    #[case(LoggingLevel::Debug, LevelFilter::DEBUG)]
    #[case(LoggingLevel::Info, LevelFilter::INFO)]
    #[case(LoggingLevel::Warn, LevelFilter::WARN)]
    #[case(LoggingLevel::Error, LevelFilter::ERROR)]
    fn logging_level_to_level_filter(#[case] level: LoggingLevel, #[case] expected_filter: LevelFilter) {
        let level_filter: LevelFilter = level.into();
        assert_eq!(level_filter, expected_filter);
    }

    #[rstest]
    #[case(RotationType::Hourly, Rotation::HOURLY)]
    #[case(RotationType::Daily, Rotation::DAILY)]
    fn rotation_type_to_rotation(#[case] rotation_type: RotationType, #[case] expected_rotation: Rotation) {
        let rotation: Rotation = rotation_type.into();
        assert_eq!(rotation, expected_rotation);
    }

    #[rstest]
    #[case("compact", LoggingFormat::Compact)]
    #[case("pretty", LoggingFormat::Pretty)]
    #[case("json", LoggingFormat::Json)]
    fn logging_format_from_str(#[case] format: &str, #[case] expected_format: LoggingFormat) {
        let logging_format = <LoggingFormat as FromStr>::from_str(format).unwrap();
        assert_eq!(logging_format, expected_format);
    }

    #[rstest]
    #[case(1, LoggingFormat::Compact)]
    #[case(2, LoggingFormat::Pretty)]
    #[case(3, LoggingFormat::Json)]
    fn logging_format_from_i64(#[case] format: i64, #[case] expected_format: LoggingFormat) {
        let logging_format = LoggingFormat::try_from(format).unwrap();
        assert_eq!(logging_format, expected_format);
    }

    #[rstest]
    #[case("stdout", ConsoleLoggingStream::Stdout)]
    #[case("stderr", ConsoleLoggingStream::Stderr)]
    fn console_logging_stream_from_str(#[case] stream: &str, #[case] expected_stream: ConsoleLoggingStream) {
        let console_logging_stream = <ConsoleLoggingStream as FromStr>::from_str(stream).unwrap();
        assert_eq!(console_logging_stream, expected_stream);
    }

    #[rstest]
    fn console_logging_stream_from_str_invalid() {
        let console_logging_stream = <ConsoleLoggingStream as FromStr>::from_str("invalid");
        assert!(console_logging_stream.is_err());
    }

    #[rstest]
    #[case(1, ConsoleLoggingStream::Stdout)]
    #[case(2, ConsoleLoggingStream::Stderr)]
    fn console_logging_stream_from_i64(#[case] stream: i64, #[case] expected_stream: ConsoleLoggingStream) {
        let console_logging_stream = ConsoleLoggingStream::try_from(stream).unwrap();
        assert_eq!(console_logging_stream, expected_stream);
    }

    #[rstest]
    fn console_logging_stream_from_i64_invalid() {
        let console_logging_stream = ConsoleLoggingStream::try_from(-1);
        assert!(console_logging_stream.is_err());
    }
}
