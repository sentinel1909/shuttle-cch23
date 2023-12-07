// src/lib.rs

// dependencies
use hyper::http::Error as HyperHttpError;
use serde_json::Error as SerdeError;
use std::convert::From;
use std::fmt::Display;

// define an enum to represent the possible errors
#[derive(Debug)]
pub enum ApiError {
    JsonError(SerdeError),
    HyperError(hyper::Error),
}

// implement the Display trait for the ReindeerError type
impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::JsonError(err) => write!(f, "JSON error: {}", err),
            ApiError::HyperError(err) => write!(f, "Hyper error: {}", err),
        }
    }
}

// implement the From trait for the ReindeerError type
impl From<SerdeError> for ApiError {
    fn from(err: SerdeError) -> ApiError {
        ApiError::JsonError(err)
    }
}

// implement the From trait for the HyperError type
impl From<hyper::Error> for ApiError {
    fn from(err: hyper::Error) -> ApiError {
        ApiError::HyperError(err)
    }
}

// implement the From trait for the HyperError type
impl From<ApiError> for HyperHttpError {
    fn from(err: ApiError) -> HyperHttpError {
        match err {
            ApiError::JsonError(_) => HyperHttpError::from(hyper::http::Error::from(err)),
            ApiError::HyperError(_) => HyperHttpError::from(hyper::http::Error::from(err)),
        }
    }
}
