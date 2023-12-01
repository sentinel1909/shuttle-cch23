// src/lib/service.rs

// dependencies
use crate::router::Router;

pub fn startup_service() -> shuttle_tower::ShuttleTower<Router> {
    // Create a new Router instance
    let service = Router::create();

    Ok(service.into())
}
