mod test_helpers;

use anyhow::Result;
use vcard_compact::parse;

use test_helpers::assert_round_trip;

// TODO: CATEGORIES
// TODO: NOTE
// TODO: PRODID
// TODO: REV
// TODO: SOUND
// TODO: UID
// TODO: CLIENTPIDMAP

#[test]
fn explanatory_url() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mock person
URL:https://example.com
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);

    let url = card.url.get(0).unwrap();
    assert_eq!("https://example.com/", &url.value.to_string());
    assert_round_trip(&card)?;
    Ok(())
}

// TODO: VERSION
