use crate::Pitou;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Favorites {
    pub values: Vec<Pitou>,
}

#[derive(Serialize, Deserialize)]
pub struct Recents {
    pub values: Vec<Pitou>,
}

#[cfg(feature = "tauri")]
mod extra;

#[cfg(feature = "tauri")]
pub use extra::*;