//! Custom data types.
use std::{fmt, str::FromStr};
use time::{
    format_description::{self, well_known::Iso8601},
    Date, OffsetDateTime, Time, UtcOffset,
};
use uriparse::uri::URI as Uri;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{Error, Result};

/// Parse a list of times separated by a comma.
pub fn parse_time_list(value: &str) -> Result<Vec<(Time, UtcOffset)>> {
    let mut values = Vec::new();
    for value in value.split(',') {
        values.push(parse_time(value)?);
    }
    Ok(values)
}

/// Parse a time.
pub fn parse_time(value: &str) -> Result<(Time, UtcOffset)> {
    if value.starts_with('-') {
        let mut value = value.split("").collect::<Vec<_>>();
        if let Some(val) = value.get_mut(1) {
            if *val == "-" {
                *val = "00";
            }
        }
        if let Some(val) = value.get_mut(2) {
            if *val == "-" {
                *val = "00";
            }
        }
        let value = value.join("");
        do_parse_time(&value)
    } else {
        do_parse_time(value)
    }
}

fn do_parse_time(value: &str) -> Result<(Time, UtcOffset)> {
    let mut offset = UtcOffset::UTC;
    if value.len() > 6 {
        let offset_value = &value[6..];
        let offset_format = format_description::parse(
            "[offset_hour sign:mandatory][offset_minute]",
        )?;
        if offset_value != "Z" {
            offset = UtcOffset::parse(offset_value, &offset_format)?;
        }
    }
    let time = Time::parse(value, &Iso8601::DEFAULT)?;
    Ok((time, offset))
}

/// Parse a list of dates separated by a comma.
pub fn parse_date_list(value: &str) -> Result<Vec<Date>> {
    let mut values = Vec::new();
    for value in value.split(',') {
        values.push(parse_date(value)?);
    }
    Ok(values)
}

/// Parse a date.
pub fn parse_date(value: &str) -> Result<Date> {
    if value.starts_with('-') {
        let mut value = value.split("").collect::<Vec<_>>();
        if let Some(val) = value.get_mut(1) {
            if *val == "-" {
                *val = "00";
            }
        }
        if let Some(val) = value.get_mut(2) {
            if *val == "-" {
                *val = "00";
            }
        }
        if let Some(val) = value.get_mut(3) {
            if *val == "-" {
                *val = "01";
            }
        }

        let value = value.join("");
        do_parse_date(&value)
    // Got a YYYY-MM format need to use 01 for the day
    } else if value.len() == 7 {
        let value = format!("{}-01", value);
        do_parse_date(&value)
    // Got a YYYY format need to use 01 for the month and day
    } else if value.len() == 4 {
        let value = format!("{}-01-01", value);
        do_parse_date(&value)
    } else {
        do_parse_date(value)
    }
}

fn do_parse_date(s: &str) -> Result<Date> {
    let date_separator = format_description::parse("[year]-[month]-[day]")?;
    let date = format_description::parse("[year][month][day]")?;

    let year_month_separator = format_description::parse("[year]-[month]")?;

    let year_month = format_description::parse("[year][month]")?;

    if let Ok(result) = Date::parse(s, &date_separator) {
        Ok(result)
    } else if let Ok(result) = Date::parse(s, &date) {
        Ok(result)
    } else if let Ok(result) = Date::parse(s, &year_month_separator) {
        Ok(result)
    } else if let Ok(result) = Date::parse(s, &year_month) {
        Ok(result)
    } else {
        Ok(Date::parse(s, &Iso8601::DEFAULT)?)
    }
}

/// Parse a list of date times separated by a comma.
pub fn parse_date_time_list(value: &str) -> Result<Vec<OffsetDateTime>> {
    let mut values = Vec::new();
    for value in value.split(',') {
        values.push(parse_date_time(value)?);
    }
    Ok(values)
}

/// Parse a date time.
pub fn parse_date_time(value: &str) -> Result<OffsetDateTime> {
    let mut it = value.splitn(2, 'T');
    let date = it
        .next()
        .ok_or_else(|| Error::InvalidDateTime(value.to_owned()))?;
    let time = it
        .next()
        .ok_or_else(|| Error::InvalidDateTime(value.to_owned()))?;

    let date = parse_date(date)?;
    let (time, offset) = parse_time(time)?;

    let utc = OffsetDateTime::now_utc()
        .replace_date(date)
        .replace_time(time)
        .replace_offset(offset);
    Ok(utc)
}

/// Parse a timestamp.
pub(crate) fn parse_timestamp(value: &str) -> Result<OffsetDateTime> {
    parse_date_time(value)
}

/// Parse a boolean.
pub fn parse_boolean(s: &str) -> Result<bool> {
    match s {
        "true" | "TRUE" => Ok(true),
        "false" | "FALSE" => Ok(false),
        _ => Err(Error::InvalidBoolean(s.to_string())),
    }
}

pub(crate) fn format_date_time(d: &OffsetDateTime) -> Result<String> {
    let offset = (*d).offset();

    let format = if offset == UtcOffset::UTC {
        format_description::parse(
            "[year][month][day]T[hour][minute][second]Z",
        )?
    } else {
        format_description::parse(
            "[year][month][day]T[hour][minute][second][offset_hour sign:mandatory][offset_minute]",
        )?
    };

    Ok(d.format(&format)?)
}

/// Date and or time.
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DateAndOrTime {
    /// Date value.
    Date(Vec<Date>),
    /// Date and time value.
    DateTime(Vec<OffsetDateTime>),
    /// Time value.
    Time(Vec<(Time, UtcOffset)>),
}

impl fmt::Display for DateAndOrTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Date(val) => write!(
                f,
                "{}",
                val.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::DateTime(val) => write!(
                f,
                "{}",
                val.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::Time(val) => write!(
                f,
                "{}",
                val.iter()
                    .map(|(time, offset)| format!("{}{}", time, offset))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        }
    }
}

impl FromStr for DateAndOrTime {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if !s.is_empty() && &s[0..1] == "T" {
            return Ok(Self::Time(parse_time_list(&s[1..])?));
        }

        match parse_date_time_list(s) {
            Ok(value) => Ok(Self::DateTime(value)),
            Err(_) => match parse_date_list(s) {
                Ok(value) => Ok(Self::Date(value)),
                Err(_) => match parse_time_list(s) {
                    Ok(value) => Ok(Self::Time(value)),
                    Err(e) => Err(e),
                },
            },
        }
    }
}

/// Integer type; may be a comma separated list.
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub enum Integer {
    /// Single integer.
    One(i64),
    /// Multiple integers.
    Many(Vec<i64>),
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::One(ref val) => write!(f, "{}", val),
            Self::Many(ref val) => {
                let values: Vec<String> =
                    val.iter().map(|v| v.to_string()).collect();
                write!(f, "{}", values.join(","))
            }
        }
    }
}

impl FromStr for Integer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.contains(',') {
            let mut value = Vec::new();
            for val in s.split(',') {
                let val: i64 = val.parse()?;
                value.push(val);
            }
            Ok(Self::Many(value))
        } else {
            Ok(Self::One(s.parse()?))
        }
    }
}

/// Float type; may be a comma separated list.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub enum Float {
    /// Single float.
    One(f64),
    /// Multiple floats.
    Many(Vec<f64>),
}

impl Eq for Float {}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::One(ref val) => write!(f, "{}", val),
            Self::Many(ref val) => {
                let values: Vec<String> =
                    val.iter().map(|v| v.to_string()).collect();
                write!(f, "{}", values.join(","))
            }
        }
    }
}

impl FromStr for Float {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.contains(',') {
            let mut value = Vec::new();
            for val in s.split(',') {
                let val: f64 = val.parse()?;
                value.push(val);
            }
            Ok(Self::Many(value))
        } else {
            Ok(Self::One(s.parse()?))
        }
    }
}

/// Value for the CLIENTPIDMAP property.
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct ClientPidMap {
    /// The source identifier.
    pub source: u64,
    /// The URI for the map.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub uri: Uri<'static>,
}

impl fmt::Display for ClientPidMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{};{}", self.source, self.uri)
    }
}

impl FromStr for ClientPidMap {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut it = s.splitn(2, ';');
        let source = it
            .next()
            .ok_or_else(|| Error::InvalidClientPidMap(s.to_string()))?;
        let uri = it
            .next()
            .ok_or_else(|| Error::InvalidClientPidMap(s.to_string()))?;
        let source: u64 = source.parse()?;

        // Must be positive according to the RFC
        // https://www.rfc-editor.org/rfc/rfc6350#section-6.7.7
        if source == 0 {
            return Err(Error::InvalidClientPidMap(s.to_string()));
        }

        let uri = Uri::try_from(uri)?.into_owned();
        Ok(ClientPidMap { source, uri })
    }
}
