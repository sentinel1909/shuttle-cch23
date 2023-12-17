// src/bin/httpd.rs

use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

// dependences
use cch23_sentinel1909_server::router::Router;
use common_features::{WebRequest, WebResponse};
use hyper::Method;
use minus1_endpoint::{svc_fake_error, svc_root};
use sync_wrapper::SyncFuture;
use tower::{service_fn, util::BoxCloneService, ServiceBuilder, ServiceExt};
use tower_http::normalize_path::NormalizePathLayer;

// function to Shuttleize the main service
#[shuttle_runtime::main]
async fn main() -> shuttle_tower::ShuttleTower<SharedRouter> {
    let mut router = Router::default();

    router.on(Method::GET, "/", service_fn(svc_root).boxed_clone());
    router.on(
        Method::GET,
        "/-1/error",
        service_fn(svc_fake_error).boxed_clone(),
    );

    let router = ServiceBuilder::new()
        .layer(NormalizePathLayer::trim_trailing_slash())
        .service(router)
        .boxed_clone();

    let shared_router = SharedRouter {
        router: Arc::new(Mutex::new(router)),
    };

    Ok(shared_router.into())
}

#[derive(Clone)]
struct SharedRouter {
    router: Arc<Mutex<BoxCloneService<WebRequest, WebResponse, Infallible>>>,
}

impl tower::Service<WebRequest> for SharedRouter {
    type Response = WebResponse;
    type Error = Infallible;
    type Future =
        SyncFuture<Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: hyper::Request<hyper::Body>) -> Self::Future {
        let router = self.router.lock().unwrap().clone();
        tokio::pin!(router);
        SyncFuture::new(router.call(req))
    }
}
