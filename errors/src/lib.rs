// src/lib.rs

// dependencies
use hyper::http::Error as HyperHttpError;
use hyper::{Body, Error, Response, StatusCode};
use serde_json::Error as SerdeError;
use std::convert::From;
use std::fmt::Display;
use std::string::FromUtf8Error;

// define an enum to represent the possible errors
#[derive(Debug)]
pub enum ApiError {
    JsonError(SerdeError),
    HyperError(hyper::Error),
    HttpError(HyperHttpError),
    ConversionError(FromUtf8Error),
    ReqwestError(reqwest::Error),
}

// implement the Display trait for the ReindeerError type
impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::JsonError(err) => write!(f, "JSON error: {}", err),
            ApiError::HyperError(err) => write!(f, "Hyper error: {}", err),
            ApiError::HttpError(err) => write!(f, "HTTP error: {}", err),
            ApiError::ConversionError(err) => write!(f, "Conversion error: {}", err),
            ApiError::ReqwestError(err) => write!(f, "Reqwest error: {}", err),
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
impl From<Error> for ApiError {
    fn from(err: Error) -> ApiError {
        ApiError::HyperError(err)
    }
}

// implement the From trait for the HyperHttpError type
impl From<HyperHttpError> for ApiError {
    fn from(err: HyperHttpError) -> ApiError {
        ApiError::HttpError(err)
    }
}

// implement the From trait for the FromUtf8Error type
impl From<FromUtf8Error> for ApiError {
    fn from(err: FromUtf8Error) -> ApiError {
        ApiError::ConversionError(err)
    }
}

// implement the From trait for the reqwest::Error type
impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> ApiError {
        ApiError::ReqwestError(err)
    }
}

// function to return a general not found error
pub fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Not Found"))
        .unwrap()
}

// function to return a general bad request error
pub fn bad_request() -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from("Bad Request"))
        .unwrap()
}

// function to return a general internal server error
pub fn internal_server_error() -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from("Internal Server Error"))
        .unwrap()
}
