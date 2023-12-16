// src/lib/router.rs

// dependencies
use hyper::{Body, Request, Response, StatusCode};
use minus1_endpoint::{fake_error, root};

// define a function to handle routing
pub async fn router(request: Request<Body>) -> Result<Response<Body>, StatusCode> {
    match (&request.method(), &request.uri().path()) {
        (&_get, &"/") => Ok(root(request).await?),
        (&_get, &"/-1/error") => Ok(fake_error(request).await?),
        _ => Ok(Response::builder()
            .status(404)
            .body(Body::from("Not Found"))
            .map_err(|_| StatusCode::NOT_FOUND)?),
    }
}
