// day6/src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 6 Challenge Endpoints

// dependencies
use errors::ApiError;
use hyper::{Body, Request, Response, StatusCode};
use serde_json::json;
use utilities::parser::{parse_elf, parse_elf_on_a_shelf, parse_shelves_with_no_elves};

pub async fn count_elf(request: Request<Body>) -> Result<Response<Body>, ApiError> {
    // get the request body
    let body = request.into_body();

    // convert the body into bytes
    let body_bytes = hyper::body::to_bytes(body).await?;

    // convert the bytes into a string
    let input_string = String::from_utf8(body_bytes.to_vec())?;

    // parse the input string, the result contains all the instances of the word elf
    let num_elves = parse_elf(&input_string).expect("Failed to parse input string");
    let num_elves_on_shelves = parse_elf_on_a_shelf(&input_string).expect("Failed to parse input string");
    let num_shelves_with_no_elves = parse_shelves_with_no_elves(input_string.as_str()).expect("Failed to parse input string");

    // the length of the second element in the tuple is the number of times the word elf appears
    let elf_count = num_elves.1.len();
    let elf_on_a_shelf_count = num_elves_on_shelves.1.len();
    let shelf_with_no_elf_count = num_shelves_with_no_elves.1.len();

    // build the response body
    let obj = json!({
        "elf": elf_count,
        "elf on a shelf": elf_on_a_shelf_count,
        "shelf with no elf on it": shelf_with_no_elf_count,
    });
    let response_body = serde_json::to_string(&obj)?;
    // return the reindeer strength
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(response_body))?)
}
