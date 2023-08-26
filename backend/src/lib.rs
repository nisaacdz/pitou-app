mod fs;
pub use fs::*;

mod database;
pub use database::*;

mod handles;
pub use handles::*;

mod json;
pub use json::*;

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
