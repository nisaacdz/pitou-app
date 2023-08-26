use backend::{CopyMsg, DeleteMsg, MoveMsg, SearchMsg};
use serde_wasm_bindgen::{from_value, to_value};

use super::invoke;
use crate::app::PitouNoArg;
use backend::Pitou;
use std::time::{Duration, Instant};

pub static mut DELETE_TASK: Option<DeleteMsg> = None;
static mut SEARCH_TASK2: Option<SearchMsg> = None;
pub static mut COPY_TASK: Option<CopyMsg> = None;
pub static mut MOVE_TASK: Option<MoveMsg> = None;

static mut SEARCH_RESULTS: Option<Vec<Pitou>> = None;

pub async fn terminate_search_stream() {
    let arg = to_value(&PitouNoArg).unwrap();
    invoke("reset_search_stream", arg).await;
}

pub async fn read_search_stream() -> Option<SearchMsg> {
    let arg = to_value(&PitouNoArg).unwrap();
    let res = invoke("read_search_stream", arg).await;
    from_value(res).unwrap()
}

// pub fn monitor_background_search() {
//     spawn_local(async move {
//         loop {
//             let res = read_search_stream().await;
//             match res {
//                 SearchMsg::Searching => unsafe {
//                     SEARCH_RESULTS.get_or_insert(Vec::new());
//                 },
//                 SearchMsg::Progressing(items) => {
//                     for item in &items {
//                         println!("front-end found new result: {}", item.path().display())
//                     }
//                     append_search_results(items);
//                 }
//                 SearchMsg::Terminated => {
//                     terminate_background_search().await;
//                     break;
//                 }
//             }
//             wait(Duration::from_millis(100)).await;
//         }
//     });
// }
