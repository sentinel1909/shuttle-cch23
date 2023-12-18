// day1/src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 1 Challenge Endpoints

// dependencies
use common_features::WebRequest;
use std::convert::Infallible;
use utilities::parameter_extractor;

// Task 1: Calibrate the incoming packet IDs
pub async fn svc_calibrate_packet_ids(request: WebRequest) -> Result<String, Infallible> {
    let segments = parameter_extractor(request.uri().path());
    let packet1 = segments[1];
    let packet2 = segments[2];

    let bitwise_or = packet1 ^ packet2;
    let calibrated_packet_id = bitwise_or.pow(3).to_string();
    Ok(calibrated_packet_id)
}
