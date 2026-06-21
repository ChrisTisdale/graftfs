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

use crate::config::ResolveError;
use std::env;
use std::path::{Path, PathBuf};

/// Resolves a given file path, expanding the home directory (`~`) if necessary,
/// and returning a canonicalized absolute path.
///
/// If the `path` does not start with the `~` character (home directory shorthand),
/// the function directly attempts to canonicalize the path using the filesystem.
///
/// If the `path` starts with `~`, the function resolves the user's home directory
/// using `env::home_dir`. It then joins the resolved home directory with the rest
/// of the path (after stripping the `~` prefix), and finally canonicalizes it.
///
/// # Parameters
/// - `path`: A reference to a `Path` representing the input path to be resolved.
///
/// # Returns
/// - `Ok(PathBuf)`: The resolved absolute path as a `PathBuf`.
/// - `Err(ConfigError)`: Returns a `ConfigError` in the following cases:
///     - The function is unable to retrieve the user’s home directory.
///     - The provided path cannot be canonicalized.
///     - The path after prefix stripping cannot be joined or further resolved.
///
/// # Errors
/// - `ConfigError::UnableToResolveDirectory`: Thrown when the home directory cannot be resolved.
/// - Any `std::io::Error` occurring during path canonicalization will also be propagated
///   as part of the `Result`.
///
/// # Example
/// ```rust
/// use std::path::Path;
/// use graftfs::config::path_resolver;
///
/// let input_path = Path::new("~/example/path");
/// match path_resolver::resolve_path(input_path) {
///     Ok(resolved_path) => println!("Resolved Path: {}", resolved_path.display()),
///     Err(error) => eprintln!("Error resolving path: {:?}", error),
/// }
/// ```
pub fn resolve_path(path: &Path) -> Result<PathBuf, ResolveError> {
    if !path.starts_with("~") {
        let path = path.canonicalize()?;
        return Ok(path);
    }

    let home_dir = env::home_dir().ok_or_else(|| ResolveError::UnableToResolveDirectory(path.display().to_string()))?;
    let path = path.strip_prefix("~")?;
    let resolved_path = home_dir.join(path);
    Ok(resolved_path.canonicalize()?)
}

/// Builds a given file path, expanding the home directory (`~`) if necessary.
/// This is the same as `resolve_path` except that it does not canonicalize the path.
///
/// If the `path` starts with `~`, the function resolves the user's home directory
/// using `env::home_dir`. It then joins the resolved home directory with the rest
/// of the path (after stripping the `~` prefix).
///
/// # Parameters
/// - `path`: A reference to a `Path` representing the input path to be resolved.
///
/// # Returns
/// - `Ok(PathBuf)`: The resolved absolute path as a `PathBuf`.
/// - `Err(ConfigError)`: Returns a `ConfigError` in the following cases:
///     - The function is unable to retrieve the user’s home directory.
///     - The provided path cannot be canonicalized.
///     - The path after prefix stripping cannot be joined or further resolved.
///
/// # Errors
/// - `ConfigError::UnableToResolveDirectory`: Thrown when the home directory cannot be resolved.
///
/// # Example
/// ```rust
/// use std::path::Path;
/// use graftfs::config::path_resolver;
///
/// let input_path = Path::new("~/example/path");
/// match path_resolver::resolve_home_path(input_path) {
///     Ok(resolved_path) => println!("Resolved Path: {}", resolved_path.display()),
///     Err(error) => eprintln!("Error resolving path: {:?}", error),
/// }
/// ```
pub fn resolve_home_path(path: &Path) -> Result<PathBuf, ResolveError> {
    if !path.starts_with("~") {
        return Ok(path.to_path_buf());
    }

    let home_dir = env::home_dir().ok_or_else(|| ResolveError::UnableToResolveDirectory(path.display().to_string()))?;
    let path = path.strip_prefix("~")?;
    let resolved_path = home_dir.join(path);
    Ok(resolved_path)
}
