// utilities/src/body_bytes.rs

// dependencies
use common_features::WebRequest;
use hyper::{body::to_bytes, body::Bytes};
use std::convert::Infallible;

// utility function to convert a request body to bytes
pub async fn body_to_bytes(request: WebRequest) -> Result<Bytes, Infallible> {
    // get the request body
    let body = request.into_body();

    // convert the body into bytes
    let body_bytes = to_bytes(body).await.unwrap();

    // return the bytes
    Ok(body_bytes)
}
