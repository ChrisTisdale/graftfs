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

use serde::de;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct StringOrI64Visitor<T: TryFrom<i64> + FromStr>
where
    <T as FromStr>::Err: Display,
    <T as TryFrom<i64>>::Error: Display,
{
    expecting: &'static str,
    phantom: PhantomData<T>,
}

impl<T: TryFrom<i64> + FromStr> StringOrI64Visitor<T>
where
    <T as FromStr>::Err: Display,
    <T as TryFrom<i64>>::Error: Display,
{
    pub const fn new(expecting: &'static str) -> Self {
        Self {
            expecting,
            phantom: PhantomData,
        }
    }
}

impl<T: TryFrom<i64> + FromStr> de::Visitor<'_> for StringOrI64Visitor<T>
where
    <T as FromStr>::Err: Display,
    <T as TryFrom<i64>>::Error: Display,
{
    type Value = T;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(self.expecting)
    }

    fn visit_i8<E: de::Error>(self, v: i8) -> Result<Self::Value, E> {
        self.visit_i64(v.into())
    }

    fn visit_i16<E: de::Error>(self, v: i16) -> Result<Self::Value, E> {
        self.visit_i64(v.into())
    }

    fn visit_i32<E: de::Error>(self, v: i32) -> Result<Self::Value, E> {
        self.visit_i64(v.into())
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
        v.try_into().map_err(de::Error::custom)
    }

    fn visit_u8<E: de::Error>(self, v: u8) -> Result<Self::Value, E> {
        self.visit_u32(v.into())
    }

    fn visit_u16<E: de::Error>(self, v: u16) -> Result<Self::Value, E> {
        self.visit_u32(v.into())
    }

    fn visit_u32<E: de::Error>(self, v: u32) -> Result<Self::Value, E> {
        self.visit_u64(v.into())
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
        let value = i64::try_from(v).map_err(de::Error::custom)?;
        value.try_into().map_err(de::Error::custom)
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        v.parse().map_err(de::Error::custom)
    }

    fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
        self.visit_str(&v)
    }
}
