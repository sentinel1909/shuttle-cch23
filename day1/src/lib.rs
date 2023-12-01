// src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 1 Challenge Endpoints

// dependencies
use hyper::body::Body;
use hyper::http::Error;
use hyper::{Response, StatusCode};

// function to handle the /1/<num1>/<num2> endpoint
pub fn recalibrate() -> Result<Response<Body>, Error> {
    // test input, need to replace with actual input from the request path
    let num1 = 4;
    let num2 = 8;

    // calculate the recalibrated the packet IDs
    let recalibrated = num1 ^ num2;

    // return the recalibrated packet ID
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(recalibrated.to_string()))
}
