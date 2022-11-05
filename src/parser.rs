//! Parse vCards based on [RFC6350](https://www.rfc-editor.org/rfc/rfc6350).

use fluent_uri::Uri as URI;
use language_tags::LanguageTag;
use logos::{Lexer, Logos};
use mime::Mime;
use std::{borrow::Cow, ops::Range};

use crate::{Error, Result, Vcard};
use crate::values::*;

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

/// Parses vCards from strings.
#[derive(Default)]
pub(crate) struct VcardParser {}

impl VcardParser {
    /// Parse a UTF-8 encoded string into a list of vCards.
    pub(crate) fn parse<S: AsRef<str>>(
        &self,
        value: S,
    ) -> Result<Vec<Vcard>> {
        let mut cards = Vec::new();
        let mut lex = Token::lexer(value.as_ref());

        while let Some(first) = lex.next() {
            // Allow leading newlines and newlines between
            // vCard definitions
            if first == Token::NewLine {
                continue;
            }

            let card = self.parse_one(&mut lex, Some(first))?;

            if card.formatted_name.is_empty() {
                return Err(Error::NoFormattedName);
            }

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
                    "VALUE" => {
                        let value: ValueType = value.parse()?;
                        params.value = Some(value);
                    }
                    "PREF" => {
                        let value: u8 = value.parse()?;
                        if value < 1 || value > 100 {
                            return Err(Error::PrefOutOfRange(value));
                        }
                        params.pref = Some(value);
                    }
                    "ALTID" => {
                        params.alt_id = Some(value);
                    }
                    "PID" => {
                        let pid: Pid = value.parse()?;
                        params.pid = Some(pid);
                    }
                    "TYPE" => {
                        let mut type_values = value
                            .split(",")
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>();

                        if let Some(types) = params.types.as_mut() {
                            types.append(&mut type_values);
                        } else {
                            params.types = Some(type_values);
                        }
                    }
                    "MEDIATYPE" => {
                        let mime: Mime = value.parse()?;
                        params.media_type = Some(mime);
                    }
                    "CALSCALE" => {
                        params.calscale = Some(value);
                    }
                    "SORT-AS" => {
                        let sort_values = value
                            .split(",")
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>();
                        params.sort_as = Some(sort_values);
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
                let begin = first_range.unwrap().start;
                let end = span.start;
                let mut value = &source[begin..end];

                // Remove double quotes if necessary
                if value.len() >= 2
                    && &value[0..1] == "\""
                    && &value[value.len()-1..] == "\"" {
                    value = &source[begin+1..end-1];
                }

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
            "BDAY" => {
                todo!()
            }
            "ANNIVERSARY" => {
                todo!()
            }
            "GENDER" => {
                if card.gender.is_some() {
                    return Err(Error::OnlyOnce(String::from("GENDER")));
                }
                let value: Gender = value.as_ref().parse()?;
                card.gender = Some(value);
            }

            // Delivery Addressing
            // https://www.rfc-editor.org/rfc/rfc6350#section-6.3
            "ADR" => {
                todo!()
            }

            // Communications
            // https://www.rfc-editor.org/rfc/rfc6350#section-6.4
            "TEL" => {
                todo!();
            }
            "EMAIL" => {
                card.email.push(Text {
                    value: value.into_owned(),
                    parameters,
                });
            }
            "IMPP" => {
                let value = URI::parse(value.as_ref())?.to_owned();
                card.impp.push(Uri { value, parameters });
            }
            "LANG" => {
                let value: LanguageTag = value.as_ref().parse()?;
                card.lang.push(value);
            }

            // Geographic
            // https://www.rfc-editor.org/rfc/rfc6350#section-6.5
            "TZ" => {
                let value_type = if let Some(parameters) = &parameters {
                    parameters.value.as_ref()
                } else {
                    None
                };

                if let Some(value_type) = value_type {
                    match value_type {
                        ValueType::UtcOffset => {
                            let mut value: UtcOffset =
                                value.as_ref().parse()?;
                            value.parameters = parameters;
                            card.timezone.push(TimeZone::UtcOffset(value));
                        }
                        ValueType::Uri => {
                            let value =
                                URI::parse(value.as_ref())?.to_owned();
                            card.timezone.push(TimeZone::Uri(Uri {
                                value,
                                parameters,
                            }));
                        }
                        _ => {
                            return Err(Error::UnsupportedValueType(
                                value_type.to_string(),
                                String::from("TZ"),
                            ))
                        }
                    }
                } else {
                    card.timezone.push(TimeZone::Text(Text {
                        value: value.into_owned(),
                        parameters,
                    }));
                }
            }
            "GEO" => {
                let value = URI::parse(value.as_ref())?.to_owned();
                card.geo.push(Uri { value, parameters });
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
                card.org.push(TextList { value, parameters });
            }
            "MEMBER" => {
                let value = URI::parse(value.as_ref())?.to_owned();
                card.member.push(Uri { value, parameters });
            }
            "RELATED" => {
                // Validate related type parameter
                if let Some(parameters) = &parameters {
                    if let Some(types) = &parameters.types {
                        for related_type in types {
                            let _: RelatedTypeValue = related_type.parse()?;
                        }
                    }
                }

                let text_or_uri =
                    self.parse_text_or_uri(value.as_ref(), parameters)?;
                card.related.push(text_or_uri);
            }

            // Explanatory
            // https://www.rfc-editor.org/rfc/rfc6350#section-6.7
            "CATEGORIES" => {
                let value = value
                    .as_ref()
                    .split(",")
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                card.categories.push(TextList { value, parameters });
            }
            "NOTE" => {
                card.note.push(Text {
                    value: value.into_owned(),
                    parameters,
                });
            }
            "PRODID" => {
                if card.prod_id.is_some() {
                    return Err(Error::OnlyOnce(String::from("PRODID")));
                }
                card.prod_id = Some(Text {
                    value: value.into_owned(),
                    parameters,
                });
            }
            "REV" => {
                todo!()
            }
            "SOUND" => {
                let value = URI::parse(value.as_ref())?.to_owned();
                card.sound.push(Uri { value, parameters });
            }
            "UID" => {
                if card.uid.is_some() {
                    return Err(Error::OnlyOnce(String::from("UID")));
                }
                let text_or_uri =
                    self.parse_text_or_uri(value.as_ref(), parameters)?;
                card.uid = Some(text_or_uri);
            }
            "CLIENTPIDMAP" => {
                todo!()
            }
            "URL" => {
                let value = URI::parse(value.as_ref())?.to_owned();
                card.url.push(Uri { value, parameters });
            }
            "VERSION" => unreachable!(),

            // Security
            // https://www.rfc-editor.org/rfc/rfc6350#section-6.8
            "KEY" => {
                let text_or_uri =
                    self.parse_text_or_uri(value.as_ref(), parameters)?;
                card.key.push(text_or_uri);
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

    /// Parse text or URI from a value.
    fn parse_text_or_uri<S: AsRef<str>>(
        &self,
        value: S,
        parameters: Option<Parameters>,
    ) -> Result<TextOrUri> {
        let value_type = if let Some(parameters) = &parameters {
            parameters.value.as_ref()
        } else {
            None
        };
        if let Some(value_type) = value_type {
            if let ValueType::Text = value_type {
                Ok(TextOrUri::Text(Text {
                    value: value.as_ref().to_string(),
                    parameters,
                }))
            } else if let ValueType::Uri = value_type {
                let value = URI::parse(value.as_ref())?.to_owned();
                Ok(TextOrUri::Uri(Uri { value, parameters }))
            } else {
                Err(Error::UnknownValueType(value_type.to_string()))
            }
        } else {
            match URI::parse(value.as_ref()) {
                Ok(value) => Ok(TextOrUri::Uri(Uri {
                    value: value.to_owned(),
                    parameters,
                })),
                Err(_) => Ok(TextOrUri::Text(Text {
                    value: value.as_ref().to_string(),
                    parameters,
                })),
            }
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
