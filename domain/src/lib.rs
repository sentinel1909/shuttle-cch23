// src/lib.rs

// domain data structures

// dependencies
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct StrengthData {
    pub name: String,
    pub strength: i32,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ContestData {
    pub name: String,
    pub strength: i32,
    pub speed: i32,
    pub height: i32,
    pub antler_width: i32,
    pub snow_magic_power: i32,
    pub favourite_food: String,
    pub candies_eaten_yesterday: i32,
}
