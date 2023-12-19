// src/bin/httpd.rs

// dependences
use cch23_sentinel1909_server::router::Router;
use common_features::{WebRequest, WebResponse};
use day11_endpoints::svc_static_files;
use day1_endpoints::svc_calibrate_packet_ids;
use day4_endpoints::svc_calculate_total_strength;
use day5_endpoints::svc_mean_grinch;
use day6_endpoints::svc_count_elf;
use day7_endpoints::svc_decode_the_receipe;
use day8_endpoints::svc_get_pokemon_weight;
use hyper::Method;
use minus1_endpoint::{svc_fake_error, svc_root};
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};
use sync_wrapper::SyncFuture;
use tower::{service_fn, util::BoxCloneService, ServiceBuilder, ServiceExt};
use tower_http::normalize_path::NormalizePathLayer;

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
    router.on(
        Method::GET,
        "/1/:packet1/:packet2",
        service_fn(svc_calibrate_packet_ids).boxed_clone(),
    );
    router.on(
        Method::POST,
        "/4/strength",
        service_fn(svc_calculate_total_strength).boxed_clone(),
    );
    router.on(Method::GET, "/5", service_fn(svc_mean_grinch).boxed_clone());

    router.on(Method::POST, "/6", service_fn(svc_count_elf).boxed_clone());

    router.on(
        Method::GET,
        "/7/decode",
        service_fn(svc_decode_the_receipe).boxed_clone(),
    );

    router.on(
        Method::GET,
        "/8/weight/:pokedex",
        service_fn(svc_get_pokemon_weight).boxed_clone(),
    );

    router.on(
        Method::GET,
        "/11/assets/decoration.png",
        service_fn(svc_static_files).boxed_clone(),
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
