use crate::File;
use serde::{Deserialize, Serialize};
use std::collections::LinkedList;

#[cfg(feature = "tauri")]
mod extra;

#[cfg(feature = "tauri")]
pub use extra::*;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum SearchMsg {
    Searching(LinkedList<File>),
    Terminated(LinkedList<File>),
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
