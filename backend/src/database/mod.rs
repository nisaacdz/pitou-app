use crate::Pitou;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Bookmarks {
    pub values: Vec<Pitou>,
}

#[derive(Serialize, Deserialize)]
pub struct History {
    pub values: Vec<Pitou>,
}

#[cfg(feature = "tauri")]
mod extra;

#[cfg(feature = "tauri")]
pub use extra::*;