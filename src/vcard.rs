//! Definition of a single vCard.

use std::{borrow::Cow, fmt};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{iter, property::*, Error, Result};

/// The vCard type.
#[derive(Debug, Default, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct Vcard {
    // General
    /// Value of the SOURCE property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub source: Vec<UriProperty>,
    /// Value of the KIND property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub kind: Option<KindProperty>,
    /// Value of the XML property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub xml: Vec<TextProperty>,

    // Identification
    /// Value of the FN property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub formatted_name: Vec<TextProperty>,
    /// Value of the N property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub name: Option<TextListProperty>,
    /// Value of the NICKNAME property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub nickname: Vec<TextProperty>,
    /// Value of the PHOTO property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub photo: Vec<UriProperty>,
    /// Value of the BDAY property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub bday: Option<DateTimeOrTextProperty>,
    /// Value of the ANNIVERSARY property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub anniversary: Option<DateTimeOrTextProperty>,
    /// Value of the GENDER property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub gender: Option<GenderProperty>,
    /// Value of the URL property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub url: Vec<UriProperty>,

    // Delivery Addressing
    /// Value of the ADR property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub address: Vec<AddressProperty>,

    // Communications
    /// Value of the TEL property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub tel: Vec<TextOrUriProperty>,
    /// Value of the EMAIL property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub email: Vec<TextProperty>,
    /// Value of the IMPP property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub impp: Vec<UriProperty>,
    /// Value of the LANG property.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub lang: Vec<LanguageProperty>,

    // Organizational
    /// Value of the TITLE property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub title: Vec<TextProperty>,
    /// Value of the ROLE property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub role: Vec<TextProperty>,
    /// Value of the LOGO property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub logo: Vec<UriProperty>,
    /// Value of the ORG property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub org: Vec<TextListProperty>,
    /// Value of the MEMBER property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub member: Vec<UriProperty>,
    /// Value of the RELATED property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub related: Vec<TextOrUriProperty>,

    // Geographic
    /// Value of the TZ property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub timezone: Vec<TimeZoneProperty>,
    /// Value of the GEO property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub geo: Vec<UriProperty>,

    // Explanatory
    /// Value of the CATEGORIES property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub categories: Vec<TextListProperty>,
    /// Value of the NOTE property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub note: Vec<TextProperty>,
    /// Value of the PRODID property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub prod_id: Option<TextProperty>,
    /// Value of the REV property.
    #[cfg_attr(feature = "zeroize", zeroize(skip))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub rev: Option<DateTimeProperty>,
    /// Value of the SOUND property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub sound: Vec<UriProperty>,
    /// Value of the UID property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub uid: Option<TextOrUriProperty>,
    /// Value of the CLIENTPIDMAP property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub client_pid_map: Vec<ClientPidMapProperty>,

    // Security
    /// Value of the KEY property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub key: Vec<TextOrUriProperty>,

    // Calendar
    /// Value of the FBURL property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub fburl: Vec<UriProperty>,
    /// Value of the CALADRURI property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub cal_adr_uri: Vec<UriProperty>,
    /// Value of the CALURI property.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub cal_uri: Vec<UriProperty>,

    // Extensions
    /// Private property extensions (`X-`).
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub extensions: Vec<ExtensionProperty>,
}

impl Vcard {
    /// Create a new vCard with the given formatted name.
    pub fn new(formatted_name: String) -> Self {
        let mut card: Vcard = Default::default();
        card.formatted_name.push(formatted_name.into());
        card
    }

    /// Validate this vCard.
    pub fn validate(&self) -> Result<()> {
        if self.formatted_name.is_empty() {
            return Err(Error::NoFormattedName);
        }
        Ok(())
    }
}

impl TryFrom<&str> for Vcard {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        let mut it = iter(value, true);
        it.next().ok_or(Error::TokenExpected)?
    }
}

impl fmt::Display for Vcard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::name::*;
        write!(f, "{}\r\n{}\r\n", BEGIN, VERSION_4)?;

        // General
        for val in &self.source {
            write!(f, "{}\r\n", content_line(val, SOURCE))?;
        }
        for val in &self.kind {
            write!(f, "{}\r\n", content_line(val, KIND))?;
        }
        for val in &self.xml {
            write!(f, "{}\r\n", content_line(val, XML))?;
        }

        // Identification
        for val in &self.formatted_name {
            write!(f, "{}\r\n", content_line(val, FN))?;
        }
        for val in &self.name {
            write!(f, "{}\r\n", content_line(val, N))?;
        }
        for val in &self.nickname {
            write!(f, "{}\r\n", content_line(val, NICKNAME))?;
        }
        for val in &self.photo {
            write!(f, "{}\r\n", content_line(val, PHOTO))?;
        }
        for val in &self.bday {
            write!(f, "{}\r\n", content_line(val, BDAY))?;
        }
        for val in &self.anniversary {
            write!(f, "{}\r\n", content_line(val, ANNIVERSARY))?;
        }
        for val in &self.gender {
            write!(f, "{}\r\n", content_line(val, GENDER))?;
        }
        for val in &self.url {
            write!(f, "{}\r\n", content_line(val, URL))?;
        }

        // Delivery Addressing
        for val in &self.address {
            write!(f, "{}\r\n", content_line(val, ADR))?;
        }

        // Organizational
        for val in &self.title {
            write!(f, "{}\r\n", content_line(val, TITLE))?;
        }
        for val in &self.role {
            write!(f, "{}\r\n", content_line(val, ROLE))?;
        }
        for val in &self.logo {
            write!(f, "{}\r\n", content_line(val, LOGO))?;
        }
        for val in &self.org {
            write!(f, "{}\r\n", content_line(val, ORG))?;
        }
        for val in &self.member {
            write!(f, "{}\r\n", content_line(val, MEMBER))?;
        }
        for val in &self.related {
            write!(f, "{}\r\n", content_line(val, RELATED))?;
        }

        // Communications
        for val in &self.tel {
            write!(f, "{}\r\n", content_line(val, TEL))?;
        }
        for val in &self.email {
            write!(f, "{}\r\n", content_line(val, EMAIL))?;
        }
        for val in &self.impp {
            write!(f, "{}\r\n", content_line(val, IMPP))?;
        }
        for val in &self.lang {
            write!(f, "{}\r\n", content_line(val, LANG))?;
        }

        // Geographic
        for val in &self.timezone {
            write!(f, "{}\r\n", content_line(val, TZ))?;
        }
        for val in &self.geo {
            write!(f, "{}\r\n", content_line(val, GEO))?;
        }

        // Explanatory
        for val in &self.categories {
            write!(f, "{}\r\n", content_line(val, CATEGORIES))?;
        }
        for val in &self.note {
            write!(f, "{}\r\n", content_line(val, NOTE))?;
        }
        for val in &self.prod_id {
            write!(f, "{}\r\n", content_line(val, PRODID))?;
        }
        for val in &self.rev {
            write!(f, "{}\r\n", content_line(val, REV))?;
        }
        for val in &self.sound {
            write!(f, "{}\r\n", content_line(val, SOUND))?;
        }
        for val in &self.uid {
            write!(f, "{}\r\n", content_line(val, UID))?;
        }
        for val in &self.client_pid_map {
            write!(f, "{}\r\n", content_line(val, CLIENTPIDMAP))?;
        }

        // Security
        for val in &self.key {
            write!(f, "{}\r\n", content_line(val, KEY))?;
        }

        // Calendar
        for val in &self.fburl {
            write!(f, "{}\r\n", content_line(val, FBURL))?;
        }
        for val in &self.cal_adr_uri {
            write!(f, "{}\r\n", content_line(val, CALADRURI))?;
        }
        for val in &self.cal_uri {
            write!(f, "{}\r\n", content_line(val, CALURI))?;
        }

        // Private property extensions
        for val in &self.extensions {
            write!(f, "{}\r\n", content_line(val, &val.name))?;
        }

        write!(f, "{}\r\n", END)
    }
}

/// Get a content line.
fn content_line(prop: &impl Property, prop_name: &str) -> String {
    let name = qualified_name(prop, prop_name);

    let params = if let Some(params) = prop.parameters() {
        params.to_string()
    } else {
        String::new()
    };

    // Handle escape sequences
    let value = prop.to_string();
    /*
    let value = value
        .replace('\\', "\\\\")
        .replace('\n', "\\n");
    */

    let line = format!("{}{}:{}", name, params, value);
    fold_line(line, 75)
}

fn fold_line(line: String, wrap_at: usize) -> String {
    use unicode_segmentation::UnicodeSegmentation;
    let mut length = 0;
    let mut folded_line = String::new();
    for grapheme in UnicodeSegmentation::graphemes(&line[..], true) {
        length += grapheme.len();
        if length % wrap_at == 0 {
            folded_line.push_str("\r\n ");
        }
        folded_line.push_str(grapheme);
    }
    folded_line
}

/// Get the fully qualified name including any group.
fn qualified_name<'a>(
    prop: &impl Property,
    prop_name: &'a str,
) -> Cow<'a, str> {
    if let Some(group) = prop.group() {
        Cow::Owned(format!("{}.{}", group, prop_name))
    } else {
        Cow::Borrowed(prop_name)
    }
}
