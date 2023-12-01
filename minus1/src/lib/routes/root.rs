// src/lib/routes/root.rs

// dependencies
use hyper::body::Body;
use hyper::http::Error;
use hyper::{Response, StatusCode};

// the root "/" route
pub fn root() -> Result<Response<Body>, Error> {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
}
