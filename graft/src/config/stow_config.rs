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

use crate::config::LinkingStrategyError;
use clap::ValueEnum;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, de};
use std::fmt::Display;
use std::str::FromStr;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linking_strategy_from_str_short() {
        let soft = <LinkingStrategy as FromStr>::from_str("short");
        assert!(soft.is_ok());
        let soft = soft.unwrap();
        assert_eq!(soft, LinkingStrategy::Short);
    }

    #[test]
    fn linking_strategy_from_str_full() {
        let trace = <LinkingStrategy as FromStr>::from_str("full");
        assert!(trace.is_ok());
        let trace = trace.unwrap();
        assert_eq!(trace, LinkingStrategy::Full);
    }

    #[test]
    fn linking_strategy_from_str_invalid() {
        let invalid = <LinkingStrategy as FromStr>::from_str("invalid");
        assert!(invalid.is_err());
    }

    #[test]
    fn linking_strategy_from_i64_soft() {
        let soft = LinkingStrategy::try_from(0);
        assert!(soft.is_ok());
        let soft = soft.unwrap();
        assert_eq!(soft, LinkingStrategy::Short);
    }

    #[test]
    fn linking_strategy_from_i64_full() {
        let full = LinkingStrategy::try_from(1);
        assert!(full.is_ok());
        let full = full.unwrap();
        assert_eq!(full, LinkingStrategy::Full);
    }

    #[test]
    fn linking_strategy_from_i64_invalid() {
        let invalid = LinkingStrategy::try_from(-1);
        assert!(invalid.is_err());
    }
}
