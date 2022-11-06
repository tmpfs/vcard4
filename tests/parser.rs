use anyhow::Result;
use language_tags::LanguageTag;
use uriparse::uri::URI as Uri;

use vcard_compact::{parameters::TypeParameter, parse, property::*, Error};

#[test]
fn parse_empty() -> Result<()> {
    let result = parse("");
    if !matches!(result, Err(Error::TokenExpected)) {
        panic!("wrong error variant");
    }
    Ok(())
}

#[test]
fn parse_wrong_token() -> Result<()> {
    let result = parse("VERSION:4.0");
    if !matches!(result, Err(Error::IncorrectToken)) {
        panic!("wrong error variant");
    }
    Ok(())
}

#[test]
fn parse_no_version() -> Result<()> {
    let input = r#"BEGIN:VCARD"#;
    let result = parse(input);
    if !matches!(result, Err(Error::TokenExpected)) {
        panic!("wrong error variant");
    }
    Ok(())
}

#[test]
fn parse_no_end() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0"#;
    let result = parse(input);
    if !matches!(result, Err(Error::TokenExpected)) {
        panic!("wrong error variant");
    }
    Ok(())
}

#[test]
fn parse_escaped_semi_colon() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\; Esq.
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    let fname = card.formatted_name.get(0).unwrap();
    assert_eq!("Mr. John Q. Public; Esq.", fname.value);
    Ok(())
}

#[test]
fn parse_folded_space() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. 
 John Q. 
 Public\, 
 Esq.
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    let fname = card.formatted_name.get(0).unwrap();
    assert_eq!("Mr. John Q. Public, Esq.", fname.value);
    Ok(())
}

#[test]
fn parse_newline() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
NOTE:Mythical Manager\NHyjinx Software Division\n
 BabsCo\, Inc.\N
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    let expected = r#"Mythical Manager
Hyjinx Software Division
BabsCo, Inc.
"#;

    let note = &card.note.get(0).unwrap().value;
    assert_eq!(expected, note);
    Ok(())
}

#[test]
fn parse_folded_tab() -> Result<()> {
    let input = "BEGIN:VCARD\nVERSION:4.0\nFN:Mr. \n\u{0009}John Q. \n\u{0009}Public\\, \n\u{0009}Esq.\nEND:VCARD";

    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    let fname = card.formatted_name.get(0).unwrap();
    assert_eq!("Mr. John Q. Public, Esq.", fname.value);
    Ok(())
}

#[test]
fn parse_parameters() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
NICKNAME;LANGUAGE=en;TYPE=work:Boss
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);

    let fname = card.formatted_name.get(0).unwrap();
    assert_eq!("Mr. John Q. Public, Esq.", fname.value);

    let nickname = card.nickname.get(0).unwrap();
    assert_eq!("Boss", nickname.value);
    assert!(nickname.parameters.is_some());

    let tag: LanguageTag = "en".parse()?;
    let parameters = nickname.parameters.as_ref().unwrap();

    let param: TypeParameter = "work".parse()?;
    assert_eq!(Some(tag), parameters.language);
    assert_eq!(&vec![param], parameters.types.as_ref().unwrap());
    Ok(())
}

#[test]
fn parse_url() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mock person
URL:https://example.com
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);

    let uri = Uri::try_from("https://example.com")?.into_owned();
    let url = card.url.get(0).unwrap();
    assert_eq!(uri, url.value);

    Ok(())
}

// General

#[test]
fn parse_source() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
SOURCE:ldap://ldap.example.com/cn=Babs%20Jensen,%20o=Babsco,%20c=US
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);
    let uri = Uri::try_from(
        "ldap://ldap.example.com/cn=Babs%20Jensen,%20o=Babsco,%20c=US",
    )?
    .to_owned();
    let url = card.source.get(0).unwrap();
    assert_eq!(uri, url.value);

    Ok(())
}

#[test]
fn parse_kind() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
KIND:individual
FN:Jane Doe
ORG:ABC\, Inc.;North American Division;Marketing
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    assert_eq!(Kind::Individual, card.kind.as_ref().unwrap().value);

    let input = r#"BEGIN:VCARD
VERSION:4.0
KIND:org
FN:ABC Marketing
ORG:ABC\, Inc.;North American Division;Marketing
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    assert_eq!(Kind::Org, card.kind.as_ref().unwrap().value);

    Ok(())
}

// TODO: XML

// Identification

#[test]
fn parse_fn() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    let fname = card.formatted_name.get(0).unwrap();
    assert_eq!("Mr. John Q. Public, Esq.", fname.value);
    Ok(())
}

#[test]
fn parse_n() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
N:Public;John;Quinlan;Mr.;Esq.
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    let name = card.name.as_ref().unwrap();
    assert_eq!(vec!["Public", "John", "Quinlan", "Mr.", "Esq."], name.value);
    Ok(())
}

#[test]
fn parse_nickname() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
N:Public;John;Quinlan;Mr.;Esq.
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    let name = card.name.as_ref().unwrap();
    assert_eq!(vec!["Public", "John", "Quinlan", "Mr.", "Esq."], name.value);
    Ok(())
}

#[test]
fn parse_photo() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
PHOTO:http://www.example.com/pub/photos/jqpublic.gif
PHOTO:data:image/jpeg;base64,MIICajCCAdOgAwIBAgICBEUwDQYJKoZIhv
 AQEEBQAwdzELMAkGA1UEBhMCVVMxLDAqBgNVBAoTI05ldHNjYXBlIENvbW11bm
 ljYXRpb25zIENvcnBvcmF0aW9uMRwwGgYDVQQLExNJbmZvcm1hdGlvbiBTeXN0
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!(2, card.photo.len());

    let photo1 = card.photo.get(0).unwrap();
    let photo2 = card.photo.get(1).unwrap();

    assert_eq!(
        "http://www.example.com/pub/photos/jqpublic.gif",
        &photo1.value.to_string()
    );

    assert!(photo2
        .value
        .to_string()
        .starts_with("data:image/jpeg;base64,"));
    assert!(photo2.value.to_string().ends_with("TeXN0"));

    Ok(())
}

// TODO: BDAY
// TODO: ANNIVERSARY

#[test]
fn parse_gender() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
GENDER:M
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!(Sex::Male, card.gender.as_ref().unwrap().value.sex);
    assert_eq!(None, card.gender.as_ref().unwrap().value.identity);

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
GENDER:F
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!(Sex::Female, card.gender.as_ref().unwrap().value.sex);
    assert_eq!(None, card.gender.as_ref().unwrap().value.identity);

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
GENDER:M;Fellow
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!(Sex::Male, card.gender.as_ref().unwrap().value.sex);
    assert_eq!(
        "Fellow",
        card.gender
            .as_ref()
            .unwrap()
            .value
            .identity
            .as_ref()
            .unwrap()
    );

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
GENDER:F;grrrl
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!(Sex::Female, card.gender.as_ref().unwrap().value.sex);
    assert_eq!(
        "grrrl",
        card.gender
            .as_ref()
            .unwrap()
            .value
            .identity
            .as_ref()
            .unwrap()
    );

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
GENDER:O;intersex
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!(Sex::Other, card.gender.as_ref().unwrap().value.sex);
    assert_eq!(
        "intersex",
        card.gender
            .as_ref()
            .unwrap()
            .value
            .identity
            .as_ref()
            .unwrap()
    );

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
GENDER:;it's complicated
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!(Sex::None, card.gender.as_ref().unwrap().value.sex);
    assert_eq!(
        "it's complicated",
        card.gender
            .as_ref()
            .unwrap()
            .value
            .identity
            .as_ref()
            .unwrap()
    );

    Ok(())
}

// Delivery Addressing

// TODO: ADR

// Communications Properties

// TODO: TEL
// TODO: EMAIL
// TODO: IMPP
// TODO: LANG

// Geographic Properties

#[test]
fn parse_tz() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
TZ:Raleigh/North America
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);

    if let TimeZoneProperty::Text(TextProperty { value, .. }) =
        card.timezone.get(0).unwrap()
    {
        assert_eq!("Raleigh/North America", value);
    } else {
        panic!("expecting text value for TZ");
    }

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
TZ;VALUE=utc-offset:-0500
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);

    if let TimeZoneProperty::UtcOffset(UtcOffsetProperty { value, .. }) =
        card.timezone.get(0).unwrap()
    {
        assert_eq!((-5, -0, -0), value.as_hms());
    } else {
        panic!("expecting utc-offset value for TZ");
    }

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
TZ;VALUE=uri:https://example.com/tz-database/acdt
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);

    if let TimeZoneProperty::Uri(UriProperty { value, .. }) =
        card.timezone.get(0).unwrap()
    {
        assert_eq!(
            "https://example.com/tz-database/acdt",
            &value.to_string()
        );
    } else {
        panic!("expecting uri value for TZ");
    }

    Ok(())
}

#[test]
fn parse_geo() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
GEO:geo:37.386013,-122.082932
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    let geo = card.geo.get(0).unwrap();

    assert_eq!("geo:37.386013,-122.082932", &geo.value.to_string());

    Ok(())
}

// Organizational Properties

#[test]
fn parse_title() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
TITLE:Research Scientist
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!("Research Scientist", card.title.get(0).unwrap().value);
    Ok(())
}

#[test]
fn parse_role() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
ROLE:Project Leader
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!("Project Leader", card.role.get(0).unwrap().value);
    Ok(())
}

#[test]
fn parse_logo() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
LOGO:http://www.example.com/pub/logos/abccorp.jpg
LOGO:data:image/jpeg;base64,MIICajCCAdOgAwIBAgICBEUwDQYJKoZIhvc
 AQEEBQAwdzELMAkGA1UEBhMCVVMxLDAqBgNVBAoTI05ldHNjYXBlIENvbW11bm
 ljYXRpb25zIENvcnBvcmF0aW9uMRwwGgYDVQQLExNJbmZvcm1hdGlvbiBTeXN0
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);

    assert_eq!(2, card.logo.len());

    let logo1 = card.logo.get(0).unwrap();
    let logo2 = card.logo.get(1).unwrap();

    assert_eq!(
        "http://www.example.com/pub/logos/abccorp.jpg",
        &logo1.value.to_string()
    );

    assert!(logo2
        .value
        .to_string()
        .starts_with("data:image/jpeg;base64,"));
    assert!(logo2.value.to_string().ends_with("TeXN0"));

    Ok(())
}

#[test]
fn parse_org() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
ORG:ABC\, Inc.;North American Division;Marketing
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!(
        vec!["ABC, Inc.", "North American Division", "Marketing"],
        card.org.get(0).unwrap().value
    );
    Ok(())
}

#[test]
fn parse_member() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
KIND:group
FN:The Doe family
MEMBER:urn:uuid:03a0e51f-d1aa-4385-8a53-e29025acd8af
MEMBER:urn:uuid:b8767877-b4a1-4c70-9acc-505d3819e519
END:VCARD
BEGIN:VCARD
VERSION:4.0
FN:John Doe
UID:urn:uuid:03a0e51f-d1aa-4385-8a53-e29025acd8af
END:VCARD
BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
UID:urn:uuid:b8767877-b4a1-4c70-9acc-505d3819e519
END:VCARD

BEGIN:VCARD
VERSION:4.0
KIND:group
FN:Funky distribution list
MEMBER:mailto:subscriber1@example.com
MEMBER:xmpp:subscriber2@example.com
MEMBER:sip:subscriber3@example.com
MEMBER:tel:+1-418-555-5555
END:VCARD"#;

    let mut vcards = parse(input)?;
    assert_eq!(4, vcards.len());

    let card = vcards.remove(0);
    assert_eq!(
        Uri::try_from("urn:uuid:03a0e51f-d1aa-4385-8a53-e29025acd8af")?,
        card.member.get(0).unwrap().value
    );
    assert_eq!(
        Uri::try_from("urn:uuid:b8767877-b4a1-4c70-9acc-505d3819e519")?,
        card.member.get(1).unwrap().value
    );

    let card = vcards.remove(0);
    if let TextOrUriProperty::Uri(UriProperty { value, .. }) =
        card.uid.as_ref().unwrap()
    {
        assert_eq!(
            &Uri::try_from("urn:uuid:03a0e51f-d1aa-4385-8a53-e29025acd8af")?,
            value
        );
    } else {
        panic!("expecting Uri for UID value");
    }

    let card = vcards.remove(0);
    if let TextOrUriProperty::Uri(UriProperty { value, .. }) =
        card.uid.as_ref().unwrap()
    {
        assert_eq!(
            &Uri::try_from("urn:uuid:b8767877-b4a1-4c70-9acc-505d3819e519")?,
            value
        );
    } else {
        panic!("expecting Uri for UID value");
    }

    let card = vcards.remove(0);
    assert_eq!(
        Uri::try_from("mailto:subscriber1@example.com")?,
        card.member.get(0).unwrap().value
    );
    assert_eq!(
        Uri::try_from("xmpp:subscriber2@example.com")?,
        card.member.get(1).unwrap().value
    );
    assert_eq!(
        Uri::try_from("sip:subscriber3@example.com")?,
        card.member.get(2).unwrap().value
    );
    assert_eq!(
        Uri::try_from("tel:+1-418-555-5555")?,
        card.member.get(3).unwrap().value
    );

    Ok(())
}

#[test]
fn parse_related() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
RELATED;TYPE=friend:urn:uuid:f81d4fae-7dec-11d0-a765-00a0c91e6bf6
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    if let TextOrUriProperty::Uri(UriProperty {
        value, parameters, ..
    }) = card.related.get(0).unwrap()
    {
        assert_eq!(
            &Uri::try_from("urn:uuid:f81d4fae-7dec-11d0-a765-00a0c91e6bf6")?,
            value
        );

        let param: TypeParameter = "friend".parse()?;
        let params = parameters.as_ref().unwrap();
        assert_eq!(Some(&param), params.types.as_ref().unwrap().get(0));
    } else {
        panic!("expecting Uri for RELATED prop");
    }

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
RELATED;TYPE=contact:http://example.com/directory/jdoe.vcf
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    if let TextOrUriProperty::Uri(UriProperty {
        value, parameters, ..
    }) = card.related.get(0).unwrap()
    {
        assert_eq!(
            &Uri::try_from("http://example.com/directory/jdoe.vcf")?,
            value
        );

        let param: TypeParameter = "contact".parse()?;
        let params = parameters.as_ref().unwrap();
        assert_eq!(Some(&param), params.types.as_ref().unwrap().get(0));
    } else {
        panic!("expecting Uri for RELATED prop");
    }

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
RELATED;TYPE=co-worker;VALUE=text:Please contact my assistant Jane 
 Doe for any inquiries.
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    if let TextOrUriProperty::Text(TextProperty {
        value, parameters, ..
    }) = card.related.get(0).unwrap()
    {
        assert_eq!(
            "Please contact my assistant Jane Doe for any inquiries.",
            value.as_str()
        );

        let param: TypeParameter = "co-worker".parse()?;
        let params = parameters.as_ref().unwrap();
        assert_eq!(Some(&param), params.types.as_ref().unwrap().get(0));
    } else {
        panic!("expecting TEXT for RELATED prop");
    }

    Ok(())
}
