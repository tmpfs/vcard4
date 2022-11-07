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

mod error;
mod name;
pub mod parameter;
mod parser;
pub mod property;
#[cfg(feature = "serde")]
mod serde;
pub mod types;
mod vcard;

pub use error::Error;
pub use vcard::Vcard;

pub use time;
pub use uriparse;

/// Result type for the vCard library.
pub type Result<T> = std::result::Result<T, Error>;

/// Parse a vCard string into a collection of vCards.
pub fn parse<S: AsRef<str>>(input: S) -> Result<Vec<Vcard>> {
    let parser = parser::VcardParser::new(true);
    parser.parse(input)
}

/// Parse a vCard string into a collection of vCards ignoring properties
/// that generate errors.
pub fn parse_loose<S: AsRef<str>>(input: S) -> Result<Vec<Vcard>> {
    let parser = parser::VcardParser::new(false);
    parser.parse(input)
}
