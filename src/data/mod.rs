use backend::Pitou;
use std::collections::HashSet;

static mut SELECTED: Option<HashSet<Pitou>> = None;
static mut DIRECTORY: Option<Pitou> = None;
static mut PARENT_DIR: Option<Pitou> = None;

#[inline]
pub fn get_or_init_selected() -> &'static mut HashSet<Pitou> {
    unsafe {
        match &mut SELECTED {
            Some(items) => items,
            None => {
                SELECTED = Some(HashSet::new());
                SELECTED.as_mut().unwrap()
            }
        }
    }
}

pub fn clear_selected() {
    get_or_init_selected().clear()
}

pub fn toggle_selected(item: &Pitou) {
    if get_or_init_selected().contains(item) {
        get_or_init_selected().remove(item);
    } else {
        get_or_init_selected().insert(item.clone());
    }
}

pub fn update_directory(newval: Option<Pitou>) {
    unsafe { DIRECTORY = newval }
}

pub fn update_parent_dir(newval: Option<Pitou>) {
    unsafe { PARENT_DIR = newval }
}

pub fn selected() -> impl Iterator<Item = &'static Pitou> {
    get_or_init_selected().iter()
}

pub fn selected_len() -> usize {
    get_or_init_selected().len()
}

pub fn directory() -> Option<&'static Pitou> {
    unsafe { DIRECTORY.as_ref() }
}

pub fn parent_dir() -> &'static Option<Pitou> {
    unsafe { &PARENT_DIR }
}
