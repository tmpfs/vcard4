//! Parse vCards based on [RFC6350](https://www.rfc-editor.org/rfc/rfc6350).

use logos::{Lexer, Logos};
use std::{borrow::Cow, ops::Range};
use uriparse::uri::URI as Uri;

#[cfg(feature = "language-tags")]
use language_tags::LanguageTag;

#[cfg(feature = "mime")]
use mime::Mime;

use crate::{
    name::*,
    parameter::*,
    property::*,
    types::*,
    Error, Result, Vcard,
};

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex("(?i:BEGIN:VCARD)")]
    Begin,

    #[regex("(?i:VERSION:4\\.0)")]
    Version,

    #[regex("(?i:([a-z0-9-]+\\.)?(SOURCE|KIND|FN|N|NICKNAME|PHOTO|BDAY|ANNIVERSARY|GENDER|ADR|TEL|EMAIL|IMPP|LANG|TZ|GEO|TITLE|ROLE|LOGO|ORG|MEMBER|RELATED|CATEGORIES|NOTE|PRODID|REV|SOUND|UID|CLIENTPIDMAP|URL|KEY|FBURL|CALADRURI|CALURI|XML|x-[a-z0-9]+))")]
    PropertyName,

    #[token(";")]
    ParameterDelimiter,

    #[regex("(?i:(LANGUAGE|VALUE|PREF|ALTID|PID|TYPE|MEDIATYPE|CALSCALE|SORT-AS|GEO|TZ|LABEL)=)")]
    ParameterKey,

    #[token(":")]
    PropertyDelimiter,

    #[regex("\\r?\\n( |\\t)")]
    FoldedLine,

    #[token("\\,")]
    EscapedComma,

    #[token("\\;")]
    EscapedSemiColon,

    #[token("\\\\")]
    EscapedBackSlash,

    #[regex("(?i:\\\\n)")]
    EscapedNewLine,

    #[regex("\\r?\\n")]
    NewLine,

    #[regex("(?i:END:VCARD)")]
    End,

    #[error]
    Text,
}

/// Parses vCards from a string.
pub(crate) struct VcardParser {
    strict: bool,
}

impl VcardParser {
    /// Create a new parser.
    pub fn new(strict: bool) -> Self {
        Self { strict }
    }
}

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
            if let Err(e) = self.parse_property(lex, card) {
                if self.strict {
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    /// Parse a single property.
    fn parse_property(
        &self,
        lex: &mut Lexer<'_, Token>,
        card: &mut Vcard,
    ) -> Result<()> {
        let mut group: Option<String> = None;
        let mut name = lex.slice();

        let period = name.find(".");

        if let Some(pos) = period {
            let group_name = &name[0..pos];
            group = Some(group_name.to_string());
            name = &name[pos..];
        }

        let delimiter = lex.next();

        if let Some(delimiter) = delimiter {
            if delimiter == Token::ParameterDelimiter {
                let parameters = self.parse_property_parameters(lex, name)?;
                self.parse_property_by_name(
                    lex,
                    card,
                    name,
                    Some(parameters),
                    group,
                )?;
            } else if delimiter == Token::PropertyDelimiter {
                self.parse_property_by_name(lex, card, name, None, group)?;
            } else {
                return Err(Error::DelimiterExpected);
            }
        } else {
            return Err(Error::TokenExpected);
        }

        Ok(())
    }

    /// Parse property parameters.
    fn parse_property_parameters(
        &self,
        lex: &mut Lexer<'_, Token>,
        name: &str,
    ) -> Result<Parameters> {
        let property_upper_name = name.to_uppercase();
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

                let (value, next_token, quoted) =
                    self.parse_property_parameters_value(lex)?;

                match &upper_name[..] {
                    LANGUAGE => {
                        let tag = parse_language_tag(Cow::Owned(value))?;
                        params.language = Some(tag);
                    }
                    VALUE => {
                        let value: ValueType = value.parse()?;
                        params.value = Some(value);
                    }
                    PREF => {
                        let value: u8 = value.parse()?;
                        if value < 1 || value > 100 {
                            return Err(Error::PrefOutOfRange(value));
                        }
                        params.pref = Some(value);
                    }
                    ALTID => {
                        params.alt_id = Some(value);
                    }
                    PID => {
                        let mut pids: Vec<Pid> = Vec::new();
                        let values = value.split(",");
                        for value in values {
                            pids.push(value.parse()?);
                        }
                        params.pid = Some(pids);
                    }
                    TYPE => {
                        // Check this parameter is allowed
                        if !TYPE_PROPERTIES
                            .contains(&&property_upper_name[..])
                        {
                            return Err(Error::TypeParameter(
                                property_upper_name,
                            ));
                        }

                        let mut type_params: Vec<TypeParameter> = Vec::new();

                        for val in value.split(",") {
                            let param: TypeParameter =
                                match &property_upper_name[..] {
                                    TEL => {
                                        TypeParameter::Telephone(val.parse()?)
                                    }
                                    RELATED => {
                                        TypeParameter::Related(val.parse()?)
                                    }
                                    _ => val.parse()?,
                                };
                            type_params.push(param);
                        }

                        if let Some(types) = params.types.as_mut() {
                            types.append(&mut type_params);
                        } else {
                            params.types = Some(type_params);
                        }
                    }
                    MEDIATYPE => {
                        parse_media_type(value, &mut params)?;
                    }
                    CALSCALE => {
                        params.calscale = Some(value);
                    }
                    SORT_AS => {
                        let sort_values = value
                            .split(",")
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>();
                        params.sort_as = Some(sort_values);
                    }
                    GEO => {
                        if !quoted {
                            return Err(Error::NotQuoted(property_upper_name));
                        }
                        let geo = Uri::try_from(&value[..])?.into_owned();
                        params.geo = Some(geo);
                    }
                    TZ => {
                        if quoted {
                            let value =
                                Uri::try_from(&value[..])?.into_owned();
                            params.timezone =
                                Some(TimeZoneParameter::Uri(value));
                        } else {
                            match value.parse::<UtcOffsetProperty>() {
                                Ok(offset) => {
                                    params.timezone =
                                        Some(TimeZoneParameter::UtcOffset(
                                            offset.value,
                                        ));
                                }
                                Err(_) => {
                                    params.timezone =
                                        Some(TimeZoneParameter::Text(value));
                                }
                            }
                        }
                    }
                    LABEL => {
                        if &property_upper_name != ADR {
                            return Err(Error::InvalidLabel(
                                property_upper_name,
                            ));
                        }
                        params.label = Some(value);
                    }
                    _ => {
                        return Err(Error::UnknownParameter(
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
            } else {
                return Err(Error::UnknownParameter(lex.slice().to_string()));
            }
        }
        Ok(params)
    }

    /// Parse the raw value for a property parameter.
    fn parse_property_parameters_value<'a>(
        &self,
        lex: &'a mut Lexer<'_, Token>,
    ) -> Result<(String, Token, bool)> {
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
                let mut quoted = false;
                let source = lex.source();
                let begin = first_range.unwrap().start;
                let end = span.start;
                let mut value = &source[begin..end];

                // Remove double quotes if necessary
                if value.len() >= 2
                    && &value[0..1] == "\""
                    && &value[value.len() - 1..] == "\""
                {
                    value = &source[begin + 1..end - 1];
                    quoted = true;
                }

                return Ok((String::from(value), token, quoted));
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
        group: Option<String>,
    ) -> Result<()> {
        let value = self.parse_property_value(lex)?;
        let upper_name = name.to_uppercase();

        if name.len() > 2 && &upper_name[0..2] == "X-" {
            self.parse_extension_property_by_name(
                card, name, value, parameters, group,
            )?;
            return Ok(());
        }

        match &upper_name[..] {
            // General properties
            // https://www.rfc-editor.org/rfc/rfc6350#section-6.1
            SOURCE => {
                let value = Uri::try_from(value.as_ref())?.into_owned();
                card.source.push(UriProperty {
                    value,
                    parameters,
                    group,
                });
            }
            KIND => {
                if card.kind.is_some() {
                    return Err(Error::OnlyOnce(upper_name));
                }
                let value: Kind = value.as_ref().parse()?;
                card.kind = Some(KindProperty {
                    value,
                    parameters,
                    group,
                });
            }
            XML => {
                card.xml.push(TextProperty {
                    value: value.into_owned(),
                    parameters,
                    group,
                });
            }
            // Identification properties
            // https://www.rfc-editor.org/rfc/rfc6350#section-6.2
            FN => {
                card.formatted_name.push(TextProperty {
                    value: value.into_owned(),
                    parameters,
                    group,
                });
            }
            N => {
                if card.name.is_some() {
                    return Err(Error::OnlyOnce(upper_name));
                }
                let value = value
                    .as_ref()
                    .split(";")
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                card.name = Some(TextListProperty {
                    value,
                    parameters,
                    group,
                });
            }
            NICKNAME => {
                card.nickname.push(TextProperty {
                    value: value.into_owned(),
                    parameters,
                    group,
                });
            }
            PHOTO => {
                let value = Uri::try_from(value.as_ref())?.into_owned();
                card.photo.push(UriProperty {
                    value,
                    parameters,
                    group,
                });
            }
            BDAY => {
                if card.bday.is_some() {
                    return Err(Error::OnlyOnce(upper_name));
                }

                let prop = parse_date_time_or_text(
                    &upper_name, value, parameters, group,
                )?;
                card.bday = Some(prop);
            }
            ANNIVERSARY => {
                if card.anniversary.is_some() {
                    return Err(Error::OnlyOnce(upper_name));
                }

                let prop = parse_date_time_or_text(
                    &upper_name,
                    value,
                    parameters,
                    group,
                )?;
                card.anniversary = Some(prop);
            }
            GENDER => {
                if card.gender.is_some() {
                    return Err(Error::OnlyOnce(upper_name));
                }
                let value: Gender = value.as_ref().parse()?;
                card.gender = Some(GenderProperty {
                    value,
                    parameters,
                    group,
                });
            }

            // Delivery Addressing
            // https://www.rfc-editor.org/rfc/rfc6350#section-6.3
            ADR => {
                todo!()
            }

            // Communications
            // https://www.rfc-editor.org/rfc/rfc6350#section-6.4
            TEL => {
                let value = self.parse_text_or_uri(
                    value.as_ref(),
                    parameters,
                    group,
                )?;
                card.tel.push(value);
            }
            EMAIL => {
                card.email.push(TextProperty {
                    value: value.into_owned(),
                    parameters,
                    group,
                });
            }
            IMPP => {
                let value = Uri::try_from(value.as_ref())?.into_owned();
                card.impp.push(UriProperty {
                    value,
                    parameters,
                    group,
                });
            }
            LANG => {
                let value = parse_language_tag(value)?;
                card.lang.push(LanguageProperty {
                    value,
                    parameters,
                    group,
                });
            }

            // Geographic
            // https://www.rfc-editor.org/rfc/rfc6350#section-6.5
            TZ => {
                let value_type = if let Some(parameters) = &parameters {
                    parameters.value.as_ref()
                } else {
                    None
                };

                if let Some(value_type) = value_type {
                    match value_type {
                        ValueType::UtcOffset => {
                            let mut value: UtcOffsetProperty =
                                value.as_ref().parse()?;
                            value.parameters = parameters;
                            value.group = group;
                            card.timezone
                                .push(TimeZoneProperty::UtcOffset(value));
                        }
                        ValueType::Uri => {
                            let value =
                                Uri::try_from(value.as_ref())?.into_owned();
                            card.timezone.push(TimeZoneProperty::Uri(
                                UriProperty {
                                    value,
                                    parameters,
                                    group,
                                },
                            ));
                        }
                        _ => {
                            return Err(Error::UnsupportedValueType(
                                value_type.to_string(),
                                String::from(upper_name),
                            ))
                        }
                    }
                } else {
                    card.timezone.push(TimeZoneProperty::Text(
                        TextProperty {
                            value: value.into_owned(),
                            parameters,
                            group,
                        },
                    ));
                }
            }
            GEO => {
                let value = Uri::try_from(value.as_ref())?.into_owned();
                card.geo.push(UriProperty {
                    value,
                    parameters,
                    group,
                });
            }

            // Organizational
            // https://www.rfc-editor.org/rfc/rfc6350#section-6.6
            TITLE => {
                card.title.push(TextProperty {
                    value: value.into_owned(),
                    parameters,
                    group,
                });
            }
            ROLE => {
                card.role.push(TextProperty {
                    value: value.into_owned(),
                    parameters,
                    group,
                });
            }
            LOGO => {
                let value = Uri::try_from(value.as_ref())?.into_owned();
                card.logo.push(UriProperty {
                    value,
                    parameters,
                    group,
                });
            }
            ORG => {
                let value = value
                    .as_ref()
                    .split(";")
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                card.org.push(TextListProperty {
                    value,
                    parameters,
                    group,
                });
            }
            MEMBER => {
                let value = Uri::try_from(value.as_ref())?.into_owned();
                card.member.push(UriProperty {
                    value,
                    parameters,
                    group,
                });
            }
            RELATED => {
                let text_or_uri = self.parse_text_or_uri(
                    value.as_ref(),
                    parameters,
                    group,
                )?;
                card.related.push(text_or_uri);
            }

            // Explanatory
            // https://www.rfc-editor.org/rfc/rfc6350#section-6.7
            CATEGORIES => {
                let value = value
                    .as_ref()
                    .split(",")
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                card.categories.push(TextListProperty {
                    value,
                    parameters,
                    group,
                });
            }
            NOTE => {
                card.note.push(TextProperty {
                    value: value.into_owned(),
                    parameters,
                    group,
                });
            }
            PRODID => {
                if card.prod_id.is_some() {
                    return Err(Error::OnlyOnce(upper_name));
                }
                card.prod_id = Some(TextProperty {
                    value: value.into_owned(),
                    parameters,
                    group,
                });
            }
            REV => {
                if card.rev.is_some() {
                    return Err(Error::OnlyOnce(upper_name));
                }
                let value = parse_timestamp(value.as_ref())?;
                card.rev = Some(DateTimeProperty {
                    value,
                    parameters,
                    group,
                });
            }
            SOUND => {
                let value = Uri::try_from(value.as_ref())?.into_owned();
                card.sound.push(UriProperty {
                    value,
                    parameters,
                    group,
                });
            }
            UID => {
                if card.uid.is_some() {
                    return Err(Error::OnlyOnce(upper_name));
                }
                let text_or_uri = self.parse_text_or_uri(
                    value.as_ref(),
                    parameters,
                    group,
                )?;
                card.uid = Some(text_or_uri);
            }
            CLIENTPIDMAP => {
                let value: ClientPidMap = value.as_ref().parse()?;
                card.client_pid_map.push(ClientPidMapProperty {
                    value,
                    parameters,
                    group,
                });
            }
            URL => {
                let value = Uri::try_from(value.as_ref())?.into_owned();
                card.url.push(UriProperty {
                    value,
                    parameters,
                    group,
                });
            }
            VERSION => {
                return Err(Error::VersionMisplaced);
            },

            // Security
            // https://www.rfc-editor.org/rfc/rfc6350#section-6.8
            KEY => {
                let text_or_uri = self.parse_text_or_uri(
                    value.as_ref(),
                    parameters,
                    group,
                )?;
                card.key.push(text_or_uri);
            }

            // Calendar
            // https://www.rfc-editor.org/rfc/rfc6350#section-6.9
            FBURL => {
                let value = Uri::try_from(value.as_ref())?.into_owned();
                card.fburl.push(UriProperty {
                    value,
                    parameters,
                    group,
                });
            }
            CALADRURI => {
                let value = Uri::try_from(value.as_ref())?.into_owned();
                card.cal_adr_uri.push(UriProperty {
                    value,
                    parameters,
                    group,
                });
            }
            CALURI => {
                let value = Uri::try_from(value.as_ref())?.into_owned();
                card.cal_uri.push(UriProperty {
                    value,
                    parameters,
                    group,
                });
            }
            _ => return Err(Error::UnknownPropertyName(name.to_string())),
        }
        Ok(())
    }

    /// Parse a private extension property (`x-`) by name.
    fn parse_extension_property_by_name<'a>(
        &self,
        card: &mut Vcard,
        name: &str,
        value: Cow<'a, str>,
        parameters: Option<Parameters>,
        group: Option<String>,
    ) -> Result<()> {
        let value_type = if let Some(parameters) = &parameters {
            parameters.value.as_ref()
        } else {
            None
        };
        let prop = if let Some(value_type) = value_type {
            match value_type {
                ValueType::Text => AnyProperty::Text(value.into_owned()),
                ValueType::Integer => {
                    AnyProperty::Integer(value.as_ref().parse()?)
                }
                ValueType::Float => {
                    AnyProperty::Float(value.as_ref().parse()?)
                }
                ValueType::Boolean => {
                    AnyProperty::Boolean(parse_boolean(value.as_ref())?)
                }
                ValueType::Date => {
                    AnyProperty::Date(parse_date(value.as_ref())?)
                }
                ValueType::DateTime => {
                    AnyProperty::DateTime(parse_date_time(value.as_ref())?)
                }
                ValueType::Time => {
                    AnyProperty::Time(parse_time(value.as_ref())?)
                }
                ValueType::DateAndOrTime => {
                    AnyProperty::DateAndOrTime(value.as_ref().parse()?)
                }
                ValueType::Timestamp => {
                    AnyProperty::Timestamp(parse_date_time(value.as_ref())?)
                }
                ValueType::LanguageTag => {
                    AnyProperty::Language(parse_language_tag(value)?)
                }
                ValueType::UtcOffset => {
                    let property: UtcOffsetProperty =
                        value.as_ref().parse()?;
                    AnyProperty::UtcOffset(property.value)
                }
                ValueType::Uri => {
                    let value = Uri::try_from(value.as_ref())?.into_owned();
                    AnyProperty::Uri(value)
                }
            }
        } else {
            AnyProperty::Text(value.into_owned())
        };

        card.extensions.push(ExtensionProperty {
            name: name.to_string(),
            value: prop,
            group,
            parameters,
        });

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
                || token == Token::EscapedNewLine
                || token == Token::EscapedBackSlash
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
                    } else if token == Token::EscapedComma {
                        value.push(',');
                        continue;
                    } else if token == Token::EscapedSemiColon {
                        value.push(';');
                        continue;
                    } else if token == Token::EscapedNewLine {
                        value.push('\n');
                        continue;
                    } else if token == Token::EscapedBackSlash {
                        value.push('\\');
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

    /// Parse text or Uri from a value.
    fn parse_text_or_uri<S: AsRef<str>>(
        &self,
        value: S,
        parameters: Option<Parameters>,
        group: Option<String>,
    ) -> Result<TextOrUriProperty> {
        let value_type = if let Some(parameters) = &parameters {
            parameters.value.as_ref()
        } else {
            None
        };
        if let Some(value_type) = value_type {
            if let ValueType::Text = value_type {
                Ok(TextOrUriProperty::Text(TextProperty {
                    value: value.as_ref().to_string(),
                    parameters,
                    group,
                }))
            } else if let ValueType::Uri = value_type {
                let value = Uri::try_from(value.as_ref())?.into_owned();
                Ok(TextOrUriProperty::Uri(UriProperty {
                    value,
                    parameters,
                    group,
                }))
            } else {
                Err(Error::UnknownValueType(value_type.to_string()))
            }
        } else {
            match Uri::try_from(value.as_ref()) {
                Ok(value) => Ok(TextOrUriProperty::Uri(UriProperty {
                    value: value.into_owned(),
                    parameters,
                    group,
                })),
                Err(_) => Ok(TextOrUriProperty::Text(TextProperty {
                    value: value.as_ref().to_string(),
                    parameters,
                    group,
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

fn parse_date_time_or_text<'a>(
    prop_name: &str,
    value: Cow<'a, str>,
    parameters: Option<Parameters>,
    group: Option<String>,
) -> Result<DateTimeOrTextProperty> {
    let value_type = if let Some(parameters) = &parameters {
        parameters.value.as_ref()
    } else {
        None
    };

    if let Some(value_type) = value_type {
        match value_type {
            ValueType::Text => {
                Ok(DateTimeOrTextProperty::Text(TextProperty {
                    value: value.into_owned(),
                    parameters,
                    group,
                }))
            }
            ValueType::DateAndOrTime => {
                let value: DateAndOrTime = value.parse()?;
                Ok(DateTimeOrTextProperty::DateTime(DateAndOrTimeProperty {
                    value,
                    parameters,
                    group,
                }))
            }
            _ => Err(Error::UnsupportedValueType(
                value_type.to_string(),
                String::from(prop_name),
            )),
        }
    } else {
        let value: DateAndOrTime = value.parse()?;
        Ok(DateTimeOrTextProperty::DateTime(DateAndOrTimeProperty {
            value,
            parameters,
            group,
        }))
    }
}

#[cfg(feature = "mime")]
fn parse_media_type(value: String, params: &mut Parameters) -> Result<()> {
    let mime: Mime = value.parse()?;
    params.media_type = Some(mime);
    Ok(())
}

#[cfg(not(feature = "mime"))]
fn parse_media_type(value: String, params: &mut Parameters) -> Result<()> {
    params.media_type = Some(value);
    Ok(())
}

#[cfg(feature = "language-tags")]
fn parse_language_tag<'a>(value: Cow<'a, str>) -> Result<LanguageTag> {
    let tag: LanguageTag = value.as_ref().parse()?;
    Ok(tag)
}

#[cfg(not(feature = "language-tags"))]
fn parse_language_tag<'a>(value: Cow<'a, str>) -> Result<String> {
    Ok(value.into_owned())
}
