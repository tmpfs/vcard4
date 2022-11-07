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
}

pub(crate) fn parse_boolean(s: &str) -> Result<bool> {
    match s {
        "true" | "TRUE" => Ok(true),
        "false" | "FALSE" => Ok(false),
        _ => Err(Error::InvalidBoolean(s.to_string())),
    }
}

pub(crate) fn format_date_time(d: &OffsetDateTime) -> Result<String> {
    let offset = d.clone().offset();

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
    Date(Date),
    /// Date and time value.
    DateTime(OffsetDateTime),
    /// Time value.
    Time(Time),
}

impl fmt::Display for DateAndOrTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Date(val) => write!(f, "{}", val),
            Self::DateTime(val) => write!(f, "{}", val),
            Self::Time(val) => write!(f, "{}", val),
        }
    }
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

        // 19531015T231000Z

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
