// src/lib/router.rs

// dependencies
use crate::routes::not_found::not_found;
use crate::routes::root::root;
use day1_endpoints::recalibrate;
use hyper::{Body, Request, Response, StatusCode};
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::Service;
use url::Url;

// a struct type to represent the app router
#[derive(Clone)]
pub struct Router;

// implementation block for the Router type; instantiates and provides a general  internal server error
impl Router {
    pub fn create() -> Self {
        Self
    }

    fn internal_server_error(&self) -> Response<Body> {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Internal Server Error"))
            .unwrap()
    }
}

// implement the Tower Service trait for the Router type
impl Service<Request<Body>> for Router {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + Sync>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Request<Body>) -> Self::Future {
        let url = _req.uri().path();
        let parsed_url = Url::parse(url).unwrap();
        let url_path_segments = parsed_url
            .path_segments()
            .map(|c| c.collect::<Vec<_>>())
            .expect("Unable to extract path segments from URL");
        let response = match url {
            "/" => match root() {
                Ok(resp) => resp,
                Err(_) => self.internal_server_error(),
            },
            "/-1/error" => self.internal_server_error(),
            "/1/" => match recalibrate(url_path_segments[1], url_path_segments[2]) {
                Ok(resp) => resp,
                Err(_) => self.internal_server_error(),
            },
            _ => match not_found() {
                Ok(resp) => resp,
                Err(_) => self.internal_server_error(),
            },
        };

        let fut = async { Ok(response) };

        Box::pin(fut)
    }
}
