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

use crate::commands::matcher_error::{MatchError, Pcre2Snafu, RustSnafu};
use crate::commands::regex_captures::RegexCaptures;
use crate::config::RegexStrategy;
use grep::matcher::{Match, Matcher};
use snafu::ResultExt;
use std::fmt::Display;
use tracing::warn;

pub enum RegexMatcher {
    Rust(grep::regex::RegexMatcher),
    Pcre2(grep::pcre2::RegexMatcher),
}

impl RegexMatcher {
    pub fn try_create_matcher<T: AsRef<str> + Display>(regex_strategy: RegexStrategy, item: T) -> Option<Self> {
        match regex_strategy {
            RegexStrategy::Rust => grep::regex::RegexMatcherBuilder::new()
                .build(item.as_ref())
                .map(RegexMatcher::Rust)
                .map_err(|e| warn!("Failed to create Rust regex matcher for {item}: {e}"))
                .ok(),
            RegexStrategy::Pcre2 => grep::pcre2::RegexMatcherBuilder::new()
                .build(item.as_ref())
                .map(RegexMatcher::Pcre2)
                .map_err(|e| warn!("Failed to create PCRE2 regex matcher for {item}: {e}"))
                .ok(),
        }
    }
}

impl Matcher for RegexMatcher {
    type Captures = RegexCaptures;
    type Error = MatchError;

    fn find_at(&self, haystack: &[u8], at: usize) -> Result<Option<Match>, Self::Error> {
        match self {
            Self::Rust(r) => r.find_at(haystack, at).context(RustSnafu),
            Self::Pcre2(r) => r.find_at(haystack, at).context(Pcre2Snafu),
        }
    }

    fn new_captures(&self) -> Result<Self::Captures, Self::Error> {
        match self {
            Self::Rust(r) => r.new_captures().map(RegexCaptures::Rust).context(RustSnafu),
            Self::Pcre2(r) => r
                .new_captures()
                .map(RegexCaptures::Pcre2)
                .context(Pcre2Snafu),
        }
    }
}
