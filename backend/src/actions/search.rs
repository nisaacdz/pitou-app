use std::{path::PathBuf, sync::Arc};
use tokio::{fs, task::JoinHandle};

use crate::{Get, KeyType, SearchArea, SearchMsg, SearchOptions, SearchStream, StrOps};

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

pub fn read_search_stream() -> Option<SearchMsg> {
    unsafe {
        match &mut SEARCH_STREAM {
            Some(stream) => Some(stream.pull()),
            None => None,
        }
    }
}

pub fn reset_search_stream() {
    unsafe {
        if let Some(val) = STREAM_HANDLE.take() {
            val.abort();
        }
        SEARCH_STREAM = None;
    }
}

pub fn search(key: String, search_in: PathBuf, options: SearchOptions) {
    unsafe {
        match &mut SEARCH_STREAM {
            Some(_) => return,
            None => {
                let stream = crate::new_search_stream();
                SEARCH_STREAM = Some(stream.clone());
                let key = Arc::new(key);

                let handle = tokio::spawn(async move {
                    recursive_search(stream.clone(), key, search_in, options).await;
                    STREAM_HANDLE.as_mut().unwrap().abort();
                    stream.terminate();
                });

                STREAM_HANDLE = Some(handle);
            }
        }
    };
}

#[async_recursion::async_recursion]
async fn recursive_search(
    stream: SearchStream,
    key: Arc<String>,
    search_in: PathBuf,
    mut options: SearchOptions,
) {
    options.depth -= 1;

    if stream.ended() {
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
                if !stream.push(de.path().get().expect("couldn't parse path")) {
                    return;
                }
            }

            if filetype.is_dir() && options.depth > 0 {
                let stream = stream.clone();
                let key = key.clone();
                let path = de.path();
                tasks.push(tokio::spawn(async move {
                    recursive_search(stream, key, path, options).await
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
