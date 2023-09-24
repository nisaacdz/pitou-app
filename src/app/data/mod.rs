// mod selections;
// use backend::{File, SearchOptions};
// pub use selections::*;

// use std::{path::PathBuf, rc::Rc};

// static mut DIRECTORY: Option<Rc<PathBuf>> = None;
// static mut SEARCH_RESULTS: Option<Rc<Vec<File>>> = None;
// static mut SEARCH_OPTIONS: Option<(Rc<String>, SearchOptions)> = None;

// pub fn update_search_results(results: Rc<Vec<File>>) {
//     unsafe {
//         SEARCH_RESULTS = Some(results);
//     }
// }

// pub fn update_search_options(options: SearchOptions, input: Rc<String>) {
//     unsafe { SEARCH_OPTIONS = Some((input, options)) }
// }

// pub fn search_options() -> Option<(Rc<String>, SearchOptions)> {
//     unsafe { SEARCH_OPTIONS.clone() }
// }

// pub fn search_results() -> Option<Rc<Vec<File>>> {
//     unsafe { SEARCH_RESULTS.clone() }
// }

// pub fn update_directory(newval: Option<Rc<PathBuf>>) {
//     unsafe { DIRECTORY = newval }
// }

// pub fn directory() -> Option<Rc<PathBuf>> {
//     unsafe { DIRECTORY.clone() }
// }

// #[allow(unused)]
// const UNIQUE_LEN: usize = 5;

// /// returns a new unique string that can be usefull for representing keys of components
// #[allow(unused)]
// pub fn unique() -> String {
//     generate_string(UNIQUE_LEN)
// }

pub use mtx::SharedBorrow;
pub use rand::generate_string;

mod rand {
    use std::ops::Range;

    static mut RAND: i32 = 1;

    pub fn get_random() -> i32 {
        let cur = unsafe { RAND };
        let mut res = cur
            .overflowing_mul(cur)
            .0
            .overflowing_add(cur)
            .0
            .overflowing_mul(923293493)
            .0
            .overflowing_add(783923092)
            .0;
        if res == cur {
            res = res.overflowing_add(39395789).0
        }

        unsafe { RAND = res }
        res
    }

    pub fn gen_range(rng: Range<i32>) -> i32 {
        let len = rng.end - rng.start;
        assert_ne!(len, 0);
        rng.start + (get_random() % len)
    }

    pub fn random() -> bool {
        get_random() > 0
    }

    /// generates a new string with the specified length
    pub fn generate_string(len: usize) -> String {
        let mut res = String::with_capacity(len);
        for _ in 0..len {
            let use_nums = random();
            let next = match use_nums {
                true => gen_range(0..10) + 48,
                false => {
                    let rdval = gen_range(0..26);
                    let use_caps = random();
                    rdval + if use_caps { 65 } else { 97 }
                }
            };
            res.push(next as u8 as char)
        }
        // crate::app::log(&format!("generated string = {}", &res));
        res
    }
}

mod mtx {
    use std::{cell::UnsafeCell, rc::Rc};

    #[derive(Clone)]
    pub struct SharedBorrow<T> {
        ptr: Rc<UnsafeCell<T>>,
    }

    impl<T> SharedBorrow<T> {
        pub fn new(value: T) -> Self {
            let ptr = Rc::new(UnsafeCell::new(value));

            Self { ptr }
        }
        pub fn as_mut(&self) -> &mut T {
            unsafe { &mut *self.ptr.get() }
        }
    }
}
