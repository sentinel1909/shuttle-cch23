// minus1/src/lib.rs

// dependencies
use hyper::{Body, Response, StatusCode};

// handler for the root "/" endpoint
pub async fn root() -> Result<Response<Body>, StatusCode> {
    let response = Response::new(Body::from(""));
    Ok(response)
}
