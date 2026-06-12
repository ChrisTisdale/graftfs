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

use crate::commands::ColorSupport;
use std::path::PathBuf;
use tracing::instrument;

#[derive(Debug)]
pub struct ListData {
    pub(crate) target: PathBuf,
    pub(crate) directory: PathBuf,
    pub(crate) dot_file_prefix: Option<String>,
    pub(crate) color_support: ColorSupport,
}

impl ListData {
    #[must_use]
    #[instrument(level = "trace")]
    pub fn new(
        target: PathBuf,
        directory: PathBuf,
        dot_file_prefix: Option<String>,
        color_support: ColorSupport,
    ) -> Self {
        Self {
            target,
            directory,
            dot_file_prefix,
            color_support,
        }
    }
}
