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
pub enum CommandError {
    #[snafu(display("Invalid path {}", path))]
    InvalidPath {
        path: String,
        source: std::io::Error,
    },
    #[snafu(display("Failed to create symlink from {} to {}", target, destination))]
    SymLinkError {
        target: String,
        destination: String,
        source: std::io::Error,
    },
    #[snafu(display("Failed to read next item in the directory"))]
    DirectoryReadError { source: std::io::Error },
    #[snafu(display("Failed to remove file {}", file))]
    FileRemoveError {
        file: String,
        source: std::io::Error,
    },
    #[snafu(display("Failed to remove directory {}", directory))]
    DirectoryRemoveError {
        directory: String,
        source: std::io::Error,
    },
    #[snafu(display("Failed to create directory {}", directory))]
    CreateDirectoryError {
        directory: String,
        source: std::io::Error,
    },
    #[snafu(display("Failed to read link {}", path))]
    ReadLinkError {
        path: String,
        source: std::io::Error,
    },
    #[snafu(display(
        "Invalid target directory: {}.  The target directory must exist and be a directory.",
        directory
    ))]
    InvalidTargetDirectory { directory: String },
    #[snafu(display(
        "Invalid stow directory: {}.  The stow directory must exist and be a directory.",
        directory
    ))]
    StowDirectoryNotFound { directory: String },
    #[snafu(display(
        "Invalid stow directory: {}.  It must not be the same as the target directory.",
        directory
    ))]
    InvalidStowDirectory { directory: String },
    #[snafu(display("Directory Entry Already Exists: {}", directory))]
    DirectoryEntryAlreadyExists { directory: String },
    #[snafu(display(
        "The stow directory contains an invalid item: {}.  It must be a file or directory and not a symbolic link.",
        item
    ))]
    InvalidStowItem { item: String },
}
