// day7/src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 7 Challenge Endpoints

// dependencies
use base64::engine::general_purpose;
use base64::Engine;
use common_features::WebRequest;
use std::convert::Infallible;

// endpoint function to decode the cookie receipe and send it back to the client
pub async fn svc_decode_the_receipe(request: WebRequest) -> Result<String, Infallible> {
    // get the cookie from the request
    let cookie = request.headers().get("cookie").unwrap().to_str().unwrap();

    // get the cookie value
    let cookie_value = cookie.split('=').collect::<Vec<&str>>()[1];

    // decode the cookie value
    let decoded_cookie = general_purpose::STANDARD_NO_PAD
        .decode(cookie_value)
        .unwrap();

    // convert the decoded cookie to a string
    let decoded_recipe = String::from_utf8(decoded_cookie).unwrap();

    // build the response body
    let response_msg = serde_json::to_string(&decoded_recipe).unwrap();

    // return the response
    Ok(response_msg)
}
