use crate::Error;
use std::{fmt, str::FromStr};
use uriparse::URI;

/// URI type for the library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uri(URI<'static>);

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl FromStr for Uri {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(URI::try_from(s)?.into_owned()))
    }
}
