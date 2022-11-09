use anyhow::Result;
use vcard4::iter;

pub fn main() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:John Doe
END:VCARD

BEGIN:VCARD
VERSION:4.0
FN:Jane Doe
END:VCARD"#;
    let mut it = iter(input, true);
    print!("{}", it.next().unwrap()?);
    print!("{}", it.next().unwrap()?);
    assert!(matches!(it.next(), None));
    Ok(())
}
