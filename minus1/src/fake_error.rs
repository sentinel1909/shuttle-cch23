// minus1/src/fake_error.rs

// dependencies
use hyper::{Body, Request, Response, StatusCode};

// handler for the "/-1/error" endpoint
pub async fn fake_error(_request: Request<Body>) -> Result<Response<Body>, StatusCode> {
    Response::builder()
        .status(500)
        .body(Body::from("Internal Server Error"))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
