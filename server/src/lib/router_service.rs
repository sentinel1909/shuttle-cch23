// src/lib/router.rs

// dependencies
use crate::router::Router;
use common_features::{IntoWebResponse, WebRequest, WebResponse};
use hyper::StatusCode;
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::{Service, ServiceExt};
use utilities::parameter_extractor;

// implement the Tower Service trait for the router type
impl Service<WebRequest> for Router {
    type Response = WebResponse;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: WebRequest) -> Self::Future {
        self.parameters = parameter_extractor(request.uri().path());
        match self
            .endpoints
            .get(&(request.method().clone(), request.uri().path().to_string()))
        {
            Some(svc) => {
                let mut svc = svc.clone();
                let fut = async move {
                    let ready_svc = match svc.ready().await {
                        Ok(svc) => svc,
                        Err(_) => return Ok(StatusCode::TOO_MANY_REQUESTS.into_web_response()),
                    };

                    match ready_svc.call(request).await {
                        Ok(res) => Ok(res),
                        Err(e) => Ok(e),
                    }
                };
                Box::pin(fut)
            }
            None => Box::pin(async { Ok(StatusCode::NOT_FOUND.into_web_response()) }),
        }
    }
}
