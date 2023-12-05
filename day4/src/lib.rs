// day4/src/lib.rs

// bring the domain module into scope
mod domain;
mod errors;

// dependencies
use crate::domain::reindeers::Reindeer;
use crate::errors::ReindeerError;
use hyper::http::Error;
use hyper::{Body, Request, Response, StatusCode};
use serde_json;

// function to convert the request body to JSON
async fn request_to_json(request: Request<Body>) -> Result<Vec<Reindeer>, ReindeerError> {
    // get the request body
    let body = request.into_body();

    // convert the body into bytes
    let body_bytes = hyper::body::to_bytes(body)
        .await
        .map_err(|err| ReindeerError::from(err))?;

    // convert the bytes into JSON, return it
    let reindeer_data: Vec<Reindeer> =
        serde_json::from_slice(&body_bytes).map_err(|err| ReindeerError::from(err))?;

    Ok(reindeer_data)
}

// function to calculate the reindeer strength based on the JSON data input
pub async fn get_strength(request: Request<Body>) -> Result<Response<Body>, Error> {
    // get the JSON data from the request body
    let reindeer_data = request_to_json(request)
        .await
        .map_err(|err| ReindeerError::from(err))?;

    let mut reindeer_strength = 0;
    for item in reindeer_data {
        reindeer_strength += item.strength;
    }

    // return the reindeer strength
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(format!(
            "Combined Reindeer Strength: {}",
            reindeer_strength
        )))
}
