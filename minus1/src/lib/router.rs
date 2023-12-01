// src/lib/router.rs

// dependencies
use day1_endpoints;
use crate::routes::not_found::not_found;
use crate::routes::root::root;
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
        let response = match _req.uri().path() {
            "/" => match root() {
                Ok(resp) => resp,
                Err(_) => self.internal_server_error(),
            },
            "/-1/error" => self.internal_server_error(),
            _ => match not_found() {
                Ok(resp) => resp,
                Err(_) => self.internal_server_error(),
            },
        };

        let fut = async { Ok(response) };

        Box::pin(fut)
    }
}
