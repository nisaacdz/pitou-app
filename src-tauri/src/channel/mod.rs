use backend::{Metadata, Pitou, Properties, SearchMsg, SearchOptions};

#[tauri::command]
pub async fn get_debug_file() -> Pitou {
    backend::debug_with_real_dir().await
}

#[tauri::command]
pub async fn properties(pitou: Pitou) -> Properties {
    pitou
        .properties()
        .await
        .expect("cannot retrieve properties of selected file")
}

#[tauri::command]
pub async fn metadata(pitou: Pitou) -> Option<Metadata> {
    pitou.metadata().await.ok()
}

#[tauri::command]
pub async fn ancestors(pitou: Pitou) -> Vec<Pitou> {
    pitou.ancestors()
}

#[tauri::command]
pub async fn append_history(pitou: Pitou) {
    backend::history::append(&pitou).await
}

#[tauri::command]
pub async fn last_history_or_default() -> Pitou {
    backend::history::last()
        .await
        .unwrap_or(Pitou::from(std::path::PathBuf::from("")))
}

#[tauri::command]
pub fn default_directory() -> Pitou {
    Pitou::from(std::path::PathBuf::from(""))
}

#[tauri::command]
pub async fn append_bookmarks(pitou: Pitou) {
    backend::bookmarks::append(&pitou).await
}

#[tauri::command]
pub async fn children(pitou: Pitou) -> Vec<Pitou> {
    pitou
        .children()
        .await
        .expect("cannot retrieve children of selected file")
}

#[tauri::command]
pub async fn children_dirs(pitou: Pitou) -> Vec<Pitou> {
    backend::Pitou::children_dirs(&pitou)
        .await
        .expect("failed searching for children directories")
}

#[tauri::command]
pub async fn siblings(pitou: Pitou) -> Vec<Pitou> {
    pitou.siblings().await.unwrap_or(vec![pitou])
}

#[tauri::command]
pub fn copy(items: Vec<Pitou>) {
    Pitou::copy(items);
}

#[tauri::command]
pub fn cut(items: Vec<Pitou>) {
    Pitou::cut(items);
}

#[tauri::command]
pub fn paste(pitou: Pitou) {
    Pitou::paste(pitou).expect("failed to paste file");
}

#[tauri::command]
pub fn delete(items: Vec<Pitou>) {
    Pitou::delete(items).expect("failed to delete items");
}

#[tauri::command]
pub async fn read_search_stream() -> Option<SearchMsg> {
    println!("read search stream invoked");
    backend::actions::read_search_stream().await
}

#[tauri::command]
pub async fn reset_search_stream() {
    backend::actions::reset_search_stream().await
}

#[tauri::command]
pub async fn restart_stream_search(key: String, pitou: Pitou, options: SearchOptions) {
    #[cfg(debug_assertions)]
    {
        println!("starting new search");
        println!("key = {}", &key);
        println!("folder = {}", pitou.path().display());
        println!("options = {:?}", &options);
    }
    backend::actions::reset_search_stream().await;
    backend::actions::stream_search(key, pitou, options)
}

#[tauri::command]
pub async fn rename(pitou: Pitou, name: String) {
    Pitou::rename(pitou, name).await
}

#[tauri::command]
pub async fn createfile(pitou: Pitou) {
    Pitou::create_file(pitou).await
}

#[tauri::command]
pub async fn createdir(pitou: Pitou) {
    Pitou::create_dir(pitou).await
}

#[tauri::command]
pub async fn clipboard() -> &'static std::collections::LinkedList<Vec<Pitou>> {
    backend::actions::clipboard::content()
}
