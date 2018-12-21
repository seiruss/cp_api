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
    Parse(&'static str, serde_json::Value),
    Custom(String)
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
            Parse(ref s, ref r) => write!(f, "Failed to parse \"{}\" from Response: {}", s, r),
            Custom(ref s) => write!(f, "{}", s)
        }
    }
}
