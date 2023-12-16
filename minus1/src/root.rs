// minus1/src/root.rs

// dependencies
use hyper::{Body, Request, Response, StatusCode};

// handler for the root "/" endpoint
pub async fn root(_request: Request<Body>) -> Result<Response<Body>, StatusCode> {
    Response::builder()
        .status(200)
        .body(Body::from(""))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
