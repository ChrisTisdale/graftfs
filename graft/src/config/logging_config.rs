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
use crate::config::logging_error::LoggingSnafu;
use crate::config::visitor::StringOrI64Visitor;
use crate::config::{FormatError, LevelError, LoggingError, RotationError, TracingSpanError};
use clap::ValueEnum;
use serde::{Deserialize, Deserializer, Serialize};
use snafu::ResultExt;
use std::fmt::Display;
use std::io::{stderr, stdout};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};
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

#[cfg(feature = "external-logging")]
use tracing_log::{LogTracer, log};

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
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(StringOrI64Visitor::<Self>::new(
            "off or trace or debug or info or warn or error",
        ))
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

#[cfg(feature = "external-logging")]
impl From<log::LevelFilter> for LoggingLevel {
    fn from(value: log::LevelFilter) -> Self {
        match value {
            log::LevelFilter::Off => Self::Off,
            log::LevelFilter::Trace => Self::Trace,
            log::LevelFilter::Debug => Self::Debug,
            log::LevelFilter::Info => Self::Info,
            log::LevelFilter::Warn => Self::Warn,
            log::LevelFilter::Error => Self::Error,
        }
    }
}

#[cfg(feature = "external-logging")]
impl From<LoggingLevel> for log::LevelFilter {
    fn from(value: LoggingLevel) -> Self {
        match value {
            LoggingLevel::Off => Self::Off,
            LoggingLevel::Trace => Self::Trace,
            LoggingLevel::Debug => Self::Debug,
            LoggingLevel::Info => Self::Info,
            LoggingLevel::Warn => Self::Warn,
            LoggingLevel::Error => Self::Error,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct TracingSpan(pub(crate) u8);

impl TracingSpan {
    pub const NONE: Self = Self(0b0000);
    pub const NEW: Self = Self(0b0001);
    pub const ENTER: Self = Self(0b0010);
    pub const EXIT: Self = Self(0b0100);
    pub const CLOSE: Self = Self(0b1000);
    pub const ALL: Self = Self(0b1111);

    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    #[must_use]
    pub fn iter_names(self) -> TracingSpanNamesIterator {
        TracingSpanNamesIterator(self.into_iter())
    }

    #[must_use]
    pub fn iter_all() -> TracingSpanIterator {
        Self::ALL.into_iter()
    }

    #[must_use]
    pub fn iter_all_names() -> TracingSpanNamesIterator {
        TracingSpanNamesIterator(Self::ALL.into_iter())
    }
}

impl Default for TracingSpan {
    fn default() -> Self {
        Self::ENTER | Self::CLOSE | Self::EXIT
    }
}

impl TryFrom<u8> for TracingSpan {
    type Error = TracingSpanError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > Self::ALL.0 {
            return Err(TracingSpanError::UnsupportedTracingSpan { span: value.into() });
        }

        Ok(Self(value))
    }
}

impl TryFrom<i64> for TracingSpan {
    type Error = TracingSpanError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        u8::try_from(value)
            .map_err(|_| TracingSpanError::UnsupportedTracingSpan { span: value })
            .and_then(Self::try_from)
    }
}

impl FromStr for TracingSpan {
    type Err = TracingSpanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            value if value.eq_ignore_ascii_case("none") => Ok(Self::NONE),
            value if value.eq_ignore_ascii_case("all") => Ok(Self::ALL),
            _ => s
                .split('|')
                .map(str::trim)
                .try_fold(Self::NONE, |c, v| match v {
                    value if value.eq_ignore_ascii_case("new") => Ok(c | Self::NEW),
                    value if value.eq_ignore_ascii_case("enter") => Ok(c | Self::ENTER),
                    value if value.eq_ignore_ascii_case("exit") => Ok(c | Self::EXIT),
                    value if value.eq_ignore_ascii_case("close") => Ok(c | Self::CLOSE),
                    _ => Err(TracingSpanError::UnsupportedTracingSpanString {
                        span: v.to_string(),
                    }),
                }),
        }
    }
}

impl Serialize for TracingSpan {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for TracingSpan {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(StringOrI64Visitor::<Self>::new(
            "none or new or enter or exit or close or all",
        ))
    }
}

impl BitOr for TracingSpan {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for TracingSpan {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAndAssign for TracingSpan {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitAnd for TracingSpan {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitXor for TracingSpan {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for TracingSpan {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for TracingSpan {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0 & Self::ALL.0)
    }
}

pub struct TracingSpanIterator {
    span: TracingSpan,
    current: u8,
}

impl Iterator for TracingSpanIterator {
    type Item = TracingSpan;

    fn next(&mut self) -> Option<Self::Item> {
        if self.span == TracingSpan::NONE {
            return None;
        }

        loop {
            if self.current & self.span.0 == 0 {
                self.current <<= 1;
                continue;
            }

            let value = self.current;
            self.span = TracingSpan(self.span.0 & !value);
            self.current = value << 1;
            return Some(TracingSpan(value));
        }
    }
}

pub struct TracingSpanNamesIterator(TracingSpanIterator);

impl Iterator for TracingSpanNamesIterator {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().and_then(|v| match v {
            TracingSpan::NEW => Some("New"),
            TracingSpan::ENTER => Some("Enter"),
            TracingSpan::EXIT => Some("Exit"),
            TracingSpan::CLOSE => Some("Close"),
            _ => None,
        })
    }
}

impl IntoIterator for TracingSpan {
    type Item = Self;
    type IntoIter = TracingSpanIterator;

    fn into_iter(self) -> Self::IntoIter {
        TracingSpanIterator {
            span: self,
            current: 1,
        }
    }
}

impl From<TracingSpan> for FmtSpan {
    fn from(value: TracingSpan) -> Self {
        value.into_iter().fold(Self::NONE, |c, v| match v {
            TracingSpan::NEW => c | Self::NEW,
            TracingSpan::ENTER => c | Self::ENTER,
            TracingSpan::EXIT => c | Self::EXIT,
            TracingSpan::CLOSE => c | Self::CLOSE,
            _ => c,
        })
    }
}

impl Display for TracingSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const ALL: u8 = TracingSpan::ALL.0;
        match self {
            Self(0) => return f.write_str("None"),
            Self(ALL) => return f.write_str("All"),
            _ => {
                let mut first = true;
                for name in self.iter_names() {
                    if !first {
                        f.write_str(" | ")?;
                    }

                    first = false;
                    f.write_str(name)?;
                }
            }
        }

        Ok(())
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
        log_level: LoggingLevel,
        logging_format: LoggingFormat,
        tracing_span: TracingSpan,
        color_support: bool,
    ) -> Result<(), LoggingError> {
        match self {
            Self::Stdout => logging_format.setup_logger(log_level, tracing_span, stdout, color_support),
            Self::Stderr => logging_format.setup_logger(log_level, tracing_span, stderr, color_support),
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
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(StringOrI64Visitor::<Self>::new("stdout or stderr"))
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
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(StringOrI64Visitor::<Self>::new("hourly or daily"))
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
        log_level: LoggingLevel,
        tracing_span: TracingSpan,
        stream: T,
        color_support: bool,
    ) -> Result<(), LoggingError>
    where
        T: for<'writer> MakeWriter<'writer> + Send + Sync + 'static,
    {
        #[cfg(feature = "external-logging")]
        {
            LogTracer::builder()
                .with_max_level(log_level)
                .init()
                .context(crate::config::logging_error::SetLoggerSnafu)?;
        }

        match self {
            Self::Compact => {
                let logger =
                    Self::setup_subscriber_builder(tracing_subscriber::fmt().compact(), log_level, tracing_span)
                        .with_ansi(color_support)
                        .with_writer(stream)
                        .finish();

                subscriber::set_global_default(logger).context(LoggingSnafu)?;
            }
            Self::Pretty => {
                let logger =
                    Self::setup_subscriber_builder(tracing_subscriber::fmt().pretty(), log_level, tracing_span)
                        .with_ansi(color_support)
                        .with_writer(stream)
                        .finish();

                subscriber::set_global_default(logger).context(LoggingSnafu)?;
            }
            Self::Json => {
                let logger = Self::setup_subscriber_builder(tracing_subscriber::fmt().json(), log_level, tracing_span)
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
        log_level: LoggingLevel,
        tracing_span: TracingSpan,
    ) -> SubscriberBuilder<TFields, Format<TFormat>>
    where
        TFields: for<'writer> FormatFields<'writer> + 'static,
    {
        subscriber_builder
            .with_level(true)
            .with_max_level(log_level)
            .with_file(true)
            .log_internal_errors(true)
            .with_span_events(tracing_span.into())
            .with_line_number(true)
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
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(StringOrI64Visitor::<Self>::new("compact or pretty or json"))
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
    #[serde(default)]
    pub tracing_span: TracingSpan,
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
    ///     config.setup_logger(None, None, None, None)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn setup_logger(
        &self,
        override_level: Option<LoggingLevel>,
        override_format: Option<LoggingFormat>,
        override_stream: Option<ConsoleLoggingStream>,
        override_span: Option<TracingSpan>,
    ) -> Result<Option<WorkerGuard>, LoggingError> {
        let config_level = override_level.unwrap_or(self.level);
        let config_format = override_format.unwrap_or(self.format);
        if config_level == LoggingLevel::Off {
            return Ok(None);
        }

        let stream = override_stream.unwrap_or(self.stream);
        let span = override_span.unwrap_or(self.tracing_span);
        let guard = self
            .file
            .as_ref()
            .and_then(|d| self.get_rolling_appender(d))
            .map(tracing_appender::non_blocking)
            .map_or_else(
                || self.set_console_logger(config_level, config_format, span, stream),
                |(appender, guard)| Self::set_file_logger(config_level, config_format, span, appender, guard),
            )?;

        Ok(guard)
    }

    fn set_console_logger(
        &self,
        config_level: LoggingLevel,
        logging_format: LoggingFormat,
        tracing_span: TracingSpan,
        stream: ConsoleLoggingStream,
    ) -> Result<Option<WorkerGuard>, LoggingError> {
        let color_support = self.color_support && supports_color::on(stream.into()).is_some();
        stream.setup_logger(config_level, logging_format, tracing_span, color_support)?;
        Ok(None)
    }

    fn set_file_logger(
        config_level: LoggingLevel,
        logging_format: LoggingFormat,
        tracing_span: TracingSpan,
        appender: NonBlocking,
        guard: WorkerGuard,
    ) -> Result<Option<WorkerGuard>, LoggingError> {
        logging_format.setup_logger(config_level, tracing_span, appender, false)?;
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
            tracing_span: TracingSpan::default(),
            max_log_files: default_max_log_files(),
            color_support: default_color_support(),
        }
    }
}

impl Display for LoggingConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LoggingConfig {{ level: {}, stream: {}, file: {:?}, logging_path: {:?}, tracing_span: \"{}\", rotation: {}, logging_format: {}, max_log_files: {}, color_support: {} }}",
            self.level,
            self.stream,
            self.file,
            self.logging_path,
            self.tracing_span,
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

    #[rstest]
    #[case(0b0000, TracingSpan::NONE)]
    #[case(0b0001, TracingSpan::NEW)]
    #[case(0b0010, TracingSpan::ENTER)]
    #[case(0b0100, TracingSpan::EXIT)]
    #[case(0b1000, TracingSpan::CLOSE)]
    #[case(0b1111, TracingSpan::ALL)]
    #[case(0b1110, TracingSpan::ENTER | TracingSpan::EXIT | TracingSpan::CLOSE)]
    #[case(0b1101, TracingSpan::NEW | TracingSpan::EXIT | TracingSpan::CLOSE)]
    #[case(0b1011, TracingSpan::NEW | TracingSpan::ENTER | TracingSpan::CLOSE)]
    #[case(0b0111, TracingSpan::NEW | TracingSpan::ENTER | TracingSpan::EXIT)]
    #[case(0b1001, TracingSpan::NEW | TracingSpan::CLOSE)]
    #[case(0b0011, TracingSpan::NEW | TracingSpan::ENTER)]
    #[case(0b0110, TracingSpan::ENTER | TracingSpan::EXIT)]
    #[case(0b1100, TracingSpan::EXIT | TracingSpan::CLOSE)]
    #[case(0b0101, TracingSpan::NEW | TracingSpan::EXIT)]
    #[case(0b1010, TracingSpan::ENTER | TracingSpan::CLOSE)]
    fn tracing_span_from_i64(#[case] span: i64, #[case] expected_span: TracingSpan) {
        let tracing_span = TracingSpan::try_from(span);
        assert!(tracing_span.is_ok());
        let tracing_span = tracing_span.unwrap();
        assert_eq!(tracing_span, expected_span);
    }

    #[rstest]
    #[case("None", TracingSpan::NONE)]
    #[case("New", TracingSpan::NEW)]
    #[case("Enter", TracingSpan::ENTER)]
    #[case("Exit", TracingSpan::EXIT)]
    #[case("Close", TracingSpan::CLOSE)]
    #[case("All", TracingSpan::ALL)]
    #[case("New | Enter | Exit | Close", TracingSpan::NEW | TracingSpan::ENTER | TracingSpan::EXIT | TracingSpan::CLOSE)]
    #[case("New | Exit | Close", TracingSpan::NEW | TracingSpan::EXIT | TracingSpan::CLOSE)]
    #[case("New | Enter | Close", TracingSpan::NEW | TracingSpan::ENTER | TracingSpan::CLOSE)]
    #[case("New | Enter | Exit", TracingSpan::NEW | TracingSpan::ENTER | TracingSpan::EXIT)]
    #[case("Enter | Exit | Close", TracingSpan::ENTER | TracingSpan::EXIT | TracingSpan::CLOSE)]
    #[case("New | Close", TracingSpan::NEW | TracingSpan::CLOSE)]
    #[case("New | Enter", TracingSpan::NEW | TracingSpan::ENTER)]
    #[case("New | Exit", TracingSpan::NEW | TracingSpan::EXIT)]
    #[case("Enter | Exit", TracingSpan::ENTER | TracingSpan::EXIT)]
    #[case("Enter | Close", TracingSpan::ENTER | TracingSpan::CLOSE)]
    #[case("Exit | Close", TracingSpan::EXIT | TracingSpan::CLOSE)]
    fn tracing_span_from_str(#[case] span: &str, #[case] expected_span: TracingSpan) {
        let tracing_span = <TracingSpan as FromStr>::from_str(span);
        assert!(tracing_span.is_ok());
        let tracing_span = tracing_span.unwrap();
        assert_eq!(tracing_span, expected_span);
    }

    #[rstest]
    #[case(TracingSpan::NONE, "None")]
    #[case(TracingSpan::NEW, "New")]
    #[case(TracingSpan::ENTER, "Enter")]
    #[case(TracingSpan::EXIT, "Exit")]
    #[case(TracingSpan::CLOSE, "Close")]
    #[case(TracingSpan::ALL, "All")]
    #[case(TracingSpan::NEW | TracingSpan::EXIT | TracingSpan::CLOSE, "New | Exit | Close")]
    #[case(TracingSpan::NEW | TracingSpan::ENTER | TracingSpan::CLOSE, "New | Enter | Close")]
    #[case(TracingSpan::NEW | TracingSpan::ENTER | TracingSpan::EXIT, "New | Enter | Exit")]
    #[case(TracingSpan::ENTER | TracingSpan::EXIT | TracingSpan::CLOSE, "Enter | Exit | Close")]
    #[case(TracingSpan::NEW | TracingSpan::CLOSE, "New | Close")]
    #[case(TracingSpan::NEW | TracingSpan::ENTER, "New | Enter")]
    #[case(TracingSpan::NEW | TracingSpan::EXIT, "New | Exit")]
    #[case(TracingSpan::ENTER | TracingSpan::EXIT, "Enter | Exit")]
    #[case(TracingSpan::ENTER | TracingSpan::CLOSE, "Enter | Close")]
    #[case(TracingSpan::EXIT | TracingSpan::CLOSE, "Exit | Close")]
    fn tracing_span_to_string(#[case] span: TracingSpan, #[case] expected_result: &str) {
        let tracing_span = span.to_string();
        assert_eq!(tracing_span, expected_result);
    }

    #[rstest]
    #[case(TracingSpan::NONE, FmtSpan::NONE)]
    #[case(TracingSpan::NEW, FmtSpan::NEW)]
    #[case(TracingSpan::ENTER, FmtSpan::ENTER)]
    #[case(TracingSpan::EXIT, FmtSpan::EXIT)]
    #[case(TracingSpan::CLOSE, FmtSpan::CLOSE)]
    #[case(TracingSpan::ALL, FmtSpan::NEW | FmtSpan::ENTER | FmtSpan::EXIT | FmtSpan::CLOSE)]
    #[case(TracingSpan::NEW | TracingSpan::EXIT | TracingSpan::CLOSE, FmtSpan::NEW | FmtSpan::EXIT | FmtSpan::CLOSE)]
    #[case(TracingSpan::NEW | TracingSpan::ENTER | TracingSpan::CLOSE, FmtSpan::NEW | FmtSpan::ENTER | FmtSpan::CLOSE)]
    #[case(TracingSpan::NEW | TracingSpan::ENTER | TracingSpan::EXIT, FmtSpan::NEW | FmtSpan::ENTER | FmtSpan::EXIT)]
    #[case(TracingSpan::ENTER | TracingSpan::EXIT | TracingSpan::CLOSE, FmtSpan::ENTER | FmtSpan::EXIT | FmtSpan::CLOSE)]
    #[case(TracingSpan::NEW | TracingSpan::CLOSE, FmtSpan::NEW | FmtSpan::CLOSE)]
    #[case(TracingSpan::NEW | TracingSpan::ENTER, FmtSpan::NEW | FmtSpan::ENTER)]
    #[case(TracingSpan::NEW | TracingSpan::EXIT, FmtSpan::NEW | FmtSpan::EXIT)]
    #[case(TracingSpan::ENTER | TracingSpan::EXIT, FmtSpan::ENTER | FmtSpan::EXIT)]
    #[case(TracingSpan::ENTER | TracingSpan::CLOSE, FmtSpan::ENTER | FmtSpan::CLOSE)]
    #[case(TracingSpan::EXIT | TracingSpan::CLOSE, FmtSpan::EXIT | FmtSpan::CLOSE)]
    fn tracing_span_into_fmt_span(#[case] span: TracingSpan, #[case] expected_result: FmtSpan) {
        let fmt_span: FmtSpan = span.into();
        assert_eq!(fmt_span, expected_result);
    }
}
