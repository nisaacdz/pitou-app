// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod channel;

use channel::*;

#[test]
fn test_tokio_spawn_in_debug() {
    tauri::async_runtime::block_on(async move { backend::actions::test_tokio_spawn().await });
}

#[test]
fn test_stream_search() {
    use backend::{SearchMsg, SearchOptions};

    let res = tauri::async_runtime::block_on(async move {
        let key = "target".to_owned();
        let options = SearchOptions::new().depth(3);
        let search_in = std::path::PathBuf::from("D:\\Workspace\\rust").into();

        let mut found = Vec::new();

        println!("initiating stream search");
        let clock = std::time::Instant::now();
        backend::actions::stream_search(key, search_in, options);
        println!(
            "kicked off stream search in {} millis",
            clock.elapsed().as_millis()
        );

        while let Some(msg) = backend::actions::read_search_stream().await {
            match msg {
                SearchMsg::Searching(items) => {
                    println!(
                        "found : {:?}",
                        items.iter().map(|p| p.name()).collect::<Vec<_>>()
                    );
                    found.extend(items);
                }
                SearchMsg::Terminated(items) => {
                    println!(
                        "found : {:?}",
                        items.iter().map(|p| p.name()).collect::<Vec<_>>()
                    );
                    found.extend(items);
                    println!("Ending...");
                    backend::actions::reset_search_stream().await;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(400));
        }
        println!("Ended.");
        found
    });

    println!("all found items ...");

    for pitou in res {
        println!("{}", pitou.path().display())
    }
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_debug_file,
            properties,
            metadata,
            ancestors,
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
            restart_stream_search,
            read_search_stream,
            reset_search_stream,
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
