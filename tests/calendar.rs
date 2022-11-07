mod test_helpers;

use anyhow::Result;
use test_helpers::{assert_media_type, assert_round_trip};
use vcard_compact::parse;

#[test]
fn calendar_fburl() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
FBURL;PREF=1:http://www.example.com/busy/janedoe
FBURL;MEDIATYPE=text/calendar:ftp://example.com/busy/project-a.ifb
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    let prop = card.fburl.get(0).unwrap();
    let pref = prop.parameters.as_ref().unwrap().pref.clone().unwrap();
    assert_eq!(1, pref);
    assert_eq!(
        "http://www.example.com/busy/janedoe",
        &prop.value.to_string()
    );

    let prop = card.fburl.get(1).unwrap();
    assert_media_type(prop.parameters.as_ref(), "text/calendar")?;
    assert_eq!(
        "ftp://example.com/busy/project-a.ifb",
        &prop.value.to_string()
    );

    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn calendar_cal_adr_uri() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
CALADRURI;PREF=1:mailto:janedoe@example.com
CALADRURI:http://example.com/calendar/jdoe
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    let prop = card.cal_adr_uri.get(0).unwrap();
    let pref = prop.parameters.as_ref().unwrap().pref.clone().unwrap();
    assert_eq!(1, pref);
    assert_eq!("mailto:janedoe@example.com", &prop.value.to_string());

    let prop = card.cal_adr_uri.get(1).unwrap();
    assert_eq!("http://example.com/calendar/jdoe", &prop.value.to_string());

    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn calendar_cal_uri() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
CALURI;PREF=1:http://cal.example.com/calA
CALURI;MEDIATYPE=text/calendar:ftp://ftp.example.com/calA.ics
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    let prop = card.cal_uri.get(0).unwrap();
    let pref = prop.parameters.as_ref().unwrap().pref.clone().unwrap();
    assert_eq!(1, pref);
    assert_eq!("http://cal.example.com/calA", &prop.value.to_string());

    let prop = card.cal_uri.get(1).unwrap();
    assert_media_type(prop.parameters.as_ref(), "text/calendar")?;
    assert_eq!("ftp://ftp.example.com/calA.ics", &prop.value.to_string());

    assert_round_trip(&card)?;
    Ok(())
}
