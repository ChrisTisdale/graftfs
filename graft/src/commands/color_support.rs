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

pub trait ColorPrinter {
    fn print_link_text(&self, item: &Path, target: &Path);
    fn print_unlink_text(&self, item: &Path);
    fn print_list_text(&self, item: &Path, target: &Path);
    fn print_remove_text(&self, item: &Path);
    fn print_create_text(&self, item: &Path);
    fn print_move_text(&self, source: &Path, target: &Path);
    fn print_simulation_text(&self);
}

impl<T: ColorPrinter> ColorPrinter for Option<Box<T>> {
    fn print_link_text(&self, item: &Path, target: &Path) {
        if let Some(support) = self {
            support.print_link_text(item, target);
        }
    }

    fn print_unlink_text(&self, item: &Path) {
        if let Some(support) = self {
            support.print_unlink_text(item);
        }
    }

    fn print_list_text(&self, item: &Path, target: &Path) {
        if let Some(support) = self {
            support.print_list_text(item, target);
        }
    }

    fn print_remove_text(&self, item: &Path) {
        if let Some(support) = self {
            support.print_remove_text(item);
        }
    }

    fn print_create_text(&self, item: &Path) {
        if let Some(support) = self {
            support.print_create_text(item);
        }
    }

    fn print_move_text(&self, source: &Path, target: &Path) {
        if let Some(support) = self {
            support.print_move_text(source, target);
        }
    }

    fn print_simulation_text(&self) {
        if let Some(support) = self {
            support.print_simulation_text();
        }
    }
}

impl ColorPrinter for ColorSupport {
    fn print_link_text(&self, item: &Path, target: &Path) {
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

    fn print_unlink_text(&self, item: &Path) {
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

    fn print_list_text(&self, item: &Path, target: &Path) {
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

    fn print_remove_text(&self, item: &Path) {
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

    fn print_create_text(&self, item: &Path) {
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

    fn print_move_text(&self, source: &Path, target: &Path) {
        match self {
            Self::None => println!("MV: {} => {}", source.display(), target.display()),
            Self::Colored(config) => println!(
                "{}{} {} {} {}",
                style("MV").with(config.move_color.move_item),
                style(":").with(config.move_color.colon),
                style(source.display()).with(config.move_color.target),
                style("=>").with(config.move_color.arrow),
                style(target.display()).with(config.move_color.target),
            ),
        }
    }

    fn print_simulation_text(&self) {
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
