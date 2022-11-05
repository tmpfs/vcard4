#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Fast and correct vCard parser based
//! on [RFC6350](https://www.rfc-editor.org/rfc/rfc6350).
//!
//! vCards inherently contain private information so this library
//! implements a `zeroize` feature (which is enabled by default) to
//! securely zero the memory for all the data in a vCard when it is
//! dropped.
//!
//! Certain external types cannot be zeroize'd and are therefore exempt:
//!
//! * `LanguageTag`
//! * `Uri`
//! * `UtcOffset`
//! * `OffsetDateTime`
//! * `Time`
//! * `Mime`
//!
//! Serde support can be enabled by using the `serde` feature.

mod error;
pub mod parameters;
mod parser;
pub mod property;
pub mod types;

pub use error::Error;

pub use fluent_uri;
pub use language_tags;
pub use time;
pub use mime;

use std::fmt::Debug;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

use property::*;

/// Result type for the vCard library.
pub type Result<T> = std::result::Result<T, Error>;

/// Parse a vCard string into a collection of vCards.
pub fn parse<S: AsRef<str>>(input: S) -> Result<Vec<Vcard>> {
    let parser: parser::VcardParser = Default::default();
    parser.parse(input)
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
    pub xml: Vec<TextProperty>,

    // Identification
    /// Value of the FN property.
    pub formatted_name: Vec<TextProperty>,
    /// Value of the N property.
    pub name: Option<TextListProperty>,
    /// Value of the NICKNAME property.
    pub nickname: Vec<TextProperty>,
    /// Value of the PHOTO property.
    pub photo: Vec<UriProperty>,
    /// Value of the BDAY property.
    pub bday: Option<DateTimeOrTextProperty>,
    /// Value of the ANNIVERSARY property.
    pub anniversary: Option<DateTimeOrTextProperty>,
    /// Value of the GENDER property.
    pub gender: Option<GenderProperty>,
    /// Value of the URL property.
    pub url: Vec<UriProperty>,

    // Organizational
    /// Value of the TITLE property.
    pub title: Vec<TextProperty>,
    /// Value of the ROLE property.
    pub role: Vec<TextProperty>,
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
    pub email: Vec<TextProperty>,
    /// Value of the IMPP property.
    pub impp: Vec<UriProperty>,
    /// Value of the LANG property.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub lang: Vec<LanguageProperty>,

    // Geographic
    /// Value of the TZ property.
    pub timezone: Vec<TimeZoneProperty>,
    /// Value of the GEO property.
    pub geo: Vec<UriProperty>,

    // Explanatory
    /// Value of the CATEGORIES property.
    pub categories: Vec<TextListProperty>,
    /// Value of the NOTE property.
    pub note: Vec<TextProperty>,
    /// Value of the PRODID property.
    pub prod_id: Option<TextProperty>,
    /// Value of the REV property.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    pub rev: Option<DateTimeProperty>,

    //pub rev: Option<Timestamp>,
    /// Value of the SOUND property.
    pub sound: Vec<UriProperty>,
    /// Value of the UID property.
    pub uid: Option<TextOrUriProperty>,

    // Security
    /// Value of the KEY property.
    pub key: Vec<TextOrUriProperty>,

    // Calendar
    /// Value of the FBURL property.
    pub fburl: Vec<UriProperty>,
    /// Value of the CALADRURI property.
    pub cal_adr_uri: Vec<UriProperty>,
    /// Value of the CALURI property.
    pub cal_uri: Vec<UriProperty>,
}
