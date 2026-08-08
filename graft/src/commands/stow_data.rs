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

use crate::config::LinkingStrategy;
use grep::pcre2::{RegexMatcher, RegexMatcherBuilder};
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use tracing::{debug, warn};

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

impl StowOptions {
    #[must_use]
    pub fn new<T: AsRef<str> + Display + Debug, I: Iterator<Item = T>, O: Iterator<Item = T>>(
        dot_file_prefix: Option<String>,
        linking_strategy: LinkingStrategy,
        no_folding: bool,
        ignored: I,
        overrides: O,
    ) -> Self {
        debug!("Creating ignore matches");
        let ignored = ignored.filter_map(Self::build_ignore_matcher).collect();
        debug!("Creating override matches");
        let overrides = overrides.filter_map(Self::build_override_matcher).collect();
        Self {
            no_folding,
            linking_strategy,
            dot_file_prefix,
            filter: StowFilter { ignored, overrides },
        }
    }

    fn build_ignore_matcher<T: AsRef<str> + Display + Debug>(item: T) -> Option<RegexMatcher> {
        Self::build_matcher("ignored", item)
    }

    fn build_override_matcher<T: AsRef<str> + Display + Debug>(item: T) -> Option<RegexMatcher> {
        Self::build_matcher("override", item)
    }

    fn build_matcher<T: AsRef<str> + Display + Debug>(match_type: &str, item: T) -> Option<RegexMatcher> {
        debug!("Adding {match_type} matched item: {item}");
        match RegexMatcherBuilder::new().build(item.as_ref()) {
            Ok(m) => Some(m),
            Err(e) => {
                warn!("Failed to create file matcher: {e}");
                None
            }
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
