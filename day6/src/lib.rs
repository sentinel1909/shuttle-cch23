// day6/src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 6 Challenge Endpoints

// dependencies
use errors::ApiError;
use hyper::{Body, Response, StatusCode};

pub fn count_elf() -> Result<Response<Body>, ApiError> {
    // create the response body
    let response_body = "The Day 6 endpoint";

    // return the reindeer strength
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(response_body))?)
}
