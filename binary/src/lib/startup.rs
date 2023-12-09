// src/lib/startup.rs

// dependencies
use crate::router::Router;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;

pub fn startup_service() -> shuttle_tower::ShuttleTower<Router> {
    // Create a new Router instance
    let router_service = Router::create();

    // Build a new service with the TraceLayer
    ServiceBuilder::new()
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
        .service(&router_service);

    Ok(router_service.into())
}
