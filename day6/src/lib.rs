// day6/src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 6 Challenge Endpoints

// dependencies
use common_features::WebRequest;
use serde_json::json;
use std::convert::Infallible;
use utilities::parser::parse_elf;

pub async fn svc_count_elf(request: WebRequest) -> Result<String, Infallible> {
    // get the request body
    let body = request.into_body();

    // convert the body into bytes
    let body_bytes = hyper::body::to_bytes(body).await.unwrap();

    // convert the bytes into a string
    let input_string = String::from_utf8(body_bytes.to_vec()).unwrap();

    // parse the input string, the result contains all the instances of the word elf
    let num_elves = parse_elf(&input_string).expect("Failed to parse input string");

    // the length of the second element in the tuple is the number of times the word elf appears
    let elf_count = num_elves.1.len();

    // build the response body
    let obj = json!({
        "elf":elf_count,
    });
    let response_msg = serde_json::to_string(&obj).unwrap();
    // return the reindeer strength
    Ok(response_msg)
}
