use anyhow::Result;
use fluent_uri::Uri as URI;
use language_tags::LanguageTag;

use vcard_compact::{property::*, *};

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

    assert_eq!(Some(tag), parameters.language);
    assert_eq!(
        &vec![String::from("work")],
        parameters.types.as_ref().unwrap()
    );
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

    let uri = URI::parse("https://example.com")?.to_owned();
    let url = card.url.get(0).unwrap();
    assert_eq!(uri.as_str(), url.value.as_str());

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
    let uri = URI::parse(
        "ldap://ldap.example.com/cn=Babs%20Jensen,%20o=Babsco,%20c=US",
    )?
    .to_owned();
    let url = card.source.get(0).unwrap();
    assert_eq!(uri.as_str(), url.value.as_str());

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

    assert_eq!(Some(Kind::Individual), card.kind);

    let input = r#"BEGIN:VCARD
VERSION:4.0
KIND:org
FN:ABC Marketing
ORG:ABC\, Inc.;North American Division;Marketing
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    assert_eq!(Some(Kind::Org), card.kind);

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
        photo1.value.as_str()
    );

    assert!(photo2.value.as_str().starts_with("data:image/jpeg;base64,"));
    assert!(photo2.value.as_str().ends_with("TeXN0"));

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
    assert_eq!(Sex::Male, card.gender.as_ref().unwrap().sex);
    assert_eq!(None, card.gender.as_ref().unwrap().identity);

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
GENDER:F
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!(Sex::Female, card.gender.as_ref().unwrap().sex);
    assert_eq!(None, card.gender.as_ref().unwrap().identity);

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
GENDER:M;Fellow
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!(Sex::Male, card.gender.as_ref().unwrap().sex);
    assert_eq!(
        "Fellow",
        card.gender.as_ref().unwrap().identity.as_ref().unwrap()
    );

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
GENDER:F;grrrl
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!(Sex::Female, card.gender.as_ref().unwrap().sex);
    assert_eq!(
        "grrrl",
        card.gender.as_ref().unwrap().identity.as_ref().unwrap()
    );

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
GENDER:O;intersex
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!(Sex::Other, card.gender.as_ref().unwrap().sex);
    assert_eq!(
        "intersex",
        card.gender.as_ref().unwrap().identity.as_ref().unwrap()
    );

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
GENDER:;it's complicated
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!(Sex::None, card.gender.as_ref().unwrap().sex);
    assert_eq!(
        "it's complicated",
        card.gender.as_ref().unwrap().identity.as_ref().unwrap()
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

    if let TimeZoneProperty::Text(Text { value, .. }) =
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

    if let TimeZoneProperty::UtcOffset(UtcOffset { value, .. }) =
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

    if let TimeZoneProperty::Uri(Uri { value, .. }) =
        card.timezone.get(0).unwrap()
    {
        assert_eq!("https://example.com/tz-database/acdt", value.as_str());
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

    assert_eq!("geo:37.386013,-122.082932", geo.value.as_str());

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
        logo1.value.as_str()
    );

    assert!(logo2.value.as_str().starts_with("data:image/jpeg;base64,"));
    assert!(logo2.value.as_str().ends_with("TeXN0"));

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
        "urn:uuid:03a0e51f-d1aa-4385-8a53-e29025acd8af",
        card.member.get(0).unwrap().value.as_str()
    );
    assert_eq!(
        "urn:uuid:b8767877-b4a1-4c70-9acc-505d3819e519",
        card.member.get(1).unwrap().value.as_str()
    );

    let card = vcards.remove(0);
    if let TextOrUriProperty::Uri(Uri { value, .. }) =
        card.uid.as_ref().unwrap()
    {
        assert_eq!(
            "urn:uuid:03a0e51f-d1aa-4385-8a53-e29025acd8af",
            value.as_str()
        );
    } else {
        panic!("expecting URI for UID value");
    }

    let card = vcards.remove(0);
    if let TextOrUriProperty::Uri(Uri { value, .. }) =
        card.uid.as_ref().unwrap()
    {
        assert_eq!(
            "urn:uuid:b8767877-b4a1-4c70-9acc-505d3819e519",
            value.as_str()
        );
    } else {
        panic!("expecting URI for UID value");
    }

    let card = vcards.remove(0);
    assert_eq!(
        "mailto:subscriber1@example.com",
        card.member.get(0).unwrap().value.as_str()
    );
    assert_eq!(
        "xmpp:subscriber2@example.com",
        card.member.get(1).unwrap().value.as_str()
    );
    assert_eq!(
        "sip:subscriber3@example.com",
        card.member.get(2).unwrap().value.as_str()
    );
    assert_eq!(
        "tel:+1-418-555-5555",
        card.member.get(3).unwrap().value.as_str()
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
    if let TextOrUriProperty::Uri(Uri { value, parameters }) =
        card.related.get(0).unwrap()
    {
        assert_eq!(
            "urn:uuid:f81d4fae-7dec-11d0-a765-00a0c91e6bf6",
            value.as_str()
        );

        let params = parameters.as_ref().unwrap();
        assert_eq!(
            Some(&String::from("friend")),
            params.types.as_ref().unwrap().get(0)
        );
    } else {
        panic!("expecting URI for RELATED prop");
    }

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
RELATED;TYPE=contact:http://example.com/directory/jdoe.vcf
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    if let TextOrUriProperty::Uri(Uri { value, parameters }) =
        card.related.get(0).unwrap()
    {
        assert_eq!("http://example.com/directory/jdoe.vcf", value.as_str());

        let params = parameters.as_ref().unwrap();
        assert_eq!(
            Some(&String::from("contact")),
            params.types.as_ref().unwrap().get(0)
        );
    } else {
        panic!("expecting URI for RELATED prop");
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
    if let TextOrUriProperty::Text(Text { value, parameters }) =
        card.related.get(0).unwrap()
    {
        assert_eq!(
            "Please contact my assistant Jane Doe for any inquiries.",
            value.as_str()
        );

        let params = parameters.as_ref().unwrap();
        assert_eq!(
            Some(&String::from("co-worker")),
            params.types.as_ref().unwrap().get(0)
        );
    } else {
        panic!("expecting TEXT for RELATED prop");
    }

    Ok(())
}
