// src/lib/routes/not_found.rs

// dependencies
use hyper::body::Body;
use hyper::http::Error;
use hyper::{Response, StatusCode};

// the handler for routes that don't exist
pub fn not_found() -> Result<Response<Body>, Error> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Not Found"))
}
