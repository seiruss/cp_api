use std::error::Error as StdError;
use std::fmt;

/// A `Result` alias where the `Err` is `cp_api::Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// The errors that can occur.
#[derive(Debug)]
pub enum Error {
    /// Reqwest errors.
    Reqwest(reqwest::Error),

    /// Occurs when setting the sid to the x-chkp-sid header name fails.
    HeaderValue(reqwest::header::InvalidHeaderValue),

    /// Occurs when converting a Reqwest HeaderMap to a HashMap fails due to non ASCII characters.
    HeaderToStr(reqwest::header::ToStrError),

    /// Serialization or deserialization errors.
    Json(serde_json::Error),

    /// I/O errors.
    Io(std::io::Error),

    /// Occurs when failing to parse an integer.
    /// Not used by cp_api, but commonly used to get port numbers in programs using this crate.
    ParseInt(std::num::ParseIntError),

    /// Occurs when parsing a Response that does not contain the expected fields.
    InvalidResponse(&'static str, serde_json::Value),

    /// Custom error message.
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match self {
            Reqwest(ref e) => e.fmt(f),
            HeaderValue(ref e) => e.fmt(f),
            HeaderToStr(ref e) => e.fmt(f),
            Json(ref e) => e.fmt(f),
            Io(ref e) => e.fmt(f),
            ParseInt(ref e) => e.fmt(f),
            InvalidResponse(ref s, ref r) => {
                write!(f, "Failed to parse expected \"{}\" field from Response: {}", s, r)
            },
            Custom(ref s) => write!(f, "{}", s),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match self {
            Reqwest(ref e) => e.description(),
            HeaderValue(ref e) => e.description(),
            HeaderToStr(ref e) => e.description(),
            Json(ref e) => e.description(),
            Io(ref e) => e.description(),
            ParseInt(ref e) => e.description(),
            InvalidResponse(_, _) => "Failed to parse expected field from Response",
            Custom(_) => "Custom error message",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        use self::Error::*;
        match self {
            Reqwest(ref e) => e.cause(),
            HeaderValue(ref e) => e.cause(),
            HeaderToStr(ref e) => e.cause(),
            Json(ref e) => e.cause(),
            Io(ref e) => e.cause(),
            ParseInt(ref e) => e.cause(),
            InvalidResponse(_, _) |
            Custom(_) => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(e: reqwest::header::InvalidHeaderValue) -> Self {
        Error::HeaderValue(e)
    }
}

impl From<reqwest::header::ToStrError> for Error {
    fn from(e: reqwest::header::ToStrError) -> Self {
        Error::HeaderToStr(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseInt(e)
    }
}
