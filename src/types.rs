//! Custom data types.
use std::{fmt::Debug, str::FromStr};
use time::{
    format_description::well_known::Iso8601, Date, OffsetDateTime, Time,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{Error, Result};

pub(crate) fn parse_time(s: &str) -> Result<Time> {
    Ok(Time::parse(s, &Iso8601::DEFAULT)?)
}

pub(crate) fn parse_date(s: &str) -> Result<Date> {
    Ok(Date::parse(s, &Iso8601::DEFAULT)?)
}

pub(crate) fn parse_date_time(s: &str) -> Result<OffsetDateTime> {
    Ok(OffsetDateTime::parse(s, &Iso8601::DEFAULT)?)
}

pub(crate) fn parse_timestamp(s: &str) -> Result<OffsetDateTime> {
    parse_date_time(s)
    //Ok(OffsetDateTime::parse(s, &Iso8601::DEFAULT)?)
}

pub(crate) fn parse_boolean(s: &str) -> Result<bool> {
    match s {
        "true" | "TRUE" => Ok(true),
        "false" | "FALSE" => Ok(false),
        _ => Err(Error::InvalidBoolean(s.to_string())),
    }
}

/// Date and or time.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DateAndOrTime {
    /// Date value.
    Date(Date),
    /// Date and time value.
    DateTime(OffsetDateTime),
    /// Time value.
    Time(Time),
}

impl FromStr for DateAndOrTime {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if !s.is_empty() && &s[0..1] == "T" {
            return Ok(Self::Time(parse_time(&s[1..])?));
        }

        match parse_date_time(s) {
            Ok(value) => Ok(Self::DateTime(value)),
            Err(_) => match parse_date(s) {
                Ok(value) => Ok(Self::Date(value)),
                Err(_) => match parse_time(s) {
                    Ok(value) => Ok(Self::Time(value)),
                    Err(e) => Err(e.into()),
                },
            },
        }
    }
}

/// Integer type; may be a comma separated list.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub enum Integer {
    /// Single integer.
    One(i64),
    /// Multiple integers.
    Many(Vec<i64>),
}

impl FromStr for Integer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.contains(",") {
            let mut value = Vec::new();
            for val in s.split(",") {
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

impl FromStr for Float {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.contains(",") {
            let mut value = Vec::new();
            for val in s.split(",") {
                let val: f64 = val.parse()?;
                value.push(val);
            }
            Ok(Self::Many(value))
        } else {
            Ok(Self::One(s.parse()?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn parse_date_and_or_time() -> Result<()> {
        let value: DateAndOrTime = "T102200".parse()?;
        if !matches!(value, DateAndOrTime::Time(_)) {
            panic!("expecting Time variant");
        }

        let value: DateAndOrTime = "T1022".parse()?;
        if !matches!(value, DateAndOrTime::Time(_)) {
            panic!("expecting Time variant");
        }

        let value: DateAndOrTime = "T10".parse()?;
        if !matches!(value, DateAndOrTime::Time(_)) {
            panic!("expecting Time variant");
        }

        /*
        let value: DateAndOrTime = "-2200".parse()?;
        if !matches!(value, DateAndOrTime::Time(_)) {
            panic!("expecting Time variant");
        }
        */

        /*
        let value: DateAndOrTime = "--00".parse()?;
        if !matches!(value, DateAndOrTime::Time(_)) {
            panic!("expecting Time variant");
        }
        */

        let value: DateAndOrTime = "102200Z".parse()?;
        if !matches!(value, DateAndOrTime::Time(_)) {
            panic!("expecting Time variant");
        }

        let value: DateAndOrTime = "102200-0800".parse()?;
        if !matches!(value, DateAndOrTime::Time(_)) {
            panic!("expecting Time variant");
        }

        let value: DateAndOrTime = "19850412".parse()?;
        if !matches!(value, DateAndOrTime::Date(_)) {
            panic!("expecting Date variant");
        }

        /*
        let value: DateAndOrTime = "1985-04".parse()?;
        if !matches!(value, DateAndOrTime::Date(_)) {
            panic!("expecting Date variant");
        }
        */

        /*
        let value: DateAndOrTime = "1985".parse()?;
        if !matches!(value, DateAndOrTime::Date(_)) {
            panic!("expecting Date variant");
        }
        */

        /*
        let value: DateAndOrTime = "--0412".parse()?;
        if !matches!(value, DateAndOrTime::Date(_)) {
            panic!("expecting Date variant");
        }
        */

        /*
        let value: DateAndOrTime = "---12".parse()?;
        if !matches!(value, DateAndOrTime::Date(_)) {
            panic!("expecting Date variant");
        }
        */

        Ok(())
    }
}
