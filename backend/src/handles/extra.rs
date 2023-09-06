use super::SearchMsg;
use crate::File;
use std::{
    collections::LinkedList,
    sync::{Arc, Mutex},
};

/// TODO
/// TODO
/// I need to handle the error case on mutex.lock
///

pub fn new_search_stream() -> SearchStream {
    let mtx = Arc::new(Mutex::new(SearchMsg::Searching(LinkedList::new())));
    SearchStream { mtx }
}

#[derive(Clone)]
pub struct SearchStream {
    mtx: Arc<Mutex<SearchMsg>>,
}

impl SearchStream {
    pub fn terminate(&self) {
        let mut lock = self.mtx.lock().expect("search stream mutex poisened");
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

    pub fn ended(&self) -> bool {
        matches!(
            &*self.mtx.lock().expect("mutex guard poisened"),
            SearchMsg::Terminated(_)
        )
    }

    /// pushes a new item into the stream returning false if the stream has already terminated
    pub fn push(&self, item: File) -> bool {
        let mut lock = self.mtx.lock().expect("mutex guard poisened");
        match &mut *lock {
            SearchMsg::Searching(items) => {
                items.push_back(item);
                true
            }
            SearchMsg::Terminated(_) => false,
        }
    }

    pub fn pull(&self) -> SearchMsg {
        let mut lock = self.mtx.lock().expect("mutex guard poisened");
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
