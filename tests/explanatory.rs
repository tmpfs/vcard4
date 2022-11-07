mod test_helpers;

use anyhow::Result;
use vcard_compact::parse;

use test_helpers::assert_round_trip;

// TODO: PRODID
// TODO: REV
// TODO: SOUND
// TODO: UID
// TODO: CLIENTPIDMAP

#[test]
fn explanatory_categories() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
CATEGORIES:TRAVEL AGENT
CATEGORIES:INTERNET,IETF,INDUSTRY,INFORMATION TECHNOLOGY
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    let prop = card.categories.get(0).unwrap();
    assert_eq!(&vec!["TRAVEL AGENT"], &prop.value);

    let prop = card.categories.get(1).unwrap();
    assert_eq!(
        &vec!["INTERNET", "IETF", "INDUSTRY", "INFORMATION TECHNOLOGY"],
        &prop.value
    );

    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn explanatory_note() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
NOTE:This fax number is operational 0800 to 1715 
 EST\, Mon-Fri.
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    let prop = card.note.get(0).unwrap();
    assert_eq!(
        "This fax number is operational 0800 to 1715 EST, Mon-Fri.",
        &prop.value
    );

    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn explanatory_url() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
URL:https://example.com/page/#section?foo=bar
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);

    let prop = card.url.get(0).unwrap();
    assert_eq!(
        "https://example.com/page/#section?foo=bar",
        &prop.value.to_string()
    );
    assert_round_trip(&card)?;
    Ok(())
}

// TODO: VERSION
