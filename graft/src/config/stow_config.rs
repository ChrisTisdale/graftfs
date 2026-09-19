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

use crate::config::matching_strategy_error::MatchingStrategyError;
use crate::config::{LinkingStrategyError, RegexStrategyError};
use clap::ValueEnum;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, de};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default, ValueEnum)]
#[repr(i64)]
pub enum RegexStrategy {
    #[default]
    Rust,
    Pcre2,
}

impl FromStr for RegexStrategy {
    type Err = RegexStrategyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.eq_ignore_ascii_case("rust") => Ok(Self::Rust),
            s if s.eq_ignore_ascii_case("pcre2") => Ok(Self::Pcre2),
            _ => Err(RegexStrategyError::InvalidRegexStrategyString {
                strategy: s.to_string(),
            }),
        }
    }
}

impl TryFrom<i64> for RegexStrategy {
    type Error = RegexStrategyError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Rust),
            1 => Ok(Self::Pcre2),
            _ => Err(RegexStrategyError::InvalidRegexStrategy { strategy: value }),
        }
    }
}

impl Serialize for RegexStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Self::Rust => "rust",
            Self::Pcre2 => "pcre2",
        })
    }
}

impl<'de> Deserialize<'de> for RegexStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RegexStrategyVisitor;

        impl Visitor<'_> for RegexStrategyVisitor {
            type Value = RegexStrategy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("rust or pcre2")
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

        deserializer.deserialize_any(RegexStrategyVisitor)
    }
}

impl Display for RegexStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rust => write!(f, "rust"),
            Self::Pcre2 => write!(f, "pcre2"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default, ValueEnum)]
#[repr(i64)]
pub enum MatchingStrategy {
    Individual,
    #[default]
    Combined,
}

impl FromStr for MatchingStrategy {
    type Err = MatchingStrategyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.eq_ignore_ascii_case("individual") => Ok(Self::Individual),
            s if s.eq_ignore_ascii_case("combined") => Ok(Self::Combined),
            _ => Err(MatchingStrategyError::InvalidMatchingStrategyString {
                strategy: s.to_string(),
            }),
        }
    }
}

impl TryFrom<i64> for MatchingStrategy {
    type Error = MatchingStrategyError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Individual),
            1 => Ok(Self::Combined),
            _ => Err(MatchingStrategyError::InvalidMatchingStrategy { strategy: value }),
        }
    }
}

impl Serialize for MatchingStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Self::Individual => "individual",
            Self::Combined => "combined",
        })
    }
}

impl<'de> Deserialize<'de> for MatchingStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MatchingStrategyVisitor;

        impl Visitor<'_> for MatchingStrategyVisitor {
            type Value = MatchingStrategy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("individual or combined")
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

        deserializer.deserialize_any(MatchingStrategyVisitor)
    }
}

impl Display for MatchingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Individual => write!(f, "individual"),
            Self::Combined => write!(f, "combined"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default, ValueEnum)]
#[repr(i64)]
pub enum LinkingStrategy {
    #[default]
    Short,
    Full,
}

impl FromStr for LinkingStrategy {
    type Err = LinkingStrategyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.eq_ignore_ascii_case("short") => Ok(Self::Short),
            s if s.eq_ignore_ascii_case("full") => Ok(Self::Full),
            _ => Err(LinkingStrategyError::InvalidLinkingStrategyString {
                strategy: s.to_string(),
            }),
        }
    }
}

impl TryFrom<i64> for LinkingStrategy {
    type Error = LinkingStrategyError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Short),
            1 => Ok(Self::Full),
            _ => Err(LinkingStrategyError::InvalidLinkingStrategy { strategy: value }),
        }
    }
}

impl Serialize for LinkingStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Self::Short => "short",
            Self::Full => "full",
        })
    }
}

impl<'de> Deserialize<'de> for LinkingStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LinkingStrategyVisitor;

        impl Visitor<'_> for LinkingStrategyVisitor {
            type Value = LinkingStrategy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("short or full")
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

        deserializer.deserialize_any(LinkingStrategyVisitor)
    }
}

impl Display for LinkingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Short => write!(f, "short"),
            Self::Full => write!(f, "full"),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct StowConfig {
    #[serde(default)]
    pub linking_strategy: LinkingStrategy,
    #[serde(default)]
    pub regex_strategy: RegexStrategy,
    #[serde(default)]
    pub matching_strategy: MatchingStrategy,
    #[serde(default)]
    pub printing_enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("short", LinkingStrategy::Short)]
    #[case("full", LinkingStrategy::Full)]
    fn linking_strategy_from_str(#[case] linking: &str, #[case] expected_linking: LinkingStrategy) {
        let strategy = <LinkingStrategy as FromStr>::from_str(linking);
        assert!(strategy.is_ok());
        let strategy = strategy.unwrap();
        assert_eq!(strategy, expected_linking);
    }

    #[rstest]
    fn linking_strategy_from_str_invalid() {
        let invalid = <LinkingStrategy as FromStr>::from_str("invalid");
        assert!(invalid.is_err());
    }

    #[rstest]
    #[case(0, LinkingStrategy::Short)]
    #[case(1, LinkingStrategy::Full)]
    fn linking_strategy_from_i64(#[case] linking: i64, #[case] expected_linking: LinkingStrategy) {
        let strategy = LinkingStrategy::try_from(linking);
        assert!(strategy.is_ok());
        let strategy = strategy.unwrap();
        assert_eq!(strategy, expected_linking);
    }

    #[rstest]
    fn linking_strategy_from_i64_invalid() {
        let invalid = LinkingStrategy::try_from(-1);
        assert!(invalid.is_err());
    }

    #[rstest]
    #[case("rust", RegexStrategy::Rust)]
    #[case("pcre2", RegexStrategy::Pcre2)]
    fn regex_strategy_from_str_rust(#[case] regex: &str, #[case] expected_regex: RegexStrategy) {
        let strategy = <RegexStrategy as FromStr>::from_str(regex);
        assert!(strategy.is_ok());
        let strategy = strategy.unwrap();
        assert_eq!(strategy, expected_regex);
    }

    #[rstest]
    fn regex_strategy_from_str_invalid() {
        let invalid = <RegexStrategy as FromStr>::from_str("invalid");
        assert!(invalid.is_err());
    }

    #[rstest]
    #[case(0, RegexStrategy::Rust)]
    #[case(1, RegexStrategy::Pcre2)]
    fn regex_strategy_from_i64_rust(#[case] regex: i64, #[case] expected_regex: RegexStrategy) {
        let strategy = RegexStrategy::try_from(regex);
        assert!(strategy.is_ok());
        let strategy = strategy.unwrap();
        assert_eq!(strategy, expected_regex);
    }

    #[rstest]
    fn regex_strategy_from_i64_invalid() {
        let invalid = RegexStrategy::try_from(-1);
        assert!(invalid.is_err());
    }

    #[rstest]
    #[case("individual", MatchingStrategy::Individual)]
    #[case("combined", MatchingStrategy::Combined)]
    fn matching_strategy_from_str(#[case] matching: &str, #[case] expected_matching: MatchingStrategy) {
        let strategy = <MatchingStrategy as FromStr>::from_str(matching);
        assert!(strategy.is_ok());
        let strategy = strategy.unwrap();
        assert_eq!(strategy, expected_matching);
    }

    #[rstest]
    fn matching_strategy_from_str_invalid() {
        let invalid = <MatchingStrategy as FromStr>::from_str("invalid");
        assert!(invalid.is_err());
    }

    #[rstest]
    #[case(0, MatchingStrategy::Individual)]
    #[case(1, MatchingStrategy::Combined)]
    fn matching_strategy_from_i64(#[case] matching: i64, #[case] expected_matching: MatchingStrategy) {
        let strategy = MatchingStrategy::try_from(matching);
        assert!(strategy.is_ok());
        let strategy = strategy.unwrap();
        assert_eq!(strategy, expected_matching);
    }

    #[rstest]
    fn matching_strategy_from_i64_invalid() {
        let invalid = MatchingStrategy::try_from(-1);
        assert!(invalid.is_err());
    }
}
