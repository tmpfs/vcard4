//! The vCard struct and types for vCard properties and values.

use fluent_uri::Uri as URI;
use language_tags::LanguageTag;
use std::{
    fmt::{self, Debug},
    str::FromStr,
};
use time::UtcOffset as UTCOffset;

use crate::{Error, Result};

/// Either text or a URI.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextOrUri {
    /// Text value.
    Text(Text),
    /// URI value.
    Uri(Uri),
}

/// Enumeration of the different types of values.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ValueType {
    /// Text value.
    Text,
    /// URI value.
    Uri,
    /// Date value.
    Date,
    /// Time value.
    Time,
    /// Date and time value.
    DateTime,
    /// Date and or time value.
    DateAndOrTime,
    /// Timestamp value.
    Timestamp,
    /// Boolean value.
    Boolean,
    /// Integer value.
    Integer,
    /// Float value.
    Float,
    /// UTC offset value.
    UtcOffset,
    /// Language tag value.
    LanguageTag,
    /*
    /// IANA token value.
    IanaToken,
    /// X-name value.
    XName,
    */
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Text => "text",
                Self::Uri => "uri",
                Self::Date => "date",
                Self::Time => "time",
                Self::DateTime => "date-time",
                Self::DateAndOrTime => "date-and-or-time",
                Self::Timestamp => "timestamp",
                Self::Boolean => "boolean",
                Self::Integer => "integer",
                Self::Float => "float",
                Self::UtcOffset => "utc-offset",
                Self::LanguageTag => "language-tag",
            }
        )
    }
}

impl FromStr for ValueType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "text" => Ok(Self::Text),
            "uri" => Ok(Self::Uri),
            "date" => Ok(Self::Date),
            "time" => Ok(Self::Time),
            "date-time" => Ok(Self::DateTime),
            "date-and-or-time" => Ok(Self::DateAndOrTime),
            "timestamp" => Ok(Self::Timestamp),
            "boolean" => Ok(Self::Boolean),
            "integer" => Ok(Self::Integer),
            "float" => Ok(Self::Float),
            "utc-offset" => Ok(Self::UtcOffset),
            "language-tag" => Ok(Self::LanguageTag),
            _ => Err(Error::UnknownValueType(s.to_string())),
        }
    }
}

/// Enumeration for sex.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Sex {
    /// No sex specified.
    None,
    /// Male sex.
    Male,
    /// Female sex.
    Female,
    /// Other sex.
    Other,
    /// Not applicable.
    NotApplicable,
    /// Unknown sex.
    Unknown,
}

impl fmt::Display for Sex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "",
                Self::Male => "M",
                Self::Female => "F",
                Self::Other => "O",
                Self::NotApplicable => "N",
                Self::Unknown => "U",
            }
        )
    }
}

impl FromStr for Sex {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "" => Ok(Self::None),
            "M" => Ok(Self::Male),
            "F" => Ok(Self::Female),
            "O" => Ok(Self::Other),
            "N" => Ok(Self::NotApplicable),
            "U" => Ok(Self::Unknown),
            _ => Err(Error::UnknownSex(s.to_string())),
        }
    }
}

/// Value for the `utc-offset` type.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UtcOffset {
    /// The value for the UTC offset.
    pub value: UTCOffset,
    /// The parameters for the property.
    pub parameters: Option<Parameters>,
}

impl fmt::Display for UtcOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (h, m, _) = self.value.as_hms();
        let sign = if h >= 0 { '+' } else { '-' };
        let h = h.abs();
        let m = m.abs();
        let h = if h < 10 {
            format!("0{}", h)
        } else {
            h.to_string()
        };
        let m = if m < 10 {
            format!("0{}", m)
        } else {
            m.to_string()
        };
        write!(f, "{}{}{}", sign, h, m,)
    }
}

impl FromStr for UtcOffset {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() == 5 {
            let sign = &s[0..1];
            if sign != "+" && sign != "-" {
                return Err(Error::InvalidUtcOffset(s.to_string()));
            }
            let hours = &s[1..3];
            let minutes = &s[3..5];
            let mut hours: i8 = hours.parse()?;
            let mut minutes: i8 = minutes.parse()?;
            if sign == "-" {
                hours = -hours;
                minutes = -minutes;
            }
            return Ok(Self {
                value: UTCOffset::from_hms(hours, minutes, 0)?,
                parameters: None,
            });
        }

        Err(Error::InvalidUtcOffset(s.to_string()))
    }
}

/// Value for a timezone (`TZ`).
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Timezone {
    /// Text value.
    Text(Text),
    /// URI value.
    Uri(Uri),
    /// UTC offset value.
    UtcOffset(UtcOffset),
}

/// Represents a gender associated with a vCard.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Gender {
    /// The sex for the gender.
    pub sex: Sex,
    /// The identity text.
    pub identity: Option<String>,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(identity) = &self.identity {
            write!(f, "{};{}", self.sex, identity)
        } else {
            write!(f, "{}", self.sex,)
        }
    }
}

impl FromStr for Gender {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.is_empty() {
            return Ok(Gender {
                sex: Sex::None,
                identity: None,
            });
        }

        let mut it = s.splitn(2, ";");
        let sex = it.next().ok_or(Error::NoSex)?;
        let sex: Sex = sex.parse()?;
        let mut gender = Gender {
            sex,
            identity: None,
        };
        if let Some(identity) = it.next() {
            gender.identity = Some(identity.to_string());
        }

        Ok(gender)
    }
}

/// Kind of vCard.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Kind {
    /// An individual.
    Individual,
    /// A group.
    Group,
    /// An organization.
    Org,
    /// A location.
    Location,
    // TODO: x-name
    // TODO: iana-token
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Individual => "individual",
                Self::Group => "group",
                Self::Org => "org",
                Self::Location => "location",
            }
        )
    }
}

impl FromStr for Kind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "individual" => Ok(Self::Individual),
            "group" => Ok(Self::Group),
            "org" => Ok(Self::Org),
            "location" => Ok(Self::Location),
            _ => Err(Error::UnknownKind(s.to_string())),
        }
    }
}

/// Parameters for a vCard property.
#[derive(Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parameters {
    /// The language tag.
    pub language: Option<LanguageTag>,
    /// The property types.
    pub types: Option<Vec<String>>,
    /// The value type hint for this property.
    pub value: Option<ValueType>,
}

/// Text property value.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Text {
    pub value: String,
    pub parameters: Option<Parameters>,
}

/// Text list property value.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextList {
    pub value: Vec<String>,
    pub parameters: Option<Parameters>,
}

#[cfg(feature = "serde")]
mod uri_from_str {
    use fluent_uri::Uri as URI;
    use serde::{
        de::{Deserializer, Error, Visitor},
        ser::Serializer,
    };
    use std::fmt;

    pub fn serialize<S>(
        source: &URI<String>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(source.as_str())
    }

    struct UriVisitor;

    impl<'de> Visitor<'de> for UriVisitor {
        type Value = URI<String>;

        fn expecting(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(URI::parse(v).map_err(Error::custom)?.to_owned())
        }
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<URI<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(UriVisitor)
    }
}

/// URI property value.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Uri {
    #[cfg_attr(feature = "serde", serde(with = "uri_from_str"))]
    pub value: URI<String>,
    pub parameters: Option<Parameters>,
}

impl PartialEq for Uri {
    fn eq(&self, other: &Self) -> bool {
        self.value.as_str() == other.value.as_str()
            && self.parameters == other.parameters
    }
}

/// The vCard type.
#[derive(Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vcard {
    // General
    pub source: Vec<Uri>,
    pub kind: Option<Kind>,
    pub xml: Vec<Text>,

    // Identification
    pub formatted_name: Vec<Text>,
    pub name: Option<TextList>,
    pub nickname: Vec<Text>,
    pub photo: Vec<Uri>,
    //pub bday: Vec<Uri>,
    //pub anniversary: Vec<Uri>,
    pub gender: Option<Gender>,
    pub url: Vec<Uri>,

    // Organizational
    pub title: Vec<Text>,
    pub role: Vec<Text>,
    pub logo: Vec<Uri>,
    pub org: Vec<TextList>,
    pub member: Vec<Uri>,
    pub related: Vec<TextOrUri>,

    // Communications
    //pub tel: Vec<Text>,
    pub email: Vec<Text>,
    pub impp: Vec<Uri>,
    pub lang: Vec<LanguageTag>,

    // Geographic
    pub timezone: Vec<Timezone>,
    pub geo: Vec<Uri>,

    // Explanatory
    pub categories: Vec<TextList>,
    pub note: Vec<Text>,
    pub prod_id: Option<Text>,

    //pub rev: Option<Timestamp>,
    pub sound: Vec<Uri>,
    pub uid: Option<TextOrUri>,

    // Security
    pub key: Vec<TextOrUri>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn parse_utc_offset() -> Result<()> {
        let east = "+1200".parse::<UtcOffset>()?;
        let west = "-0500".parse::<UtcOffset>()?;

        assert_eq!("+1200", east.to_string());
        assert_eq!("-0500", west.to_string());

        assert!("0500".parse::<UtcOffset>().is_err());
        assert!("foo".parse::<UtcOffset>().is_err());
        assert!("+4400".parse::<UtcOffset>().is_err());

        Ok(())
    }
}
