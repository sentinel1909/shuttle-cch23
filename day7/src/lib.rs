// day6/src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 7 Challenge Endpoints

// dependencies
use base64::engine::general_purpose;
use base64::Engine;
use domain::Kitchen;
use errors::ApiError;
use hyper::{Body, Request, Response, StatusCode};

// endpoint function to decode the cookie receipe and send it back to the client
pub async fn decode_the_receipe(request: Request<Body>) -> Result<Response<Body>, ApiError> {
    // get the cookie from the request
    let cookie = request.headers().get("cookie").unwrap().to_str().unwrap();

    // get the cookie value
    let cookie_value = cookie.split('=').collect::<Vec<&str>>()[1];

    // decode the cookie value
    let decoded_cookie = general_purpose::STANDARD_NO_PAD
        .decode(cookie_value)
        .unwrap();

    // convert the decoded cookie to a string
    let decoded_recipe = String::from_utf8(decoded_cookie).unwrap();

    // build the response body
    let response_body = serde_json::to_string(&decoded_recipe).unwrap();

    // return the response
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(response_body))?)
}

pub async fn bake(request: Request<Body>) -> Result<Response<Body>, ApiError> {
    // get the cookie from the request
    let cookie = request.headers().get("cookie").unwrap().to_str().unwrap();

    // get the cookie value
    let cookie_value = cookie.split('=').collect::<Vec<&str>>()[1];

    // decode the cookie value
    let decoded_cookie = general_purpose::STANDARD_NO_PAD
        .decode(cookie_value)
        .unwrap();

    // convert the decoded cookie to a string
    let decoded_recipe = String::from_utf8(decoded_cookie).unwrap();

    // create a JSON object from the decoded recipe
    let recipe: Kitchen = serde_json::from_str(&decoded_recipe)?;

    // build the response body
    let response_body = serde_json::to_string(&recipe)?;

    // return the response
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(response_body))?)
}
