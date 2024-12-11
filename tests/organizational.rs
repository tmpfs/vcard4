mod test_helpers;

use anyhow::Result;
use test_helpers::assert_round_trip;
use vcard4::{parameter::TypeParameter, parse, property::*, Uri};

#[test]
fn organizational_title() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
TITLE:Research Scientist
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!("Research Scientist", card.title.get(0).unwrap().value);
    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn organizational_role() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
ROLE:Project Leader
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!("Project Leader", card.role.get(0).unwrap().value);
    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn organizational_logo() -> Result<()> {
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

    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn organizational_org() -> Result<()> {
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
    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn organizational_member() -> Result<()> {
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
        "urn:uuid:03a0e51f-d1aa-4385-8a53-e29025acd8af".parse::<Uri>()?,
        card.member.get(0).unwrap().value
    );
    assert_eq!(
        "urn:uuid:b8767877-b4a1-4c70-9acc-505d3819e519".parse::<Uri>()?,
        card.member.get(1).unwrap().value
    );
    assert_round_trip(&card)?;

    let card = vcards.remove(0);
    if let TextOrUriProperty::Uri(UriProperty { value, .. }) =
        card.uid.as_ref().unwrap()
    {
        assert_eq!(
            &"urn:uuid:03a0e51f-d1aa-4385-8a53-e29025acd8af"
                .parse::<Uri>()?,
            value
        );
    } else {
        panic!("expecting Uri for UID value");
    }
    assert_round_trip(&card)?;

    let card = vcards.remove(0);
    if let TextOrUriProperty::Uri(UriProperty { value, .. }) =
        card.uid.as_ref().unwrap()
    {
        assert_eq!(
            &"urn:uuid:b8767877-b4a1-4c70-9acc-505d3819e519"
                .parse::<Uri>()?,
            value
        );
    } else {
        panic!("expecting Uri for UID value");
    }
    assert_round_trip(&card)?;

    let card = vcards.remove(0);
    assert_eq!(
        "mailto:subscriber1@example.com".parse::<Uri>()?,
        card.member.get(0).unwrap().value
    );
    assert_eq!(
        "xmpp:subscriber2@example.com".parse::<Uri>()?,
        card.member.get(1).unwrap().value
    );
    assert_eq!(
        "sip:subscriber3@example.com".parse::<Uri>()?,
        card.member.get(2).unwrap().value
    );
    assert_eq!(
        "tel:+1-418-555-5555".parse::<Uri>()?,
        card.member.get(3).unwrap().value
    );
    assert_round_trip(&card)?;

    Ok(())
}

#[test]
fn organizational_related() -> Result<()> {
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
            &"urn:uuid:f81d4fae-7dec-11d0-a765-00a0c91e6bf6"
                .parse::<Uri>()?,
            value
        );

        let param: TypeParameter = "friend".parse()?;
        let params = parameters.as_ref().unwrap();
        assert_eq!(Some(&param), params.types.as_ref().unwrap().get(0));
    } else {
        panic!("expecting Uri for RELATED prop");
    }
    assert_round_trip(&card)?;

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
            &"http://example.com/directory/jdoe.vcf".parse::<Uri>()?,
            value
        );

        let param: TypeParameter = "contact".parse()?;
        let params = parameters.as_ref().unwrap();
        assert_eq!(Some(&param), params.types.as_ref().unwrap().get(0));
    } else {
        panic!("expecting Uri for RELATED prop");
    }
    assert_round_trip(&card)?;

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
    assert_round_trip(&card)?;

    Ok(())
}
