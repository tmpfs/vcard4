//! Builder for creating vCards.
//!
use crate::{
    property::{DeliveryAddress, Gender, Kind, TextListProperty},
    Vcard,
};
use time::{Date, OffsetDateTime};
use uriparse::uri::URI as Uri;

#[cfg(feature = "language-tags")]
use language_tags::LanguageTag;

/// Build vCard instances.
///
/// This is a high-level interface for creating vCards programatically;
/// if you need to assign parameters or use a group then either use
/// [Vcard](Vcard) directly or update properties after finishing a builder.
///
/// The card is not validated so it is possible to create
/// invalid vCards using the builder. To ensure you have a valid vCard call
/// [validate](Vcard::validate) afterwards.
///
/// The builder does not support the CLIENTPIDMAP property, if you need to
/// use a CLIENTPIDMAP use [Vcard](Vcard).
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

    // General

    /// Set the kind of vCard.
    pub fn kind(mut self, value: Kind) -> Self {
        self.card.kind = Some(value.into());
        self
    }

    /// Add a source for the vCard.
    pub fn source(mut self, value: Uri<'static>) -> Self {
        self.card.source.push(value.into());
        self
    }

    /// Add XML to the vCard.
    pub fn xml(mut self, value: String) -> Self {
        self.card.xml.push(value.into());
        self
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

    // Geographical

    /// Add a timezone to the vCard.
    pub fn timezone(mut self, value: String) -> Self {
        self.card.timezone.push(value.into());
        self
    }

    /// Add a geographic location to the vCard.
    pub fn geo(mut self, value: Uri<'static>) -> Self {
        self.card.geo.push(value.into());
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

    /// Add logo to the vCard.
    pub fn logo(mut self, value: Uri<'static>) -> Self {
        self.card.logo.push(value.into());
        self
    }

    /// Add an organization to the vCard.
    pub fn org(mut self, value: Vec<String>) -> Self {
        self.card.org.push(TextListProperty::new_semi_colon(value));
        self
    }

    /// Add a member to the vCard.
    ///
    /// The vCard should be of the group kind to be valid.
    pub fn member(mut self, value: Uri<'static>) -> Self {
        self.card.member.push(value.into());
        self
    }

    /// Add a related entry to the vCard.
    pub fn related(mut self, value: Uri<'static>) -> Self {
        self.card.related.push(value.into());
        self
    }

    // Explanatory

    /// Add categories to the vCard.
    pub fn categories(mut self, value: Vec<String>) -> Self {
        self.card
            .categories
            .push(TextListProperty::new_comma(value));
        self
    }

    /// Add a note to the vCard.
    pub fn note(mut self, value: String) -> Self {
        self.card.note.push(value.into());
        self
    }

    /// Add a product identifier to the vCard.
    pub fn prod_id(mut self, value: String) -> Self {
        self.card.prod_id = Some(value.into());
        self
    }

    /// Set the revision of the vCard.
    pub fn rev(mut self, value: OffsetDateTime) -> Self {
        self.card.rev = Some(value.into());
        self
    }

    /// Add a sound to the vCard.
    pub fn sound(mut self, value: Uri<'static>) -> Self {
        self.card.sound.push(value.into());
        self
    }

    /// Set the UID for the vCard.
    pub fn uid(mut self, value: Uri<'static>) -> Self {
        self.card.uid = Some(value.into());
        self
    }

    /// Add a URL to the vCard.
    pub fn url(mut self, value: Uri<'static>) -> Self {
        self.card.url.push(value.into());
        self
    }

    // Security

    /// Add a key to the vCard.
    pub fn key(mut self, value: Uri<'static>) -> Self {
        self.card.key.push(value.into());
        self
    }

    // Calendar

    /// Add a fburl to the vCard.
    pub fn fburl(mut self, value: Uri<'static>) -> Self {
        self.card.fburl.push(value.into());
        self
    }

    /// Add a calendar address URI to the vCard.
    pub fn cal_adr_uri(mut self, value: Uri<'static>) -> Self {
        self.card.cal_adr_uri.push(value.into());
        self
    }

    /// Add a calendar URI to the vCard.
    pub fn cal_uri(mut self, value: Uri<'static>) -> Self {
        self.card.cal_uri.push(value.into());
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
    use crate::property::{DeliveryAddress, Kind, LanguageProperty};
    use time::{Date, Month, OffsetDateTime, Time};

    #[test]
    fn builder_vcard() {
        let mut rev = OffsetDateTime::now_utc();
        rev = rev.replace_date(
            Date::from_calendar_date(2000, Month::January, 3).unwrap());
        rev = rev.replace_time(Time::MIDNIGHT);

        let card = VcardBuilder::new("Jane Doe".to_owned())
            // General
            .source(
                "http://directory.example.com/addressbooks/jdoe.vcf"
                    .try_into()
                    .unwrap(),
            )
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
            // Geographical
            .timezone("Raleigh/North America".to_owned())
            .geo("geo:37.386013,-122.082932".try_into().unwrap())
            // Organizational
            .org(vec!["Mock Hospital".to_owned(), "Surgery".to_owned()])
            .title("Dr".to_owned())
            .role("Master Surgeon".to_owned())
            .logo("https://example.com/mock.jpeg".try_into().unwrap())
            .related("https://example.com/johndoe".try_into().unwrap())
            // Explanatory
            .categories(vec!["Medical".to_owned(), "Health".to_owned()])
            .note("Saved my life!".to_owned())
            .prod_id("Contact App v1".to_owned())
            .rev(rev)
            .sound("https://example.com/janedoe.wav".try_into().unwrap())
            .uid(
                "urn:uuid:f81d4fae-7dec-11d0-a765-00a0c91e6bf6"
                    .try_into()
                    .unwrap(),
            )
            .url("https://example.com/janedoe".try_into().unwrap())
            // Security
            .key("urn:eth:0x00".try_into().unwrap())
            // Calendar
            .fburl("https://www.example.com/busy/janedoe".try_into().unwrap())
            .cal_adr_uri(
                "https://www.example.com/calendar/janedoe"
                    .try_into()
                    .unwrap(),
            )
            .cal_uri("https://calendar.example.com".try_into().unwrap())
            .finish();

        let expected = "BEGIN:VCARD\r\nVERSION:4.0\r\nSOURCE:http://directory.example.com/addressbooks/jdoe.vcf\r\nFN:Jane Doe\r\nN:Doe;Jane;Claire;Dr.;MS\r\nNICKNAME:JC\r\nPHOTO:file:///images/jdoe.jpeg\r\nBDAY:19860207\r\nANNIVERSARY:20020318\r\nGENDER:F\r\nURL:https://example.com/janedoe\r\nADR:;;123 Main Street;Mock City;Mock State;123;Mock Country\r\nTITLE:Dr\r\nROLE:Master Surgeon\r\nLOGO:https://example.com/mock.jpeg\r\nORG:Mock Hospital;Surgery\r\nRELATED:https://example.com/johndoe\r\nTEL:+10987654321\r\nEMAIL:janedoe@example.com\r\nIMPP:im://example.com/messenger\r\nTZ:Raleigh/North America\r\nGEO:geo:37.386013,-122.082932\r\nCATEGORIES:Medical,Health\r\nNOTE:Saved my life!\r\nPRODID:Contact App v1\r\nREV:20000103T000000Z\r\nSOUND:https://example.com/janedoe.wav\r\nUID:urn:uuid:f81d4fae-7dec-11d0-a765-00a0c91e6bf6\r\nKEY:urn:eth:0x00\r\nFBURL:https://www.example.com/busy/janedoe\r\nCALADRURI:https://www.example.com/calendar/janedoe\r\nCALURI:https://calendar.example.com/\r\nEND:VCARD\r\n";

        let vcard = format!("{}", card);
        assert_eq!(expected, &vcard);
    }

    #[test]
    fn builder_member_group() {
        let card = VcardBuilder::new("Mock Company".to_owned())
            .kind(Kind::Group)
            .member("https://example.com/foo".try_into().unwrap())
            .member("https://example.com/bar".try_into().unwrap())
            .finish();
        assert_eq!(2, card.member.len());
        assert!(card.validate().is_ok());
    }

    #[test]
    fn builder_member_invalid() {
        let card = VcardBuilder::new("Mock Company".to_owned())
            .member("https://example.com/bar".try_into().unwrap())
            .finish();
        assert_eq!(1, card.member.len());
        assert!(card.validate().is_err());
    }

    #[cfg(not(feature = "language-tags"))]
    #[test]
    fn builder_language() {
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
    fn builder_language_tags() {
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
