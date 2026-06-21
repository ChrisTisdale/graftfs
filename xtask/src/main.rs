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

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use clap::CommandFactory;
use graftfs::CommandLineProcessor;

fn main() -> Result<(), anyhow::Error> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("dist") => dist()?,
        _ => print_help(),
    }

    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:

dist            builds application and man pages
"
    );
}

fn dist() -> Result<(), anyhow::Error> {
    let out_dir = dist_dir()?;
    let _ = fs::remove_dir_all(&out_dir);
    fs::create_dir_all(&out_dir)?;

    dist_binary()?;
    dist_manpage()?;
    Ok(())
}

fn dist_manpage() -> Result<(), anyhow::Error> {
    let out_dir = dist_dir()?;
    let cmd = CommandLineProcessor::command();
    let man = clap_mangen::Man::new(cmd);

    let mut buffer: Vec<u8> = Vec::default();
    man.render(&mut buffer)?;

    fs::write(out_dir.join("graft.1"), buffer)?;

    Ok(())
}

fn dist_binary() -> Result<(), anyhow::Error> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let root = project_root()?;
    let out_dir = dist_dir()?;
    let status = Command::new(cargo)
        .current_dir(&root)
        .args(["build", "--release"])
        .status()?;

    if !status.success() {
        return Err(anyhow::Error::msg("cargo build failed"));
    }

    let source = root.join("target/release/graft");
    let destination = out_dir.join("graft");

    fs::copy(&source, &destination)?;

    if Command::new("strip")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
    {
        eprintln!("stripping the binary");
        let status = Command::new("strip").arg(&source).status()?;
        if !status.success() {
            return Err(anyhow::Error::msg("strip failed"));
        }
    } else {
        eprintln!("no `strip` utility found");
    }

    Ok(())
}

fn project_root() -> Result<PathBuf, anyhow::Error> {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .map(Path::to_path_buf)
        .ok_or_else(|| anyhow::Error::msg("could not find project root"))
}

fn dist_dir() -> Result<PathBuf, anyhow::Error> {
    project_root().map(|p| p.join("target/dist"))
}
