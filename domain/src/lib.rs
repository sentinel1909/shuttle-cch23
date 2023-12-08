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
    pub speed: f32,
    pub height: i32,
    pub antler_width: i32,
    pub snow_magic_power: i32,
    pub favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    pub candies_eaten_yesterday: i32,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq, Serialize)]
pub struct ContestWinners {
    pub fastest: String,
    pub tallest: String,
    pub magician: String,
    pub consumer: String,
}
