//! Type for vCards.

use std::{borrow::Cow, fmt};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::property::*;

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
    /// Value of the TEL property.
    pub tel: Vec<TextOrUriProperty>,
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
    /// Value of the SOUND property.
    pub sound: Vec<UriProperty>,
    /// Value of the UID property.
    pub uid: Option<TextOrUriProperty>,
    /// Value of the CLIENTPIDMAP property.
    pub client_pid_map: Vec<ClientPidMapProperty>,

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

    // Extensions
    /// Private property extensions (`X-`).
    pub extensions: Vec<ExtensionProperty>,
}

impl fmt::Display for Vcard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::name::*;
        write!(f, "{}\r\n{}\r\n", BEGIN, VERSION_4)?;

        for val in &self.source {
            write!(f, "{}\r\n", content_line(val, SOURCE))?;
        }

        write!(f, "{}\r\n", END)
    }
}

/// Get a content line.
fn content_line(prop: &impl Property, prop_name: &'static str) -> String {
    let name = qualified_name(prop, prop_name);

    let params = if let Some(params) = prop.parameters() {
        params.to_string()
    } else {
        String::new()
    };

    // Handle escape sequences
    let value = prop.to_string();
    let value = value.replace("\\", "\\\\");
    let value = value.replace("\n", "\\n");
    let value = value.replace(",", "\\,");
    let value = value.replace(";", "\\;");

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
