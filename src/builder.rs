//! Builder for creating vCards.
//!
//! This is a high-level interface for creating vCards programatically; 
//! if you need to assign parameters or use a group then either use 
//! Vcard directly or update properties after finishing a builder.

use uriparse::uri::URI as Uri;
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
    pub fn formatted_name(mut self, formatted_name: String) -> Self {
        if let Some(name) = self.card.formatted_name.get_mut(0) {
            *name = formatted_name.into();
        }
        self
    }

    /// Set the name for the vCard.
    ///
    /// Should be family name, given name, additional names, honorific 
    /// prefixes followed by honorific suffixes.
    pub fn name(mut self, names: [String; 5]) -> Self {
        self.card.name = Some(TextListProperty::new_semi_colon(names.to_vec()));
        self
    }

    /// Add a nickname to the vCard.
    pub fn nickname(mut self, nickname: String) -> Self {
        self.card.nickname.push(nickname.into());
        self
    }

    /// Add a photo to the vCard.
    pub fn photo(mut self, photo: Uri<'static>) -> Self {
        self.card.photo.push(UriProperty::new(photo));
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
            .finish();
        println!("{}", card);
    }
}
