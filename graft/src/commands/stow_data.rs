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

use crate::commands::regex_matcher::RegexMatcher;
use crate::config::{LinkingStrategy, MatchingStrategy, RegexStrategy};
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use tracing::{Level, debug, enabled, trace};

#[derive(Default)]
pub struct StowFilter {
    pub(crate) ignored: Vec<RegexMatcher>,
    pub(crate) overrides: Vec<RegexMatcher>,
}

impl Debug for StowFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StowFilter")
            .field("ignored", &self.ignored.len())
            .field("overrides", &self.overrides.len())
            .finish()
    }
}

#[derive(Default, Debug)]
pub struct StowOptions {
    pub(crate) filter: StowFilter,
    pub(crate) dot_file_prefix: Option<String>,
    pub(crate) linking_strategy: LinkingStrategy,
    pub(crate) no_folding: bool,
}

#[derive(Debug)]
pub struct StowData {
    pub(crate) target: PathBuf,
    pub(crate) packages: Vec<PathBuf>,
    pub(crate) options: StowOptions,
}

#[derive(Default)]
pub struct StowStrategies {
    pub linking: LinkingStrategy,
    pub regex: RegexStrategy,
    pub matching: MatchingStrategy,
}

impl StowStrategies {
    fn create_matcher<T: AsRef<str> + Display, I: Iterator<Item = T>>(
        &self,
        regex_strings: I,
        match_type: &str,
    ) -> Vec<RegexMatcher> {
        match self.matching {
            MatchingStrategy::Individual => regex_strings
                .filter_map(|item| {
                    debug!("Adding {match_type} matched item: {item}");
                    RegexMatcher::try_create_matcher(self.regex, item)
                })
                .collect(),
            MatchingStrategy::Combined => {
                let regex_strings = regex_strings.collect::<Vec<T>>();
                if enabled!(Level::DEBUG) {
                    for item in &regex_strings {
                        debug!("Adding {match_type} matched item: {item}");
                    }
                }

                let matcher = RegexMatcher::try_create_combined_matcher(self.regex, &regex_strings);
                matcher.map_or_else(Vec::new, |m| vec![m])
            }
        }
    }
}

impl Display for StowStrategies {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StowStrategies: linking={}, regex={}, matching={}",
            self.linking, self.regex, self.matching
        )
    }
}

impl StowOptions {
    #[must_use]
    pub fn new<T: AsRef<str> + Display, I: Iterator<Item = T>, O: Iterator<Item = T>>(
        dot_file_prefix: Option<String>,
        strategies: &StowStrategies,
        no_folding: bool,
        ignored: I,
        overrides: O,
    ) -> Self {
        trace!(
            "Creating stow options.  dot_file_prefix={}, strategies={{ {} }}, no_folding={}",
            dot_file_prefix.as_deref().unwrap_or_default(),
            strategies,
            no_folding
        );

        debug!("Creating ignore matches");
        let ignored = strategies.create_matcher(ignored, "ignored");

        debug!("Creating override matches");
        let overrides = strategies.create_matcher(overrides, "override");

        Self {
            no_folding,
            linking_strategy: strategies.linking,
            dot_file_prefix,
            filter: StowFilter { ignored, overrides },
        }
    }
}

impl StowData {
    #[must_use]
    pub const fn new(target: PathBuf, packages: Vec<PathBuf>, options: StowOptions) -> Self {
        Self {
            target,
            packages,
            options,
        }
    }
}
