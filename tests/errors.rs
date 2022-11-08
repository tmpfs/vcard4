mod test_helpers;

use anyhow::Result;
use vcard_compact::{parameter::*, parse, types::*, Error};

#[test]
fn error_empty() -> Result<()> {
    let result = parse("");
    assert!(matches!(result, Err(Error::TokenExpected)));
    Ok(())
}

#[test]
fn error_wrong_token() -> Result<()> {
    let result = parse("VERSION:4.0");
    assert!(matches!(result, Err(Error::IncorrectToken)));
    Ok(())
}

#[test]
fn error_no_version() -> Result<()> {
    let input = r#"BEGIN:VCARD"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::TokenExpected)));
    Ok(())
}

#[test]
fn error_no_end() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::TokenExpected)));
    Ok(())
}

#[test]
fn error_parse_from_str() -> Result<()> {
    assert!(parse_boolean("foo").is_err());

    assert!("foo".parse::<TelephoneType>().is_err());
    assert!("foo".parse::<RelatedType>().is_err());
    assert!("foo".parse::<ValueType>().is_err());

    assert!("0;urn:uid:".parse::<ClientPidMap>().is_err());

    Ok(())
}

#[test]
fn error_type_unknown() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN;TYPE=FOO:Jane Doe
END:VCARD"#;
    let result = parse(input);
    assert!(matches!(result, Err(Error::UnknownRelatedType(_))));
    Ok(())
}
