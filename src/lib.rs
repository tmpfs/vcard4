//! Compact, fast and correct vCard parser based
//! on [RFC6350](https://www.rfc-editor.org/rfc/rfc6350).

mod error;

pub use error::Error;

/// Result type for the vCard library.
pub type Result<T> = std::result::Result<T, Error>;

use logos::{Lexer, Logos};
use std::{borrow::Cow, ops::Range};

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex("(?i:BEGIN:VCARD)")]
    Begin,

    #[regex("(?i:VERSION:4\\.0)")]
    Version,

    #[regex("(?i:SOURCE|KIND|FN|N|NICKNAME|PHOTO|BDAY|ANNIVERSARY|GENDER|ADR|TEL|EMAIL|IMPP|LANG|TZ|GEO|TITLE|ROLE|LOGO|ORG|MEMBER|RELATED|CATEGORIES|NOTE|PRODID|REV|SOUND|UID|CLIENTPIDMAP|URL|KEY|FBURL|CALADRURI|CALURI|XML)")]
    PropertyName,

    #[token(";")]
    ParameterDelimiter,

    #[token(":")]
    PropertyDelimiter,

    #[regex("\\r?\\n( |\\t)")]
    FoldedLine,

    #[token("\\,")]
    EscapedComma,

    #[token("\\;")]
    EscapedSemiColon,

    #[regex("\\r?\\n")]
    NewLine,

    #[regex("(?i:END:VCARD)")]
    End,

    #[error]
    Error,
}

/// Value of a property.
pub enum Value {
    Text(String),
}

/// Property of a vCard.
#[derive(Debug)]
pub struct Property {}

/// Single vCard with a collection of properties.
#[derive(Debug, Default)]
pub struct Vcard {
    formatted_name: Vec<String>,
}

/// Parses vCards from strings.
struct VcardParser {}

impl VcardParser {
    /// Create a new vCard parser.
    fn new() -> Self {
        Self {}
    }

    /// Parse a UTF-8 encoded string into a list of vCards.
    fn parse<S: AsRef<str>>(&self, value: S) -> Result<Vec<Vcard>> {
        let mut cards = Vec::new();
        let mut lex = Token::lexer(value.as_ref());

        while let Some(first) = lex.next() {
            let card = self.parse_one(&mut lex, Some(first))?;
            cards.push(card);
        }

        if cards.is_empty() {
            return Err(Error::TokenExpected);
        }

        Ok(cards)
    }

    /// Parse a single vCard.
    fn parse_one(&self, lex: &mut Lexer<'_, Token>, first: Option<Token>) -> Result<Vcard> {
        self.assert_token(first, Token::Begin)?;
        self.assert_token(lex.next(), Token::NewLine)?;

        let version = lex.next();
        self.assert_token(version, Token::Version)?;
        self.assert_token(lex.next(), Token::NewLine)?;

        let mut card: Vcard = Default::default();

        self.parse_properties(lex, &mut card)?;

        Ok(card)
    }

    /// Parse the properties of a vCard.
    fn parse_properties(&self, lex: &mut Lexer<'_, Token>, card: &mut Vcard) -> Result<()> {
            
        while let Some(first) = lex.next() {
            if first == Token::End {
                break;
            }

            self.assert_token(Some(first), Token::PropertyName)?;

            let name = lex.slice();
            let next = lex.next();
            self.assert_token(next, Token::PropertyDelimiter)?;

            self.parse_property_by_name(lex, card, name);
        }

        //let first = lex.next();
        Ok(())
    }

    /// Parse a property by name.
    fn parse_property_by_name(
        &self,
        lex: &mut Lexer<'_, Token>,
        card: &mut Vcard,
        name: &str,
    ) -> Result<()> {
        let value = self.parse_property_value(lex)?;
        let upper_name = name.to_uppercase();
        let value = match &upper_name[..] {
            "FN" => {
                card.formatted_name.push(value.into_owned());
            }
            _ => return Err(Error::UnknownPropertyName(name.to_string())),
        };
        Ok(())
    }

    /// Get the slice for the property value.
    fn parse_property_value<'a>(&self, lex: &'a mut Lexer<'_, Token>) -> Result<Cow<'a, str>> {
        let mut first_range: Option<Range<usize>> = None;
        let mut last_range: Option<Range<usize>> = None;

        let mut needs_folding = false;
        let mut tokens = Vec::new();

        while let Some(token) = lex.next() {
            let span = lex.span();
            if first_range.is_none() {
                first_range = Some(span.clone());
            }

            if token == Token::FoldedLine
                || token == Token::EscapedSemiColon
                || token == Token::EscapedComma
            {
                needs_folding = true;
            }

            if token == Token::NewLine {
                last_range = Some(span.clone());
                break;
            }

            tokens.push((token, span));
        }

        if let (Some(first), Some(last)) = (first_range, last_range) {
            if needs_folding {
                let mut value = String::new();
                for (token, span) in tokens {
                    if token == Token::FoldedLine {
                        continue;
                    }
                    if token == Token::EscapedComma {
                        value.push(',');
                        continue;
                    }
                    if token == Token::EscapedSemiColon {
                        value.push(';');
                        continue;
                    }

                    let source = lex.source();
                    value.push_str(&source[span]);
                }
                Ok(Cow::Owned(value))
            } else {
                let source = lex.source();
                Ok(Cow::Borrowed(&source[first.start..last.start]))
            }
        } else {
            Err(Error::InvalidPropertyValue)
        }
    }

    /// Assert we have an expected token.
    fn assert_token(&self, value: Option<Token>, expected: Token) -> Result<()> {
        if let Some(value) = value {
            if value == expected {
                Ok(())
            } else {
                //println!("{:#?}", value);
                //println!("{:#?}", expected);
                Err(Error::IncorrectToken)
            }
        } else {
            Err(Error::TokenExpected)
        }
    }
}

/// Parse a vCard string into a collection of vCards.
pub fn parse<S: AsRef<str>>(input: S) -> Result<Vec<Vcard>> {
    let parser = VcardParser::new();
    parser.parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn parse_empty() -> Result<()> {
        let result = parse("");
        if !matches!(result, Err(Error::TokenExpected)) {
            panic!("wrong error variant");
        }
        Ok(())
    }

    #[test]
    fn parse_wrong_token() -> Result<()> {
        let result = parse("VERSION:4.0");
        if !matches!(result, Err(Error::IncorrectToken)) {
            panic!("wrong error variant");
        }
        Ok(())
    }

    #[test]
    fn parse_no_version() -> Result<()> {
        let input = r#"BEGIN:VCARD"#;
        let result = parse(input);
        if !matches!(result, Err(Error::TokenExpected)) {
            panic!("wrong error variant");
        }
        Ok(())
    }

    #[test]
    fn parse_no_end() -> Result<()> {
        let input = r#"BEGIN:VCARD
VERSION:4.0"#;
        let result = parse(input);
        if !matches!(result, Err(Error::TokenExpected)) {
            panic!("wrong error variant");
        }
        Ok(())
    }

    #[test]
    fn parse_escaped_comma() -> Result<()> {
        let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
END:VCARD"#;
        let mut vcards = parse(input)?;
        assert_eq!(1, vcards.len());

        let card = vcards.remove(0);
        let fname = card.formatted_name.get(0).unwrap();
        assert_eq!("Mr. John Q. Public, Esq.", fname);
        Ok(())
    }

    #[test]
    fn parse_escaped_semi_colon() -> Result<()> {
        let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\; Esq.
END:VCARD"#;
        let mut vcards = parse(input)?;
        assert_eq!(1, vcards.len());

        let card = vcards.remove(0);
        let fname = card.formatted_name.get(0).unwrap();
        assert_eq!("Mr. John Q. Public; Esq.", fname);
        Ok(())
    }
}
