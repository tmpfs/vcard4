mod test_helpers;

use anyhow::Result;
use test_helpers::assert_round_trip;
use uriparse::uri::URI as Uri;
use vcard_compact::{
    parameter::ValueType,
    parse,
    property::AnyProperty,
    types::{
        parse_date_list, parse_date_time_list, parse_time_list, DateAndOrTime,
    },
};

#[test]
fn extension_text() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
X-FOO;VALUE=text:This is some text.
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    let prop = card.extensions.get(0).unwrap();

    assert!(prop.group.is_none());
    assert_eq!("X-FOO", &prop.name);
    assert_eq!(
        &ValueType::Text,
        prop.parameters.as_ref().unwrap().value.as_ref().unwrap()
    );
    assert_eq!(
        &AnyProperty::Text("This is some text.".to_string()),
        &prop.value
    );

    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn extension_uri() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
X-FOO;VALUE=uri:http://example.com/foo
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    let prop = card.extensions.get(0).unwrap();

    assert!(prop.group.is_none());
    assert_eq!("X-FOO", &prop.name);
    assert_eq!(
        &ValueType::Uri,
        prop.parameters.as_ref().unwrap().value.as_ref().unwrap()
    );
    assert_eq!(
        &AnyProperty::Uri(
            Uri::try_from("http://example.com/foo")?.into_owned()
        ),
        &prop.value
    );

    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn extension_date_only() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
X-FOO;VALUE=date:20221107
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    let prop = card.extensions.get(0).unwrap();

    assert!(prop.group.is_none());
    assert_eq!("X-FOO", &prop.name);
    assert_eq!(
        &ValueType::Date,
        prop.parameters.as_ref().unwrap().value.as_ref().unwrap()
    );

    let expected = parse_date_list("20221107")?;
    assert_eq!(&AnyProperty::Date(expected), &prop.value);

    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn extension_time_only() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
X-FOO;VALUE=time:2200,1800Z,140000-0800
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    let prop = card.extensions.get(0).unwrap();

    assert!(prop.group.is_none());
    assert_eq!("X-FOO", &prop.name);
    assert_eq!(
        &ValueType::Time,
        prop.parameters.as_ref().unwrap().value.as_ref().unwrap()
    );

    let expected = parse_time_list("2200,1800Z,140000-0800")?;
    assert_eq!(&AnyProperty::Time(expected), &prop.value);

    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn extension_date_time_only() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
X-FOO;VALUE=date-time:20221107T2200
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    let prop = card.extensions.get(0).unwrap();

    assert!(prop.group.is_none());
    assert_eq!("X-FOO", &prop.name);
    assert_eq!(
        &ValueType::DateTime,
        prop.parameters.as_ref().unwrap().value.as_ref().unwrap()
    );

    let expected = parse_date_time_list("20221107T2200")?;
    assert_eq!(&AnyProperty::DateTime(expected), &prop.value);

    assert_round_trip(&card)?;
    Ok(())
}

#[test]
fn extension_date_and_or_time() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
X-FOO;VALUE=date-and-or-time:19961022T140000
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    let prop = card.extensions.get(0).unwrap();

    assert!(prop.group.is_none());
    assert_eq!("X-FOO", &prop.name);
    assert_eq!(
        &ValueType::DateAndOrTime,
        prop.parameters.as_ref().unwrap().value.as_ref().unwrap()
    );

    let value = "19961022T140000";
    let expected: DateAndOrTime = value.parse()?;
    assert_eq!(&AnyProperty::DateAndOrTime(vec![expected]), &prop.value);

    assert_round_trip(&card)?;
    Ok(())
}

// TODO: timestamp
// TODO: boolean
// TODO: integer
// TODO: float
// TODO: utc-offset
// TODO: language-tag
