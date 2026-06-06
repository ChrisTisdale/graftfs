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

use crate::config::LevelError;
use crate::config::format_error::FormatError;
use crate::config::rotation_error::RotationError;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, de};
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::level_filters::LevelFilter;
use tracing_appender::rolling::Rotation;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
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
            _ => Err(LevelError::InvalidLevelString(s.to_string())),
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
            _ => Err(LevelError::InvalidLevel(value)),
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
            _ => Err(RotationError::InvalidRotationTypeString(s.to_string())),
        }
    }
}

impl TryFrom<i64> for RotationType {
    type Error = RotationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Hourly),
            2 => Ok(Self::Daily),
            _ => Err(RotationError::InvalidRotationType(value)),
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

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(i64)]
pub enum LoggingFormat {
    #[default]
    Compact = 1,
    Pretty,
    Json,
}

impl FromStr for LoggingFormat {
    type Err = FormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.eq_ignore_ascii_case("compact") => Ok(Self::Compact),
            s if s.eq_ignore_ascii_case("pretty") => Ok(Self::Pretty),
            s if s.eq_ignore_ascii_case("json") => Ok(Self::Json),
            _ => Err(FormatError::InvalidFormatTypeString(s.to_string())),
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
            _ => Err(FormatError::InvalidFormatType(value)),
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

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingConfig {
    #[serde(default)]
    pub level: LoggingLevel,
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

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: LoggingLevel::default(),
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
            "LoggingConfig {{ level: {}, file: {:?}, logging_path: {:?}, rotation: {}, logging_format: {}, max_log_files: {}, color_support: {} }}",
            self.level,
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
mod test {
    use super::*;

    #[test]
    fn logging_level_from_str_off() {
        let logging_level = LoggingLevel::from_str("off").unwrap();
        assert_eq!(logging_level, LoggingLevel::Off);
    }

    #[test]
    fn logging_level_from_str_trace() {
        let logging_level = LoggingLevel::from_str("trace").unwrap();
        assert_eq!(logging_level, LoggingLevel::Trace);
    }

    #[test]
    fn logging_level_from_str_debug() {
        let logging_level = LoggingLevel::from_str("debug").unwrap();
        assert_eq!(logging_level, LoggingLevel::Debug);
    }

    #[test]
    fn logging_level_from_str_info() {
        let logging_level = LoggingLevel::from_str("info").unwrap();
        assert_eq!(logging_level, LoggingLevel::Info);
    }

    #[test]
    fn logging_level_from_str_warn() {
        let logging_level = LoggingLevel::from_str("warn").unwrap();
        assert_eq!(logging_level, LoggingLevel::Warn);
    }

    #[test]
    fn logging_level_from_str_error() {
        let logging_level = LoggingLevel::from_str("error").unwrap();
        assert_eq!(logging_level, LoggingLevel::Error);
    }

    #[test]
    fn logging_level_from_str_invalid() {
        let result = LoggingLevel::from_str("invalid");
        match result {
            Err(LevelError::InvalidLevelString(s)) => assert_eq!(s, "invalid"),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn logging_level_from_i64_off() {
        let logging_level = LoggingLevel::try_from(0).unwrap();
        assert_eq!(logging_level, LoggingLevel::Off);
    }

    #[test]
    fn logging_level_from_i64_trace() {
        let logging_level = LoggingLevel::try_from(1).unwrap();
        assert_eq!(logging_level, LoggingLevel::Trace);
    }

    #[test]
    fn logging_level_from_i64_debug() {
        let logging_level = LoggingLevel::try_from(2).unwrap();
        assert_eq!(logging_level, LoggingLevel::Debug);
    }

    #[test]
    fn logging_level_from_i64_info() {
        let logging_level = LoggingLevel::try_from(3).unwrap();
        assert_eq!(logging_level, LoggingLevel::Info);
    }

    #[test]
    fn logging_level_from_i64_warn() {
        let logging_level = LoggingLevel::try_from(4).unwrap();
        assert_eq!(logging_level, LoggingLevel::Warn);
    }

    #[test]
    fn logging_level_from_i64_error() {
        let logging_level = LoggingLevel::try_from(5).unwrap();
        assert_eq!(logging_level, LoggingLevel::Error);
    }

    #[test]
    fn logging_level_from_i64_invalid() {
        let result = LoggingLevel::try_from(-1);
        match result {
            Err(LevelError::InvalidLevel(i)) => assert_eq!(i, -1),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn rotation_type_from_str_hourly() {
        let rotation_type = RotationType::from_str("hourly").unwrap();
        assert_eq!(rotation_type, RotationType::Hourly);
    }

    #[test]
    fn rotation_type_from_str_daily() {
        let rotation_type = RotationType::from_str("daily").unwrap();
        assert_eq!(rotation_type, RotationType::Daily);
    }

    #[test]
    fn rotation_type_from_str_invalid() {
        let result = RotationType::from_str("invalid");
        match result {
            Err(RotationError::InvalidRotationTypeString(s)) => assert_eq!(s, "invalid"),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn rotation_type_from_i64_hourly() {
        let rotation_type = RotationType::try_from(1).unwrap();
        assert_eq!(rotation_type, RotationType::Hourly);
    }

    #[test]
    fn rotation_type_from_i64_daily() {
        let rotation_type = RotationType::try_from(2).unwrap();
        assert_eq!(rotation_type, RotationType::Daily);
    }

    #[test]
    fn invalid_rotation_type_from_i64() {
        let result = RotationType::try_from(-1);
        match result {
            Err(RotationError::InvalidRotationType(i)) => assert_eq!(i, -1),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn logging_level_off_to_level_filter() {
        let level_filter: LevelFilter = LoggingLevel::Off.into();
        assert_eq!(level_filter, LevelFilter::OFF);
    }

    #[test]
    fn logging_level_trace_to_level_filter() {
        let level_filter: LevelFilter = LoggingLevel::Trace.into();
        assert_eq!(level_filter, LevelFilter::TRACE);
    }

    #[test]
    fn logging_level_debug_to_level_filter() {
        let level_filter: LevelFilter = LoggingLevel::Debug.into();
        assert_eq!(level_filter, LevelFilter::DEBUG);
    }

    #[test]
    fn logging_level_info_to_level_filter() {
        let level_filter: LevelFilter = LoggingLevel::Info.into();
        assert_eq!(level_filter, LevelFilter::INFO);
    }

    #[test]
    fn logging_level_warn_to_level_filter() {
        let level_filter: LevelFilter = LoggingLevel::Warn.into();
        assert_eq!(level_filter, LevelFilter::WARN);
    }

    #[test]
    fn logging_level_error_to_level_filter() {
        let level_filter: LevelFilter = LoggingLevel::Error.into();
        assert_eq!(level_filter, LevelFilter::ERROR);
    }

    #[test]
    fn rotation_type_hourly_to_rotation() {
        let rotation: Rotation = RotationType::Hourly.into();
        assert_eq!(rotation, Rotation::HOURLY);
    }

    #[test]
    fn rotation_type_daily_to_rotation() {
        let rotation: Rotation = RotationType::Daily.into();
        assert_eq!(rotation, Rotation::DAILY);
    }

    #[test]
    fn logging_format_compact_from_str_compact() {
        let logging_format = LoggingFormat::from_str("compact").unwrap();
        assert_eq!(logging_format, LoggingFormat::Compact);
    }

    #[test]
    fn logging_format_pretty_from_str_pretty() {
        let logging_format = LoggingFormat::from_str("pretty").unwrap();
        assert_eq!(logging_format, LoggingFormat::Pretty);
    }

    #[test]
    fn logging_format_json_from_str_json() {
        let logging_format = LoggingFormat::from_str("json").unwrap();
        assert_eq!(logging_format, LoggingFormat::Json);
    }

    #[test]
    fn logging_format_compact_from_i64_compact() {
        let logging_format = LoggingFormat::try_from(1).unwrap();
        assert_eq!(logging_format, LoggingFormat::Compact);
    }

    #[test]
    fn logging_format_pretty_from_i64_pretty() {
        let logging_format = LoggingFormat::try_from(2).unwrap();
        assert_eq!(logging_format, LoggingFormat::Pretty);
    }

    #[test]
    fn logging_format_json_from_i64_json() {
        let logging_format = LoggingFormat::try_from(3).unwrap();
        assert_eq!(logging_format, LoggingFormat::Json);
    }
}
