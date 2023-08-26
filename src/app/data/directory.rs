use backend::Pitou;

static mut DIRECTORY: Option<Pitou> = None;

pub fn set(newdir: Pitou) {
    unsafe { DIRECTORY = Some(newdir) }
}

pub fn get() -> Option<&'static Pitou> {
    unsafe { DIRECTORY.as_ref() }
}
