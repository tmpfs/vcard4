use anyhow::Result;
use vcard4::parse;

const VCF: &str = include_str!("simon-perrault.vcf");

pub fn main() -> Result<()> {
    let cards = parse(VCF)?;
    let card = cards.first().unwrap();
    print!("{}", card);
    Ok(())
}
