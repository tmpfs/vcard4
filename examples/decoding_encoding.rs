use anyhow::Result;
use vcard4::parse;

pub fn main() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:John Doe
NICKNAME:Johnny
END:VCARD"#;
    let cards = parse(input)?;
    let card = cards.first().unwrap();
    let encoded = card.to_string();
    let decoded = parse(&encoded)?.remove(0);
    assert_eq!(card, &decoded);
    Ok(())
}
