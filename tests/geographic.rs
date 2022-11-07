mod test_helpers;

use anyhow::Result;
use test_helpers::assert_round_trip;
use vcard_compact::{parse, property::*};

// Geographic Properties

#[test]
fn geographic_tz() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
TZ:Raleigh/North America
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);

    if let TimeZoneProperty::Text(TextProperty { value, .. }) =
        card.timezone.get(0).unwrap()
    {
        assert_eq!("Raleigh/North America", value);
    } else {
        panic!("expecting text value for TZ");
    }
    assert_round_trip(&card)?;

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
TZ;VALUE=utc-offset:-0500
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);

    if let TimeZoneProperty::UtcOffset(UtcOffsetProperty { value, .. }) =
        card.timezone.get(0).unwrap()
    {
        assert_eq!((-5, -0, -0), value.as_hms());
    } else {
        panic!("expecting utc-offset value for TZ");
    }
    assert_round_trip(&card)?;

    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
TZ;VALUE=uri:https://example.com/tz-database/acdt
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    assert_round_trip(&card)?;

    let card = vcards.remove(0);

    if let TimeZoneProperty::Uri(UriProperty { value, .. }) =
        card.timezone.get(0).unwrap()
    {
        assert_eq!(
            "https://example.com/tz-database/acdt",
            &value.to_string()
        );
    } else {
        panic!("expecting uri value for TZ");
    }
    assert_round_trip(&card)?;

    Ok(())
}

#[test]
fn geographic_geo() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
GEO:geo:37.386013,-122.082932
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    let geo = card.geo.get(0).unwrap();

    assert_eq!("geo:37.386013,-122.082932", &geo.value.to_string());
    assert_round_trip(&card)?;
    Ok(())
}
