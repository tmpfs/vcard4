//! Custom data types.
use std::{fmt::Debug, str::FromStr};
use time::{
    format_description::well_known::Iso8601, Date, OffsetDateTime, Time,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

fn parse_time(s: &str) -> Result<Time> {
    Ok(Time::parse(s, &Iso8601::DEFAULT)?)
}

fn parse_date(s: &str) -> Result<Date> {
    Ok(Date::parse(s, &Iso8601::DEFAULT)?)
}

fn parse_date_time(s: &str) -> Result<OffsetDateTime> {
    Ok(OffsetDateTime::parse(s, &Iso8601::DEFAULT)?)
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
