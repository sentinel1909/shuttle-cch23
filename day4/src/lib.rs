// day4/src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 4 Challenge Endpoints

// dependencies
use common_features::WebRequest;
use domain::StrengthData;
use hyper::body;
use std::convert::Infallible;

// function to convert the request body to JSON
async fn strength_data_from_json(request: WebRequest) -> Result<Vec<StrengthData>, Infallible> {
    // get the request body
    let body = request.into_body();

    // convert the body into bytes
    let body_bytes = body::to_bytes(body).await.unwrap();

    // convert the bytes into a vector of StrengthData, return it
    let strength_data: Vec<StrengthData> = serde_json::from_slice(&body_bytes).unwrap();

    Ok(strength_data)
}

// day 4 calculate total strength endpoint
pub async fn svc_calculate_total_strength(request: WebRequest) -> Result<String, Infallible> {
    // get the JSON data from the request body
    let strength_data = strength_data_from_json(request).await?;

    let mut total_strength = 0;
    for entry in strength_data {
        total_strength += entry.strength;
    }

    // create the response body
    let response_msg = total_strength.to_string();

    Ok(response_msg)
}
