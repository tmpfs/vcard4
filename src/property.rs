//! Types for properties.

use language_tags::LanguageTag;
use fluent_uri::Uri;
use std::{
    fmt::{self, Debug},
    str::FromStr,
};
use time::{OffsetDateTime, UtcOffset as UTCOffset};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{parameters::Parameters, types::DateAndOrTime, Error, Result};

/// Language property.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct LanguageProperty {
    /// The value for the property.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub value: LanguageTag,
    /// The property parameters.
    pub parameters: Option<Parameters>,
}

/// Date time property.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct DateTimeProperty {
    /// The value for the property.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub value: OffsetDateTime,
    /// The property parameters.
    pub parameters: Option<Parameters>,
}

/// Date and or time property.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DateAndOrTimeProperty {
    /// The value for the property.
    pub value: DateAndOrTime,
    /// The property parameters.
    pub parameters: Option<Parameters>,
}

/// Either text or a Uri.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub enum TextOrUriProperty {
    /// Text value.
    Text(TextProperty),
    /// Uri value.
    Uri(UriProperty),
}

/// Either text or a date and or time.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub enum DateTimeOrTextProperty {
    /// Date time value.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    DateTime(DateAndOrTimeProperty),
    /// Text value.
    Text(TextProperty),
}

/// Value for a UTC offset property.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct UtcOffsetProperty {
    /// The value for the UTC offset.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub value: UTCOffset,
    /// The parameters for the property.
    pub parameters: Option<Parameters>,
}

impl fmt::Display for UtcOffsetProperty {
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

impl FromStr for UtcOffsetProperty {
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

/// Value for a timezone property.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub enum TimeZoneProperty {
    /// Text value.
    Text(TextProperty),
    /// Uri value.
    Uri(UriProperty),
    /// UTC offset value.
    UtcOffset(UtcOffsetProperty),
}

/// Text property value.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct TextProperty {
    /// Value for this property.
    pub value: String,

    /// Parameters for this property.
    pub parameters: Option<Parameters>,
}

/// Text list property value.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct TextListProperty {
    /// Value for this property.
    pub value: Vec<String>,

    /// Parameters for this property.
    pub parameters: Option<Parameters>,
}

#[cfg(feature = "serde")]
mod uri_from_str {
    use fluent_uri::Uri;
    use serde::{
        de::{Deserializer, Error, Visitor},
        ser::Serializer,
    };
    use std::fmt;

    pub fn serialize<S>(
        source: &Uri<String>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(source.as_str())
    }

    struct UriVisitor;

    impl<'de> Visitor<'de> for UriVisitor {
        type Value = Uri<String>;

        fn expecting(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Uri::parse(v).map_err(Error::custom)?.to_owned())
        }
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Uri<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(UriVisitor)
    }
}

/// Uri property value.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct UriProperty {
    /// Value for this property.
    #[cfg_attr(feature = "serde", serde(with = "uri_from_str"))]
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub value: Uri<String>,

    /// Parameters for this property.
    pub parameters: Option<Parameters>,
}

impl PartialEq for UriProperty {
    fn eq(&self, other: &Self) -> bool {
        self.value.as_str() == other.value.as_str()
            && self.parameters == other.parameters
    }
}

/// Property for a vCard kind.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct KindProperty {
    /// The value for the property.
    pub value: Kind,
    /// The property parameters.
    pub parameters: Option<Parameters>,
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

/// Property for a vCard gender.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct GenderProperty {
    /// The value for the property.
    pub value: Gender,
    /// The property parameters.
    pub parameters: Option<Parameters>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn parse_utc_offset() -> Result<()> {
        let east = "+1200".parse::<UtcOffsetProperty>()?;
        let west = "-0500".parse::<UtcOffsetProperty>()?;

        assert_eq!("+1200", east.to_string());
        assert_eq!("-0500", west.to_string());

        assert!("0500".parse::<UtcOffsetProperty>().is_err());
        assert!("foo".parse::<UtcOffsetProperty>().is_err());
        assert!("+4400".parse::<UtcOffsetProperty>().is_err());

        Ok(())
    }
}
