use std::sync::Arc;
use tokio::{fs, sync::Mutex, task::JoinHandle};

use crate::{KeyType, Pitou, SearchArea, SearchMsg, SearchOptions, SearchStream, StrOps};

macro_rules! should_include {
    ($filetype:expr, $options:expr, $file_name:expr, $key:expr) => {
        if ($filetype.is_dir() && $options.include_dirs)
            || ($filetype.is_file() && $options.include_files)
            || ($filetype.is_symlink() && $options.include_links)
        {
            match $options.keytype {
                KeyType::Regex => $file_name.matches(&*$key).next().is_some(),
                KeyType::RawSearch(area) => match area {
                    SearchArea::StartsWith => {
                        if $options.case_sensitive {
                            $file_name.starts_with(&*$key)
                        } else {
                            $file_name.starts_with_ignore_case(&*$key)
                        }
                    }
                    SearchArea::EndsWith => {
                        if $options.case_sensitive {
                            $file_name.ends_with(&*$key)
                        } else {
                            $file_name.ends_with_ignore_case(&*$key)
                        }
                    }
                    SearchArea::Contains => {
                        if $options.case_sensitive {
                            $file_name.contains(&*$key)
                        } else {
                            $file_name.contains_ignore_case(&*$key)
                        }
                    }
                },
            }
        } else {
            false
        }
    };
}

pub async fn test_me() {
    let search_in = std::path::PathBuf::from("D:\\Workspace\\").into();
    let search_key = "new".to_owned();
    let options = SearchOptions::new().depth(4);

    let clock = std::time::Instant::now();

    let results = search(search_key, search_in, options).await;

    println!(
        "finished searching. Total seconds ellapsed = {}",
        clock.elapsed().as_secs()
    );

    for (idx, res) in results.into_iter().enumerate() {
        let idx = idx + 1;
        println!("{idx}. {} \n", res.path().display());
    }
}

static mut SEARCH_STREAM: Option<SearchStream> = None;
static mut STREAM_HANDLE: Option<JoinHandle<()>> = None;

#[cfg(debug_assertions)]
pub async fn test_tokio_spawn() {
    tokio::spawn(async move {
        println!("Hello, World!");
    })
    .await
    .unwrap();
}

pub async fn read_search_stream() -> Option<SearchMsg> {
    unsafe {
        match &mut SEARCH_STREAM {
            Some(stream) => Some(stream.pull().await),
            None => None,
        }
    }
}

pub async fn reset_search_stream() {
    unsafe {
        if let Some(val) = STREAM_HANDLE.take() {
            val.abort();
        }
        SEARCH_STREAM = None;
    }
}

pub fn stream_search(key: String, search_in: Pitou, options: SearchOptions) {
    unsafe {
        match &mut SEARCH_STREAM {
            Some(_) => return,
            None => {
                let stream = crate::new_search_stream();
                SEARCH_STREAM = Some(stream.clone());
                let key = Arc::new(key);

                let handle = tokio::spawn(async move {
                    recursive_stream_search(stream.clone(), key, search_in, options).await;
                    STREAM_HANDLE.as_mut().unwrap().abort();
                    stream.terminate().await;
                });

                STREAM_HANDLE = Some(handle);
            }
        }
    };
}

#[async_recursion::async_recursion]
async fn recursive_stream_search(
    stream: SearchStream,
    key: Arc<String>,
    search_in: Pitou,
    mut options: SearchOptions,
) {
    options.depth -= 1;

    if stream.ended().await {
        return;
    }

    let mut tasks = Vec::new();

    if let Ok(mut rd) = fs::read_dir(search_in).await {
        while let Ok(Some(de)) = rd.next_entry().await {
            let name = de.file_name();

            let file_name = if let Some(name) = name.to_str() {
                name
            } else if options.skip_errors {
                continue;
            } else {
                //TODO
                continue;
            };

            let filetype = if let Ok(filetype) = de.file_type().await {
                filetype
            } else if options.skip_errors {
                continue;
            } else {
                //TODO
                continue;
            };

            if should_include!(filetype, options, file_name, key) {
                if !stream.push(de.path().into()).await {
                    return;
                }
            }

            if filetype.is_dir() && options.depth > 0 {
                let stream = stream.clone();
                let key = key.clone();
                tasks.push(tokio::spawn(async move {
                    recursive_stream_search(stream, key, de.path().into(), options).await
                }));
            }
        }
    } else if !options.skip_errors {
        //TODO
    }

    for task in tasks {
        task.await.unwrap();
    }
}

pub async fn search(key: String, search_in: Pitou, options: SearchOptions) -> Vec<Pitou> {
    let res = Arc::new(Mutex::new(Vec::new()));
    let key = Arc::new(key);

    recursive_search(res.clone(), key, search_in, options).await;

    Arc::into_inner(res).unwrap().into_inner()
}

#[async_recursion::async_recursion]
pub async fn recursive_search(
    finds: Arc<Mutex<Vec<Pitou>>>,
    key: Arc<String>,
    search_in: Pitou,
    mut options: SearchOptions,
) {
    options.depth -= 1;

    #[cfg(debug_assertions)]
    println!("now looking in {}", search_in.path().display());

    let mut tasks = Vec::new();

    if let Ok(mut rd) = fs::read_dir(search_in).await {
        while let Ok(Some(de)) = rd.next_entry().await {
            let name = de.file_name();

            let file_name = if let Some(name) = name.to_str() {
                name
            } else if options.skip_errors {
                continue;
            } else {
                todo!()
            };

            let filetype = if let Ok(filetype) = de.file_type().await {
                filetype
            } else if options.skip_errors {
                continue;
            } else {
                todo!()
            };

            if should_include!(filetype, options, file_name, key) {
                finds.lock().await.push(de.path().into())
            }

            if filetype.is_dir() && options.depth > 0 {
                let finds = finds.clone();
                let key = key.clone();
                tasks.push(tokio::spawn(async move {
                    recursive_search(finds, key, de.path().into(), options).await
                }));
            }
        }
    } else if !options.skip_errors {
        todo!()
    }

    for task in tasks {
        task.await.unwrap();
    }
}

pub async fn search_eq_1(search_in: Pitou, key: String) -> Arc<Mutex<Vec<Pitou>>> {
    let items = Arc::new(Mutex::new(Vec::new()));
    {
        let items = items.clone();
        tokio::spawn(async move {
            if let Ok(mut rd) = fs::read_dir(search_in).await {
                let items = items.clone();
                while let Ok(Some(de)) = rd.next_entry().await {
                    if let Some(str_val) = de.file_name().to_str() {
                        if str_val.starts_with(&key) {
                            items.lock().await.push(de.path().into());
                        }
                    }
                }
            }
        })
        .await
        .unwrap();
    }
    items
}
