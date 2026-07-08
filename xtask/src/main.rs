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

use clap::builder::Styles;
use clap::{Args, CommandFactory, Parser, Subcommand, ValueEnum};
use graft::CommandLineProcessor;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

const ALL_FEATURES: &str = "--all-features";
const APP_NAME: &str = "xtask";
const STYLES: Styles = Styles::styled();

#[derive(Debug, Clone, PartialEq, Eq, Default, ValueEnum)]
enum Configuration {
    #[default]
    Release,
    Debug,
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq)]
struct DistributeArgs {
    #[clap(
        short = 'f',
        long = "feature",
        help = "Features to enable when building the graft application"
    )]
    features: Vec<String>,
    #[clap(
        short = 'a',
        long = "all-features",
        help = "Enable all features when building the graft application",
        conflicts_with = "features"
    )]
    all_features: bool,
    #[clap(
        short = 'c',
        long = "configuration",
        help = "Configuration to use when building the graft application",
        default_value = "release"
    )]
    configuration: Configuration,
    #[clap(
        short = 'o',
        long = "output",
        help = "The output directory used for the generated binary and man pages.  If not specified, the target/dist directory at the root of the project will be used.  The directory will be deleted if it exists before building."
    )]
    output: Option<PathBuf>,
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
enum XCommands {
    #[clap(name = "dist", about = "Builds application and man pages")]
    Dist(#[clap(flatten)] DistributeArgs),
}

#[derive(Parser)]
#[command(version, name = APP_NAME, about, author, propagate_version = true, styles = STYLES, help_template = "\
{before-help}{name} {version}: {author-with-newline}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
")]
#[clap(rename_all = "snake_case")]
struct XCommandLineArgs {
    #[clap(subcommand)]
    command: XCommands,
}

fn main() -> Result<(), anyhow::Error> {
    match process_command() {
        Ok(()) => Ok(()),
        Err(e) => {
            if let Some(e) = e.downcast_ref::<clap::Error>() {
                e.exit()
            }

            Err(e)
        }
    }
}

fn process_command() -> Result<(), anyhow::Error> {
    let args = XCommandLineArgs::try_parse()?;
    match args.command {
        XCommands::Dist(args) => process_dist(args),
    }
}

fn process_dist(args: DistributeArgs) -> Result<(), anyhow::Error> {
    let output = args.output.map_or_else(default_dist_directory, Ok)?;

    let mut additional_args = if args.all_features {
        vec![ALL_FEATURES.to_string()]
    } else {
        args.features
            .iter()
            .map(|f| format!("--feature {f}"))
            .collect()
    };

    let configuration = match args.configuration {
        Configuration::Release => "--release",
        Configuration::Debug => "--debug",
    };

    additional_args.push(configuration.to_string());
    dist(additional_args, &output)
}

fn dist(additional_args: Vec<String>, out_dir: &Path) -> Result<(), anyhow::Error> {
    let _ = fs::remove_dir_all(out_dir);
    fs::create_dir_all(out_dir)?;

    let with_nushell = additional_args
        .iter()
        .any(|a| a.ends_with(" nushell") || a.eq(ALL_FEATURES));
    dist_binary(additional_args, out_dir)?;
    dist_manpage(out_dir, with_nushell)?;
    Ok(())
}

#[derive(Clone, Copy, ValueEnum)]
#[allow(clippy::enum_variant_names)]
enum Shell {
    Bash,
    Fish,
    Elvish,
    PowerShell,
    Zsh,
}

#[derive(Clone, Copy, ValueEnum)]
#[allow(clippy::enum_variant_names)]
enum ShelWithNusehll {
    Bash,
    Fish,
    Elvish,
    PowerShell,
    Zsh,
    Nushell,
}

fn dist_manpage(out_dir: &Path, with_nushell: bool) -> Result<(), anyhow::Error> {
    let cmd = CommandLineProcessor::command();
    for cmd in cmd.get_subcommands() {
        render_subcommand(cmd, out_dir, with_nushell)?;
    }

    let man = clap_mangen::Man::new(cmd);

    let mut buffer: Vec<u8> = Vec::default();
    man.render(&mut buffer)?;

    fs::write(out_dir.join("graft.1"), buffer)?;

    Ok(())
}

fn render_subcommand(cmd: &clap::Command, out_dir: &Path, with_nushell: bool) -> Result<(), anyhow::Error> {
    let name = cmd.get_name().to_string();
    let mut cmd = cmd.clone();
    if name.eq_ignore_ascii_case("completions") {
        let args = cmd.get_arguments().cloned().collect::<Vec<_>>();
        let mut temp = clap::Command::new("completions");
        for mut arg in args {
            if arg.get_id().as_str().eq_ignore_ascii_case("shell") {
                if with_nushell {
                    arg = arg.value_parser(clap::value_parser!(ShelWithNusehll));
                } else {
                    arg = arg.value_parser(clap::value_parser!(Shell));
                }
            }

            temp = temp.arg(arg);
        }

        cmd = temp;
    }

    let man = clap_mangen::Man::new(cmd);

    let mut buffer: Vec<u8> = Vec::default();
    man.render(&mut buffer)?;

    fs::write(out_dir.join(format!("graft-{name}.1")), buffer)?;
    Ok(())
}

fn dist_binary(mut additional_args: Vec<String>, out_dir: &Path) -> Result<(), anyhow::Error> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let root = project_root()?;

    let mut args = vec!["build".to_string()];
    args.append(&mut additional_args);

    let status = Command::new(cargo).current_dir(&root).args(args).status()?;
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

fn default_dist_directory() -> Result<PathBuf, anyhow::Error> {
    project_root().map(|p| p.join("target/dist"))
}
