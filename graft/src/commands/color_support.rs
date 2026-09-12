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

use crate::config::ColorSettings;
use crossterm::style::{Stylize, style};
use std::fmt::Display;
use std::path::Path;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum ColorSupport {
    #[default]
    None,
    Colored(Box<ColorSettings>),
}

impl Display for ColorSupport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Colored(_) => write!(f, "Colored"),
        }
    }
}

impl ColorSupport {
    pub fn print_link_text(&self, item: &Path, target: &Path) {
        match self {
            Self::None => println!("LINK: {} => {}", item.display(), target.display()),
            Self::Colored(config) => println!(
                "{}{} {} {} {}",
                style("LINK").with(config.link.link),
                style(":").with(config.link.colon),
                style(item.display()).with(config.link.source),
                style("=>").with(config.link.arrow),
                style(target.display()).with(config.link.target),
            ),
        }
    }

    pub fn print_unlink_text(&self, item: &Path) {
        match self {
            Self::None => println!("UNLINK: {}", item.display()),
            Self::Colored(config) => println!(
                "{}{} {}",
                style("UNLINK").with(config.unlink.unlink),
                style(":").with(config.unlink.colon),
                style(item.display()).with(config.unlink.target),
            ),
        }
    }

    pub fn print_list_text(&self, item: &Path, target: &Path) {
        match self {
            Self::None => println!("LINK: {} => {}", item.display(), target.display()),
            Self::Colored(config) => println!(
                "{}{} {} {} {}",
                style("LINK").with(config.list.link),
                style(":").with(config.list.colon),
                style(item.display()).with(config.list.source),
                style("=>").with(config.list.arrow),
                style(target.display()).with(config.list.target),
            ),
        }
    }

    pub fn print_remove_text(&self, item: &Path) {
        match self {
            Self::None => println!("RM: {}", item.display()),
            Self::Colored(config) => println!(
                "{}{} {}",
                style("RM").with(config.remove.remove),
                style(":").with(config.remove.colon),
                style(item.display()).with(config.remove.target),
            ),
        }
    }

    pub fn print_create_text(&self, item: &Path) {
        match self {
            Self::None => println!("MKDIR: {}", item.display()),
            Self::Colored(config) => println!(
                "{}{} {}",
                style("MKDIR").with(config.create.create),
                style(":").with(config.create.colon),
                style(item.display()).with(config.create.target),
            ),
        }
    }

    pub fn print_simulation_text(&self) {
        match self {
            Self::None => println!("\nNote: Running in simulation mode.  The file system isn't being modified"),
            Self::Colored(config) => println!(
                "\n{}{} {}",
                style("Note").with(config.simulation.note),
                style(":").with(config.simulation.colon),
                style("Running in simulation mode.  The file system isn't being modified").with(config.simulation.text)
            ),
        }
    }
}
