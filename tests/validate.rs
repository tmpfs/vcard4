use anyhow::Result;
use vcard_compact::Vcard;

#[test]
fn validate() -> Result<()> {
    let card: Vcard = Default::default();
    assert!(card.validate().is_err());
    Ok(())
}
