mod fs;
pub use fs::*;

mod database;
pub use database::*;

mod handles;
pub use handles::*;

mod json;
pub use json::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "tauri")]
pub mod actions;

pub trait StrOps {
    fn starts_with_ignore_case(&self, key: &str) -> bool;
    fn ends_with_ignore_case(&self, key: &str) -> bool;
    fn contains_ignore_case(&self, key: &str) -> bool;
}

// static RT: OnceCell<Runtime> = OnceCell::const_new();

// async fn get_rt() -> &'static Runtime {
//     async fn init_rt() -> Runtime {
//         Builder::new_multi_thread().thread_stack_size(5 * 1024 * 1024).build().unwrap()
//     }
//     RT.get_or_init(init_rt).await
// }

impl<T> StrOps for T
where
    T: AsRef<str>,
{
    fn starts_with_ignore_case(&self, key: &str) -> bool {
        let whole = self.as_ref();
        if key.len() > whole.len() {
            return false;
        }
        whole
            .chars()
            .take(key.len())
            .zip(key.chars())
            .all(|(w, k)| w.eq_ignore_ascii_case(&k))
    }
    fn contains_ignore_case(&self, key: &str) -> bool {
        let whole = self.as_ref();
        let key = key.as_bytes();
        if key.len() > whole.len() {
            return false;
        }

        whole.as_bytes().windows(key.len()).any(|window| {
            (0..key.len())
                .all(|idx| (key[idx] as char).eq_ignore_ascii_case(&(window[idx] as char)))
        })
    }
    fn ends_with_ignore_case(&self, key: &str) -> bool {
        let whole = self.as_ref();
        if key.len() > whole.len() {
            return false;
        }

        whole
            .bytes()
            .rev()
            .take(key.len())
            .zip(key.bytes().rev())
            .all(|(a, b)| (a as char).eq_ignore_ascii_case(&(b as char)))
    }
}

/// Represents specific patterns to exclude from a list of files
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Filter {
    pub dot_hidden: bool,
    pub sys_hidden: bool,
    pub dir: bool,
    pub link: bool,
    pub file: bool,
}

impl Filter {
    pub const DEFAULT: Self = Self {
        dot_hidden: true,
        sys_hidden: true,
        dir: false,
        link: false,
        file: false,
    };

    /// Returns true if the given file should be excluded and false otherwise
    pub fn exclude(self, file: &File) -> bool {
        (self.dot_hidden && file.name().starts_with('.'))
            || (self.sys_hidden && file.name().starts_with('~'))
            || (self.file && file.metadata().is_file())
            || (self.dir && file.metadata().is_dir())
            || (self.link && file.metadata().is_link())
    }

    /// Returns true if the given file should be included and false otherwise
    pub fn include(self, file: &File) -> bool {
        !self.exclude(file)
    }
}

impl Default for Filter {
    fn default() -> Self {
        Filter::DEFAULT
    }
}

#[derive(Clone, Copy)]
pub enum SolftFilter {}

#[derive(Clone, Copy)]
pub enum HardFilter {
    DotHidden,
    SystemHidden,
    Both,
}

#[derive(Clone, Copy)]
pub enum Sort {
    Name(bool),
    Modified(bool),
    Accessed(bool),
}
