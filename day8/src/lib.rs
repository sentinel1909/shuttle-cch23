// day6/src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 8 Challenge Endpoints

// dependencies
use errors::ApiError;
use domain::PokemonWeight;
use hyper::{Body, Response, StatusCode};
use reqwest::Client;

// endpoint which returns the weight in kilograms of a Pokemon
pub async fn get_weight(pokedex_number: i32) -> Result<Response<Body>, ApiError> {
    
    // create a reqwest client
    let client = Client::new();

    // get the weight in kilograms of the Pokemon
    let pokemon = client.get(&format!("https://pokeapi.co/api/v2/pokemon/{}", pokedex_number))
        .send()
        .await?
        .json::<PokemonWeight>()
        .await?;

    // create a response body
    let response_msg = (pokemon.weight / 10f32).to_string();

    // return the response
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(response_msg))?)

}
