mod selections;
use backend::{File, SearchOptions};
pub use selections::*;

use std::{path::PathBuf, rc::Rc};

static mut DIRECTORY: Option<Rc<PathBuf>> = None;
static mut SEARCH_RESULTS: Option<Rc<Vec<File>>> = None;
static mut SEARCH_OPTIONS: Option<(Rc<String>, SearchOptions)> = None;

pub fn update_search_results(results: Rc<Vec<File>>) {
    unsafe {
        SEARCH_RESULTS = Some(results);
    }
}

pub fn update_search_options(options: SearchOptions, input: Rc<String>) {
    unsafe { SEARCH_OPTIONS = Some((input, options)) }
}

pub fn search_options() -> Option<(Rc<String>, SearchOptions)> {
    unsafe { SEARCH_OPTIONS.clone() }
}

pub fn search_results() -> Option<Rc<Vec<File>>> {
    unsafe { SEARCH_RESULTS.clone() }
}

pub fn update_directory(newval: Option<Rc<PathBuf>>) {
    unsafe { DIRECTORY = newval }
}

pub fn directory() -> Option<Rc<PathBuf>> {
    unsafe { DIRECTORY.clone() }
}

#[allow(unused)]
const UNIQUE_LEN: usize = 5;

/// returns a new unique string that can be usefull for representing keys of components
#[allow(unused)]
pub fn unique() -> String {
    generate_string(UNIQUE_LEN)
}

mod rand {
    use std::ops::{Add, Range, Rem, Sub};

    pub trait FromI32 {
        fn from_u64(val: i32) -> Self;
    }

    pub trait Num:
        Sized
        + FromI32
        + std::fmt::Debug
        + Default
        + PartialEq
        + Copy
        + Add<Output = Self>
        + Sub<Output = Self>
        + Rem<Output = Self>
    {
    }

    macro_rules! impl_num_for {
        ($($t:ty),*) => {
            $(impl Num for $t {})*
        };
    }

    macro_rules! impl_from_i32 {
        ($($val:ty),*) => {
            $(impl FromI32 for $val {
                fn from_u64(arg: i32) -> $val {
                    arg as $val
                }
            })*
        };
    }

    impl_from_i32!(i32, u32, u8);
    impl_num_for!(i32, u32, u8);

    static mut RAND: i32 = i32::MAX;
    pub fn get_random() -> i32 {
        let cur = unsafe { RAND };

        let res = cur
            .overflowing_mul(i32::MAX)
            .0
            .overflowing_add(i32::MAX)
            .0
            .overflowing_mul(cur)
            .0
            .overflowing_add(cur)
            .0;
        unsafe {
            RAND = res;
        }

        res
    }

    pub fn gen_range<T: Num>(rng: Range<T>) -> T {
        let len = rng.end - rng.start;
        assert_ne!(len, T::default());
        rng.start + T::from_u64(get_random()) % len
    }

    pub fn random() -> bool {
        get_random() > 0
    }
}

/// generates a new string with the specified length
pub fn generate_string(len: usize) -> String {
    let mut res = String::with_capacity(len);
    for _ in 0..len {
        let use_nums = rand::random();
        let next = match use_nums {
            true => rand::gen_range(0..10) + 48u8,
            false => {
                let rdval = rand::gen_range(0..26);

                let use_caps = rand::random();

                rdval + if use_caps { 65 } else { 97 }
            }
        };

        res.push(next as char)
    }
    res
}

pub use mutex::SharedBorrow;

mod mutex {
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