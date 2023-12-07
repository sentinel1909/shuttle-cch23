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
    HttpError(HyperHttpError),
}

// implement the Display trait for the ReindeerError type
impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::JsonError(err) => write!(f, "JSON error: {}", err),
            ApiError::HyperError(err) => write!(f, "Hyper error: {}", err),
            ApiError::HttpError(err) => write!(f, "HTTP error: {}", err),
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

// implement the From trait for the HyperHttpError type
impl From<HyperHttpError> for ApiError {
    fn from(err: HyperHttpError) -> ApiError {
        ApiError::HttpError(err)
    }
}
