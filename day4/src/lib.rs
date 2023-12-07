// day4/src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 4 Challenge Endpoints

// dependencies
use domain::{ContestData, StrengthData};
use errors::ApiError;
use hyper::{Body, Request, Response, StatusCode};

// function to convert the request body to JSON
async fn strength_data_from_json(request: Request<Body>) -> Result<Vec<StrengthData>, ApiError> {
    // get the request body
    let body = request.into_body();

    // convert the body into bytes
    let body_bytes = hyper::body::to_bytes(body)
        .await?;

    // convert the bytes into a vector of StrengthData, return it
    let strength_data: Vec<StrengthData> =
        serde_json::from_slice(&body_bytes).map_err(|err| ApiError::from(err))?;

    Ok(strength_data)
}

// function to convert the request body to JSON
async fn contest_data_from_json(request: Request<Body>) -> Result<Vec<ContestData>, ApiError> {
    // get the request body
    let body = request.into_body();

    // convert the body into bytes
    let body_bytes = hyper::body::to_bytes(body)
        .await?;

    // convert the bytes into a vector of StrengthData, return it
    let contest_data: Vec<ContestData> =
        serde_json::from_slice(&body_bytes).map_err(|err| ApiError::from(err))?;

    Ok(contest_data)
}

// function to calculate the reindeer strength based on the JSON data input
pub async fn get_strength_result(request: Request<Body>) -> Result<Response<Body>, ApiError> {
    // get the JSON data from the request body
    let strength_data = strength_data_from_json(request).await?;

    let mut total_strength = 0;
    for entry in strength_data {
        total_strength += entry.strength;
    }

    // create the response body
    let response_body = format!("Combined Reindeer Strength: {}", total_strength);

    // return the reindeer strength
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(response_body))?)
}

// function to calculate the outcome of the reindeer comparison contest
pub async fn get_contest_result(request: Request<Body>) -> Result<Response<Body>, ApiError> {
    // get the JSON data from the request body
    let _contest_data = contest_data_from_json(request).await?;

    // create the response body
    let response_body = "Endpoint not implemented yet!";

    // return the contest result
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(response_body))?)
}
