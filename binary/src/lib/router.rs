// src/lib/router.rs

// dependenciescrate::routes
use day1_endpoints::{calibrate_packet_ids, calibrate_sled_ids};
use day4_endpoints::{get_contest_result, get_strength_result};
use day5_endpoints::grinch;
use hyper::{Body, Request, Response, StatusCode};
use minus1_endpoint::root;
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::Service;

// a struct type to represent the app router
#[derive(Clone)]
pub struct Router;

// implementation block for the Router type; instantiates and provides a general  internal server error
impl Router {
    pub fn create() -> Self {
        Self
    }
}

// function to return a general internal server error
fn internal_server_error() -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from("Internal Server Error"))
        .unwrap()
}

// function to return a general not found error
fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Not Found"))
        .unwrap()
}

// function to return a general bad request error
fn bad_request() -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from("Bad Request"))
        .unwrap()
}

// implement the Tower Service trait for the Router type
impl Service<Request<Body>> for Router {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + Sync>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let fut = async move {
            // get the url from the request, convert it to a string
            let url = request.uri().to_string();

            // get the method from the request, convert it to a string
            let method = request.method().to_string();

            // get the path portion from the request url
            let path = request.uri().path();

            // split the path into segments, parse the segments into i32 values
            let path_segments: Vec<Result<i32, _>> = path
                .split('/')
                .map(|segment| segment.parse::<i32>())
                .collect();

            // filter out any segments that are not Ok, collect the remaining values into a vector
            let values: Vec<i32> = path_segments
                .into_iter()
                .filter_map(|path_segment| path_segment.ok())
                .collect();

            // create a dynamic path based on the values in the vector
            if values.len() > 1 {
                let _calibrate_url = values
                    .iter()
                    .map(|&value| value.to_string())
                    .collect::<Vec<String>>()
                    .join("/");
            }

            // match the url against the routes
            let response = match url.as_str() {
                "/" if method == "GET" => match root() {
                    Ok(resp) => resp,
                    Err(_) => internal_server_error(),
                },
                "/-1/error" => internal_server_error(),
                _calibrate_url if method == "GET" && values.len() == 3 && values[0] == 1 => {
                    match calibrate_packet_ids(values) {
                        Ok(resp) => resp,
                        Err(_) => bad_request(),
                    }
                }
                _calibrate_url if method == "GET" && values.len() < 21 && values[0] == 1 => {
                    match calibrate_sled_ids(values) {
                        Ok(resp) => resp,
                        Err(_) => bad_request(),
                    }
                }
                "/4/strength" if method == "POST" => match get_strength_result(request).await {
                    Ok(resp) => resp,
                    Err(_) => bad_request(),
                },
                "/4/contest" if method == "POST" => match get_contest_result(request).await {
                    Ok(resp) => resp,
                    Err(_) => bad_request(),
                },
                "/5/grinch" if method == "GET" => match grinch() {
                    Ok(resp) => resp,
                    Err(_) => internal_server_error(),
                },
                _ => not_found(),
            };

            Ok(response)
        };

        Box::pin(fut)
    }
}
