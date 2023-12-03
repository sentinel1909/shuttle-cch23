// src/lib/router.rs

// dependencies
use crate::routes::root::root;
use day1_endpoints::recalibrate;
use hyper::{Body, Request, Response, StatusCode};
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::Service;

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

    fn bad_request(&self) -> Response<Body> {
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Bad Request"))
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

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let url = &request.uri().to_string();
        let path = &request.uri().path();
        let values: Vec<Result<i32, _>> = path.split('/').map(|s| s.parse::<i32>()).collect();
        let path_segments: Vec<i32> = values.into_iter().filter_map(|value| value.ok()).collect();
        if path_segments.len() > 1 {
            let _dyn_path = format!("/1/{}/{}", path_segments[1], path_segments[2]);
        }
        let response = match url.as_str() {
            "/" => match root() {
                Ok(resp) => resp,
                Err(_) => self.internal_server_error(),
            },
            "/-1/error" => self.internal_server_error(),
            _dyn_path if path_segments.len() == 3 => match recalibrate(path_segments[1], path_segments[2]) {
                Ok(resp) => resp,
                Err(_) => self.bad_request(),
            },
            _ => self.bad_request(),
        };

        let fut = async { Ok(response) };

        Box::pin(fut)
    }
}
