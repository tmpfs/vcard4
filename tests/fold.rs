mod test_helpers;

use anyhow::Result;
use test_helpers::assert_round_trip;
use vcard_compact::parse;

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
    assert_round_trip(&card)?;
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
    assert_round_trip(&card)?;
    Ok(())
}
