//! The vCard struct and types for vCard properties and values.

use language_tags::LanguageTag;
use std::{
    fmt::Debug,
    str::FromStr,
};
use time::{
    format_description::well_known::Iso8601, Date, OffsetDateTime, Time,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{property::*, Error, Result};

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

/// The vCard type.
#[derive(Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct Vcard {
    // General
    /// Value of the SOURCE property.
    pub source: Vec<UriProperty>,
    /// Value of the KIND property.
    pub kind: Option<KindProperty>,
    /// Value of the XML property.
    pub xml: Vec<Text>,

    // Identification
    /// Value of the FN property.
    pub formatted_name: Vec<Text>,
    /// Value of the N property.
    pub name: Option<TextListProperty>,
    /// Value of the NICKNAME property.
    pub nickname: Vec<Text>,
    /// Value of the PHOTO property.
    pub photo: Vec<UriProperty>,
    /// Value of the BDAY property.
    pub bday: Option<DateTimeOrTextProperty>,
    /// Value of the ANNIVERSARY property.
    pub anniversary: Option<DateTimeOrTextProperty>,
    /// Value of the GENDER property.
    pub gender: Option<Gender>,
    /// Value of the URL property.
    pub url: Vec<UriProperty>,

    // Organizational
    /// Value of the TITLE property.
    pub title: Vec<Text>,
    /// Value of the ROLE property.
    pub role: Vec<Text>,
    /// Value of the LOGO property.
    pub logo: Vec<UriProperty>,
    /// Value of the ORG property.
    pub org: Vec<TextListProperty>,
    /// Value of the MEMBER property.
    pub member: Vec<UriProperty>,
    /// Value of the RELATED property.
    pub related: Vec<TextOrUriProperty>,

    // Communications
    //pub tel: Vec<Text>,
    /// Value of the EMAIL property.
    pub email: Vec<Text>,
    /// Value of the IMPP property.
    pub impp: Vec<UriProperty>,
    /// Value of the LANG property.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub lang: Vec<LanguageTag>,

    // Geographic
    /// Value of the TZ property.
    pub timezone: Vec<TimeZoneProperty>,
    /// Value of the GEO property.
    pub geo: Vec<UriProperty>,

    // Explanatory
    /// Value of the CATEGORIES property.
    pub categories: Vec<TextListProperty>,
    /// Value of the NOTE property.
    pub note: Vec<Text>,
    /// Value of the PRODID property.
    pub prod_id: Option<Text>,
    /// Value of the REV property.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub rev: Option<OffsetDateTime>,

    //pub rev: Option<Timestamp>,
    /// Value of the SOUND property.
    pub sound: Vec<UriProperty>,
    /// Value of the UID property.
    pub uid: Option<TextOrUriProperty>,

    // Security
    /// Value of the KEY property.
    pub key: Vec<TextOrUriProperty>,
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
