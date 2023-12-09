// day6/src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 7 Challenge Endpoints

// dependencies
use errors::ApiError;
use hyper::{Body, Request, Response, StatusCode};
use tower_cookies::{Cookie, Cookies};

pub async fn decode(request: Request<Body>) -> Result<Response<Body>, ApiError> {
    
    // set the cookie
    cookies.add(Cookie::new("recipe", "eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ=="));
    
    // build the response body
    let response_body = format!("Cookie: {}", cookies.get("recipe").unwrap().value());

    // return the response
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(response_body))?)
}
