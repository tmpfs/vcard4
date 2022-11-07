mod test_helpers;

use anyhow::Result;
use vcard_compact::parse;

use test_helpers::assert_round_trip;

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
fn explanatory_prod_id() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
PRODID:-//ONLINE DIRECTORY//NONSGML Version 1//EN
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    let prop = card.prod_id.as_ref().unwrap();
    assert_eq!("-//ONLINE DIRECTORY//NONSGML Version 1//EN", &prop.value);

    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn explanatory_rev() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
REV:19951031T222710Z
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    let prop = card.rev.as_ref().unwrap();
    assert_eq!("1995-10-31 22:27:10.0 +00:00:00", &prop.value.to_string());
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
