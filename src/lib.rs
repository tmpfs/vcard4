#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Compact, fast and correct vCard parser based
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
//! * `Mime`
//!
//! Serde support can be enabled by using the `serde` feature.

mod error;
mod parser;
mod values;

pub use error::Error;
pub use values::*;

pub use fluent_uri;
pub use language_tags;
pub use time;

/// Result type for the vCard library.
pub type Result<T> = std::result::Result<T, Error>;

/// Parse a vCard string into a collection of vCards.
pub fn parse<S: AsRef<str>>(input: S) -> Result<Vec<Vcard>> {
    let parser: parser::VcardParser = Default::default();
    parser.parse(input)
}
