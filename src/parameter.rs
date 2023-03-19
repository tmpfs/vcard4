//! Types for property parameters.

use std::{
    fmt::{self, Debug},
    str::FromStr,
};
use time::UtcOffset;
use uriparse::uri::URI as Uri;

#[cfg(feature = "language-tags")]
use language_tags::LanguageTag;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

#[cfg(feature = "mime")]
use mime::Mime;

use crate::{
    helper::format_utc_offset,
    name::{HOME, WORK},
    Error, Result,
};

/// Names of properties that are allowed to specify a TYPE parameter.
pub(crate) const TYPE_PROPERTIES: [&str; 23] = [
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
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub enum TypeParameter {
    /// Related to a home environment.
    Home,
    /// Related to a work environment.
    Work,
    /// Type for the TEL property.
    Telephone(TelephoneType),
    /// Type for the RELATED property.
    Related(RelatedType),
    /// Extension type parameter specified using the X- syntax.
    Extension(String),
}

impl fmt::Display for TypeParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Home => write!(f, "{}", HOME),
            Self::Work => write!(f, "{}", WORK),
            Self::Telephone(ref tel) => write!(f, "{}", tel),
            Self::Related(ref rel) => write!(f, "{}", rel),
            Self::Extension(ref value) => write!(f, "X-{}", value),
        }
    }
}

impl FromStr for TypeParameter {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            HOME => Ok(Self::Home),
            WORK => Ok(Self::Work),
            _ => {
                if s.starts_with("x-") || s.starts_with("X-") {
                    let value = if s.len() > 2 {
                        s[2..].to_string()
                    } else {
                        String::new()
                    };
                    Ok(Self::Extension(value))
                } else {
                    match s.parse::<TelephoneType>() {
                        Ok(tel) => Ok(Self::Telephone(tel)),
                        Err(_) => match s.parse::<RelatedType>() {
                            Ok(value) => Ok(Self::Related(value)),
                            Err(_) => Ok(Self::Extension(s.to_string())),
                        },
                    }
                }
            }
        }
    }
}

/// Values for a PID parameter.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct Pid {
    /// Digits before a period.
    pub local: u64,
    /// Digits after a period.
    pub source: Option<u64>,
}

impl Pid {
    /// Create a new property identifier.
    pub fn new(local: u64, source: Option<u64>) -> Self {
        Self { local, source }
    }
}

impl fmt::Display for Pid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(source) = self.source {
            write!(f, "{}.{}", self.local, source)
        } else {
            write!(f, "{}", self.local)
        }
    }
}

impl FromStr for Pid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.splitn(2, '.');
        let local = parts
            .next()
            .ok_or_else(|| Error::InvalidPid(s.to_string()))?;
        let local: u64 = local
            .parse()
            .map_err(|_| Error::InvalidPid(s.to_string()))?;
        let mut pid = Pid {
            local,
            source: None,
        };
        if let Some(source) = parts.next() {
            let source: u64 = source
                .parse()
                .map_err(|_| Error::InvalidPid(s.to_string()))?;
            pid.source = Some(source);
        }
        Ok(pid)
    }
}

/// Enumeration of related types.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub enum RelatedType {
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

impl fmt::Display for RelatedType {
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

impl FromStr for RelatedType {
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
            _ => Err(Error::UnknownRelatedType(s.to_string())),
        }
    }
}

/// Enumeration of telephone types.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum TelephoneType {
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

impl fmt::Display for TelephoneType {
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

impl FromStr for TelephoneType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match &s.to_lowercase()[..] {
            "text" => Ok(Self::Text),
            "voice" => Ok(Self::Voice),
            "fax" => Ok(Self::Fax),
            "cell" => Ok(Self::Cell),
            "video" => Ok(Self::Video),
            "pager" => Ok(Self::Pager),
            "textphone" => Ok(Self::TextPhone),
            _ => Err(Error::UnknownTelephoneType(s.to_string())),
        }
    }
}

/// Enumeration of types for the VALUE parameter.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
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

/// Value for a TZ parameter.
///
/// This is a different type so that we do not
/// create infinite type recursion in `Parameters` which would
/// require us to wrap it in a `Box`.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
#[allow(clippy::large_enum_variant)]
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
#[derive(Debug, Default, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct Parameters {
    /// The LANGUAGE tag.
    #[cfg(feature = "language-tags")]
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub language: Option<LanguageTag>,

    /// The LANGUAGE tag.
    #[cfg(not(feature = "language-tags"))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub language: Option<String>,

    /// The VALUE type hint.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub value: Option<ValueType>,
    /// The PREF hint.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub pref: Option<u8>,
    /// The ALTID tag.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub alt_id: Option<String>,
    /// The PID value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub pid: Option<Vec<Pid>>,
    /// The TYPE parameter.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub types: Option<Vec<TypeParameter>>,
    /// The MEDIATYPE value.
    #[cfg(feature = "mime")]
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            with = "crate::serde::media_type",
            skip_serializing_if = "Option::is_none",
        )
    )]
    pub media_type: Option<Mime>,

    /// The MEDIATYPE value.
    #[cfg(not(feature = "mime"))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub media_type: Option<String>,

    /// The CALSCALE parameter.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub calscale: Option<String>,
    /// The SORT-AS parameter.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub sort_as: Option<Vec<String>>,
    /// The GEO parameter.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub geo: Option<Uri<'static>>,
    /// The TZ parameter.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub timezone: Option<TimeZoneParameter>,
    /// The LABEL parameter.
    ///
    /// This only applies to the ADR property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub label: Option<String>,

    /// Any `X-` parameter extensions.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Vec<(String, Vec<String>)>>,
}

impl fmt::Display for Parameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::name::*;
        if let Some(language) = &self.language {
            write!(f, ";{}={}", LANGUAGE, language)?;
        }
        if let Some(value) = &self.value {
            write!(f, ";{}={}", VALUE, value)?;
        }
        if let Some(pref) = &self.pref {
            write!(f, ";{}={}", PREF, pref)?;
        }
        if let Some(alt_id) = &self.alt_id {
            write!(f, ";{}=\"{}\"", ALTID, alt_id)?;
        }
        if let Some(pids) = &self.pid {
            write!(f, ";{}={}", PID, comma_delimited(pids))?;
        }
        if let Some(types) = &self.types {
            write!(f, ";{}={}", TYPE, comma_delimited(types))?;
        }
        if let Some(media_type) = &self.media_type {
            write!(f, ";{}={}", MEDIATYPE, media_type)?;
        }
        if let Some(calscale) = &self.calscale {
            write!(f, ";{}={}", CALSCALE, calscale)?;
        }
        if let Some(sort_as) = &self.sort_as {
            write!(f, ";{}=\"{}\"", SORT_AS, comma_delimited(sort_as))?;
        }
        if let Some(geo) = &self.geo {
            write!(f, ";{}=\"{}\"", GEO, geo)?;
        }
        if let Some(tz) = &self.timezone {
            match tz {
                TimeZoneParameter::Text(val) => {
                    write!(f, ";{}={}", TZ, val)?;
                }
                TimeZoneParameter::UtcOffset(val) => {
                    write!(f, ";{}=", TZ)?;
                    format_utc_offset(f, val)?;
                }
                // URI must be quoted
                TimeZoneParameter::Uri(val) => {
                    write!(f, ";{}=\"{}\"", TZ, val)?;
                }
            }
        }
        if let Some(label) = &self.label {
            write!(f, ";{}=\"{}\"", LABEL, escape_parameter(label))?;
        }
        if let Some(extensions) = &self.extensions {
            for (name, value) in extensions {
                write!(f, ";{}=\"{}\"", name, comma_delimited(value))?;
            }
        }
        Ok(())
    }
}

fn escape_parameter(s: &str) -> String {
    s.replace('\n', "\\n")
}

fn comma_delimited(items: &Vec<impl std::fmt::Display>) -> String {
    let mut value = String::new();
    for (index, item) in items.iter().enumerate() {
        value.push_str(&item.to_string());
        if index < items.len() - 1 {
            value.push(',');
        }
    }
    value
}
