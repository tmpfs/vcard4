mod test_helpers;

use anyhow::Result;
use test_helpers::{assert_language, assert_round_trip};
use vcard_compact::{
    parameter::{TelephoneType, TypeParameter},
    parse,
    property::TextOrUriProperty,
};

// Communications Properties

#[test]
fn communications_tel() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
TEL;VALUE=uri;PREF=1;TYPE="voice,home":tel:+1-555-555-5555;ext=5555
TEL;VALUE=uri;TYPE=home:tel:+33-01-23-45-67
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    let prop = card.tel.get(0).unwrap();
    if let TextOrUriProperty::Uri(prop) = prop {
        let pref = prop.parameters.as_ref().unwrap().pref.clone().unwrap();
        assert_eq!(1, pref);

        let types = prop.parameters.as_ref().unwrap().types.as_ref().unwrap();
        assert_eq!(
            &TypeParameter::Telephone(TelephoneType::Voice),
            types.get(0).unwrap()
        );

        assert_eq!(&TypeParameter::Home, types.get(1).unwrap());

        assert_eq!("tel:+1-555-555-5555;ext=5555", &prop.value.to_string());
        assert_round_trip(&card)?;
    } else {
        panic!("expecting URI for TEL property");
    }

    let prop = card.tel.get(1).unwrap();
    if let TextOrUriProperty::Uri(prop) = prop {
        let types = prop.parameters.as_ref().unwrap().types.as_ref().unwrap();
        assert_eq!(&TypeParameter::Home, types.get(0).unwrap());

        assert_eq!("tel:+33-01-23-45-67", &prop.value.to_string());
        assert_round_trip(&card)?;
    } else {
        panic!("expecting URI for TEL property");
    }

    Ok(())
}

#[test]
fn communications_email() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
EMAIL;TYPE=work:jqpublic@xyz.example.com
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    let prop = card.email.get(0).unwrap();

    let types = prop.parameters.as_ref().unwrap().types.as_ref().unwrap();
    assert_eq!(&TypeParameter::Work, types.get(0).unwrap());

    assert_eq!("jqpublic@xyz.example.com", &prop.value);
    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn communications_impp() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
IMPP;PREF=1:xmpp:alice@example.com
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    let prop = card.impp.get(0).unwrap();
    let pref = prop.parameters.as_ref().unwrap().pref.clone().unwrap();
    assert_eq!(1, pref);
    assert_eq!("xmpp:alice@example.com", &prop.value.to_string());
    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn communications_lang() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
LANG;TYPE=work;PREF=1:en
LANG;TYPE=work;PREF=2:fr
LANG;TYPE=home:fr
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    let prop = card.lang.get(0).unwrap();
    let types = prop.parameters.as_ref().unwrap().types.as_ref().unwrap();
    assert_eq!(&TypeParameter::Work, types.get(0).unwrap());
    let pref = prop.parameters.as_ref().unwrap().pref.clone().unwrap();
    assert_eq!(1, pref);
    assert_language(&prop.value, "en")?;

    let prop = card.lang.get(1).unwrap();
    let types = prop.parameters.as_ref().unwrap().types.as_ref().unwrap();
    assert_eq!(&TypeParameter::Work, types.get(0).unwrap());
    let pref = prop.parameters.as_ref().unwrap().pref.clone().unwrap();
    assert_eq!(2, pref);
    assert_language(&prop.value, "fr")?;

    let prop = card.lang.get(2).unwrap();
    let types = prop.parameters.as_ref().unwrap().types.as_ref().unwrap();
    assert_eq!(&TypeParameter::Home, types.get(0).unwrap());
    assert_language(&prop.value, "fr")?;

    assert_round_trip(&card)?;
    Ok(())
}
