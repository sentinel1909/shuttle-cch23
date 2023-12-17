// minus1/src/fake_error.rs

// dependencies
use common_features::WebRequest;
use hyper::StatusCode;
use std::convert::Infallible;

// handler function for the root, index endpoint
pub async fn svc_fake_error(_request: WebRequest) -> Result<StatusCode, Infallible> {
    Ok(StatusCode::INTERNAL_SERVER_ERROR)
}