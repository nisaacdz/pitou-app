pub mod clipboard;

#[cfg(feature = "tauri")]
mod search;

use crate::fs::Get;

use std::path::PathBuf;

#[cfg(feature = "tauri")]
pub use search::*;

use fs_extra::dir::CopyOptions;
use tokio::fs;

use crate::Path;

use self::clipboard::Item;

pub fn delete(items: Vec<Path>) -> std::io::Result<()> {
    trash::delete_all(items).unwrap();
    Ok(())
}

pub fn copy(items: Vec<Path>) {
    clipboard::put(Item::Copied(items))
}

pub fn cut(items: Vec<Path>) {
    clipboard::put(Item::Cut(items))
}

pub fn paste(directory: Path) -> std::io::Result<()> {
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

pub fn open(file: PathBuf) -> std::io::Result<()> {
    open::that_detached(file.as_os_str())
}

pub fn share(_file: Path) -> std::io::Result<()> {
    todo!()
}

pub async fn rename(file: PathBuf, newname: String) {
    let newpath = file.parent().unwrap_or(&PathBuf::new()).join(newname);
    fs::rename(&file, newpath).await.unwrap();
}

pub async fn create_file(file: PathBuf) {
    fs::File::create(file).await.expect("couldn't create file");
}

pub async fn create_dir(dir: PathBuf) {
    fs::create_dir(dir).await.expect("couldn't create dir");
}

pub async fn read_link(link: PathBuf) -> Option<crate::File> {
    fs::read_link(link)
        .await
        .map(|v| v.get().ok())
        .unwrap_or(None)
}
