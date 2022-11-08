mod test_helpers;

use anyhow::Result;
use test_helpers::assert_round_trip;
use vcard_compact::{parse, property::*};

#[test]
fn identification_fn() -> Result<()> {
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
fn identification_n() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
N:Public;John;Quinlan;Mr.;Esq.
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    let name = card.name.as_ref().unwrap();
    assert_eq!(vec!["Public", "John", "Quinlan", "Mr.", "Esq."], name.value);
    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn identification_nickname() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
N:Public;John;Quinlan;Mr.;Esq.
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    let name = card.name.as_ref().unwrap();
    assert_eq!(vec!["Public", "John", "Quinlan", "Mr.", "Esq."], name.value);
    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn identification_photo() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
PHOTO:http://www.example.com/pub/photos/jqpublic.gif
PHOTO:data:image/jpeg;base64,MIICajCCAdOgAwIBAgICBEUwDQYJKoZIhv
 AQEEBQAwdzELMAkGA1UEBhMCVVMxLDAqBgNVBAoTI05ldHNjYXBlIENvbW11bm
 ljYXRpb25zIENvcnBvcmF0aW9uMRwwGgYDVQQLExNJbmZvcm1hdGlvbiBTeXN0
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!(2, card.photo.len());

    let photo1 = card.photo.get(0).unwrap();
    let photo2 = card.photo.get(1).unwrap();

    assert_eq!(
        "http://www.example.com/pub/photos/jqpublic.gif",
        &photo1.value.to_string()
    );

    assert!(photo2
        .value
        .to_string()
        .starts_with("data:image/jpeg;base64,"));
    assert!(photo2.value.to_string().ends_with("TeXN0"));

    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn identification_bday() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
BDAY:19531015
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    let bday = card.bday.as_ref().unwrap();
    assert_eq!("19531015", &bday.to_string(),);
    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn identification_anniversary() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
ANNIVERSARY:19960415
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    let anniversary = card.anniversary.as_ref().unwrap();
    assert_eq!("19960415", &anniversary.to_string(),);
    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn identification_gender() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
GENDER:M
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!(Sex::Male, card.gender.as_ref().unwrap().value.sex);
    assert_eq!(None, card.gender.as_ref().unwrap().value.identity);
    assert_round_trip(&card)?;

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
GENDER:F
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!(Sex::Female, card.gender.as_ref().unwrap().value.sex);
    assert_eq!(None, card.gender.as_ref().unwrap().value.identity);
    assert_round_trip(&card)?;

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
GENDER:M;Fellow
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!(Sex::Male, card.gender.as_ref().unwrap().value.sex);
    assert_eq!(
        "Fellow",
        card.gender
            .as_ref()
            .unwrap()
            .value
            .identity
            .as_ref()
            .unwrap()
    );
    assert_round_trip(&card)?;

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
GENDER:F;grrrl
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!(Sex::Female, card.gender.as_ref().unwrap().value.sex);
    assert_eq!(
        "grrrl",
        card.gender
            .as_ref()
            .unwrap()
            .value
            .identity
            .as_ref()
            .unwrap()
    );
    assert_round_trip(&card)?;

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
GENDER:O;intersex
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!(Sex::Other, card.gender.as_ref().unwrap().value.sex);
    assert_eq!(
        "intersex",
        card.gender
            .as_ref()
            .unwrap()
            .value
            .identity
            .as_ref()
            .unwrap()
    );
    assert_round_trip(&card)?;

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
GENDER:;it's complicated
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_eq!(Sex::None, card.gender.as_ref().unwrap().value.sex);
    assert_eq!(
        "it's complicated",
        card.gender
            .as_ref()
            .unwrap()
            .value
            .identity
            .as_ref()
            .unwrap()
    );
    assert_round_trip(&card)?;

    Ok(())
}
