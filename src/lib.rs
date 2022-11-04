//! Compact, fast and correct vCard parser based
//! on [RFC6350](https://www.rfc-editor.org/rfc/rfc6350).

mod error;

pub use error::Error;

/// Result type for the vCard library.
pub type Result<T> = std::result::Result<T, Error>;

use language_tags::LanguageTag;
use logos::{Lexer, Logos};
use std::{
    borrow::Cow,
    fmt::{self, Debug},
    ops::Range,
    str::FromStr,
};
use fluent_uri::{Uri as URI};

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

/// Either text or a URI.
#[derive(Debug)]
pub enum TextOrUri {
    Text(Text),
    Uri(Uri),
}

/// Enumeration of the different types of values.
#[derive(Debug)]
pub enum ValueType {
    /// Text value.
    Text,
    /// URI value.
    Uri,
    /// Date value.
    Date,
    /// Time value.
    Time,
    /// Date and time value.
    DateTime,
    /// Date and or time value.
    DateAndOrTime,
    /// Timestamp value.
    Timestamp,
    /// Boolean value.
    Boolean,
    /// Integer value.
    Integer,
    /// Float value.
    Float,
    /// UTC offset value.
    UtcOffset,
    /// Language tag value.
    LanguageTag,
    /*
    /// IANA token value.
    IanaToken,
    /// X-name value.
    XName,
    */
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Text => "text",
                Self::Uri => "uri",
                Self::Date => "date",
                Self::Time => "time",
                Self::DateTime => "date-time",
                Self::DateAndOrTime => "date-and-or-time",
                Self::Timestamp => "timestamp",
                Self::Boolean => "boolean",
                Self::Integer => "integer",
                Self::Float => "float",
                Self::UtcOffset => "utc-offset",
                Self::LanguageTag => "language-tag",
            }
        )
    }
}

impl FromStr for ValueType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "text" => Ok(Self::Text),
            "uri" => Ok(Self::Uri),
            "date" => Ok(Self::Date),
            "time" => Ok(Self::Time),
            "date-time" => Ok(Self::DateTime),
            "date-and-or-time" => Ok(Self::DateAndOrTime),
            "timestamp" => Ok(Self::Timestamp),
            "boolean" => Ok(Self::Boolean),
            "integer" => Ok(Self::Integer),
            "float" => Ok(Self::Float),
            "utc-offset" => Ok(Self::UtcOffset),
            "language-tag" => Ok(Self::LanguageTag),
            _ => Err(Error::UnknownValueType(s.to_string())),
        }
    }
}

/// Kind of vCard.
#[derive(Debug)]
pub enum Kind {
    /// An individual.
    Individual,
    /// A group.
    Group,
    /// An organization.
    Org,
    /// A location.
    Location,

    // TODO: x-name
    // TODO: iana-token
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Individual => "individual",
                Self::Group => "group",
                Self::Org => "org",
                Self::Location => "location",
            }
        )
    }
}

impl FromStr for Kind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "individual" => Ok(Self::Individual),
            "group" => Ok(Self::Group),
            "org" => Ok(Self::Org),
            "location" => Ok(Self::Location),
            _ => Err(Error::UnknownKind(s.to_string())),
        }
    }
}

/// Parameters for a vCard property.
#[derive(Debug, Default)]
pub struct Parameters {
    /// The language tag.
    pub language: Option<LanguageTag>,
    /// The property types.
    pub types: Option<Vec<String>>,
    /// The value type hint for this property.
    pub value: Option<ValueType>,
}

/// Text property value.
#[derive(Debug)]
pub struct Text {
    pub value: String,
    pub parameters: Option<Parameters>,
}

/// Text list property value.
#[derive(Debug)]
pub struct TextList {
    pub value: Vec<String>,
    pub parameters: Option<Parameters>,
}

/// URI property value.
#[derive(Debug)]
pub struct Uri {
    pub value: URI<String>,
    pub parameters: Option<Parameters>,
}

/// The vCard type.
#[derive(Debug, Default)]
pub struct Vcard {
    // Organizational
    pub source: Vec<Uri>,
    pub kind: Option<Kind>,
    pub xml: Vec<Text>,

    // Identification
    pub formatted_name: Vec<Text>,
    pub name: Option<TextList>,
    pub nickname: Vec<Text>,
    pub photo: Vec<Uri>,
    pub url: Vec<Uri>,

    // Organizational
    pub title: Vec<Text>,
    pub role: Vec<Text>,
    pub logo: Vec<Uri>,
    pub org: Vec<TextList>,
    pub member: Vec<Uri>,
    pub related: Vec<TextOrUri>,

    // Explanatory
    pub categories: Vec<TextList>,
    pub note: Vec<Text>,
    //pub prod_id: Option<Text>,

    //pub rev: Option<Text>,
    pub sound: Vec<Uri>,
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
    fn parse_one(
        &self,
        lex: &mut Lexer<'_, Token>,
        first: Option<Token>,
    ) -> Result<Vcard> {
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
        &self,
        lex: &mut Lexer<'_, Token>,
        card: &mut Vcard,
    ) -> Result<()> {
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
                    self.parse_property_by_name(
                        lex,
                        card,
                        name,
                        Some(parameters),
                    )?;
                } else if delimiter == Token::PropertyDelimiter {
                    self.parse_property_by_name(lex, card, name, None)?;
                } else {
                    return Err(Error::DelimiterExpected);
                }
            } else {
                return Err(Error::TokenExpected);
            }
        }

        Ok(())
    }

    /// Parse property parameters.
    fn parse_property_parameters(
        &self,
        lex: &mut Lexer<'_, Token>,
    ) -> Result<Parameters> {
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
                        let types = value
                            .split(",")
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>();
                        params.types = Some(types);
                    }
                    "VALUE" => {
                        let value: ValueType = value.parse()?;
                        params.value = Some(value);
                    }
                    _ => {
                        return Err(Error::UnknownParameterName(
                            parameter_name.to_string(),
                        ))
                    }
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
        &self,
        lex: &'a mut Lexer<'_, Token>,
    ) -> Result<(String, Token)> {
        let mut first_range: Option<Range<usize>> = None;

        while let Some(token) = lex.next() {
            let span = lex.span();
            if first_range.is_none() {
                first_range = Some(span.clone());
            }

            if token == Token::PropertyDelimiter
                || token == Token::ParameterDelimiter
                || token == Token::ParameterKey
            {
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
            // General properties
            // https://www.rfc-editor.org/rfc/rfc6350#section-6.1
            "SOURCE" => {
                let value = URI::parse(value.as_ref())?.to_owned();
                card.source.push(Uri { value, parameters });
            }
            "KIND" => {
                if card.kind.is_some() {
                    return Err(Error::OnlyOnce(String::from("KIND")));
                }
                let value: Kind = value.as_ref().parse()?;
                card.kind = Some(value);
            }
            "XML" => {
                card.xml.push(Text {
                    value: value.into_owned(),
                    parameters,
                });
            }
            // Identification properties
            // https://www.rfc-editor.org/rfc/rfc6350#section-6.2
            "FN" => {
                card.formatted_name.push(Text {
                    value: value.into_owned(),
                    parameters,
                });
            }
            "N" => {
                if card.name.is_some() {
                    return Err(Error::OnlyOnce(String::from("N")));
                }
                let value = value
                    .as_ref()
                    .split(";")
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                card.name = Some(TextList { value, parameters });
            }
            "NICKNAME" => {
                card.nickname.push(Text {
                    value: value.into_owned(),
                    parameters,
                });
            }
            "PHOTO" => {
                let value = URI::parse(value.as_ref())?.to_owned();
                card.photo.push(Uri { value, parameters });
            }
            "URL" => {
                let value = URI::parse(value.as_ref())?.to_owned();
                card.url.push(Uri { value, parameters });
            }

            // Organizational
            // https://www.rfc-editor.org/rfc/rfc6350#section-6.6
            "TITLE" => {
                card.title.push(Text {
                    value: value.into_owned(),
                    parameters,
                });
            }
            "ROLE" => {
                card.role.push(Text {
                    value: value.into_owned(),
                    parameters,
                });
            }

            "LOGO" => {
                let value = URI::parse(value.as_ref())?.to_owned();
                card.logo.push(Uri { value, parameters });
            }
            "ORG" => {
                let value = value
                    .as_ref()
                    .split(";")
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                card.org.push(TextList {
                    value,
                    parameters,
                });
            }
            "MEMBER" => {
                let value = URI::parse(value.as_ref())?.to_owned();
                card.member.push(Uri { value, parameters });
            }
            "RELATED" => {
                let value_type = if let Some(parameters) = &parameters {
                    parameters.value.as_ref()
                } else {
                    None
                };

                if let Some(value_type) = value_type {
                    if let ValueType::Text = value_type {
                        card.related.push(TextOrUri::Text(Text {
                            value: value.as_ref().to_string(),
                            parameters,
                        }));
                    } else if let ValueType::Uri = value_type {
                        let value = URI::parse(value.as_ref())?.to_owned();
                        card.related
                            .push(TextOrUri::Uri(Uri { value, parameters }));
                    } else {
                        return Err(Error::UnknownValueType(
                            value_type.to_string(),
                        ));
                    }
                } else {
                    match URI::parse(value.as_ref()) {
                        Ok(value) => {
                            card.related.push(TextOrUri::Uri(Uri {
                                value: value.to_owned(),
                                parameters,
                            }));
                        }
                        Err(_) => {
                            card.related.push(TextOrUri::Text(Text {
                                value: value.as_ref().to_string(),
                                parameters,
                            }));
                        }
                    }
                }
            }

            // Explanatory
            // https://www.rfc-editor.org/rfc/rfc6350#section-6.7
            "CATEGORIES" => {
                let value = value
                    .as_ref()
                    .split(",")
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                card.categories.push(TextList {
                    value,
                    parameters,
                });
            }
            "NOTE" => {
                card.note.push(Text {
                    value: value.into_owned(),
                    parameters,
                });
            }
            "PRODID" => {
                todo!()
            }
            "REV" => {
                todo!()
            }
            "SOUND" => {
                let value = URI::parse(value.as_ref())?.to_owned();
                card.sound.push(Uri { value, parameters });
            }

            _ => return Err(Error::UnknownPropertyName(name.to_string())),
        }
        Ok(())
    }

    /// Get the slice for the property value.
    fn parse_property_value<'a>(
        &self,
        lex: &'a mut Lexer<'_, Token>,
    ) -> Result<Cow<'a, str>> {
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
    fn assert_token(
        &self,
        value: Option<Token>,
        expected: Token,
    ) -> Result<()> {
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
    use fluent_uri::{Uri as URI};

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

        let nickname = card.nickname.get(0).unwrap();
        assert_eq!("Boss", nickname.value);
        assert!(nickname.parameters.is_some());

        let tag: LanguageTag = "en".parse()?;
        let parameters = nickname.parameters.as_ref().unwrap();

        assert_eq!(Some(tag), parameters.language);
        assert_eq!(
            &vec![String::from("work")],
            parameters.types.as_ref().unwrap()
        );
        Ok(())
    }

    #[test]
    fn parse_url() -> Result<()> {
        let input = r#"BEGIN:VCARD
VERSION:4.0
FN:Mock person
URL:https://example.com
END:VCARD"#;
        let mut vcards = parse(input)?;
        assert_eq!(1, vcards.len());

        let card = vcards.remove(0);

        let uri = URI::parse("https://example.com")?.to_owned();
        let url = card.url.get(0).unwrap();
        assert_eq!(uri.as_str(), url.value.as_str());

        Ok(())
    }
}
