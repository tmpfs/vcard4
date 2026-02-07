//! Types for properties.

use std::{
    fmt::{self, Display},
    str::FromStr,
};
use time::{Time, UtcOffset};

#[cfg(feature = "language-tags")]
use language_tags::LanguageTag;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
use serde_with::{DisplayFromStr, serde_as};

#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{
    Date, DateTime, Error, Result, Uri, escape_value,
    helper::{
        format_date, format_date_and_or_time_list, format_date_list,
        format_date_time, format_date_time_list, format_float_list,
        format_integer_list, format_time, format_time_list,
        format_timestamp_list, format_utc_offset, parse_date,
        parse_date_time, parse_time, parse_utc_offset,
    },
    parameter::Parameters,
};

const INDIVIDUAL: &str = "individual";
const GROUP: &str = "group";
const ORG: &str = "org";
const LOCATION: &str = "location";

/// Trait for vCard properties.
pub trait Property: Display {
    /// Get the property group.
    fn group(&self) -> Option<&String>;

    /// Get the property parameters.
    fn parameters(&self) -> Option<&Parameters>;
}

/// Delivery address for the ADR property.
#[derive(Default, Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct DeliveryAddress {
    /// The post office box.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub po_box: Option<String>,
    /// The extended address (e.g: apartment or suite number).
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extended_address: Option<String>,
    /// The street address.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub street_address: Option<String>,
    /// The locality (e.g: city).
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub locality: Option<String>,
    /// The region (e.g: state or province).
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub region: Option<String>,
    /// The postal code.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub postal_code: Option<String>,
    /// The country name.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub country_name: Option<String>,
}

impl fmt::Display for DeliveryAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{};{};{};{};{};{};{}",
            self.po_box
                .as_ref()
                .map(|s| escape_value(&s[..], true))
                .unwrap_or_default(),
            self.extended_address
                .as_ref()
                .map(|s| escape_value(&s[..], true))
                .unwrap_or_default(),
            self.street_address
                .as_ref()
                .map(|s| escape_value(&s[..], true))
                .unwrap_or_default(),
            self.locality
                .as_ref()
                .map(|s| escape_value(&s[..], true))
                .unwrap_or_default(),
            self.region
                .as_ref()
                .map(|s| escape_value(&s[..], true))
                .unwrap_or_default(),
            self.postal_code
                .as_ref()
                .map(|s| escape_value(&s[..], true))
                .unwrap_or_default(),
            self.country_name
                .as_ref()
                .map(|s| escape_value(&s[..], true))
                .unwrap_or_default(),
        )
    }
}

impl FromStr for DeliveryAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut it = s.splitn(7, ';');
        let po_box = it
            .next()
            .ok_or_else(|| Error::InvalidAddress(s.to_string()))?;
        let extended_address = it
            .next()
            .ok_or_else(|| Error::InvalidAddress(s.to_string()))?;
        let street_address = it
            .next()
            .ok_or_else(|| Error::InvalidAddress(s.to_string()))?;
        let locality = it
            .next()
            .ok_or_else(|| Error::InvalidAddress(s.to_string()))?;
        let region = it
            .next()
            .ok_or_else(|| Error::InvalidAddress(s.to_string()))?;
        let postal_code = it
            .next()
            .ok_or_else(|| Error::InvalidAddress(s.to_string()))?;
        let country_name = it
            .next()
            .ok_or_else(|| Error::InvalidAddress(s.to_string()))?;

        let po_box = if !po_box.is_empty() {
            Some(po_box.to_owned())
        } else {
            None
        };
        let extended_address = if !extended_address.is_empty() {
            Some(extended_address.to_owned())
        } else {
            None
        };
        let street_address = if !street_address.is_empty() {
            Some(street_address.to_owned())
        } else {
            None
        };
        let locality = if !locality.is_empty() {
            Some(locality.to_owned())
        } else {
            None
        };
        let region = if !region.is_empty() {
            Some(region.to_owned())
        } else {
            None
        };
        let postal_code = if !postal_code.is_empty() {
            Some(postal_code.to_owned())
        } else {
            None
        };
        let country_name = if !country_name.is_empty() {
            Some(country_name.to_owned())
        } else {
            None
        };

        Ok(Self {
            po_box,
            extended_address,
            street_address,
            locality,
            region,
            postal_code,
            country_name,
        })
    }
}

/// The ADR property.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct AddressProperty {
    /// Group for this property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub group: Option<String>,
    /// The value for the property.
    pub value: DeliveryAddress,
    /// The property parameters.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub parameters: Option<Parameters>,
}

impl From<DeliveryAddress> for AddressProperty {
    fn from(value: DeliveryAddress) -> Self {
        Self {
            value,
            group: None,
            parameters: None,
        }
    }
}

/// Value for the CLIENTPIDMAP property.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval, serde_as)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct ClientPidMap {
    /// The source identifier.
    pub source: u64,
    /// The URI for the map.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    #[cfg_attr(feature = "serde", serde_as(as = "DisplayFromStr"))]
    pub uri: Uri,
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

        let uri = uri.parse()?;
        Ok(ClientPidMap { source, uri })
    }
}

/// Client PID map property.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct ClientPidMapProperty {
    /// Group for this property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub group: Option<String>,
    /// The value for the property.
    pub value: ClientPidMap,
    /// The property parameters.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub parameters: Option<Parameters>,
}

/// Extension property.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct ExtensionProperty {
    /// The property name.
    pub name: String,
    /// Group for this property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub group: Option<String>,
    /// The value for the property.
    pub value: AnyProperty,
    /// The property parameters.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub parameters: Option<Parameters>,
}

/// Value for any property type.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval, serde_as)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
#[cfg_attr(feature = "serde", serde(untagged, rename_all = "camelCase"))]
#[allow(clippy::large_enum_variant)]
pub enum AnyProperty {
    /// Text property.
    Text(String),
    /// Integer property.
    Integer(Vec<i64>),
    /// Float property.
    Float(Vec<f64>),
    /// Boolean property.
    Boolean(bool),

    /// Date value.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    Date(Vec<Date>),
    /// Date and time value.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    DateTime(Vec<DateTime>),
    /// Time value.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    Time(Vec<(Time, UtcOffset)>),
    /// Date and or time value.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    DateAndOrTime(Vec<DateAndOrTime>),
    /// Timetamp value.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    Timestamp(Vec<DateTime>),
    /// URI property.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    Uri(#[cfg_attr(feature = "serde", serde_as(as = "DisplayFromStr"))] Uri),
    /// UTC offset property.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    UtcOffset(UtcOffset),
    /// Language property.
    #[cfg(feature = "language-tags")]
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    Language(LanguageTag),

    /// Language property.
    #[cfg(not(feature = "language-tags"))]
    Language(String),
}

impl Eq for AnyProperty {}

impl fmt::Display for AnyProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(val) => write!(f, "{}", escape_value(val, false)),
            Self::Integer(val) => format_integer_list(f, val),
            Self::Float(val) => format_float_list(f, val),
            Self::Boolean(val) => write!(f, "{}", val),
            Self::Date(val) => format_date_list(f, val),
            Self::DateTime(val) => format_date_time_list(f, val),
            Self::Time(val) => format_time_list(f, val),
            Self::DateAndOrTime(val) => format_date_and_or_time_list(f, val),
            Self::Timestamp(val) => format_timestamp_list(f, val),
            Self::UtcOffset(val) => format_utc_offset(f, val),
            Self::Uri(val) => write!(f, "{}", val),
            Self::Language(val) => write!(f, "{}", val),
        }
    }
}

/// Language property.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct LanguageProperty {
    /// Group for this property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub group: Option<String>,
    /// The value for the property.
    #[cfg(feature = "language-tags")]
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub value: LanguageTag,

    /// The value for the property.
    #[cfg(not(feature = "language-tags"))]
    pub value: String,

    /// The property parameters.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub parameters: Option<Parameters>,
}

#[cfg(not(feature = "language-tags"))]
impl From<String> for LanguageProperty {
    fn from(value: String) -> Self {
        Self {
            value,
            group: None,
            parameters: None,
        }
    }
}

#[cfg(feature = "language-tags")]
impl From<LanguageTag> for LanguageProperty {
    fn from(value: LanguageTag) -> Self {
        Self {
            value,
            group: None,
            parameters: None,
        }
    }
}

/// Date time property.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct DateTimeProperty {
    /// Group for this property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub group: Option<String>,
    /// The value for the property.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub value: DateTime,
    /// The property parameters.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub parameters: Option<Parameters>,
}

impl From<DateTime> for DateTimeProperty {
    fn from(value: DateTime) -> Self {
        Self {
            value,
            group: None,
            parameters: None,
        }
    }
}

impl fmt::Display for DateTimeProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_date_time(&self.value).map_err(|_| fmt::Error)?
        )
    }
}

/// Date and or time.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DateAndOrTime {
    /// Date value.
    Date(Date),
    /// Date and time value.
    DateTime(DateTime),
    /// Time value.
    Time((Time, UtcOffset)),
}

impl From<Date> for DateAndOrTime {
    fn from(value: Date) -> Self {
        Self::Date(value)
    }
}

impl From<DateTime> for DateAndOrTime {
    fn from(value: DateTime) -> Self {
        Self::DateTime(value)
    }
}

impl From<(Time, UtcOffset)> for DateAndOrTime {
    fn from(value: (Time, UtcOffset)) -> Self {
        Self::Time(value)
    }
}

impl fmt::Display for DateAndOrTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Date(val) => {
                write!(f, "{}", format_date(val).map_err(|_| fmt::Error)?)
            }
            Self::DateTime(val) => write!(
                f,
                "{}",
                format_date_time(val).map_err(|_| fmt::Error)?
            ),
            Self::Time(val) => {
                write!(f, "{}", format_time(val).map_err(|_| fmt::Error)?)
            }
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
                Ok(value) => Ok(Self::Date(value.into())),
                Err(_) => match parse_time(s) {
                    Ok(val) => Ok(Self::Time(val)),
                    Err(e) => Err(e),
                },
            },
        }
    }
}

/// Date and or time property.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DateAndOrTimeProperty {
    /// Group for this property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub group: Option<String>,
    /// The value for the property.
    pub value: Vec<DateAndOrTime>,
    /// The property parameters.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub parameters: Option<Parameters>,
}

impl From<Date> for DateAndOrTimeProperty {
    fn from(value: Date) -> Self {
        Self {
            value: vec![value.into()],
            group: None,
            parameters: None,
        }
    }
}

impl From<DateTime> for DateAndOrTimeProperty {
    fn from(value: DateTime) -> Self {
        Self {
            value: vec![value.into()],
            group: None,
            parameters: None,
        }
    }
}

impl From<(Time, UtcOffset)> for DateAndOrTimeProperty {
    fn from(value: (Time, UtcOffset)) -> Self {
        Self {
            value: vec![value.into()],
            group: None,
            parameters: None,
        }
    }
}

impl fmt::Display for DateAndOrTimeProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_date_and_or_time_list(f, &self.value)
    }
}

/// Either text or a Uri.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[allow(clippy::large_enum_variant)]
pub enum TextOrUriProperty {
    /// Text value.
    Text(TextProperty),
    /// Uri value.
    Uri(UriProperty),
}

impl From<String> for TextOrUriProperty {
    fn from(value: String) -> Self {
        Self::Text(value.into())
    }
}

impl From<Uri> for TextOrUriProperty {
    fn from(value: Uri) -> Self {
        Self::Uri(value.into())
    }
}

impl Property for TextOrUriProperty {
    fn group(&self) -> Option<&String> {
        match self {
            Self::Text(val) => val.group(),
            Self::Uri(val) => val.group(),
        }
    }

    fn parameters(&self) -> Option<&Parameters> {
        match self {
            Self::Text(val) => val.parameters(),
            Self::Uri(val) => val.parameters(),
        }
    }
}

impl fmt::Display for TextOrUriProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(val) => write!(f, "{}", val),
            Self::Uri(val) => write!(f, "{}", val),
        }
    }
}

/// Either text or a date and or time.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DateTimeOrTextProperty {
    /// Date time value.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    DateTime(DateAndOrTimeProperty),
    /// Text value.
    Text(TextProperty),
}

impl From<String> for DateTimeOrTextProperty {
    fn from(value: String) -> Self {
        Self::Text(value.into())
    }
}

impl From<Date> for DateTimeOrTextProperty {
    fn from(value: Date) -> Self {
        Self::DateTime(value.into())
    }
}

impl From<DateTime> for DateTimeOrTextProperty {
    fn from(value: DateTime) -> Self {
        Self::DateTime(value.into())
    }
}

impl From<(Time, UtcOffset)> for DateTimeOrTextProperty {
    fn from(value: (Time, UtcOffset)) -> Self {
        Self::DateTime(value.into())
    }
}

impl Property for DateTimeOrTextProperty {
    fn group(&self) -> Option<&String> {
        match self {
            Self::Text(val) => val.group(),
            Self::DateTime(val) => val.group(),
        }
    }

    fn parameters(&self) -> Option<&Parameters> {
        match self {
            Self::Text(val) => val.parameters(),
            Self::DateTime(val) => val.parameters(),
        }
    }
}

impl fmt::Display for DateTimeOrTextProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(val) => write!(f, "{}", val),
            Self::DateTime(val) => write!(f, "{}", val),
        }
    }
}

/// Value for a UTC offset property.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct UtcOffsetProperty {
    /// Group for this property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub group: Option<String>,
    /// The value for the UTC offset.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub value: UtcOffset,
    /// The parameters for the property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub parameters: Option<Parameters>,
}

impl From<UtcOffset> for UtcOffsetProperty {
    fn from(value: UtcOffset) -> Self {
        Self {
            value,
            group: None,
            parameters: None,
        }
    }
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
        let value = parse_utc_offset(s)?;
        Ok(Self {
            value,
            parameters: None,
            group: None,
        })
    }
}

/// Value for a timezone property.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[allow(clippy::large_enum_variant)]
pub enum TimeZoneProperty {
    /// Text value.
    Text(TextProperty),
    /// Uri value.
    Uri(UriProperty),
    /// UTC offset value.
    UtcOffset(UtcOffsetProperty),
}

impl From<String> for TimeZoneProperty {
    fn from(value: String) -> Self {
        Self::Text(value.into())
    }
}

impl From<Uri> for TimeZoneProperty {
    fn from(value: Uri) -> Self {
        Self::Uri(value.into())
    }
}

impl From<UtcOffset> for TimeZoneProperty {
    fn from(value: UtcOffset) -> Self {
        Self::UtcOffset(value.into())
    }
}

impl Property for TimeZoneProperty {
    fn group(&self) -> Option<&String> {
        match self {
            Self::Text(val) => val.group(),
            Self::Uri(val) => val.group(),
            Self::UtcOffset(val) => val.group(),
        }
    }

    fn parameters(&self) -> Option<&Parameters> {
        match self {
            Self::Text(val) => val.parameters(),
            Self::Uri(val) => val.parameters(),
            Self::UtcOffset(val) => val.parameters(),
        }
    }
}

impl fmt::Display for TimeZoneProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(val) => write!(f, "{}", val),
            Self::Uri(val) => write!(f, "{}", val),
            Self::UtcOffset(val) => write!(f, "{}", val),
        }
    }
}

/// Text property value.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct TextProperty {
    /// Group for this property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub group: Option<String>,
    /// Value for this property.
    pub value: String,
    /// Parameters for this property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub parameters: Option<Parameters>,
}

impl fmt::Display for TextProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", escape_value(&self.value, false))
    }
}

impl From<String> for TextProperty {
    fn from(value: String) -> Self {
        Self {
            value,
            group: None,
            parameters: None,
        }
    }
}

/// Delimiter used for a text list.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum TextListDelimiter {
    /// Text list with a comma delimiter.
    Comma,
    /// Text list with a semi-colon delimiter.
    SemiColon,
}

/// Text list property value.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct TextListProperty {
    /// Group for this property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub group: Option<String>,
    /// Value for this property.
    pub value: Vec<String>,
    /// Parameters for this property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub parameters: Option<Parameters>,
    /// Delimiter for the list property.
    pub delimiter: TextListDelimiter,
}

impl TextListProperty {
    /// Create a new text list property delimited with a semi-colon.
    pub fn new_semi_colon(value: Vec<String>) -> Self {
        Self {
            value,
            group: None,
            parameters: None,
            delimiter: TextListDelimiter::SemiColon,
        }
    }

    /// Create a new text list property delimited with a comma.
    pub fn new_comma(value: Vec<String>) -> Self {
        Self {
            value,
            group: None,
            parameters: None,
            delimiter: TextListDelimiter::Comma,
        }
    }
}

impl fmt::Display for TextListProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, item) in self.value.iter().enumerate() {
            let semi_colons =
                matches!(self.delimiter, TextListDelimiter::SemiColon);
            write!(f, "{}", escape_value(item, semi_colons))?;
            if index < self.value.len() - 1 {
                write!(
                    f,
                    "{}",
                    match self.delimiter {
                        TextListDelimiter::Comma => ',',
                        TextListDelimiter::SemiColon => ';',
                    }
                )?;
            }
        }
        Ok(())
    }
}

/// Uri property value.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval, serde_as)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct UriProperty {
    /// Group for this property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub group: Option<String>,
    /// Value for this property.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    #[cfg_attr(feature = "serde", serde_as(as = "DisplayFromStr"))]
    pub value: Uri,
    /// Parameters for this property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub parameters: Option<Parameters>,
}

impl From<Uri> for UriProperty {
    fn from(value: Uri) -> Self {
        Self {
            value,
            group: None,
            parameters: None,
        }
    }
}

impl TryFrom<&str> for UriProperty {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let uri: Uri = value.parse()?;
        Ok(uri.into())
    }
}

/// Property for a vCard kind.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct KindProperty {
    /// Group for this property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub group: Option<String>,
    /// The value for the property.
    pub value: Kind,
    /// The property parameters.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub parameters: Option<Parameters>,
}

impl From<Kind> for KindProperty {
    fn from(value: Kind) -> Self {
        Self {
            value,
            group: None,
            parameters: None,
        }
    }
}

/// Kind of vCard.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
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
                Self::Individual => INDIVIDUAL,
                Self::Group => GROUP,
                Self::Org => ORG,
                Self::Location => LOCATION,
            }
        )
    }
}

impl FromStr for Kind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            INDIVIDUAL => Ok(Self::Individual),
            GROUP => Ok(Self::Group),
            ORG => Ok(Self::Org),
            LOCATION => Ok(Self::Location),
            _ => Err(Error::UnknownKind(s.to_string())),
        }
    }
}

/// Property for a vCard gender.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct GenderProperty {
    /// Group for this property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub group: Option<String>,
    /// The value for the property.
    pub value: Gender,
    /// The property parameters.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub parameters: Option<Parameters>,
}

impl From<Gender> for GenderProperty {
    fn from(value: Gender) -> Self {
        Self {
            value,
            group: None,
            parameters: None,
        }
    }
}

/// Represents a gender associated with a vCard.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct Gender {
    /// The sex for the gender.
    pub sex: Sex,
    /// The identity text.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub identity: Option<String>,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(identity) = &self.identity {
            write!(f, "{};{}", self.sex, escape_value(identity, true))
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

        let mut it = s.splitn(2, ';');
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
#[derive(Debug, Eq, PartialEq, Clone)]
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

macro_rules! property_impl {
    ($prop:ty) => {
        impl Property for $prop {
            fn group(&self) -> Option<&String> {
                self.group.as_ref()
            }

            fn parameters(&self) -> Option<&Parameters> {
                self.parameters.as_ref()
            }
        }
    };
}

macro_rules! display_impl {
    ($prop:ty) => {
        impl fmt::Display for $prop {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.value)
            }
        }
    };
}

property_impl!(AddressProperty);
display_impl!(AddressProperty);

property_impl!(UriProperty);
display_impl!(UriProperty);

property_impl!(KindProperty);
display_impl!(KindProperty);

property_impl!(TextProperty);

property_impl!(LanguageProperty);
display_impl!(LanguageProperty);

property_impl!(DateTimeProperty);

property_impl!(DateAndOrTimeProperty);

property_impl!(ClientPidMapProperty);
display_impl!(ClientPidMapProperty);

property_impl!(GenderProperty);
display_impl!(GenderProperty);

property_impl!(ExtensionProperty);
display_impl!(ExtensionProperty);

// Bespoke Display implementations
property_impl!(TextListProperty);
property_impl!(UtcOffsetProperty);

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
