#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn get_debug_file() -> backend::Pitou {
    backend::debug_with_real_dir().await
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn properties(pitou: backend::Pitou) -> backend::Properties {
    pitou
        .properties()
        .await
        .expect("cannot retrieve properties of selected file")
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn children(pitou: backend::Pitou) -> Vec<backend::Pitou> {
    pitou
        .children()
        .await
        .expect("cannot retrieve children of selected file")
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn siblings(pitou: backend::Pitou) -> Vec<backend::Pitou> {
    pitou
        .siblings()
        .await
        .expect("cannot retrieve siblings of selected file")
}
