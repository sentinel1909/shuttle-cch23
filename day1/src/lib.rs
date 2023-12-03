// src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 1 Challenge Endpoints

// dependencies
use hyper::body::Body;
use hyper::http::Error;
use hyper::{Response, StatusCode};

// function to handle the /1/<num1>/<num2> endpoint
pub fn calibrate_packet_ids(num1: i32, num2: i32) -> Result<Response<Body>, Error> {
    // calculate the recalibrated the packet ID
    let bitwise_or = (num1 ^ num2) as i32;
    let packet_id = bitwise_or.pow(3);

    // return the recalibrated packet ID
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(packet_id.to_string()))
}

