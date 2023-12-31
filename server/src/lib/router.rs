// src/lib/router.rs

// dependenciescrate::routes
use day1_endpoints::{calibrate_packet_ids, calibrate_sled_ids};
use day4_endpoints::{get_contest_result, get_strength_result};
use day5_endpoints::grinch;
use day6_endpoints::count_elf;
use day7_endpoints::{bake, decode_the_receipe};
use day8_endpoints::get_weight;
use errors::{bad_request, internal_server_error, not_found};
use hyper::{Body, Request, Response};
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
                let _dyn_path = values
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
                _dyn_path if method == "GET" && values.len() == 3 && values[0] == 1 => {
                    match calibrate_packet_ids(values) {
                        Ok(resp) => resp,
                        Err(_) => bad_request(),
                    }
                }
                _dyn_path if method == "GET" && values.len() < 21 && values[0] == 1 => {
                    match calibrate_sled_ids(values) {
                        Ok(resp) => resp,
                        Err(_) => bad_request(),
                    }
                }
                "/4/strength"
                    if method == "POST" && request.headers().contains_key("content-type") =>
                {
                    match get_strength_result(request).await {
                        Ok(resp) => resp,
                        Err(_) => bad_request(),
                    }
                }
                "/4/contest"
                    if method == "POST" && request.headers().contains_key("content-type") =>
                {
                    match get_contest_result(request).await {
                        Ok(resp) => resp,
                        Err(_) => bad_request(),
                    }
                }
                "/5/grinch" if method == "GET" => match grinch() {
                    Ok(resp) => resp,
                    Err(_) => bad_request(),
                },
                "/6" if method == "POST" && request.headers().contains_key("content-type") => {
                    match count_elf(request).await {
                        Ok(resp) => resp,
                        Err(_) => bad_request(),
                    }
                }
                "/7/decode" if method == "GET" && request.headers().contains_key("cookie") => {
                    match decode_the_receipe(request).await {
                        Ok(resp) => resp,
                        Err(_) => bad_request(),
                    }
                }
                "/7/bake" if method == "GET" && request.headers().contains_key("cookie") => {
                    match bake(request).await {
                        Ok(resp) => resp,
                        Err(_) => bad_request(),
                    }
                }
                _dyn_path if method == "GET" && values.len() == 2 && values[0] == 8 => match get_weight(values[1]).await {
                    Ok(resp) => resp,
                    Err(_) => bad_request(),
                },
                _ => not_found(),
            };

            Ok(response)
        };

        Box::pin(fut)
    }
}
