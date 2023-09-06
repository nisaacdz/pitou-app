use backend::File;
use std::{cell::RefCell, collections::{LinkedList, HashSet}, rc::Rc};

static mut SELECTED: Option<Rc<RefCell<HashSet<File>>>> = None;

static mut PERSISTED: LinkedList<File> = LinkedList::new();

pub fn init_selections(newval: Rc<RefCell<HashSet<File>>>) {
    unsafe {
        SELECTED = Some(newval);
    }
}

pub fn persist(file: File) {
    unsafe {
        PERSISTED.push_back(file)
    }
}

pub fn get_persistent() -> impl Iterator<Item = File>{
    unsafe {
        if PERSISTED.is_empty() { LinkedList::new().into_iter() } else { PERSISTED.split_off(0).into_iter() }
    }
}

pub fn all() -> Option<Rc<RefCell<HashSet<File>>>> {
    unsafe { SELECTED.clone() }
}
