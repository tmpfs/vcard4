#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Fast and correct vCard parser based
//! on [RFC6350](https://www.rfc-editor.org/rfc/rfc6350).
//!
//! vCards inherently contain private information so this library
//! implements a `zeroize` feature (which is enabled by default) to
//! securely zero the memory for all the data in a vCard when it is
//! dropped.
//!
//! Certain external types cannot be zeroize'd due to the restrictions on
//! implementing external traits on external types and are therefore exempt:
//!
//! * `Uri`
//! * `Time` / `UtcOffset` / `OffsetDateTime`
//! * `LanguageTag` (feature: `language-tags`)
//! * `Mime` (feature: `mime`)
//!
//! If the `mime` feature is enabled the MEDIATYPE parameter is parsed
//! to a `Mime` struct otherwise it is a `String`.
//!
//! If the `language-tags` feature is enabled the LANG property
//! and the LANGUAGE parameter are parsed using the
//! [language-tags](https://docs.rs/language-tags/latest/language_tags/) crate.
//!
//! Serde support can be enabled with the `serde` feature.
//!
//! ## Implementation
//!
//! * The `XML` property is parsed and propagated but it is not
//!   validated per RFC as it is optional.
//! * IANA Tokens are not implemented.
//!
//! ## Examples
//!
//! Create a new vCard:
//!
//! ```
//! use vcard4::Vcard;
//! let mut card = Vcard::new(String::from("John Doe"));
//! card.nickname.push(String::from("Johnny").into());
//! print!("{}", card);
//! ```
//!
//! Decoding and encoding:
//!
//! ```
//! use anyhow::Result;
//! use vcard4::parse;
//! pub fn main() -> Result<()> {
//!     let input = r#"BEGIN:VCARD
//! VERSION:4.0
//! FN:John Doe
//! NICKNAME:Johnny
//! END:VCARD"#;
//!     let cards = parse(input)?;
//!     let card = cards.first().unwrap();
//!     let encoded = card.to_string();
//!     let decoded = parse(&encoded)?.remove(0);
//!     assert_eq!(card, &decoded);
//!     Ok(())
//! }
//! ```
//!
//! Iterative parsing is useful if you only need the first vCard or
//! wish to ignore vCards that have errors (possibly during an
//! import operation):
//!
//! ```
//! use anyhow::Result;
//! use vcard4::iter;
//!
//! pub fn main() -> Result<()> {
//!     let input = r#"BEGIN:VCARD
//! VERSION:4.0
//! FN:John Doe
//! END:VCARD
//!
//! BEGIN:VCARD
//! VERSION:4.0
//! FN:Jane Doe
//! END:VCARD"#;
//!     let mut it = iter(input, true);
//!     print!("{}", it.next().unwrap()?);
//!     print!("{}", it.next().unwrap()?);
//!     assert!(matches!(it.next(), None));
//!     Ok(())
//! }
//! ```

mod error;
mod iter;
mod name;
pub mod parameter;
mod parser;
pub mod property;
#[cfg(feature = "serde")]
mod serde;
pub mod types;
mod vcard;

pub use error::Error;
pub use iter::VcardIterator;
pub use vcard::Vcard;

pub use time;
pub use uriparse;

/// Result type for the vCard library.
pub type Result<T> = std::result::Result<T, Error>;

/// Parse a vCard string into a collection of vCards.
pub fn parse<S: AsRef<str>>(input: S) -> Result<Vec<Vcard>> {
    let parser = parser::VcardParser::new(input.as_ref(), true);
    parser.parse()
}

/// Parse a vCard string into a collection of vCards ignoring properties
/// that generate errors.
pub fn parse_loose<S: AsRef<str>>(input: S) -> Result<Vec<Vcard>> {
    let parser = parser::VcardParser::new(input.as_ref(), false);
    parser.parse()
}

/// Create a parser iterator.
pub fn iter(source: &str, strict: bool) -> VcardIterator<'_> {
    VcardIterator::new(source, strict)
}

/// Helper for escaping values.
pub(crate) fn escape_value(value: &str, semi_colons: bool) -> String {
    use aho_corasick::AhoCorasick;
    if semi_colons {
        let patterns = &["\\", "\n", ",", ";"];
        let replace_with = &["\\\\", "\\n", "\\,", "\\;"];
        let ac = AhoCorasick::new(patterns);
        ac.replace_all(value, replace_with)
    } else {
        let patterns = &["\\", "\n", ","];
        let replace_with = &["\\\\", "\\n", "\\,"];
        let ac = AhoCorasick::new(patterns);
        ac.replace_all(value, replace_with)
    }
}

pub(crate) fn unescape_value(value: &str) -> String {
    use aho_corasick::AhoCorasick;
    let patterns = &["\r", "\n ", "\n\t", "\\n", "\\N", "\\,"];
    let replace_with = &["", "", "", "\n", "\n", ","];
    let ac = AhoCorasick::new(patterns);
    ac.replace_all(value, replace_with)
}

pub(crate) fn escape_control(value: &str) -> String {
    let values = value
        .chars()
        .map(|c| c.escape_unicode().to_string())
        .collect::<Vec<_>>();
    values.join("")
}
