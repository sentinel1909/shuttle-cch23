// src/lib/startup.rs

// dependencies
use crate::cch23service::Cch23Service;
use tower::ServiceBuilder;

// function to Shuttleize the main service
pub fn startup_service() -> shuttle_tower::ShuttleTower<Cch23Service> {
    Ok(ServiceBuilder::new().service(Cch23Service).into())
}
