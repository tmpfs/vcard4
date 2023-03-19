mod test_helpers;

use anyhow::Result;
use vcard4::parse;

use test_helpers::assert_round_trip;

#[test]
fn parse_version3() -> Result<()> {
    let input = r#"BEGIN:VCARD
VERSION:3.0
PRODID:-//Apple Inc.//macOS 12.6.3//EN
N:;Mock;;;
FN:Mock
TEL;type=CELL;type=VOICE;type=pref:01234567890
END:VCARD"#;

    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_round_trip(&card)?;
    Ok(())
}
