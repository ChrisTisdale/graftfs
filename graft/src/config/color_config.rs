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

use crossterm::style::Color;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ColorConfig {
    pub enabled: bool,
    #[serde(flatten)]
    pub settings: ColorSettings,
}

impl Default for ColorSettings {
    fn default() -> Self {
        Self {
            link: Color::Rgb {
                r: 166,
                g: 218,
                b: 149,
            },
            unlink: Color::Rgb {
                r: 237,
                g: 135,
                b: 150,
            },
            list: Color::Rgb {
                r: 145,
                g: 215,
                b: 227,
            },
            remove: Color::Rgb {
                r: 237,
                g: 135,
                b: 150,
            },
            create: Color::Rgb {
                r: 166,
                g: 218,
                b: 149,
            },
            arrow: Color::Rgb {
                r: 138,
                g: 173,
                b: 244,
            },
            source: Color::Rgb {
                r: 238,
                g: 212,
                b: 159,
            },
            target: Color::Rgb {
                r: 245,
                g: 189,
                b: 230,
            },
        }
    }
}

impl Display for ColorSettings {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ColorSettings {{ link_color: {:?}, unlink_color: {:?}, list_color: {:?}, remove_color: {:?}, create_color: {:?}, arrow_color: {:?}, source_color: {:?}, target_color: {:?} }}",
            self.link, self.unlink, self.list, self.remove, self.create, self.arrow, self.source, self.target
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ColorSettings {
    pub link: Color,
    pub unlink: Color,
    pub list: Color,
    pub remove: Color,
    pub create: Color,
    pub arrow: Color,
    pub source: Color,
    pub target: Color,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            settings: ColorSettings::default(),
        }
    }
}

impl Display for ColorConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ColorConfig {{ enabled: {}, settings: {} }}",
            self.enabled, self.settings
        )
    }
}
