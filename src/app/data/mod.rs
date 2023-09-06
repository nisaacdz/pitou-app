mod selections;
use std::{rc::Rc, path::PathBuf};

pub use selections::*;

static mut DIRECTORY: Option<Rc<PathBuf>> = None;

pub fn update_directory(newval: Option<Rc<PathBuf>>) {
    unsafe { DIRECTORY = newval }
}

pub fn directory() -> Option<Rc<PathBuf>> {
    unsafe { DIRECTORY.clone() }
}

const UNIQUE_LEN: usize = 10;

/// returns a new unique string that can be usefull for representing keys of components
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
