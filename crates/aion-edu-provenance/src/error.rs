//! A small, typed error for the provenance layer (no panics in the library).

use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Json(serde_json::Error),
    /// An error surfaced by aion-context (already human-readable).
    Aion(String),
    /// A semantic refusal (e.g. endorsing a program that does not exist).
    Refused(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "io: {e}"),
            Error::Json(e) => write!(f, "json: {e}"),
            Error::Aion(e) => write!(f, "aion: {e}"),
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
impl From<aion_context::AionError> for Error {
    fn from(e: aion_context::AionError) -> Self {
        Error::Aion(e.to_string())
    }
}
