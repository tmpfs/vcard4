//! Builder for creating vCards.
//!
//! This is a high-level interface for creating vCards programatically; 
//! if you need to assign parameters or use a group then either use 
//! Vcard directly or update properties after finishing a builder.

use uriparse::uri::URI as Uri;
use time::Date;
use crate::{Vcard, property::*};

/// Build vCard instances.
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
    
    /// Update the formatted name for the vCard.
    pub fn formatted_name(mut self, value: String) -> Self {
        if let Some(name) = self.card.formatted_name.get_mut(0) {
            *name = value.into();
        }
        self
    }

    /// Set the name for the vCard.
    ///
    /// Should be family name, given name, additional names, honorific 
    /// prefixes followed by honorific suffixes.
    pub fn name(mut self, value: [String; 5]) -> Self {
        self.card.name = Some(
            TextListProperty::new_semi_colon(value.to_vec()));
        self
    }

    /// Add a nickname to the vCard.
    pub fn nickname(mut self, value: String) -> Self {
        self.card.nickname.push(value.into());
        self
    }

    /// Add a photo to the vCard.
    pub fn photo(mut self, value: Uri<'static>) -> Self {
        self.card.photo.push(UriProperty::new(value));
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

    /// Finish building the vCard.
    pub fn finish(self) -> Vcard {
        self.card
    }
}

#[cfg(test)]
mod tests {
    use time::{Date, Month};
    use super::VcardBuilder;

    #[test]
    fn builder_vcard() {
        let card = 
            VcardBuilder::new("Jane Doe".to_owned())
            .name(
                [
                    "Doe".to_owned(),
                    "Jane".to_owned(),
                    "Claire".to_owned(),
                    "Dr.".to_owned(),
                    "MS".to_owned(),
                ])
            .nickname("Doc J".to_owned())
            .photo("file:///images/jdoe.jpeg".try_into().unwrap())
            .birthday(
                Date::from_calendar_date(1986, Month::February, 7).unwrap())
            .anniversary(
                Date::from_calendar_date(2002, Month::March, 18).unwrap())
            .gender("F")
            .finish();
        println!("{}", card);
    }
}
