// src/lib/router.rs

// dependencies
use hyper::{Body, Request, Response};
use minus1_endpoint::root;
use tower::BoxError;

// define a function to handle routing
pub async fn router(request: Request<Body>) -> Result<Response<Body>, BoxError> {
    match request.uri().path() {
        "/" => Ok(root().await.unwrap()),
        "/-1/error" => Ok(Response::builder().status(500).body(Body::from("Internal Server Error")).unwrap()),
        _ => Ok(Response::builder().status(404).body(Body::from("Not Found")).unwrap()),
    }
}