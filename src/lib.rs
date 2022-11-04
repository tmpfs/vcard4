//! Compact, fast and correct vCard parser based
//! on [RFC6350](https://www.rfc-editor.org/rfc/rfc6350).

mod error;

pub use error::Error;

/// Result type for the vCard library.
pub type Result<T> = std::result::Result<T, Error>;

use std::{borrow::Cow, ops::Range};
use logos::{Lexer, Logos};
use language_tags::LanguageTag;
use url::Url;

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

    #[regex("(?i:(LANGUAGE|VALUE|PREF|ALTID|PID|TYPE|MEDIATYPE|CALSCALE|SORT-AS|GEO|TZ)=)")]
    ParameterKey,

    #[token(":")]
    PropertyDelimiter,

    #[regex("\\r?\\n( |\\t)")]
    FoldedLine,

    #[token("\\,")]
    EscapedComma,

    #[token("\\;")]
    EscapedSemiColon,

    //#[token(",")]
    //Comma,

    #[regex("\\r?\\n")]
    NewLine,

    #[regex("(?i:END:VCARD)")]
    End,

    #[error]
    Text,
}

/// Parameters for a vCard property.
#[derive(Debug, Default)]
pub struct Parameters {
    /// The language tag.
    pub language: Option<LanguageTag>,
    /// The property types.
    pub types: Option<Vec<String>>,
}

/// Text property value.
#[derive(Debug)]
pub struct Text {
    pub value: String,
    pub parameters: Option<Parameters>,
}

/// URI property value.
#[derive(Debug)]
pub struct Uri {
    pub value: Url,
    pub parameters: Option<Parameters>,
}

/// The vCard type.
#[derive(Debug, Default)]
pub struct Vcard {
    pub formatted_name: Vec<Text>,
    pub nicknames: Vec<Text>,
    pub url: Vec<Uri>,
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
    fn parse_properties(
        &self, lex: &mut Lexer<'_, Token>, card: &mut Vcard) -> Result<()> {
            
        while let Some(first) = lex.next() {
            if first == Token::End {
                break;
            }

            self.assert_token(Some(first), Token::PropertyName)?;

            let name = lex.slice();

            let delimiter = lex.next();

            if let Some(delimiter) = delimiter {
                if delimiter == Token::ParameterDelimiter {
                    let parameters = self.parse_property_parameters(lex)?;
                    self.parse_property_by_name(lex, card, name, Some(parameters))?;
                } else if delimiter == Token::PropertyDelimiter {
                    self.parse_property_by_name(lex, card, name, None)?;
                } else {
                    return Err(Error::DelimiterExpected)
                }
            } else {
                return Err(Error::TokenExpected)
            }
        }

        Ok(())
    }

    /// Parse property parameters.
    fn parse_property_parameters(
        &self, lex: &mut Lexer<'_, Token>) -> Result<Parameters> {
        let mut params: Parameters = Default::default();

        let mut next: Option<Token> = lex.next();

        while let Some(token) = next.take() {
            if token == Token::PropertyDelimiter {
                break;
            }

            if token == Token::ParameterKey {
                let source = lex.source();
                let span = lex.span();
                let parameter_name = &source[span.start..(span.end - 1)];

                let upper_name = parameter_name.to_uppercase();

                let (value, next_token) = 
                    self.parse_property_parameters_value(lex)?;

                match &upper_name[..] {
                    "LANGUAGE" => {
                        let tag: LanguageTag = value.parse()?;
                        params.language = Some(tag);
                    }
                    "TYPE" => {
                        let types = value.split(",")
                            .map(|s| s.to_string()).collect::<Vec<_>>();
                        params.types = Some(types);
                    }
                    _ => return Err(Error::UnknownParameterName(parameter_name.to_string())),
                }

                if next_token == Token::PropertyDelimiter {
                    break;
                } else if next_token == Token::ParameterKey {
                    next = Some(next_token);
                } else {
                    next = lex.next();
                }
            }
        }
        Ok(params)
    }

    /// Parse the raw value for a property parameter.
    fn parse_property_parameters_value<'a>(
        &self, lex: &'a mut Lexer<'_, Token>) -> Result<(String, Token)> {
        let mut first_range: Option<Range<usize>> = None;

        while let Some(token) = lex.next() {
            let span = lex.span();
            if first_range.is_none() {
                first_range = Some(span.clone());
            }

            if token == Token::PropertyDelimiter
                || token == Token::ParameterDelimiter
                || token == Token::ParameterKey {
                let source = lex.source();
                let value = &source[first_range.unwrap().start..span.start];
                return Ok((String::from(value), token));
            }
        }
        Err(Error::TokenExpected)
    }

    /// Parse a property by name.
    fn parse_property_by_name(
        &self,
        lex: &mut Lexer<'_, Token>,
        card: &mut Vcard,
        name: &str,
        parameters: Option<Parameters>,
    ) -> Result<()> {
        let value = self.parse_property_value(lex)?;
        let upper_name = name.to_uppercase();
        match &upper_name[..] {
            "FN" => {
                card.formatted_name.push(Text { value: value.into_owned(), parameters });
            }
            "NICKNAME" => {
                card.nicknames.push(Text { value: value.into_owned(), parameters });
            }
            "URL" => {
                let url: Url = value.as_ref().parse()?;
                card.url.push(Uri { value: url, parameters });
            }
            _ => return Err(Error::UnknownPropertyName(name.to_string())),
        }
        Ok(())
    }

    /// Get the slice for the property value.
    fn parse_property_value<'a>(&self, lex: &'a mut Lexer<'_, Token>) -> Result<Cow<'a, str>> {
        let mut first_range: Option<Range<usize>> = None;
        let mut last_range: Option<Range<usize>> = None;

        let mut needs_transform = false;
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
                needs_transform = true;
            }

            if token == Token::NewLine {
                last_range = Some(span.clone());
                break;
            }

            tokens.push((token, span));
        }

        if let (Some(first), Some(last)) = (first_range, last_range) {
            if needs_transform {
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
        assert_eq!("Mr. John Q. Public, Esq.", fname.value);
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
        assert_eq!("Mr. John Q. Public; Esq.", fname.value);
        Ok(())
    }

    #[test]
    fn parse_folded_space() -> Result<()> {
        let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. 
 John Q. 
 Public\, 
 Esq.
END:VCARD"#;
        let mut vcards = parse(input)?;
        assert_eq!(1, vcards.len());

        let card = vcards.remove(0);
        let fname = card.formatted_name.get(0).unwrap();
        assert_eq!("Mr. John Q. Public, Esq.", fname.value);
        Ok(())
    }

    #[test]
    fn parse_folded_tab() -> Result<()> {
        let input = "BEGIN:VCARD\nVERSION:4.0\nFN:Mr. \n\u{0009}John Q. \n\u{0009}Public\\, \n\u{0009}Esq.\nEND:VCARD";
        
        let mut vcards = parse(input)?;
        assert_eq!(1, vcards.len());

        let card = vcards.remove(0);
        let fname = card.formatted_name.get(0).unwrap();
        assert_eq!("Mr. John Q. Public, Esq.", fname.value);
        Ok(())
    }

    #[test]
    fn parse_parameters() -> Result<()> {
        let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mr. John Q. Public\, Esq.
NICKNAME;LANGUAGE=en;TYPE=work:Boss
END:VCARD"#;
        let mut vcards = parse(input)?;
        assert_eq!(1, vcards.len());

        let card = vcards.remove(0);

        let fname = card.formatted_name.get(0).unwrap();
        assert_eq!("Mr. John Q. Public, Esq.", fname.value);

        let nickname = card.nicknames.get(0).unwrap();
        assert_eq!("Boss", nickname.value);
        assert!(nickname.parameters.is_some());

        let tag: LanguageTag = "en".parse()?;
        let parameters = nickname.parameters.as_ref().unwrap();

        assert_eq!(Some(tag), parameters.language);
        assert_eq!(
            &vec![String::from("work")],
            parameters.types.as_ref().unwrap());
        Ok(())
    }

    #[test]
    fn parse_url() -> Result<()> {
        let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mock person
URL: https://example.com
END:VCARD"#;
        let mut vcards = parse(input)?;
        assert_eq!(1, vcards.len());

        let card = vcards.remove(0);

        let uri: Url = "https://example.com".parse()?;
        let url = card.url.get(0).unwrap();
        assert_eq!(uri, url.value);

        Ok(())
    }
}
