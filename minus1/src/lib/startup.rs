// src/lib/startup.rs

// dependencies
use crate::router::Router;
// use tower::ServiceBuilder;
// use tower_http::trace::TraceLayer;

pub fn startup_service() -> shuttle_tower::ShuttleTower<Router> {
    // Create a new Router instance
    let router_service = Router::create();

    // Add tracing middleware to the router service
    // let wrapped_router_service = ServiceBuilder::new()
    //    .layer(TraceLayer::new_for_http())
    //    .service(&router_service);

    Ok(router_service.into())
}
