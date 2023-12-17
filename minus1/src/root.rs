// minus1/src/root.rs

// dependencies
use common_features::WebRequest;
use std::convert::Infallible;

// handler function for the root, index endpoint
pub async fn svc_root(_request: WebRequest) -> Result<&'static str, Infallible> {
    Ok("Hello, World!")
}
