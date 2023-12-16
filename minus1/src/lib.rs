// minus1/src/lib.rs

// dependencies
use hyper::{Body, Response, StatusCode};

// handler for the root "/" endpoint
pub async fn root() -> Result<Response<Body>, StatusCode> {
    Response::builder()
        .status(200)
        .body(Body::from(""))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// handler for the "/-1/error" endpoint
pub async fn fake_error() -> Result<Response<Body>, StatusCode> {
    Response::builder()
        .status(500)
        .body(Body::from(""))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
