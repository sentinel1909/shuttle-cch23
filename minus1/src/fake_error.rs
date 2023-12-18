// minus1/src/fake_error.rs

// dependencies
use common_features::WebRequest;
use hyper::StatusCode;
use std::convert::Infallible;

// endpoint handler function for Task 2, returns a 500 Internal Server Error status code
pub async fn svc_fake_error(_request: WebRequest) -> Result<StatusCode, Infallible> {
    Ok(StatusCode::INTERNAL_SERVER_ERROR)
}
