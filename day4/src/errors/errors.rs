// src/domain/errors.rs

// dependencies
use hyper::http::Error as HyperHttpError;
use serde_json::Error as SerdeError;
use std::convert::From;
use std::fmt::Display;

// define an enum to represent the possible errors
#[derive(Debug)]
pub enum ReindeerError {
    JsonError(SerdeError),
    HyperError(hyper::Error),
}

// implement the Display trait for the ReindeerError type
impl Display for ReindeerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReindeerError::JsonError(err) => write!(f, "JSON error: {}", err),
            ReindeerError::HyperError(err) => write!(f, "Hyper error: {}", err),
        }
    }
}

// implement the From trait for the ReindeerError type
impl From<SerdeError> for ReindeerError {
    fn from(err: SerdeError) -> ReindeerError {
        ReindeerError::JsonError(err)
    }
}

// implement the From trait for the HyperError type
impl From<hyper::Error> for ReindeerError {
    fn from(err: hyper::Error) -> ReindeerError {
        ReindeerError::HyperError(err)
    }
}

// implement the From trait for the HyperError type
impl From<ReindeerError> for HyperHttpError {
    fn from(err: ReindeerError) -> HyperHttpError {
        match err {
            ReindeerError::JsonError(_) => HyperHttpError::from(hyper::http::Error::from(err)),
            ReindeerError::HyperError(_) => HyperHttpError::from(hyper::http::Error::from(err)),
        }
    }
}
