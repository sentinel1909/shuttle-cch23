// day1/src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 1 Challenge Endpoints

// dependencies
use common_features::WebRequest;
use std::convert::Infallible;

// Task 1: Calibrate the incoming packet IDs
pub async fn svc_calibrate_packet_ids(request: WebRequest) -> Result<String, Infallible> {
    let path_segments: Vec<Result<i32, _>> = request
        .uri()
        .path()
        .split('/')
        .map(|segment| segment.parse::<i32>())
        .collect();
    let segments: Vec<i32> = path_segments
        .into_iter()
        .filter_map(|path_segment| path_segment.ok())
        .collect();
    let packet1 = segments[1];
    let packet2 = segments[2];

    let bitwise_or = packet1 ^ packet2;
    let calibrated_packet_id = bitwise_or.pow(3).to_string();
    Ok(calibrated_packet_id)
}
