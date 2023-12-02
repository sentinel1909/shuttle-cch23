// src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 1 Challenge Endpoints

// dependencies
use hyper::body::Body;
use hyper::http::Error;
use hyper::{Response, StatusCode};

// function to handle the /1/<num1>/<num2> endpoint
pub fn recalibrate(num1: &str, num2: &str) -> Result<Response<Body>, Error> {
    // convert the string values to integers
    let num1 = num1.parse::<i32>().unwrap();
    let num2 = num2.parse::<i32>().unwrap();

    // calculate the recalibrated the packet IDs
    let bitwise_or = (num1 ^ num2) as i32;
    let recalibrated = bitwise_or.pow(3);

    // return the recalibrated packet ID
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(recalibrated.to_string()))
}
