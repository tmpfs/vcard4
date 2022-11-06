//! Types for property parameters.

use language_tags::LanguageTag;
use std::{
    fmt::{self, Debug},
    str::FromStr,
};
use time::UtcOffset;
use uriparse::uri::URI as Uri;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

#[cfg(feature = "mime")]
use mime::Mime;

use crate::{Error, Result};

/// Names of properties that are allowed to specify a TYPE parameter.
pub const TYPE_PROPERTIES: [&'static str; 23] = [
    "FN",
    "NICKNAME",
    "PHOTO",
    "ADR",
    "TEL",
    "EMAIL",
    "IMPP",
    "LANG",
    "TZ",
    "GEO",
    "TITLE",
    "ROLE",
    "LOGO",
    "ORG",
    "RELATED",
    "CATEGORIES",
    "NOTE",
    "SOUND",
    "URL",
    "KEY",
    "FBURL",
    "CALADRURI",
    "CALURI",
];

/// Value for a TYPE parameter.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub enum TypeParameter {
    /// Related to a home environment.
    Home,
    /// Related to a work environment.
    Work,
    /// Type for the TEL property.
    Telephone(TelephoneTypeValue),
    /// Type for the RELATED property.
    Related(RelatedTypeValue),
}

impl fmt::Display for TypeParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Home => write!(f, "home"),
            Self::Work => write!(f, "work"),
            Self::Telephone(ref tel) => write!(f, "{}", tel),
            Self::Related(ref rel) => write!(f, "{}", rel),
        }
    }
}

impl FromStr for TypeParameter {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "home" => Ok(Self::Home),
            "work" => Ok(Self::Work),
            _ => match s.parse::<TelephoneTypeValue>() {
                Ok(tel) => Ok(Self::Telephone(tel)),
                Err(_) => {
                    let rel: RelatedTypeValue = s.parse()?;
                    Ok(Self::Related(rel))
                }
            },
        }
    }
}

/// Values for a PID parameter.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct Pid {
    /// Digits before a period.
    pub major: u64,
    /// Digits after a period.
    pub minor: Option<u64>,
}

impl fmt::Display for Pid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(minor) = self.minor {
            write!(f, "{}.{}", self.major, minor)
        } else {
            write!(f, "{}", self.major)
        }
    }
}

impl FromStr for Pid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.splitn(2, ".");
        let major = parts.next().ok_or(Error::InvalidPid(s.to_string()))?;
        let major: u64 = major
            .parse()
            .map_err(|_| Error::InvalidPid(s.to_string()))?;
        let mut pid = Pid {
            major: major,
            minor: None,
        };
        if let Some(minor) = parts.next() {
            let minor: u64 = minor
                .parse()
                .map_err(|_| Error::InvalidPid(s.to_string()))?;
            pid.minor = Some(minor);
        }
        Ok(pid)
    }
}

/// Enumeration of related types.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub enum RelatedTypeValue {
    /// Contact relationship.
    Contact,
    /// Acquaintance relationship.
    Acquaintance,
    /// Friend relationship.
    Friend,
    /// Met relationship.
    Met,
    /// Co-worker relationship.
    CoWorker,
    /// Colleague relationship.
    Colleague,
    /// Co-resident relationship.
    CoResident,
    /// Neighbor relationship.
    Neighbor,
    /// Child relationship.
    Child,
    /// Parent relationship.
    Parent,
    /// Sibling relationship.
    Sibling,
    /// Spouse relationship.
    Spouse,
    /// Kin relationship.
    Kin,
    /// Muse relationship.
    Muse,
    /// Crush relationship.
    Crush,
    /// Date relationship.
    Date,
    /// Sweetheart relationship.
    Sweetheart,
    /// Oneself.
    Me,
    /// Agent relationship.
    Agent,
    /// Emergency relationship.
    Emergency,
}

impl fmt::Display for RelatedTypeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Contact => "contact",
                Self::Acquaintance => "acquaintance",
                Self::Friend => "friend",
                Self::Met => "met",
                Self::CoWorker => "co-worker",
                Self::Colleague => "colleague",
                Self::CoResident => "co-resident",
                Self::Neighbor => "neighbor",
                Self::Child => "child",
                Self::Parent => "parent",
                Self::Sibling => "sibling",
                Self::Spouse => "spouse",
                Self::Kin => "kin",
                Self::Muse => "muse",
                Self::Crush => "crush",
                Self::Date => "date",
                Self::Sweetheart => "sweetheart",
                Self::Me => "me",
                Self::Agent => "agent",
                Self::Emergency => "emergency",
            }
        )
    }
}

impl FromStr for RelatedTypeValue {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "contact" => Ok(Self::Contact),
            "acquaintance" => Ok(Self::Acquaintance),
            "friend" => Ok(Self::Friend),
            "met" => Ok(Self::Met),
            "co-worker" => Ok(Self::CoWorker),
            "colleague" => Ok(Self::Colleague),
            "co-resident" => Ok(Self::CoResident),
            "neighbor" => Ok(Self::Neighbor),
            "child" => Ok(Self::Child),
            "parent" => Ok(Self::Parent),
            "sibling" => Ok(Self::Sibling),
            "spouse" => Ok(Self::Spouse),
            "kin" => Ok(Self::Kin),
            "muse" => Ok(Self::Muse),
            "crush" => Ok(Self::Crush),
            "date" => Ok(Self::Date),
            "sweetheart" => Ok(Self::Sweetheart),
            "me" => Ok(Self::Me),
            "agent" => Ok(Self::Agent),
            "emergency" => Ok(Self::Emergency),
            _ => Err(Error::UnknownRelatedTypeValue(s.to_string())),
        }
    }
}

/// Enumeration of telephone types.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub enum TelephoneTypeValue {
    /// Indicates that the telephone number supports
    /// text messages (SMS).
    Text,
    /// Indicates a voice telephone number.
    Voice,
    /// Indicates a facsimile telephone number.
    Fax,
    /// Indicates a cellular or mobile telephone number.
    Cell,
    /// Indicates a video conferencing telephone number.
    Video,
    /// Indicates a paging device telephone number.
    Pager,
    /// Indicates a telecommunication device for people with
    /// hearing or speech difficulties.  
    TextPhone,
}

impl fmt::Display for TelephoneTypeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Text => "text",
                Self::Voice => "voice",
                Self::Fax => "fax",
                Self::Cell => "cell",
                Self::Video => "video",
                Self::Pager => "pager",
                Self::TextPhone => "textphone",
            }
        )
    }
}

impl FromStr for TelephoneTypeValue {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "text" => Ok(Self::Text),
            "voice" => Ok(Self::Voice),
            "fax" => Ok(Self::Fax),
            "cell" => Ok(Self::Cell),
            "video" => Ok(Self::Video),
            "pager" => Ok(Self::Pager),
            "textphone" => Ok(Self::TextPhone),
            _ => Err(Error::UnknownTelephoneTypeValue(s.to_string())),
        }
    }
}

/// Enumeration of the different types of values.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub enum ValueType {
    /// Text value.
    Text,
    /// Uri value.
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

/// Value for a timezone parameter.
///
/// This is a different type so that we do not
/// create infinite type recursion in `Parameters` which would
/// require us to wrap it in a `Box`.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub enum TimeZoneParameter {
    /// Text value.
    Text(String),
    /// Uri value.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    Uri(Uri<'static>),
    /// UTC offset value.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    UtcOffset(UtcOffset),
}

/// Parameters for a vCard property.
#[derive(Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct Parameters {
    /// The LANGUAGE tag.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub language: Option<LanguageTag>,
    /// The VALUE type hint.
    pub value: Option<ValueType>,
    /// The PREF hint.
    pub pref: Option<u8>,
    /// The ALTID tag.
    pub alt_id: Option<String>,
    /// The PID value.
    pub pid: Option<Vec<Pid>>,
    /// The TYPE parameter.
    pub types: Option<Vec<TypeParameter>>,
    /// The MEDIATYPE value.
    #[cfg(feature = "mime")]
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    #[cfg_attr(
        feature = "serde",
        serde(
            with = "crate::serde::mime",
            skip_serializing_if = "Option::is_none"
        )
    )]
    pub media_type: Option<Mime>,

    /// The MEDIATYPE value.
    #[cfg(not(feature = "mime"))]
    pub media_type: Option<String>,

    /// The CALSCALE parameter.
    pub calscale: Option<String>,
    /// The SORT-AS parameter.
    pub sort_as: Option<Vec<String>>,
    /// The GEO parameter.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub geo: Option<Uri<'static>>,
    /// The TZ parameter.
    pub timezone: Option<TimeZoneParameter>,
    /// The LABEL parameter.
    ///
    /// This only applies to the ADR property.
    pub label: Option<String>,
}
