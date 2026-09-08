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

const DEFAULT_LINK_COLOR: Color = Color::Rgb {
    r: 166,
    g: 218,
    b: 149,
};

const DEFAULT_UNLINK_COLOR: Color = Color::Rgb {
    r: 237,
    g: 135,
    b: 150,
};

const DEFAULT_LIST_COLOR: Color = Color::Rgb {
    r: 145,
    g: 215,
    b: 227,
};

const DEFAULT_REMOVE_COLOR: Color = Color::Rgb {
    r: 237,
    g: 135,
    b: 150,
};

const DEFAULT_CREATE_COLOR: Color = Color::Rgb {
    r: 166,
    g: 218,
    b: 149,
};

const DEFAULT_ARROW_COLOR: Color = Color::Rgb {
    r: 138,
    g: 173,
    b: 244,
};

const DEFAULT_SOURCE_COLOR: Color = Color::Rgb {
    r: 238,
    g: 212,
    b: 159,
};

const DEFAULT_TARGET_COLOR: Color = Color::Rgb {
    r: 245,
    g: 189,
    b: 230,
};

const DEFAULT_WARNING_COLOR: Color = Color::Rgb {
    r: 238,
    g: 212,
    b: 159,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ColorConfig {
    pub enabled: bool,
    #[serde(flatten)]
    pub(crate) settings: SerializeColorSettings,
}

impl ColorConfig {
    #[must_use]
    pub fn color_settings(&self) -> ColorSettings {
        self.settings.clone().into()
    }

    #[must_use]
    pub fn new(settings: ColorSettings, enabled: bool) -> Self {
        Self {
            enabled,
            settings: settings.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct SerializeColorSettings {
    pub link: Option<Color>,
    pub unlink: Option<Color>,
    pub list: Option<Color>,
    pub remove: Option<Color>,
    pub create: Option<Color>,
    pub arrow: Option<Color>,
    pub source: Option<Color>,
    pub target: Option<Color>,
    pub warning: Option<Color>,
}

impl From<ColorSettings> for SerializeColorSettings {
    fn from(value: ColorSettings) -> Self {
        Self {
            link: Some(value.link),
            unlink: Some(value.unlink),
            list: Some(value.list),
            remove: Some(value.remove),
            create: Some(value.create),
            arrow: Some(value.arrow),
            source: Some(value.source),
            target: Some(value.target),
            warning: Some(value.warning),
        }
    }
}

impl Default for ColorSettings {
    fn default() -> Self {
        Self {
            link: DEFAULT_LINK_COLOR,
            unlink: DEFAULT_UNLINK_COLOR,
            list: DEFAULT_LIST_COLOR,
            remove: DEFAULT_REMOVE_COLOR,
            create: DEFAULT_CREATE_COLOR,
            arrow: DEFAULT_ARROW_COLOR,
            source: DEFAULT_SOURCE_COLOR,
            target: DEFAULT_TARGET_COLOR,
            warning: DEFAULT_WARNING_COLOR,
        }
    }
}

impl From<SerializeColorSettings> for ColorSettings {
    fn from(value: SerializeColorSettings) -> Self {
        Self {
            link: value.link.unwrap_or(DEFAULT_LINK_COLOR),
            unlink: value.unlink.unwrap_or(DEFAULT_UNLINK_COLOR),
            list: value.list.unwrap_or(DEFAULT_LIST_COLOR),
            remove: value.remove.unwrap_or(DEFAULT_REMOVE_COLOR),
            create: value.create.unwrap_or(DEFAULT_CREATE_COLOR),
            arrow: value.arrow.unwrap_or(DEFAULT_ARROW_COLOR),
            source: value.source.unwrap_or(DEFAULT_SOURCE_COLOR),
            target: value.target.unwrap_or(DEFAULT_TARGET_COLOR),
            warning: value.warning.unwrap_or(DEFAULT_WARNING_COLOR),
        }
    }
}

impl Display for SerializeColorSettings {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ColorSettings {{ link_color: {:?}, unlink_color: {:?}, list_color: {:?}, remove_color: {:?}, create_color: {:?}, arrow_color: {:?}, source_color: {:?}, target_color: {:?}, warning_color: {:?} }}",
            self.link,
            self.unlink,
            self.list,
            self.remove,
            self.create,
            self.arrow,
            self.source,
            self.target,
            self.warning
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
    pub warning: Color,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            settings: SerializeColorSettings::default(),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_color_settings_default() {
        let settings = ColorSettings::default();
        assert_eq!(settings.link, DEFAULT_LINK_COLOR);
        assert_eq!(settings.unlink, DEFAULT_UNLINK_COLOR);
        assert_eq!(settings.list, DEFAULT_LIST_COLOR);
        assert_eq!(settings.remove, DEFAULT_REMOVE_COLOR);
        assert_eq!(settings.create, DEFAULT_CREATE_COLOR);
        assert_eq!(settings.arrow, DEFAULT_ARROW_COLOR);
        assert_eq!(settings.source, DEFAULT_SOURCE_COLOR);
        assert_eq!(settings.target, DEFAULT_TARGET_COLOR);
        assert_eq!(settings.warning, DEFAULT_WARNING_COLOR);
    }
}
