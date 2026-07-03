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

use snafu::Snafu;

#[derive(Debug, Snafu)]
#[non_exhaustive]
#[snafu(visibility(pub))]
pub enum ConfigError {
    #[snafu(display("Failed to read file {file}"))]
    FileReadError {
        file: String,
        source: std::io::Error,
    },
    #[snafu(display("Failed to parse TOML file {file}"))]
    TomlError {
        file: String,
        #[snafu(source(from(toml::de::Error, Box::new)))]
        source: Box<toml::de::Error>,
    },
    #[snafu(display("Failed to strip path prefix {prefix} from {item}"))]
    StripPrefixError {
        prefix: String,
        item: String,
        source: std::path::StripPrefixError,
    },
    #[snafu(display("Unable to find home directory"))]
    UnableToFindHomeDirectory,
    #[snafu(display("Failed to resolve the file {file}"))]
    ResolveError {
        file: String,
        source: crate::config::ResolveError,
    },
}
