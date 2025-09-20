//! Utilities for parsing dates, times and primitive values.
use std::fmt;
use time::{
    format_description::{self, well_known::Iso8601},
    Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset,
};

use crate::{property::DateAndOrTime, DateTime, Error, Result};

// UTC OFFSET

/// Parse a UTC offset.
pub fn parse_utc_offset(value: &str) -> Result<UtcOffset> {
    if value == "Z" {
        return Ok(UtcOffset::UTC);
    }

    //println!("Parsing value {}", value);

    let offset_format = format_description::parse(
        "[offset_hour sign:mandatory][offset_minute]",
    )?;

    let offset_hours =
        format_description::parse("[offset_hour sign:mandatory]")?;

    if let Ok(result) = UtcOffset::parse(value, &offset_format) {
        Ok(result)
    } else {
        Ok(UtcOffset::parse(value, &offset_hours)?)
    }
}

pub(crate) fn format_utc_offset(
    f: &mut fmt::Formatter<'_>,
    val: &UtcOffset,
) -> fmt::Result {
    let offset = format_description::parse(
        "[offset_hour sign:mandatory][offset_minute]",
    )
    .map_err(|_| fmt::Error)?;
    write!(f, "{}", val.format(&offset).map_err(|_| fmt::Error)?)
}

// TIME

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
        let mut parts = value.split("").collect::<Vec<_>>();
        let val = parts
            .get_mut(1)
            .ok_or_else(|| Error::InvalidTime(value.to_string()))?;
        if *val == "-" {
            *val = "00";
        }

        let val = parts
            .get_mut(2)
            .ok_or_else(|| Error::InvalidTime(value.to_string()))?;

        if val.is_empty() {
            return Err(Error::InvalidTime(value.to_string()));
        }

        if *val == "-" {
            *val = "00";
        }
        let value = parts.join("");
        do_parse_time(&value)
    } else {
        do_parse_time(value)
    }
}

fn do_parse_time(mut value: &str) -> Result<(Time, UtcOffset)> {
    let mut offset = UtcOffset::UTC;
    let pos = value.find('-').or_else(|| value.find('+'));
    if let Some(pos) = pos {
        let offset_value = &value[pos..];
        offset = parse_utc_offset(offset_value)?;
        value = &value[0..pos];
    }

    if value.ends_with('Z') {
        value = &value[0..value.len() - 1];
    }

    let time = Time::parse(value, &Iso8601::DEFAULT)?;
    Ok((time, offset))
}

pub(crate) fn format_time(value: &(Time, UtcOffset)) -> Result<String> {
    let (time, offset) = value;
    let format = format_description::parse("[hour][minute][second]")?;
    let offset_format = format_description::parse(
        "[offset_hour sign:mandatory][offset_minute]",
    )?;

    let result = format!(
        "{}{}",
        time.format(&format)?,
        offset.format(&offset_format)?
    );
    Ok(result)
}

pub(crate) fn format_time_list(
    f: &mut fmt::Formatter<'_>,
    val: &[(Time, UtcOffset)],
) -> fmt::Result {
    for (index, item) in val.iter().enumerate() {
        write!(f, "{}", &format_time(item).map_err(|_| fmt::Error)?)?;
        if index < val.len() - 1 {
            write!(f, ",")?;
        }
    }
    Ok(())
}

// DATE

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
        let mut parts = value.split("").collect::<Vec<_>>();
        let val = parts
            .get_mut(1)
            .ok_or_else(|| Error::InvalidDate(value.to_string()))?;
        if *val == "-" {
            *val = "00";
        }
        let val = parts
            .get_mut(2)
            .ok_or_else(|| Error::InvalidDate(value.to_string()))?;
        if *val == "-" {
            *val = "00";
        }
        if let Some(val) = parts.get_mut(3)
            && *val == "-" {
                *val = "01";
            }

        let value = parts.join("");
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

pub(crate) fn format_date(value: &crate::Date) -> Result<String> {
    let date = format_description::parse("[year][month][day]")?;
    Ok(value.as_ref().format(&date)?)
}

pub(crate) fn format_date_list(
    f: &mut fmt::Formatter<'_>,
    val: &[crate::Date],
) -> fmt::Result {
    for (index, item) in val.iter().enumerate() {
        write!(f, "{}", &format_date(item).map_err(|_| fmt::Error)?)?;
        if index < val.len() - 1 {
            write!(f, ",")?;
        }
    }
    Ok(())
}

// DATETIME

/// Parse a list of date times separated by a comma.
pub fn parse_date_time_list(value: &str) -> Result<Vec<DateTime>> {
    let mut values = Vec::new();
    for value in value.split(',') {
        values.push(parse_date_time(value)?);
    }
    Ok(values)
}

/// Parse a date time.
pub fn parse_date_time(value: &str) -> Result<DateTime> {
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
    Ok(utc.into())
}

pub(crate) fn format_date_time(d: &DateTime) -> Result<String> {
    let d = d.as_ref();
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

pub(crate) fn format_date_time_list(
    f: &mut fmt::Formatter<'_>,
    val: &[DateTime],
) -> fmt::Result {
    for (index, item) in val.iter().enumerate() {
        write!(f, "{}", &format_date_time(item).map_err(|_| fmt::Error)?)?;
        if index < val.len() - 1 {
            write!(f, ",")?;
        }
    }
    Ok(())
}

// TIMESTAMP

/// Parse a timestamp.
pub fn parse_timestamp(value: &str) -> Result<DateTime> {
    let offset_format = format_description::parse(
            "[year][month][day]T[hour][minute][second][offset_hour sign:mandatory][offset_minute]",
        )?;
    let offset_format_hours = format_description::parse(
            "[year][month][day]T[hour][minute][second][offset_hour sign:mandatory]",
        )?;
    let utc_format = format_description::parse(
        "[year][month][day]T[hour][minute][second]Z",
    )?;
    let implicit_utc_format = format_description::parse(
        "[year][month][day]T[hour][minute][second]",
    )?;

    if let Ok(result) = OffsetDateTime::parse(value, &offset_format) {
        Ok(result.into())
    } else if let Ok(result) =
        OffsetDateTime::parse(value, &offset_format_hours)
    {
        Ok(result.into())
    } else if let Ok(result) = PrimitiveDateTime::parse(value, &utc_format) {
        let result = OffsetDateTime::now_utc().replace_date_time(result);
        Ok(result.into())
    } else {
        let result = PrimitiveDateTime::parse(value, &implicit_utc_format)?;
        let result = OffsetDateTime::now_utc().replace_date_time(result);
        Ok(result.into())
    }
}

pub(crate) fn format_timestamp_list(
    f: &mut fmt::Formatter<'_>,
    val: &[DateTime],
) -> fmt::Result {
    for (index, item) in val.iter().enumerate() {
        write!(f, "{}", &format_date_time(item).map_err(|_| fmt::Error)?)?;
        if index < val.len() - 1 {
            write!(f, ",")?;
        }
    }
    Ok(())
}

/// Parse a list of date and or time types possibly separated by a comma.
pub fn parse_timestamp_list(value: &str) -> Result<Vec<DateTime>> {
    let mut values = Vec::new();
    for value in value.split(',') {
        values.push(parse_timestamp(value)?);
    }
    Ok(values)
}

// DATE AND OR TIME

/// Parse a list of date and or time types possibly separated by a comma.
pub fn parse_date_and_or_time_list(
    value: &str,
) -> Result<Vec<DateAndOrTime>> {
    let mut values = Vec::new();
    for value in value.split(',') {
        values.push(value.parse()?);
    }
    Ok(values)
}

pub(crate) fn format_date_and_or_time_list(
    f: &mut fmt::Formatter<'_>,
    val: &[DateAndOrTime],
) -> fmt::Result {
    for (index, item) in val.iter().enumerate() {
        write!(f, "{}", item)?;
        if index < val.len() - 1 {
            write!(f, ",")?;
        }
    }
    Ok(())
}

// Primitives

/// Parse a boolean.
pub fn parse_boolean(value: &str) -> Result<bool> {
    let lower = value.to_lowercase();
    match &lower[..] {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(Error::InvalidBoolean(value.to_string())),
    }
}

/// Parse a list of integers.
pub fn parse_integer_list(value: &str) -> Result<Vec<i64>> {
    let mut values = Vec::new();
    for value in value.split(',') {
        values.push(value.parse()?);
    }
    Ok(values)
}

pub(crate) fn format_integer_list(
    f: &mut fmt::Formatter<'_>,
    val: &[i64],
) -> fmt::Result {
    for (index, item) in val.iter().enumerate() {
        write!(f, "{}", item)?;
        if index < val.len() - 1 {
            write!(f, ",")?;
        }
    }
    Ok(())
}

/// Parse a list of floats.
pub fn parse_float_list(value: &str) -> Result<Vec<f64>> {
    let mut values = Vec::new();
    for value in value.split(',') {
        values.push(value.parse()?);
    }
    Ok(values)
}

pub(crate) fn format_float_list(
    f: &mut fmt::Formatter<'_>,
    val: &[f64],
) -> fmt::Result {
    for (index, item) in val.iter().enumerate() {
        write!(f, "{}", item)?;
        if index < val.len() - 1 {
            write!(f, ",")?;
        }
    }
    Ok(())
}
