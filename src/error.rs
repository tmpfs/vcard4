use thiserror::Error;

/// Errors generated by the vCard library.
#[derive(Debug, Error)]
pub enum Error {
    /// Error generated when a token was expected but no more tokens
    /// are available; end-of-file (EOF) was reached.
    #[error("input token was expected but reached EOF")]
    TokenExpected,

    /// Error generated when an expected token is of the wrong type.
    #[error("input token was incorrect")]
    IncorrectToken,

    /// Error generated when an unknown property name is encountered.
    #[error("property name '{0}' is not supported")]
    UnknownPropertyName(String),

    /// Error generated when a property value is invalid.
    #[error("property value is invalid")]
    InvalidPropertyValue,

    /// Error generated when a boolean is invalid.
    #[error("value '{0}' is not a valid boolean")]
    InvalidBoolean(String),

    /// Error generated when a CLIENTPIDMAP value could not be parsed.
    #[error("client PID map '{0}' is not valid")]
    InvalidClientPidMap(String),

    /// Error generated when a property or parameter delimiter was expected.
    #[error("property or parameter delimiter expected")]
    DelimiterExpected,

    /// Error generated when a property name is not supported.
    #[error("parameter name '{0}' is not supported")]
    UnknownParameterName(String),

    /// Error generated when a value type is not supported.
    #[error("value type '{0}' is not supported")]
    UnknownValueType(String),

    /// Error generated when a TYPE for a RELATED property is not supported.
    #[error("related type value '{0}' is not supported")]
    UnknownRelatedTypeValue(String),

    /// Error generated when a VALUE for a property is not supported.
    #[error("value '{0}' is not supported in this context '{1}'")]
    UnsupportedValueType(String, String),

    /// Error generated when a KIND is not supported.
    #[error("kind '{0}' is not supported")]
    UnknownKind(String),

    /// Error generated when the sex of a GENDER is not supported.
    #[error("sex '{0}' is not supported")]
    UnknownSex(String),

    /// Error generated when the a GENDER does not specify the sex.
    #[error("gender value is missing sex")]
    NoSex,

    /// Error generated when a property appears more than once.
    #[error("property '{0}' may only appear exactly once")]
    OnlyOnce(String),

    /// Error generated when the FN property is not specified.
    #[error("formatted name (FN) is required")]
    NoFormattedName,

    /// Error generated when a utc-offset data type is invalid.
    #[error("value '{0}' for UTC offset is invalid")]
    InvalidUtcOffset(String),

    /// Error generated when a PREF is out of bounds.
    #[error("pref '{0}' is out of bounds, must be between 1 and 100")]
    PrefOutOfRange(u8),

    /// Error generated when a PID is invalid.
    #[error("pid '{0}' is invalid")]
    InvalidPid(String),

    /// Error generated when an unquoted value was encountered when it must
    /// be quoted; eg: the GEO parameter URI.
    #[error("'{0}' must be enclosed in quotes")]
    NotQuoted(String),

    /// Errors generated by the language tags library.
    #[error(transparent)]
    LanguageParse(#[from] language_tags::ParseError),

    /// Errors generated by the URI library.
    #[error(transparent)]
    UriParse(#[from] fluent_uri::ParseError),

    /// Errors generated by time library.
    #[error(transparent)]
    ComponentRange(#[from] time::error::ComponentRange),

    /// Errors generated by time library parsing.
    #[error(transparent)]
    TimeParse(#[from] time::error::Parse),

    /// Error generated parsing a string to an integer.
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    /// Error generated parsing a string to a float.
    #[error(transparent)]
    ParseFloat(#[from] std::num::ParseFloatError),

    /// Error generated parsing a media type.
    #[error(transparent)]
    Mime(#[from] mime::FromStrError),
}
