mod test_helpers;

use anyhow::Result;
use test_helpers::assert_round_trip;
use vcard4::parse;

#[test]
fn delivery_adr() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
ADR;GEO="geo:12.3457,78.910";LABEL="Mr. John Q. Public, Esq.\n
 Mail Drop: TNE QB\n123 Main Street\nAny Town, CA  91921-1234\n
 U.S.A.":;;123 Main Street;Any Town;CA;91921-1234;U.S.A.
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());
    let card = vcards.remove(0);

    let prop = card.address.get(0).unwrap();
    let geo = prop.parameters.as_ref().unwrap().geo.as_ref().unwrap();
    assert_eq!("geo:12.3457,78.910", &geo.to_string());

    let label = prop.parameters.as_ref().unwrap().label.as_ref().unwrap();
    assert_eq!(
        r#"Mr. John Q. Public, Esq.
Mail Drop: TNE QB
123 Main Street
Any Town, CA  91921-1234
U.S.A."#,
        label
    );

    let address = &prop.value;
    assert!(address.po_box.is_none());
    assert!(address.extended_address.is_none());
    assert_eq!("123 Main Street", address.street_address.as_ref().unwrap());
    assert_eq!("Any Town", address.locality.as_ref().unwrap());
    assert_eq!("CA", address.region.as_ref().unwrap());
    assert_eq!("91921-1234", address.postal_code.as_ref().unwrap());
    assert_eq!("U.S.A.", address.country_name.as_ref().unwrap());

    assert_round_trip(&card)?;
    Ok(())
}
