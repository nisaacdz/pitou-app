// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod channel;
use channel::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_debug_file,
            properties,
            metadata,
            ancestors,
            children,
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
