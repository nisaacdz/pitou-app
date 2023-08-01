use backend::Pitou;

static mut SELECTED: Option<Vec<Pitou>> = None;
static mut DIRECTORY: Option<Pitou> = None;
static mut PARENT_DIR: Option<Pitou> = None;

pub fn screen_size() -> (usize, usize) {
    
    todo!()
}

pub fn update_selected<I: Iterator<Item = Pitou>>(newval: Option<I>) {
    let newval = match newval {
        Some(v) => {
            let res = v.collect::<Vec<_>>();
            if res.len() == 0 {
                None
            } else {
                Some(res)
            }
        }
        None => None,
    };

    unsafe { SELECTED = newval }
}

pub fn reset_selected() {
    unsafe { SELECTED = None }
}

pub fn update_directory(newval: Option<Pitou>) {
    unsafe { DIRECTORY = newval }
}

pub fn update_parent_dir(newval: Option<Pitou>) {
    unsafe { PARENT_DIR = newval }
}

pub fn get_selected() -> Option<Vec<Pitou>> {
    unsafe { SELECTED.take() }
}

pub fn selected() -> Option<&'static Vec<Pitou>> {
    unsafe { SELECTED.as_ref() }
}

pub fn directory() -> Option<&'static Pitou> {
    unsafe { DIRECTORY.as_ref() }
}

pub fn parent_dir() -> &'static Option<Pitou> {
    unsafe { &PARENT_DIR }
}
