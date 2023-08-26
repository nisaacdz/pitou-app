mod options;
use crate::Pitou;
pub use options::*;
use serde::{Deserialize, Serialize};
use std::collections::LinkedList;

#[cfg(feature = "tauri")]
mod extra;

#[cfg(feature = "tauri")]
pub use extra::*;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum SearchMsg {
    Searching(LinkedList<Pitou>),
    Terminated(LinkedList<Pitou>),
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum DeleteMsg {
    Pending,
    Completed,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum CopyMsg {
    Copying,
    Completed,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum MoveMsg {
    Moving,
    Completed,
}
