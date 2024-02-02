// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod channel;

use channel::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            properties,
            children,
            children_dirs,
            siblings,
            copy,
            cut,
            paste,
            rename,
            delete,
            createdir,
            createfile,
            clipboard,
            last_history_or_default,
            append_history,
            append_bookmarks,
            default_directory,
            size,
            open,
            restart_stream_search,
            read_search_stream,
            reset_search_stream,
            drives,
            locals,
            read_link,
            retrieve,
            folder_size,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn size(window: tauri::window::Window) -> [u32; 2] {
    let monitor = window.current_monitor().unwrap().unwrap();
    let rt = monitor.scale_factor();
    let size = monitor.size();
    [(size.width as f64 * rt) as u32, (size.height as f64 * rt) as u32]
}
