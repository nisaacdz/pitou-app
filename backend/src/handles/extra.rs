use std::{collections::LinkedList, sync::Arc};

use tokio::sync::Mutex;

use super::SearchMsg;
use crate::Pitou;

pub fn new_search_stream() -> SearchStream {
    let mtx = Arc::new(Mutex::new(SearchMsg::Searching(LinkedList::new())));
    SearchStream { mtx }
}

#[derive(Clone)]
pub struct SearchStream {
    mtx: Arc<Mutex<SearchMsg>>,
}

impl SearchStream {
    pub async fn terminate(&self) {
        let mut lock = self.mtx.lock().await;
        match &mut *lock {
            SearchMsg::Searching(items) => {
                if items.len() == 0 {
                    *lock = SearchMsg::Terminated(LinkedList::new())
                } else {
                    *lock = SearchMsg::Terminated(items.split_off(0));
                }
            }
            SearchMsg::Terminated(_) => (),
        }
    }

    pub async fn ended(&self) -> bool {
        matches!(&*self.mtx.lock().await, SearchMsg::Terminated(_))
    }

    /// pushes a new item into the stream returning false if the stream has already terminated
    pub async fn push(&self, item: Pitou) -> bool {
        let mut lock = self.mtx.lock().await;
        match &mut *lock {
            SearchMsg::Searching(items) => {
                items.push_back(item);
                true
            }
            SearchMsg::Terminated(_) => false,
        }
    }

    pub async fn pull(&self) -> SearchMsg {
        println!("pull invoked!");
        let mut lock = self.mtx.lock().await;
        println!("Mutex guard received!");
        match &mut *lock {
            SearchMsg::Searching(items) => {
                SearchMsg::Searching(std::mem::replace(items, LinkedList::new()))
            }
            SearchMsg::Terminated(items) => {
                SearchMsg::Terminated(std::mem::replace(items, LinkedList::new()))
            }
        }
    }
}
