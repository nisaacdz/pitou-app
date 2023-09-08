use backend::{File, Filter, Path, Properties, SearchMsg, SearchOptions};

#[tauri::command]
pub async fn properties(path: Path) -> Properties {
    backend::File::properties(path.into_inner())
        .await
        .expect("cannot retrieve properties of selected file")
}

#[tauri::command]
pub async fn append_history(path: Path) {
    backend::history::append(&path).await
}

#[tauri::command]
pub fn locals() -> backend::Locals {
    backend::File::locals()
}

#[tauri::command]
pub async fn last_history_or_default() -> Option<Path> {
    backend::history::last().await.map(|v| v.into())
}

#[tauri::command]
pub fn default_directory() -> Path {
    backend::File::home_directory().into()
}

#[tauri::command]
pub async fn append_bookmarks(path: Path) {
    backend::bookmarks::append(&path).await
}

#[tauri::command]
pub fn open(path: Path) {
    backend::actions::open(path.into_inner()).expect("couldn't open with default application")
}

#[tauri::command]
pub async fn read_link(path: Path) -> Option<File> {
    backend::actions::read_link(path.into_inner()).await
}

#[tauri::command]
pub fn retrieve(path: Path) -> Option<File> {
    backend::File::retrieve(path.into_inner())
}

#[tauri::command]
pub async fn children(path: Path, filter: Filter) -> Vec<File> {
    backend::File::children(&path.into_inner(), filter)
        .await
        .expect("cannot retrieve children of selected file")
}

#[tauri::command]
pub async fn children_dirs(path: Path, filter: Filter) -> Vec<File> {
    backend::File::children_dirs(&path.into_inner(), filter)
        .await
        .expect("failed searching for children directories")
}

#[tauri::command]
pub async fn siblings(path: Path, filter: Filter) -> Vec<File> {
    backend::File::siblings(&path.into_inner(), filter)
        .await
        .expect("couldn't get siblings of selected file")
}

#[tauri::command]
pub fn copy(items: Vec<Path>) {
    backend::actions::copy(items);
}

#[tauri::command]
pub fn cut(items: Vec<Path>) {
    backend::actions::cut(items);
}

#[tauri::command]
pub fn paste(path: Path) {
    backend::actions::paste(path).expect("failed to paste file");
}

#[tauri::command]
pub fn delete(items: Vec<Path>) {
    backend::actions::delete(items).expect("failed to delete items");
}

#[tauri::command]
pub fn read_search_stream() -> Option<SearchMsg> {
    backend::actions::read_search_stream()
}

#[tauri::command]
pub fn reset_search_stream() {
    backend::actions::reset_search_stream()
}

#[tauri::command]
pub fn drives() -> Vec<backend::Drive> {
    backend::drives()
}

#[tauri::command]
pub async fn restart_stream_search(key: String, path: Path, options: SearchOptions) {
    backend::actions::reset_search_stream();
    backend::actions::search(key, path.into_inner(), options);
}

#[tauri::command]
pub async fn rename(path: Path, name: String) {
    backend::actions::rename(path.into_inner(), name).await
}

#[tauri::command]
pub async fn createfile(path: Path) {
    backend::actions::create_file(path.into_inner()).await
}

#[tauri::command]
pub async fn createdir(path: Path) {
    backend::actions::create_dir(path.into_inner()).await
}

#[tauri::command]
pub fn clipboard() -> &'static std::collections::LinkedList<Vec<Path>> {
    backend::actions::clipboard::content()
}
