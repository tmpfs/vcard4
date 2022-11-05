//! The vCard struct and types for vCard properties and values.

use fluent_uri::Uri as URI;
use language_tags::LanguageTag;
use mime::Mime;
use std::{
    fmt::{self, Debug},
    str::FromStr,
};
use time::UtcOffset as UTCOffset;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{Error, Result};

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
            write!(
                f,
                "{}.{}",
                self.major,
                minor,
            )
        } else {
            write!(
                f,
                "{}",
                self.major,
            )
        }
    }
}

impl FromStr for Pid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.splitn(2, ".");
        let major = parts.next()
            .ok_or(Error::InvalidPid(s.to_string()))?;
        let major: u64 = major.parse().map_err(|_| Error::InvalidPid(s.to_string()))?;
        let mut pid = Pid { major: major, minor: None };
        if let Some(minor) = parts.next() {
            let minor: u64 = minor.parse().map_err(|_| Error::InvalidPid(s.to_string()))?;
            pid.minor = Some(minor);
        }
        Ok(pid)
    }
}

/// Either text or a URI.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub enum TextOrUri {
    /// Text value.
    Text(Text),
    /// URI value.
    Uri(Uri),
}

/// Enumeration of related types.
#[derive(Debug, PartialEq)]
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


/// Enumeration of the different types of values.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct UtcOffset {
    /// The value for the UTC offset.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub enum TimeZone {
    /// Text value.
    Text(Text),
    /// URI value.
    Uri(Uri),
    /// UTC offset value.
    UtcOffset(UtcOffset),
}

/// Represents a gender associated with a vCard.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
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
    pub pid: Option<Pid>,
    /// The MEDIATYPE value.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub media_type: Option<Mime>,
    /// The property TYPE.
    pub types: Option<Vec<String>>,
}

/// Text property value.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct Text {
    /// Value for this property.
    pub value: String,

    /// Parameters for this property.
    pub parameters: Option<Parameters>,
}

/// Text list property value.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct TextList {
    /// Value for this property.
    pub value: Vec<String>,

    /// Parameters for this property.
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct Uri {
    /// Value for this property.
    #[cfg_attr(feature = "serde", serde(with = "uri_from_str"))]
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub value: URI<String>,

    /// Parameters for this property.
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct Vcard {
    // General
    /// Value of the SOURCE property.
    pub source: Vec<Uri>,
    /// Value of the KIND property.
    pub kind: Option<Kind>,
    /// Value of the XML property.
    pub xml: Vec<Text>,

    // Identification
    /// Value of the FN property.
    pub formatted_name: Vec<Text>,
    /// Value of the N property.
    pub name: Option<TextList>,
    /// Value of the NICKNAME property.
    pub nickname: Vec<Text>,
    /// Value of the PHOTO property.
    pub photo: Vec<Uri>,
    //pub bday: Vec<Uri>,
    //pub anniversary: Vec<Uri>,
    /// Value of the GENDER property.
    pub gender: Option<Gender>,
    /// Value of the URL property.
    pub url: Vec<Uri>,

    // Organizational
    /// Value of the TITLE property.
    pub title: Vec<Text>,
    /// Value of the ROLE property.
    pub role: Vec<Text>,
    /// Value of the LOGO property.
    pub logo: Vec<Uri>,
    /// Value of the ORG property.
    pub org: Vec<TextList>,
    /// Value of the MEMBER property.
    pub member: Vec<Uri>,
    /// Value of the RELATED property.
    pub related: Vec<TextOrUri>,

    // Communications
    //pub tel: Vec<Text>,
    /// Value of the EMAIL property.
    pub email: Vec<Text>,
    /// Value of the IMPP property.
    pub impp: Vec<Uri>,
    /// Value of the LANG property.
    
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub lang: Vec<LanguageTag>,

    // Geographic
    /// Value of the TZ property.
    pub timezone: Vec<TimeZone>,
    /// Value of the GEO property.
    pub geo: Vec<Uri>,

    // Explanatory
    /// Value of the CATEGORIES property.
    pub categories: Vec<TextList>,
    /// Value of the NOTE property.
    pub note: Vec<Text>,
    /// Value of the PRODID property.
    pub prod_id: Option<Text>,

    //pub rev: Option<Timestamp>,

    /// Value of the SOUND property.
    pub sound: Vec<Uri>,
    /// Value of the UID property.
    pub uid: Option<TextOrUri>,

    // Security
    /// Value of the KEY property.
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
