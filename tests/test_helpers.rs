use anyhow::Result;

use vcard_compact::{parse, Vcard};

#[allow(dead_code)]
pub fn assert_round_trip(card: &Vcard) -> Result<()> {
    let encoded = card.to_string();
    let mut cards = parse(&encoded)?;
    let decoded = cards.remove(0);
    assert_eq!(card, &decoded);
    Ok(())
}
