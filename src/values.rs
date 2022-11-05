//! The vCard struct and types for vCard properties and values.

use fluent_uri::Uri as URI;
use language_tags::LanguageTag;
use std::{
    fmt::{self, Debug},
    str::FromStr,
};

use crate::{Error, Result};

/// Either text or a URI.
#[derive(Debug)]
pub enum TextOrUri {
    Text(Text),
    Uri(Uri),
}

/// Enumeration of the different types of values.
#[derive(Debug)]
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

/// Represents a gender associated with a vCard.
#[derive(Debug)]
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
#[derive(Debug, Default)]
pub struct Parameters {
    /// The language tag.
    pub language: Option<LanguageTag>,
    /// The property types.
    pub types: Option<Vec<String>>,
    /// The value type hint for this property.
    pub value: Option<ValueType>,
}

/// Text property value.
#[derive(Debug)]
pub struct Text {
    pub value: String,
    pub parameters: Option<Parameters>,
}

/// Text list property value.
#[derive(Debug)]
pub struct TextList {
    pub value: Vec<String>,
    pub parameters: Option<Parameters>,
}

/// URI property value.
#[derive(Debug)]
pub struct Uri {
    pub value: URI<String>,
    pub parameters: Option<Parameters>,
}

/// The vCard type.
#[derive(Debug, Default)]
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
