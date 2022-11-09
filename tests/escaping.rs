mod test_helpers;

use anyhow::Result;
use test_helpers::assert_round_trip;
use vcard_compact::parse;

#[test]
fn escape_semi_colon() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\; Esq.
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    let fname = card.formatted_name.get(0).unwrap();
    assert_eq!("Mr. John Q. Public; Esq.", fname.value);
    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn escape_comma() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    let fname = card.formatted_name.get(0).unwrap();
    assert_eq!("Mr. John Q. Public, Esq.", fname.value);
    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn escape_backslash() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\ Esq.
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    let fname = card.formatted_name.get(0).unwrap();
    assert_eq!("Mr. John Q. Public\\ Esq.", fname.value);
    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn escape_newline() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
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
    assert_round_trip(&card)?;
    Ok(())
}
