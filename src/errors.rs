use std::fmt;

#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    JsonParse(serde_json::Error),
    BuildError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Http(e) => write!(f, "HTTP error: {}", e),
            Error::JsonParse(e) => write!(f, "JSON parse error: {}", e),
            Error::BuildError(msg) => write!(f, "Build error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Http(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonParse(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
