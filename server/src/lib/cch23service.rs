// src/lib/router.rs

// dependencies
use crate::router::router;
use hyper::{Body, Request, Response};
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::{Service, ServiceBuilder};
use tower_http::normalize_path::NormalizePathLayer;

// a struct type to represent a wrapper around the Cch23Service type
#[derive(Clone)]
pub struct Cch23Service;

// implement the Default trait for the Router type
impl Default for Cch23Service {
    fn default() -> Self {
        Self
    }
}

// implement the Tower Service trait for the Cch23Sesvice type
impl Service<Request<Body>> for Cch23Service {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + Sync>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
       
        let mut service = ServiceBuilder::new().layer(NormalizePathLayer::trim_trailing_slash()).service_fn(router);
        let response = service.call(request);
        Box::pin(async move { Ok(response.await.unwrap()) })
    }
}
