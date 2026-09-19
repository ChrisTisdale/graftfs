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

use crate::command_line_args::{CompletionPrinter, ConfigPrinter, ConfigUpgrader};
use crate::commands::{Command, CommandOperation};
use std::fmt::{Display, Formatter};

pub enum Executor<T: CommandOperation> {
    Command(Command<T>),
    Completion(CompletionPrinter),
    Export(ConfigPrinter),
    Upgrade(ConfigUpgrader),
}

impl<T: CommandOperation> Display for Executor<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Command(cmd) => write!(f, "Command({cmd})"),
            Self::Completion(cmd) => write!(f, "Completion({cmd})"),
            Self::Export(cmd) => write!(f, "Export({cmd})"),
            Self::Upgrade(cmd) => write!(f, "Upgrade({cmd})"),
        }
    }
}
