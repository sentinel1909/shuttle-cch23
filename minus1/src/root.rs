// minus1/src/root.rs

// dependencies
use common_features::WebRequest;
use std::convert::Infallible;

// endpoint handler function for Task 1, returns a 200 OK status code for GET requests
pub async fn svc_root(_request: WebRequest) -> Result<&'static str, Infallible> {
    Ok("Welcome to my solutions for the Shuttle Christmas Code Hunt, 2023 Edition!")
}
