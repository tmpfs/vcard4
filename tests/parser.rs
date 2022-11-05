use anyhow::Result;
use fluent_uri::Uri as URI;
use language_tags::LanguageTag;

use vcard_compact::*;

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
    let uri = URI::parse("ldap://ldap.example.com/cn=Babs%20Jensen,%20o=Babsco,%20c=US")?.to_owned();
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
    let name = card.name.unwrap();
    assert_eq!(
        vec!["Public", "John", "Quinlan", "Mr.", "Esq."],
        name.value);
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
    let name = card.name.unwrap();
    assert_eq!(
        vec!["Public", "John", "Quinlan", "Mr.", "Esq."],
        name.value);
    Ok(())
}
