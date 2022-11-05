use thiserror::Error;

/// Errors generated by the vCard library.
#[derive(Debug, Error)]
pub enum Error {
    #[error("input token was expected but reached EOF")]
    TokenExpected,

    #[error("input token was incorrect")]
    IncorrectToken,

    #[error("property name '{0}' is not supported")]
    UnknownPropertyName(String),

    #[error("property value is invalid")]
    InvalidPropertyValue,

    #[error("property or parameter delimiter expected")]
    DelimiterExpected,

    #[error("parameter name '{0}' is not supported")]
    UnknownParameterName(String),

    #[error("value type '{0}' is not supported")]
    UnknownValueType(String),

    #[error("kind '{0}' is not supported")]
    UnknownKind(String),

    #[error("sex '{0}' is not supported")]
    UnknownSex(String),

    #[error("gender value is missing sex")]
    NoSex,

    #[error("property '{0}' may only appear exactly once")]
    OnlyOnce(String),

    #[error("formatted name (FN) is required")]
    NoFormattedName,

    #[error(transparent)]
    LanguageParse(#[from] language_tags::ParseError),

    #[error(transparent)]
    UriParse(#[from] fluent_uri::ParseError),
}
