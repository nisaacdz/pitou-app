use crate::Path;
use std::collections::LinkedList;

static mut CLIPBOARD: LinkedList<Vec<Path>> = LinkedList::new();
static mut SHOULD_CUT: bool = false;

pub enum Item<T> {
    None,
    Cut(T),
    Copied(T),
}

pub fn content() -> &'static LinkedList<Vec<Path>> {
    unsafe { &CLIPBOARD }
}

pub fn clear() {
    unsafe {
        SHOULD_CUT = false;
        CLIPBOARD.clear()
    }
}

pub fn drop(idx: usize) {
    unsafe {
        if idx == 0 {
            SHOULD_CUT = false;
        }
        CLIPBOARD = CLIPBOARD.iter().skip(idx).map(Clone::clone).collect();
    }
}

pub(super) fn get() -> Item<&'static Vec<Path>> {
    unsafe {
        if let Some(v) = CLIPBOARD.back() {
            if SHOULD_CUT {
                SHOULD_CUT = false;
                Item::Cut(v)
            } else {
                Item::Copied(v)
            }
        } else {
            Item::None
        }
    }
}

pub(super) fn put(item: Item<Vec<Path>>) {
    match item {
        Item::None => (),
        Item::Cut(vals) => unsafe {
            CLIPBOARD.push_back(vals);
            SHOULD_CUT = true;
        },
        Item::Copied(vals) => unsafe { CLIPBOARD.push_back(vals) },
    }
}
