#![forbid(unsafe_code)]
//#![deny(missing_docs)]

//! Compact, fast and correct vCard parser based
//! on [RFC6350](https://www.rfc-editor.org/rfc/rfc6350).

mod error;
mod parser;
mod values;

pub use error::Error;
pub use values::*;

/// Result type for the vCard library.
pub type Result<T> = std::result::Result<T, Error>;

/// Parse a vCard string into a collection of vCards.
pub fn parse<S: AsRef<str>>(input: S) -> Result<Vec<Vcard>> {
    let parser: parser::VcardParser = Default::default();
    parser.parse(input)
}
