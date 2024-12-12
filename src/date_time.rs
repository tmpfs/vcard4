use crate::Error;
use std::{fmt, str::FromStr};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

#[cfg(feature = "serde")]
use serde_with::{serde_as, DeserializeFromStr, SerializeDisplay};

/// Date and time that serializes to and from RFC3339.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval, serde_as)]
#[cfg_attr(feature = "serde", derive(DeserializeFromStr, SerializeDisplay))]
pub struct DateTime(OffsetDateTime);

impl DateTime {
    /// Create UTC date and time.
    pub fn now_utc() -> Self {
        Self(OffsetDateTime::now_utc())
    }
}

impl From<OffsetDateTime> for DateTime {
    fn from(value: OffsetDateTime) -> Self {
        Self(value)
    }
}

impl From<DateTime> for OffsetDateTime {
    fn from(value: DateTime) -> Self {
        value.0
    }
}

impl AsRef<OffsetDateTime> for DateTime {
    fn as_ref(&self) -> &OffsetDateTime {
        &self.0
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0.format(&Rfc3339).map_err(|_| fmt::Error::default())?
        )
    }
}

impl FromStr for DateTime {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(OffsetDateTime::parse(s, &Rfc3339)?))
    }
}

/// Date that serializes to and from RFC3339.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval, serde_as)]
#[cfg_attr(feature = "serde", derive(DeserializeFromStr, SerializeDisplay))]
pub struct Date(time::Date);

impl From<time::Date> for Date {
    fn from(value: time::Date) -> Self {
        Self(value)
    }
}

impl From<Date> for time::Date {
    fn from(value: Date) -> Self {
        value.0
    }
}

impl AsRef<time::Date> for Date {
    fn as_ref(&self) -> &time::Date {
        &self.0
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_string(),)
    }
}

impl FromStr for Date {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(OffsetDateTime::parse(s, &Rfc3339)?.date()))
    }
}
