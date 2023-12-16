// src/lib/router.rs

// dependencies
use hyper::{Body, Request, Response, StatusCode};
use minus1_endpoint::root;

// define a function to handle routing
pub async fn router(request: Request<Body>) -> Result<Response<Body>, StatusCode> {
    match request.uri().path() {
        "/" => Ok(root().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?),
        "/-1/error" => Ok(Response::builder().status(500).body(Body::from("Internal Server Error")).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?),
        _ => Ok(Response::builder().status(404).body(Body::from("Not Found")).map_err(|_| StatusCode::NOT_FOUND)?),
    }
}