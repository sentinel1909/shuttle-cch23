// src/lib/startup.rs

// dependencies
use crate::cch23service::Cch23Service;
use tower::ServiceBuilder;

pub fn startup_service() -> shuttle_tower::ShuttleTower<Cch23Service> {
    let cch23_service = Cch23Service;

    ServiceBuilder::new().service(&cch23_service);

    Ok(cch23_service.into())
}
