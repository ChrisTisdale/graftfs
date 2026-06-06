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
use crossterm::style::{StyledContent, Stylize, style};
use std::fmt::Display;
use std::path::Path;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum ColorSupport {
    #[default]
    None,
    Colored(ColorSettings),
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
    fn format_link_text<'a>(&self, text: &'a str) -> StyledContent<&'a str> {
        match self {
            Self::None => style(text),
            Self::Colored(config) => style(text).with(config.link),
        }
    }

    pub fn print_link_text(&self, item: &Path, target: &Path) {
        println!(
            "{}: {} {} {}",
            self.format_link_text("LINK"),
            self.format_source_text(item.display().to_string().as_str()),
            self.format_arrow_text("=>"),
            self.format_target_text(target.display().to_string().as_str())
        );
    }

    fn format_unlink_text<'a>(&self, text: &'a str) -> StyledContent<&'a str> {
        match self {
            Self::None => style(text),
            Self::Colored(config) => style(text).with(config.unlink),
        }
    }

    pub fn print_unlink_text(&self, item: &Path) {
        println!(
            "{}: {}",
            self.format_unlink_text("UNLINK"),
            self.format_target_text(item.display().to_string().as_str())
        );
    }

    fn format_list_text<'a>(&self, text: &'a str) -> StyledContent<&'a str> {
        match self {
            Self::None => style(text),
            Self::Colored(config) => style(text).with(config.list),
        }
    }

    pub fn print_list_text(&self, item: &Path, target: &Path) {
        println!(
            "{}: {} {} {}",
            self.format_list_text("LINK"),
            self.format_source_text(item.display().to_string().as_str()),
            self.format_arrow_text("=>"),
            self.format_target_text(target.display().to_string().as_str())
        );
    }

    fn format_remove_text<'a>(&self, text: &'a str) -> StyledContent<&'a str> {
        match self {
            Self::None => style(text),
            Self::Colored(config) => style(text).with(config.remove),
        }
    }

    pub fn print_remove_text(&self, item: &Path) {
        println!(
            "{}: {}",
            self.format_remove_text("RM"),
            self.format_target_text(item.display().to_string().as_str())
        );
    }

    fn format_create_text<'a>(&self, text: &'a str) -> StyledContent<&'a str> {
        match self {
            Self::None => style(text),
            Self::Colored(config) => style(text).with(config.create),
        }
    }

    pub fn print_create_text(&self, item: &Path) {
        println!(
            "{}: {}",
            self.format_create_text("MKDIR"),
            self.format_target_text(item.display().to_string().as_str())
        );
    }

    pub fn format_arrow_text<'a>(&self, text: &'a str) -> StyledContent<&'a str> {
        match self {
            Self::None => style(text),
            Self::Colored(config) => style(text).with(config.arrow),
        }
    }

    pub fn format_source_text<'a>(&self, text: &'a str) -> StyledContent<&'a str> {
        match self {
            Self::None => style(text),
            Self::Colored(config) => style(text).with(config.source),
        }
    }

    pub fn format_target_text<'a>(&self, text: &'a str) -> StyledContent<&'a str> {
        match self {
            Self::None => style(text),
            Self::Colored(config) => style(text).with(config.target),
        }
    }
}
