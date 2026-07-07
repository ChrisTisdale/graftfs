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

use crate::shell_converter_error::ShellConverterError;
use clap::builder::PossibleValue;
use clap::{Command, ValueEnum};
use clap_complete::Generator;
use std::fmt::{Display, Formatter};
use std::io::{Error, Write};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum Shell {
    Bash,
    Fish,
    Elvish,
    PowerShell,
    Zsh,
    #[cfg(feature = "nushell")]
    Nushell,
}

impl TryFrom<clap_complete::Shell> for Shell {
    type Error = ShellConverterError;

    fn try_from(value: clap_complete::Shell) -> Result<Self, Self::Error> {
        match value {
            clap_complete::Shell::Bash => Ok(Self::Bash),
            clap_complete::Shell::Elvish => Ok(Self::Elvish),
            clap_complete::Shell::Fish => Ok(Self::Fish),
            clap_complete::Shell::PowerShell => Ok(Self::PowerShell),
            clap_complete::Shell::Zsh => Ok(Self::Zsh),
            _ => Err(ShellConverterError),
        }
    }
}

impl Shell {
    #[must_use]
    pub fn from_env() -> Option<Self> {
        clap_complete::Shell::from_env().and_then(|s| s.try_into().ok())
    }
}

impl ValueEnum for Shell {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Bash,
            Self::Elvish,
            Self::Fish,
            Self::PowerShell,
            Self::Zsh,
            #[cfg(feature = "nushell")]
            Self::Nushell,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Bash => PossibleValue::new("bash"),
            Self::Elvish => PossibleValue::new("elvish"),
            Self::Fish => PossibleValue::new("fish"),
            Self::PowerShell => PossibleValue::new("powershell"),
            Self::Zsh => PossibleValue::new("zsh"),
            #[cfg(feature = "nushell")]
            Self::Nushell => PossibleValue::new("nushell").alias("nu"),
        })
    }
}

impl Display for Shell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bash => write!(f, "bash"),
            Self::Fish => write!(f, "fish"),
            Self::Elvish => write!(f, "elvish"),
            Self::PowerShell => write!(f, "powershell"),
            Self::Zsh => write!(f, "zsh"),
            #[cfg(feature = "nushell")]
            Self::Nushell => write!(f, "nushell"),
        }
    }
}

impl Generator for Shell {
    fn file_name(&self, name: &str) -> String {
        match self {
            Self::Bash => clap_complete::Shell::Bash.file_name(name),
            Self::Fish => clap_complete::Shell::Fish.file_name(name),
            Self::Elvish => clap_complete::Shell::Elvish.file_name(name),
            Self::PowerShell => clap_complete::Shell::PowerShell.file_name(name),
            Self::Zsh => clap_complete::Shell::Zsh.file_name(name),
            #[cfg(feature = "nushell")]
            Self::Nushell => clap_complete_nushell::Nushell.file_name(name),
        }
    }

    fn generate(&self, cmd: &Command, buf: &mut dyn Write) {
        match self {
            Self::Bash => clap_complete::Shell::Bash.generate(cmd, buf),
            Self::Fish => clap_complete::Shell::Fish.generate(cmd, buf),
            Self::Elvish => clap_complete::Shell::Elvish.generate(cmd, buf),
            Self::PowerShell => clap_complete::Shell::PowerShell.generate(cmd, buf),
            Self::Zsh => clap_complete::Shell::Zsh.generate(cmd, buf),
            #[cfg(feature = "nushell")]
            Self::Nushell => clap_complete_nushell::Nushell.generate(cmd, buf),
        }
    }

    fn try_generate(&self, cmd: &Command, buf: &mut dyn Write) -> Result<(), Error> {
        match self {
            Self::Bash => clap_complete::Shell::Bash.try_generate(cmd, buf),
            Self::Fish => clap_complete::Shell::Fish.try_generate(cmd, buf),
            Self::Elvish => clap_complete::Shell::Elvish.try_generate(cmd, buf),
            Self::PowerShell => clap_complete::Shell::PowerShell.try_generate(cmd, buf),
            Self::Zsh => clap_complete::Shell::Zsh.try_generate(cmd, buf),
            #[cfg(feature = "nushell")]
            Self::Nushell => clap_complete_nushell::Nushell.try_generate(cmd, buf),
        }
    }
}
