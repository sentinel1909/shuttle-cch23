// src/bin/httpd.rs

// dependences
use cch23_sentinel1909_server::router::Router;
use hyper::{Method, Server};
use minus1_endpoint::{svc_root, svc_fake_error};
use std::net::SocketAddr;
use tower::{make::Shared, service_fn, ServiceBuilder, ServiceExt};
use tower_http::normalize_path::NormalizePathLayer;

// function to Shuttleize the main service
#[shuttle_runtime::main]
async fn main() -> shuttle_tower::ShuttleTower<Router> {
    let mut router = Router::default();

    router.on(Method::GET, "/", service_fn(svc_root).boxed_clone());
    router.on(Method::GET, "/-1/error", service_fn(svc_fake_error).boxed_clone());

    ServiceBuilder::new()
        .layer(NormalizePathLayer::trim_trailing_slash())
        .service(router);

    // let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    // let make_service = Shared::new(hyper_service);

    // let server = Server::bind(&addr).serve(make_service);

    // if let Err(e) = server.await {
    //    eprintln!("server error: {}", e);
    // }

    Ok(router.into())
}
