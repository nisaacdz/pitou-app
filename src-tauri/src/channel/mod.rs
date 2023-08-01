#[tauri::command]
pub async fn get_debug_file() -> backend::Pitou {
    backend::debug_with_real_dir().await
}

#[tauri::command]
pub async fn properties(pitou: backend::Pitou) -> backend::Properties {
    pitou
        .properties()
        .await
        .expect("cannot retrieve properties of selected file")
}

#[tauri::command]
pub async fn metadata(pitou: backend::Pitou) -> Option<backend::Metadata> {
    pitou.metadata().await.ok()
}

#[tauri::command]
pub async fn ancestors(pitou: backend::Pitou) -> Vec<backend::Pitou> {
    pitou.ancestors()
}

#[tauri::command]
pub async fn children(pitou: backend::Pitou) -> Vec<backend::Pitou> {
    pitou
        .children()
        .await
        .expect("cannot retrieve children of selected file")
}

#[tauri::command]
pub(crate) async fn siblings(pitou: backend::Pitou) -> Vec<backend::Pitou> {
    pitou.siblings().await.unwrap_or(vec![pitou])
}

#[tauri::command]
pub(crate) fn copy(items: Vec<backend::Pitou>) {
    backend::Pitou::copy(items);
}

#[tauri::command]
pub(crate) fn cut(items: Vec<backend::Pitou>) {
    backend::Pitou::cut(items);
}

#[tauri::command]
pub(crate) fn paste(pitou: backend::Pitou) {
    backend::Pitou::paste(pitou).expect("failed to paste file");
}

#[tauri::command]
pub(crate) fn delete(items: Vec<backend::Pitou>) {
    backend::Pitou::delete(items)
        .expect("failed to delete items");
}

#[tauri::command]
pub(crate) async fn rename(pitou: backend::Pitou, name: String) {
    backend::Pitou::rename(pitou, name).await
}

#[tauri::command]
pub(crate) async fn createfile(pitou: backend::Pitou) {
    backend::Pitou::create_file(pitou).await
}

#[tauri::command]
pub(crate) async fn createdir(pitou: backend::Pitou) {
    backend::Pitou::create_dir(pitou).await
}

#[tauri::command]
pub(crate) async fn clipboard() -> &'static std::collections::LinkedList<Vec<backend::Pitou>> {
    backend::actions::clipboard::content()
}
