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
pub enum CliError {
    #[snafu(display("Failed to setup logging"))]
    LoggingError { source: crate::config::LoggingError },
    #[snafu(display("Invalid path: {path}"))]
    InvalidPath {
        path: String,
        source: std::io::Error,
    },
    #[snafu(display("Failed to parse command line arguments"))]
    CommandLineParsingError { source: clap::Error },
    #[snafu(display("Failed to parse command line arguments"))]
    MatchingError { source: clap::parser::MatchesError },
    #[snafu(display("Invalid configuration file: {file}"))]
    InvalidConfigFile {
        file: String,
        source: crate::config::ConfigError,
    },
    #[snafu(display("Failed to handle the requested command"))]
    CommandError {
        source: crate::commands::CommandError,
    },
    #[snafu(display("Invalid target directory.  The target directory must exist and be a directory."))]
    InvalidTargetDirectory,
    #[snafu(display("Invalid configuration file: {file}"))]
    InvalidConfigurationFile { file: String },
    #[snafu(display("Failed to build the request command {command}"))]
    CommandBuildError {
        command: String,
        source: crate::commands::CommandBuildError,
    },
    #[snafu(display("Failed to resolve the file {file}"))]
    ResolveError {
        file: String,
        source: crate::config::ResolveError,
    },
    #[snafu(display("Printing completions requested."))]
    PrintCompletions {
        printer: crate::command_line_args::CompletionPrinter,
    },
    #[snafu(display("Invalid or unknown shell.  Please specify a valid shell."))]
    InvalidShell,
    #[snafu(display("Failed to create output file {path}"))]
    OutputFileCreationError {
        path: String,
        source: std::io::Error,
    },
    #[snafu(display("Failed to generate completions"))]
    GenerateCompletionsError { source: std::io::Error },
}
