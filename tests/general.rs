mod test_helpers;

use anyhow::Result;
use vcard_compact::{parse, property::*};
use test_helpers::assert_round_trip;

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

    let url = card.source.get(0).unwrap();
    assert_eq!("ldap://ldap.example.com/cn=Babs%20Jensen,%20o=Babsco,%20c=US", &url.value.to_string());
    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn parse_source_folded() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
SOURCE;LANGUAGE=en:http://directory.example.com/addressbooks/jdoe/
 Jean%20Dupont.vcf
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    println!("{}", card);

    let url = card.source.get(0).unwrap();
    assert_eq!("http://directory.example.com/addressbooks/jdoe/Jean%20Dupont.vcf", &url.value.to_string());
    assert_round_trip(&card)?;
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
    assert_round_trip(&card)?;

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
    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn parse_xml() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
XML:<root></root>
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    let xml = card.xml.get(0).unwrap();
    assert_eq!("<root></root>", &xml.value);
    assert_round_trip(&card)?;
    Ok(())
}
