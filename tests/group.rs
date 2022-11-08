mod test_helpers;

use anyhow::Result;
use test_helpers::assert_round_trip;
use vcard_compact::parse;

#[test]
fn group() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
HOME.TITLE:Boss
WORK.TITLE:Researcher
END:VCARD"#;

    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    assert_eq!(Some("HOME".to_string()), card.title.get(0).unwrap().group);
    assert_eq!("Boss", &card.title.get(0).unwrap().value.to_string());

    assert_eq!(Some("WORK".to_string()), card.title.get(1).unwrap().group);
    assert_eq!("Researcher", &card.title.get(1).unwrap().value.to_string());

    assert_round_trip(&card)?;
    Ok(())
}
