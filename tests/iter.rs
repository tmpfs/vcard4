use anyhow::Result;
use vcard4::iter;

#[test]
fn iter_one() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
END:VCARD"#;
    let mut it = iter(input, true);
    assert!(matches!(it.next(), Some(Ok(_))));
    assert!(matches!(it.next(), None));
    Ok(())
}

#[test]
fn iter_many() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
END:VCARD
BEGIN:VCARD
VERSION:4.0
FN:John Doe
END:VCARD"#;
    let mut it = iter(input, true);
    assert!(matches!(it.next(), Some(Ok(_))));
    assert!(matches!(it.next(), Some(Ok(_))));
    assert!(matches!(it.next(), None));
    Ok(())
}

#[test]
fn iter_error() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0"#;
    let mut it = iter(input, true);
    assert!(matches!(it.next(), Some(Err(_))));
    Ok(())
}

#[test]
fn iter_error_expected() -> Result<()> {
    let input = r#""#;
    let mut it = iter(input, true);
    assert!(matches!(it.next(), None));
    Ok(())
}
