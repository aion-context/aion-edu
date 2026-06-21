//! Typed error for the teaching layer.

use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Json(serde_json::Error),
    /// Anthropic API / transport error.
    Api(String),
    /// Provenance (aion-context) error.
    Provenance(String),
    /// A teaching-loop invariant was violated (e.g. rubric failed verification).
    Refused(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "io: {e}"),
            Error::Json(e) => write!(f, "json: {e}"),
            Error::Api(e) => write!(f, "api: {e}"),
            Error::Provenance(e) => write!(f, "provenance: {e}"),
            Error::Refused(e) => write!(f, "refused: {e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e)
    }
}
impl From<aion_edu_provenance::Error> for Error {
    fn from(e: aion_edu_provenance::Error) -> Self {
        Error::Provenance(e.to_string())
    }
}
