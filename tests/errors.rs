mod test_helpers;

use anyhow::Result;
use vcard_compact::{parse, Error};

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
