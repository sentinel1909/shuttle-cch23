// src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 1 Challenge Endpoints

// dependencies
use errors::ApiError;
use hyper::body::Body;
use hyper::{Response, StatusCode};

// function to return the calibrated packet ID
pub fn calibrate_packet_ids(packets: Vec<i32>) -> Result<Response<Body>, ApiError> {
    // recalibrated the packet ID
    let bitwise_or = (packets[1] ^ packets[2]) as i32;
    let calibrated_packet_id = bitwise_or.pow(3);

    // return the recalibrated packet ID
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(calibrated_packet_id.to_string()))?)
}

// function to return the calibrated sled ID
pub fn calibrate_sled_ids(packets: Vec<i32>) -> Result<Response<Body>, ApiError> {
    // recalibrate the sled ID
    let bitwise_or = packets.iter().fold(1, |packet, &x| packet ^ x);
    let calibrated_sled_id = bitwise_or.pow(3);

    // return the recalibrated sled ID
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(calibrated_sled_id.to_string()))?)
}
