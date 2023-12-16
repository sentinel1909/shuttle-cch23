// src/bin/httpd.rs

// dependencies
use cch23_sentinel1909::cch23service::Cch23Service;
use cch23_sentinel1909::startup::startup_service;

// main function, launches the app
#[shuttle_runtime::main]
async fn main() -> shuttle_tower::ShuttleTower<Cch23Service> {
    // call the main Tower service which will run the app
    startup_service()
}
