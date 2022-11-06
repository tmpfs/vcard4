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
//! Certain external types cannot be zeroize'd and are therefore exempt:
//!
//! * `LanguageTag`
//! * `Uri`
//! * `UtcOffset`
//! * `OffsetDateTime`
//! * `Time`
//! * `Mime`
//!
//! Serde support can be enabled by using the `serde` feature.
//!
//! If the `mime` feature is enabled the MEDIATYPE parameter is parsed 
//! to a `Mime` struct otherwise it is a `String`.

mod error;
pub mod parameters;
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
