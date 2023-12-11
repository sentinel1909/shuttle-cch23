// src/lib.rs

// domain data structures

// dependencies
use serde::{Deserialize, Serialize};

// struct to represent the sled data, per the day 4, part 1 challenge
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct StrengthData {
    pub name: String,
    pub strength: i32,
}

// struct to represent the contest data, per the day 4, part 1 challenge
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ContestData {
    pub name: String,
    pub strength: i32,
    pub speed: f32,
    pub height: i32,
    pub antler_width: i32,
    pub snow_magic_power: i32,
    pub favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    pub candies_eaten_yesterday: i32,
}

// struct to represent the kitchen data, per the day 7, part 1 challenge
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Kitchen {
    pub recipe: Recipe,
    pub pantry: Pantry,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Recipe {
    pub flour: i32,
    pub sugar: i32,
    pub butter: i32,
    #[serde(rename = "baking powder")]
    pub baking_powder: i32,
    #[serde(rename = "chocolate chips")]
    pub chocolate_chips: i32,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Pantry {
    pub flour: i32,
    pub sugar: i32,
    pub butter: i32,
    #[serde(rename = "baking powder")]
    pub baking_powder: i32,
    #[serde(rename = "chocolate chips")]
    pub chocolate_chips: i32,
}
