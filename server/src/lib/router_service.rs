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

// implement the Tower Service trait for the router type
impl Service<WebRequest> for Router {
    type Response = WebResponse;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut request: WebRequest) -> Self::Future {
       let method = request.method();
        let path = request.uri().path().trim_matches('/');

        for endpoint in self.endpoints.iter() {
            if let Some(params) = endpoint.matcher.match_request(method, path.as_ref()) {
                request.extensions_mut().insert(params);
                let mut svc = endpoint.service.clone();
                let fut = async move {
                    let svc = match svc.ready().await {
                        Ok(svc) => svc,
                        Err(_) => return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_web_response()),
                    };
                    match svc.call(request).await {
                        Ok(resp) => Ok(resp),
                        Err(resp) => Ok(resp),
                    }
                };
                return Box::pin(fut);
            }
        }

        let fut = async move { Ok(StatusCode::NOT_FOUND.into_web_response()) };
        Box::pin(fut) 
        
    }
}
