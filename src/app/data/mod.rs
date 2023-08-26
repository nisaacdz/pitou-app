use backend::Pitou;
pub mod selections;

static mut DIRECTORY: Option<Pitou> = None;

pub fn update_directory(newval: Option<Pitou>) {
    unsafe { DIRECTORY = newval }
    selections::clear();
}

pub fn directory() -> Option<&'static Pitou> {
    unsafe { DIRECTORY.as_ref() }
}
