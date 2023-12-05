// src/lib/domain/reindeers.rs

// dependencies
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Reindeer {
    pub name: String,
    pub strength: i32,
}
