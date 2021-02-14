//! Error and Result types for the library
use std::convert::From;
use std::error::Error;
use std::fmt::Display;

/// Errors returned by strava API methods
///
// TODO some of these should take other error types.
#[derive(Debug)]
pub enum ApiError {
    /// The given access token has insufficient permission for accessing the requested resource.
    InvalidAccessToken,
    /// Error in the underlying http implementation
    Http(reqwest::Error),
    BadRequest(String),
    Io(std::io::Error),
}

/// A Result type for strava methods
pub type Result<T> = ::std::result::Result<T, ApiError>;

impl Error for ApiError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ApiError::Http(ref e) => Some(e),
            _ => None,
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ApiError::InvalidAccessToken => write!(f, "ApiError::InvalidAccessToken"),
            ApiError::Http(ref e) => write!(f, "ApiError::Http ({})", e),
            ApiError::BadRequest(ref s) => write!(f, "ApiError::BadRequest ({})", s),
            ApiError::Io(ref s) => write!(f, "ApiError::Io ({})", s),
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(e: reqwest::Error) -> ApiError {
        ApiError::Http(e)
    }
}
impl From<std::io::Error> for ApiError {
    fn from(e: std::io::Error) -> ApiError {
        ApiError::Io(e)
    }
}
