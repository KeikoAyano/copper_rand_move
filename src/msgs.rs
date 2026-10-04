/*
    define all payloads
*/

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, Encode, Decode, Serialize, Deserialize, cu29::prelude::Reflect)]
pub struct Location {
    // the location
    x: f64,
    y: f64,
}

impl Location {
    pub fn new(x: f64, y: f64) -> Location {
        Location{x, y}
    }
    pub fn show(&self) {
        println!("Location: ({}, {})", self.x, self.y);
    }
}