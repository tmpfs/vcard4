mod test_helpers;

use anyhow::Result;
use vcard4::parse;

use test_helpers::assert_round_trip;

#[test]
fn parse_photo() -> Result<()> {
    let input = include_str!("../fixtures/photo.vcf");

    let mut vcards = parse(input)?;
    assert_eq!(1, vcards.len());

    let card = vcards.remove(0);
    assert_round_trip(&card)?;
    Ok(())
}
