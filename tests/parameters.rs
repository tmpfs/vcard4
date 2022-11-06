mod test_helpers;

use anyhow::Result;

//#[cfg(feature = "language-tags")]
//use language_tags::LanguageTag;

use vcard_compact::{parameter::TypeParameter, parse};

use test_helpers::assert_round_trip;

#[test]
fn parse_parameters() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
NICKNAME;LANGUAGE=en;TYPE=work:Boss
END:VCARD"#;
    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);

    let fname = card.formatted_name.get(0).unwrap();
    assert_eq!("Mr. John Q. Public, Esq.", fname.value);

    let nickname = card.nickname.get(0).unwrap();
    assert_eq!("Boss", nickname.value);
    assert!(nickname.parameters.is_some());

    //let tag: LanguageTag = "en".parse()?;
    let parameters = nickname.parameters.as_ref().unwrap();

    let param: TypeParameter = "work".parse()?;
    //assert_eq!(Some(tag), parameters.language);
    assert_eq!(&vec![param], parameters.types.as_ref().unwrap());
    assert_round_trip(&card)?;
    Ok(())
}
