use anyhow::Result;
use vcard4::Vcard;

#[test]
fn validate() -> Result<()> {
    let card: Vcard = Default::default();
    assert!(card.validate().is_err());
    Ok(())
}
