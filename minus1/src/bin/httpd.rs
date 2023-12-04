// src/bin/httpd.rs

// dependencies
use cch23_sentinel1909::router::Router;
use cch23_sentinel1909::startup::startup_service;

// main function, launches the app
#[shuttle_runtime::main]
async fn main() -> shuttle_tower::ShuttleTower<Router> {

    // call the router service
    startup_service()
}
