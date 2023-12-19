// src/lib/router.rs

// dependencies
use crate::router::Router;
use common_features::{IntoWebResponse, WebRequest, WebResponse};
use hyper::StatusCode;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::{Service, ServiceExt};

// implement the Tower Service trait for the router type
impl Service<WebRequest> for Router {
    type Response = WebResponse;
    type Error = WebResponse;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut request: WebRequest) -> Self::Future {
       let method = request.method();
        let path = request.uri().path().trim_matches('/');

        for endpoint in self.endpoints.iter() {
            if let Some(params) = endpoint.matcher.match_request(method, path.as_ref()) {
                request.extensions_mut().insert(params);
                let svc = endpoint.service.clone();
                let fut = async move { svc.ready().await.call(request).await };
                return Box::pin(fut);
            }
        }

        let fut = async move { Err(StatusCode::NOT_FOUND.into_web_response()) };
        Box::pin(fut) 
        
    }
}
