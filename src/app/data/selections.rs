use backend::File;
use std::{cell::RefCell, collections::HashSet, rc::Rc};

static mut SELECTED: Option<Rc<RefCell<HashSet<File>>>> = None;

pub fn init_selections(newval: Rc<RefCell<HashSet<File>>>) {
    unsafe {
        SELECTED = Some(newval);
    }
}

pub fn all() -> Option<Rc<RefCell<HashSet<File>>>> {
    unsafe { SELECTED.clone() }
}
