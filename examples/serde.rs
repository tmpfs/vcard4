#[cfg(feature = "serde")]
pub fn main() -> anyhow::Result<()> {
    use vcard4::parse;

    let uri = "file:///images/jdoe.jpeg";

    let parsed = uri.parse::<vcard4::Uri>()?;

    const VCF: &str = include_str!("simon-perrault.vcf");

    let cards = parse(VCF)?;
    let card = cards.first().unwrap();
    print!("{}", serde_json::to_string_pretty(&card).unwrap());
    Ok(())
}

#[cfg(not(feature = "serde"))]
pub fn main() {
    panic!("serde feature is required");
}
