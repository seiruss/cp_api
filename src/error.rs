use std::fmt;

/// A `Result` alias where the `Err` is `cp_api::Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// The Errors that can occur while using the API.
#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    HeaderValue(reqwest::header::InvalidHeaderValue),
    HeaderToStr(reqwest::header::ToStrError),
    Json(serde_json::Error),
    Io(std::io::Error),
    LogFileNotSet,
    NoPassword(serde_json::Value),
    QueryCall(serde_json::Value),
    Parse(&'static str, serde_json::Value)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match *self {
            Reqwest(ref e) => e.fmt(f),
            HeaderValue(ref e) => e.fmt(f),
            HeaderToStr(ref e) => e.fmt(f),
            Json(ref e) => e.fmt(f),
            Io(ref e) => e.fmt(f),
            LogFileNotSet => write!(f, "Log file is not set"),
            NoPassword(ref e) => {
                write!(f, "Failed to get the password to obfuscate from Response: {}", e)
            },
            QueryCall(ref e) => write!(f, "Failed to run call in query: {}", e),
            Parse(ref s, ref e) => write!(f, "Failed to parse {} from Response: {}", s, e)
        }
    }
}
