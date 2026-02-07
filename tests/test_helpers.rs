use anyhow::Result;

use vcard4::{Vcard, parameter::Parameters, parse};

#[cfg(feature = "language-tags")]
use language_tags::LanguageTag;

#[allow(dead_code)]
pub fn assert_round_trip(card: &Vcard) -> Result<()> {
    let encoded = card.to_string();
    let mut cards = parse(&encoded)?;
    let decoded = cards.remove(0);
    assert_eq!(card, &decoded);
    assert_serde_round_trip(card)?;
    Ok(())
}

#[cfg(feature = "serde")]
#[allow(dead_code)]
pub fn assert_serde_round_trip(card: &Vcard) -> Result<()> {
    let data = serde_json::to_string_pretty(card)?;
    //println!("{}", data);
    let decoded: Vcard = serde_json::from_str(&data)?;
    assert_eq!(card, &decoded);
    Ok(())
}

#[cfg(not(feature = "serde"))]
#[allow(dead_code)]
pub fn assert_serde_round_trip(_card: &Vcard) -> Result<()> {
    Ok(())
}

#[cfg(feature = "mime")]
#[allow(dead_code)]
pub fn assert_media_type(
    parameters: Option<&Parameters>,
    expected: &str,
) -> Result<()> {
    use mime::Mime;
    let params = parameters.unwrap();
    let expected: Mime = expected.parse()?;
    assert_eq!(&expected, params.media_type.as_ref().unwrap());
    Ok(())
}

#[cfg(not(feature = "mime"))]
#[allow(dead_code)]
pub fn assert_media_type(
    parameters: Option<&Parameters>,
    expected: &str,
) -> Result<()> {
    let params = parameters.unwrap();
    assert_eq!(expected, params.media_type.as_ref().unwrap());
    Ok(())
}

#[cfg(feature = "language-tags")]
#[allow(dead_code)]
pub fn assert_language(value: &LanguageTag, expected: &str) -> Result<()> {
    let expected: LanguageTag = expected.parse()?;
    assert_eq!(&expected, value);
    Ok(())
}

#[cfg(not(feature = "language-tags"))]
#[allow(dead_code)]
pub fn assert_language(value: &str, expected: &str) -> Result<()> {
    assert_eq!(expected, value);
    Ok(())
}
