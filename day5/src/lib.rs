// day5/src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 5 Challenge Endpoints
// this challenge was pulled due to the over capacity issues with Shuttle
// I decided to make something up

// dependences
use common_features::WebRequest;
use std::convert::Infallible;

// day 5 endpoint
pub async fn svc_mean_grinch(_reqeust: WebRequest) -> Result<&'static str, Infallible> {
    Ok("You're a mean one, Mr. Grinch!")
}
