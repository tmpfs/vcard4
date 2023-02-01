//! Builder for creating vCards.
//!
use crate::{
    property::{DeliveryAddress, Gender, TextListProperty},
    Vcard,
};
use time::Date;
use uriparse::uri::URI as Uri;

#[cfg(feature = "language-tags")]
use language_tags::LanguageTag;

/// Build vCard instances.
///
/// This is a high-level interface for creating vCards programatically;
/// if you need to assign parameters or use a group then either use
/// [Vcard](Vcard) directly or update properties after finishing a builder.
pub struct VcardBuilder {
    card: Vcard,
}

impl VcardBuilder {
    /// Create a new builder.
    pub fn new(formatted_name: String) -> Self {
        Self {
            card: Vcard::new(formatted_name),
        }
    }

    // Identification

    /// Add a formatted name to the vCard.
    pub fn formatted_name(mut self, value: String) -> Self {
        self.card.formatted_name.push(value.into());
        self
    }

    /// Set the name for the vCard.
    ///
    /// Should be family name, given name, additional names, honorific
    /// prefixes followed by honorific suffixes.
    pub fn name(mut self, value: [String; 5]) -> Self {
        self.card.name =
            Some(TextListProperty::new_semi_colon(value.to_vec()));
        self
    }

    /// Add a nickname to the vCard.
    pub fn nickname(mut self, value: String) -> Self {
        self.card.nickname.push(value.into());
        self
    }

    /// Add a photo to the vCard.
    pub fn photo(mut self, value: Uri<'static>) -> Self {
        self.card.photo.push(value.into());
        self
    }

    /// Set a birthday for the vCard.
    ///
    /// It is less usual to assign a time of birth so this function accepts
    /// a date, if you need to assign a time set `bday` directly on the vCard.
    pub fn birthday(mut self, value: Date) -> Self {
        self.card.bday = Some(value.into());
        self
    }

    /// Set an anniversary for the vCard.
    pub fn anniversary(mut self, value: Date) -> Self {
        self.card.anniversary = Some(value.into());
        self
    }

    /// Set the gender for the vCard.
    ///
    /// If the value cannot be parsed in to a gender according to
    /// RFC6350 then the gender will not be set.
    pub fn gender(mut self, value: &str) -> Self {
        if let Ok(gender) = value.parse::<Gender>() {
            self.card.gender = Some(gender.into());
        }
        self
    }

    /// Add a URL to the vCard.
    pub fn url(mut self, value: Uri<'static>) -> Self {
        self.card.url.push(value.into());
        self
    }

    /// Add an address to the vCard.
    pub fn address(mut self, value: DeliveryAddress) -> Self {
        self.card.address.push(value.into());
        self
    }

    // Communications

    /// Add a telephone number to the vCard.
    pub fn telephone(mut self, value: String) -> Self {
        self.card.tel.push(value.into());
        self
    }

    /// Add an email address to the vCard.
    pub fn email(mut self, value: String) -> Self {
        self.card.email.push(value.into());
        self
    }

    /// Add an instant messaging URI to the vCard.
    pub fn impp(mut self, value: Uri<'static>) -> Self {
        self.card.impp.push(value.into());
        self
    }

    #[cfg(feature = "language-tags")]
    /// Add a preferred language to the vCard.
    pub fn lang(mut self, value: LanguageTag) -> Self {
        self.card.lang.push(value.into());
        self
    }

    #[cfg(not(feature = "language-tags"))]
    /// Add a preferred language to the vCard.
    pub fn lang(mut self, value: String) -> Self {
        self.card.lang.push(value.into());
        self
    }

    // Organizational

    /// Add a title to the vCard.
    pub fn title(mut self, value: String) -> Self {
        self.card.title.push(value.into());
        self
    }

    /// Add a role to the vCard.
    pub fn role(mut self, value: String) -> Self {
        self.card.role.push(value.into());
        self
    }

    /// Finish building the vCard.
    pub fn finish(self) -> Vcard {
        self.card
    }
}

#[cfg(test)]
mod tests {
    use super::VcardBuilder;
    use crate::property::{DeliveryAddress, LanguageProperty};
    use time::{Date, Month};

    #[test]
    fn builder_vcard() {
        let card = VcardBuilder::new("Jane Doe".to_owned())
            // Identification
            .name([
                "Doe".to_owned(),
                "Jane".to_owned(),
                "Claire".to_owned(),
                "Dr.".to_owned(),
                "MS".to_owned(),
            ])
            .nickname("JC".to_owned())
            .photo("file:///images/jdoe.jpeg".try_into().unwrap())
            .birthday(
                Date::from_calendar_date(1986, Month::February, 7).unwrap(),
            )
            .anniversary(
                Date::from_calendar_date(2002, Month::March, 18).unwrap(),
            )
            .gender("F")
            .url("https://example.com/jdoe".try_into().unwrap())
            .address(DeliveryAddress {
                po_box: None,
                extended_address: None,
                street_address: Some("123 Main Street".to_owned()),
                locality: Some("Mock City".to_owned()),
                region: Some("Mock State".to_owned()),
                country_name: Some("Mock Country".to_owned()),
                postal_code: Some("123".to_owned()),
            })
            // Communication
            .telephone("+10987654321".to_owned())
            .email("janedoe@example.com".to_owned())
            .impp("im://example.com/messenger".try_into().unwrap())

            // Organizational
            .title("Dr".to_owned())
            .role("Surgeon".to_owned())
            .finish();
        println!("{}", card);
    }

    #[cfg(not(feature = "language-tags"))]
    #[test]
    fn builder_vcard_language() {
        let card = VcardBuilder::new("Jane Doe".to_owned())
            .lang("en".to_owned())
            .lang("fr".to_owned())
            .finish();
        assert_eq!(
            card.lang.get(0).unwrap(),
            &LanguageProperty {
                value: "en".to_owned(),
                group: None,
                parameters: None
            }
        );
        assert_eq!(
            card.lang.get(1).unwrap(),
            &LanguageProperty {
                value: "fr".to_owned(),
                group: None,
                parameters: None
            }
        );
    }

    #[cfg(feature = "language-tags")]
    #[test]
    fn builder_vcard_language_tags() {
        use language_tags::LanguageTag;
        let card = VcardBuilder::new("Jane Doe".to_owned())
            .lang("en".parse::<LanguageTag>().unwrap())
            .lang("fr".parse::<LanguageTag>().unwrap())
            .finish();
        assert_eq!(
            card.lang.get(0).unwrap(),
            &LanguageProperty {
                value: "en".parse::<LanguageTag>().unwrap(),
                group: None,
                parameters: None
            }
        );
        assert_eq!(
            card.lang.get(1).unwrap(),
            &LanguageProperty {
                value: "fr".parse::<LanguageTag>().unwrap(),
                group: None,
                parameters: None
            }
        );
    }
}
