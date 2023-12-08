// minus1/src/lib.rs

// dependencies
use errors::ApiError;
use hyper::body::Body;
use hyper::{Response, StatusCode};

// the root "/" route
pub fn root() -> Result<Response<Body>, ApiError> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())?)
}
