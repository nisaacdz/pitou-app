pub mod clipboard;

#[cfg(feature = "tauri")]
mod search;

#[cfg(feature = "tauri")]
pub use search::*;

use std::{io, path::PathBuf};

use fs_extra::dir::CopyOptions;
use tokio::fs;

use self::clipboard::Item;

use super::Pitou;

impl Pitou {
    pub fn delete(items: Vec<Self>) -> io::Result<()> {
        trash::delete_all(items).unwrap();
        Ok(())
    }

    pub fn copy(items: Vec<Self>) {
        clipboard::put(Item::Copied(items))
    }

    pub fn cut(items: Vec<Self>) {
        clipboard::put(Item::Cut(items))
    }

    pub fn paste(directory: Self) -> io::Result<()> {
        match clipboard::get() {
            clipboard::Item::None => Ok(()),
            clipboard::Item::Cut(items) => {
                let _res = fs_extra::move_items(items, directory, &CopyOptions::new()).unwrap();
                Ok(())
            }
            clipboard::Item::Copied(items) => {
                let _res = fs_extra::copy_items(items, directory, &CopyOptions::new()).unwrap();
                Ok(())
            }
        }
    }

    pub fn open(_file: Self) -> io::Result<()> {
        todo!()
    }

    pub fn open_with(_file: Self) -> io::Result<()> {
        todo!()
    }

    pub fn share(_file: Self) -> io::Result<()> {
        todo!()
    }

    pub async fn rename(file: Self, newname: String) {
        let newpath = file
            .path()
            .parent()
            .unwrap_or(&PathBuf::new())
            .join(newname);
        fs::rename(file.path(), newpath).await.unwrap();
    }

    pub async fn create_file(file: Self) {
        fs::File::create(file).await.expect("couldn't create file");
    }

    pub async fn create_dir(dir: Self) {
        fs::create_dir(dir).await.expect("couldn't create dir");
    }
}
