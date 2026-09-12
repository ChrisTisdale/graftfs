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

const DEFAULT_NOTE_COLOR: Color = Color::Rgb {
    r: 245,
    g: 169,
    b: 127,
};

const DEFAULT_COLON_COLOR: Color = Color::Reset;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ColorConfig {
    pub enabled: bool,
    #[serde(flatten)]
    pub(crate) settings: SerializeColorSettings,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct V1ColorConfig {
    pub enabled: bool,
    #[serde(flatten)]
    pub(crate) settings: V1SerializeColorSettings,
}

impl Default for V1ColorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            settings: V1SerializeColorSettings::default(),
        }
    }
}

impl From<V1ColorConfig> for ColorConfig {
    fn from(value: V1ColorConfig) -> Self {
        Self {
            enabled: value.enabled,
            settings: value.settings.into(),
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LinkConfig {
    pub link: Color,
    pub colon: Color,
    pub source: Color,
    pub arrow: Color,
    pub target: Color,
}

impl Default for LinkConfig {
    fn default() -> Self {
        Self {
            link: DEFAULT_LINK_COLOR,
            colon: DEFAULT_COLON_COLOR,
            source: DEFAULT_SOURCE_COLOR,
            arrow: DEFAULT_ARROW_COLOR,
            target: DEFAULT_TARGET_COLOR,
        }
    }
}

impl Display for LinkConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}{:?}{:?}{:?}{:?}",
            self.link, self.colon, self.source, self.arrow, self.target
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListConfig {
    pub link: Color,
    pub colon: Color,
    pub source: Color,
    pub arrow: Color,
    pub target: Color,
}

impl Default for ListConfig {
    fn default() -> Self {
        Self {
            link: DEFAULT_LIST_COLOR,
            colon: DEFAULT_COLON_COLOR,
            source: DEFAULT_SOURCE_COLOR,
            arrow: DEFAULT_ARROW_COLOR,
            target: DEFAULT_TARGET_COLOR,
        }
    }
}

impl Display for ListConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}{:?}{:?}{:?}{:?}",
            self.link, self.colon, self.source, self.arrow, self.target
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UnlinkConfig {
    pub unlink: Color,
    pub colon: Color,
    pub target: Color,
}

impl Default for UnlinkConfig {
    fn default() -> Self {
        Self {
            unlink: DEFAULT_UNLINK_COLOR,
            colon: DEFAULT_COLON_COLOR,
            target: DEFAULT_TARGET_COLOR,
        }
    }
}

impl Display for UnlinkConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}{:?}", self.unlink, self.colon, self.target)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RemoveConfig {
    pub remove: Color,
    pub colon: Color,
    pub target: Color,
}

impl Default for RemoveConfig {
    fn default() -> Self {
        Self {
            remove: DEFAULT_REMOVE_COLOR,
            colon: DEFAULT_COLON_COLOR,
            target: DEFAULT_TARGET_COLOR,
        }
    }
}

impl Display for RemoveConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}{:?}", self.remove, self.colon, self.target)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateConfig {
    pub create: Color,
    pub colon: Color,
    pub target: Color,
}

impl Default for CreateConfig {
    fn default() -> Self {
        Self {
            create: DEFAULT_CREATE_COLOR,
            colon: DEFAULT_COLON_COLOR,
            target: DEFAULT_TARGET_COLOR,
        }
    }
}

impl Display for CreateConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}{:?}", self.create, self.colon, self.target)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SimulationConfig {
    pub note: Color,
    pub colon: Color,
    pub text: Color,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            note: DEFAULT_NOTE_COLOR,
            colon: DEFAULT_NOTE_COLOR,
            text: DEFAULT_NOTE_COLOR,
        }
    }
}

impl Display for SimulationConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}{:?}", self.note, self.colon, self.text)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct SerializeColorSettings {
    pub link: SerializeLinkConfig,
    pub unlink: SerializeUnlinkConfig,
    pub list: SerializeListConfig,
    pub remove: SerializeRemoveConfig,
    pub create: SerializeCreateConfig,
    pub simulation: SerializeSimulationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct V1SerializeColorSettings {
    pub link: Option<Color>,
    pub unlink: Option<Color>,
    pub list: Option<Color>,
    pub remove: Option<Color>,
    pub create: Option<Color>,
    pub arrow: Option<Color>,
    pub source: Option<Color>,
    pub target: Option<Color>,
}

impl Default for V1SerializeColorSettings {
    fn default() -> Self {
        Self {
            link: Some(DEFAULT_LINK_COLOR),
            unlink: Some(DEFAULT_UNLINK_COLOR),
            list: Some(DEFAULT_LIST_COLOR),
            remove: Some(DEFAULT_REMOVE_COLOR),
            create: Some(DEFAULT_CREATE_COLOR),
            arrow: Some(DEFAULT_ARROW_COLOR),
            source: Some(DEFAULT_SOURCE_COLOR),
            target: Some(DEFAULT_TARGET_COLOR),
        }
    }
}

impl From<V1SerializeColorSettings> for SerializeColorSettings {
    fn from(value: V1SerializeColorSettings) -> Self {
        Self {
            link: SerializeLinkConfig {
                link: value.link,
                colon: None,
                source: value.source,
                arrow: value.arrow,
                target: value.target,
            },
            unlink: SerializeUnlinkConfig {
                unlink: value.unlink,
                colon: None,
                target: value.target,
            },
            list: SerializeListConfig {
                link: value.list,
                colon: None,
                source: value.source,
                arrow: value.arrow,
                target: value.target,
            },
            remove: SerializeRemoveConfig {
                remove: value.remove,
                colon: None,
                target: value.target,
            },
            create: SerializeCreateConfig {
                create: value.create,
                colon: None,
                target: value.target,
            },
            simulation: SerializeSimulationConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SerializeLinkConfig {
    pub link: Option<Color>,
    pub colon: Option<Color>,
    pub source: Option<Color>,
    pub arrow: Option<Color>,
    pub target: Option<Color>,
}

impl Default for SerializeLinkConfig {
    fn default() -> Self {
        LinkConfig::default().into()
    }
}

impl Display for SerializeLinkConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}{:?}{:?}{:?}{:?}",
            self.link.unwrap_or(DEFAULT_LINK_COLOR),
            self.colon.unwrap_or(DEFAULT_COLON_COLOR),
            self.source.unwrap_or(DEFAULT_SOURCE_COLOR),
            self.arrow.unwrap_or(DEFAULT_ARROW_COLOR),
            self.target.unwrap_or(DEFAULT_TARGET_COLOR)
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SerializeListConfig {
    pub link: Option<Color>,
    pub colon: Option<Color>,
    pub source: Option<Color>,
    pub arrow: Option<Color>,
    pub target: Option<Color>,
}

impl Default for SerializeListConfig {
    fn default() -> Self {
        ListConfig::default().into()
    }
}

impl Display for SerializeListConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}{:?}{:?}{:?}{:?}",
            self.link.unwrap_or(DEFAULT_LIST_COLOR),
            self.colon.unwrap_or(DEFAULT_COLON_COLOR),
            self.source.unwrap_or(DEFAULT_SOURCE_COLOR),
            self.arrow.unwrap_or(DEFAULT_ARROW_COLOR),
            self.target.unwrap_or(DEFAULT_TARGET_COLOR)
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SerializeUnlinkConfig {
    pub unlink: Option<Color>,
    pub colon: Option<Color>,
    pub target: Option<Color>,
}

impl Default for SerializeUnlinkConfig {
    fn default() -> Self {
        UnlinkConfig::default().into()
    }
}

impl Display for SerializeUnlinkConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}{:?}{:?}",
            self.unlink.unwrap_or(DEFAULT_NOTE_COLOR),
            self.colon.unwrap_or(DEFAULT_NOTE_COLOR),
            self.target.unwrap_or(DEFAULT_TARGET_COLOR)
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SerializeRemoveConfig {
    pub remove: Option<Color>,
    pub colon: Option<Color>,
    pub target: Option<Color>,
}

impl Default for SerializeRemoveConfig {
    fn default() -> Self {
        RemoveConfig::default().into()
    }
}

impl Display for SerializeRemoveConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}{:?}{:?}",
            self.remove.unwrap_or(DEFAULT_REMOVE_COLOR),
            self.colon.unwrap_or(DEFAULT_COLON_COLOR),
            self.target.unwrap_or(DEFAULT_TARGET_COLOR)
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SerializeCreateConfig {
    pub create: Option<Color>,
    pub colon: Option<Color>,
    pub target: Option<Color>,
}

impl Default for SerializeCreateConfig {
    fn default() -> Self {
        CreateConfig::default().into()
    }
}

impl Display for SerializeCreateConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}{:?}{:?}",
            self.create.unwrap_or(DEFAULT_CREATE_COLOR),
            self.colon.unwrap_or(DEFAULT_COLON_COLOR),
            self.target.unwrap_or(DEFAULT_TARGET_COLOR)
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SerializeSimulationConfig {
    pub note: Option<Color>,
    pub colon: Option<Color>,
    pub text: Option<Color>,
}

impl Default for SerializeSimulationConfig {
    fn default() -> Self {
        SimulationConfig::default().into()
    }
}

impl Display for SerializeSimulationConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}{:?}{:?}",
            self.note.unwrap_or(DEFAULT_NOTE_COLOR),
            self.colon.unwrap_or(DEFAULT_NOTE_COLOR),
            self.text.unwrap_or(DEFAULT_NOTE_COLOR)
        )
    }
}

impl From<LinkConfig> for SerializeLinkConfig {
    fn from(value: LinkConfig) -> Self {
        Self {
            link: Some(value.link),
            colon: Some(value.colon),
            source: Some(value.source),
            arrow: Some(value.arrow),
            target: Some(value.target),
        }
    }
}

impl From<SerializeLinkConfig> for LinkConfig {
    fn from(value: SerializeLinkConfig) -> Self {
        Self {
            link: value.link.unwrap_or(DEFAULT_LINK_COLOR),
            colon: value.colon.unwrap_or(DEFAULT_COLON_COLOR),
            source: value.source.unwrap_or(DEFAULT_SOURCE_COLOR),
            arrow: value.arrow.unwrap_or(DEFAULT_ARROW_COLOR),
            target: value.target.unwrap_or(DEFAULT_TARGET_COLOR),
        }
    }
}

impl From<ListConfig> for SerializeListConfig {
    fn from(value: ListConfig) -> Self {
        Self {
            link: Some(value.link),
            colon: Some(value.colon),
            source: Some(value.source),
            arrow: Some(value.arrow),
            target: Some(value.target),
        }
    }
}

impl From<SerializeListConfig> for ListConfig {
    fn from(value: SerializeListConfig) -> Self {
        Self {
            link: value.link.unwrap_or(DEFAULT_LIST_COLOR),
            colon: value.colon.unwrap_or(DEFAULT_COLON_COLOR),
            source: value.source.unwrap_or(DEFAULT_SOURCE_COLOR),
            arrow: value.arrow.unwrap_or(DEFAULT_ARROW_COLOR),
            target: value.target.unwrap_or(DEFAULT_TARGET_COLOR),
        }
    }
}

impl From<UnlinkConfig> for SerializeUnlinkConfig {
    fn from(value: UnlinkConfig) -> Self {
        Self {
            unlink: Some(value.unlink),
            colon: Some(value.colon),
            target: Some(value.target),
        }
    }
}

impl From<SerializeUnlinkConfig> for UnlinkConfig {
    fn from(value: SerializeUnlinkConfig) -> Self {
        Self {
            unlink: value.unlink.unwrap_or(DEFAULT_UNLINK_COLOR),
            colon: value.colon.unwrap_or(DEFAULT_COLON_COLOR),
            target: value.target.unwrap_or(DEFAULT_TARGET_COLOR),
        }
    }
}

impl From<RemoveConfig> for SerializeRemoveConfig {
    fn from(value: RemoveConfig) -> Self {
        Self {
            remove: Some(value.remove),
            colon: Some(value.colon),
            target: Some(value.target),
        }
    }
}

impl From<SerializeRemoveConfig> for RemoveConfig {
    fn from(value: SerializeRemoveConfig) -> Self {
        Self {
            remove: value.remove.unwrap_or(DEFAULT_REMOVE_COLOR),
            colon: value.colon.unwrap_or(DEFAULT_COLON_COLOR),
            target: value.target.unwrap_or(DEFAULT_TARGET_COLOR),
        }
    }
}

impl From<CreateConfig> for SerializeCreateConfig {
    fn from(value: CreateConfig) -> Self {
        Self {
            create: Some(value.create),
            colon: Some(value.colon),
            target: Some(value.target),
        }
    }
}

impl From<SerializeCreateConfig> for CreateConfig {
    fn from(value: SerializeCreateConfig) -> Self {
        Self {
            create: value.create.unwrap_or(DEFAULT_CREATE_COLOR),
            colon: value.colon.unwrap_or(DEFAULT_COLON_COLOR),
            target: value.target.unwrap_or(DEFAULT_TARGET_COLOR),
        }
    }
}

impl From<SimulationConfig> for SerializeSimulationConfig {
    fn from(value: SimulationConfig) -> Self {
        Self {
            note: Some(value.note),
            colon: Some(value.colon),
            text: Some(value.text),
        }
    }
}

impl From<SerializeSimulationConfig> for SimulationConfig {
    fn from(value: SerializeSimulationConfig) -> Self {
        Self {
            note: value.note.unwrap_or(DEFAULT_NOTE_COLOR),
            colon: value.colon.unwrap_or(DEFAULT_NOTE_COLOR),
            text: value.text.unwrap_or(DEFAULT_NOTE_COLOR),
        }
    }
}

impl From<ColorSettings> for SerializeColorSettings {
    fn from(value: ColorSettings) -> Self {
        Self {
            link: value.link.into(),
            unlink: value.unlink.into(),
            list: value.list.into(),
            remove: value.remove.into(),
            create: value.create.into(),
            simulation: value.simulation.into(),
        }
    }
}

impl From<SerializeColorSettings> for ColorSettings {
    fn from(value: SerializeColorSettings) -> Self {
        Self {
            link: value.link.into(),
            unlink: value.unlink.into(),
            list: value.list.into(),
            remove: value.remove.into(),
            create: value.create.into(),
            simulation: value.simulation.into(),
        }
    }
}

impl Display for SerializeColorSettings {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ColorSettings {{ link_color: {:?}, unlink_color: {:?}, list_color: {:?}, remove_color: {:?}, create_color: {:?}, simulation_color: {:?} }}",
            self.link, self.unlink, self.list, self.remove, self.create, self.simulation,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct ColorSettings {
    pub link: LinkConfig,
    pub unlink: UnlinkConfig,
    pub list: ListConfig,
    pub remove: RemoveConfig,
    pub create: CreateConfig,
    pub simulation: SimulationConfig,
}

impl Display for ColorSettings {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ColorSettings {{ link_color: {:?}, unlink_color: {:?}, list_color: {:?}, remove_color: {:?}, create_color: {:?}, simulation_color: {:?} }}",
            self.link, self.unlink, self.list, self.remove, self.create, self.simulation
        )
    }
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
