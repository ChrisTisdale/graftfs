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
use tracing::dispatcher::SetGlobalDefaultError;

#[derive(Debug, Snafu)]
#[non_exhaustive]
#[snafu(visibility(pub))]
pub enum LoggingError {
    #[snafu(display("Error setting global default logger"))]
    LoggingError { source: SetGlobalDefaultError },
}
