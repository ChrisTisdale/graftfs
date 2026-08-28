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

use grep::matcher::{Captures, Match};

pub enum RegexCaptures {
    Rust(grep::regex::RegexCaptures),
    Pcre2(grep::pcre2::RegexCaptures),
}

impl Captures for RegexCaptures {
    fn len(&self) -> usize {
        match self {
            Self::Rust(c) => c.len(),
            Self::Pcre2(c) => c.len(),
        }
    }

    fn get(&self, i: usize) -> Option<Match> {
        match self {
            Self::Rust(c) => c.get(i),
            Self::Pcre2(c) => c.get(i),
        }
    }
}
