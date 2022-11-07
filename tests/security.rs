mod test_helpers;

use anyhow::Result;
use test_helpers::{assert_media_type, assert_round_trip};
use vcard_compact::{parse, property::*};

#[test]
fn security_key() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
KEY:http://www.example.com/keys/jdoe.cer
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    let prop = card.key.get(0).unwrap();
    if let TextOrUriProperty::Uri(prop) = prop {
        assert_eq!(
            "http://www.example.com/keys/jdoe.cer",
            &prop.value.to_string()
        );
        assert_round_trip(&card)?;
    } else {
        panic!("expecting URI variant");
    }
    Ok(())
}

#[test]
fn security_key_mediatype() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
KEY;MEDIATYPE=application/pgp-keys:ftp://example.com/keys/jdoe
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    let prop = card.key.get(0).unwrap();
    if let TextOrUriProperty::Uri(prop) = prop {
        assert_media_type(prop.parameters.as_ref(), "application/pgp-keys")?;
        assert_eq!("ftp://example.com/keys/jdoe", &prop.value.to_string());
        assert_round_trip(&card)?;
    } else {
        panic!("expecting URI variant");
    }
    Ok(())
}
