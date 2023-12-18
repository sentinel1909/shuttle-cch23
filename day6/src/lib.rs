// day6/src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 6 Challenge Endpoints

// dependencies
use common_features::WebRequest;
use serde_json::json;
use std::convert::Infallible;
use utilities::body_bytes::body_to_bytes;
use utilities::parser::parse_elf;

// endpoint which counts the number of times the word "elf" appears in a phrase
pub async fn svc_count_elf(request: WebRequest) -> Result<String, Infallible> {
    // convert the incoming request body into a string
    let input_string = String::from_utf8(body_to_bytes(request).await.unwrap().to_vec()).unwrap();

    // parse the input string, the result contains all the instances of the word elf
    let num_elves = parse_elf(&input_string).expect("Failed to parse input string");

    // the length of the second element in the tuple is the number of times the word elf appears
    let elf_count = num_elves.1.len();

    // build the response message
    let obj = json!({
        "elf":elf_count,
    });

    let response_msg = serde_json::to_string(&obj).unwrap();

    // return the reindeer strength
    Ok(response_msg)
}
