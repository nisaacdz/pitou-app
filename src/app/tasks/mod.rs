use std::{cell::RefCell, collections::LinkedList, path::PathBuf};

use super::invoke;
use backend::{Drive, File, Filter, Locals, Path, SearchMsg, SearchOptions};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::JsValue;

use serde::Serialize;

#[derive(Serialize)]
struct SearchArgs<'a> {
    key: &'a String,
    path: &'a Path,
    options: SearchOptions,
}

#[derive(Serialize)]
struct RetrieveArgs<'a> {
    path: &'a Path,
    filter: Filter,
}

#[derive(Serialize)]
struct PathArg<'a> {
    path: &'a Path,
}

#[derive(Serialize)]
struct NoArg;

#[derive(Serialize)]
struct PathNameArgs<'a> {
    path: &'a Path,
    name: &'a str,
}

#[derive(Serialize)]
struct ItemsArg<'a, I: Iterator<Item = &'a Path>> {
    #[serde(with = "items_serde")]
    pub items: RefCell<Option<I>>,
}

impl<'a, I: Iterator<Item = &'a Path>> ItemsArg<'a, I> {
    fn new(items: I) -> Self {
        Self {
            items: RefCell::new(Some(items)),
        }
    }
}

pub fn to_js_items<'a, I: Iterator<Item = &'a File>>(iter: I) -> JsValue {
    to_value(&ItemsArg::new(iter.map(|f| f.path().as_ref()))).unwrap()
}

mod items_serde {
    use backend::Path;
    use serde::Serializer;
    use std::cell::RefCell;

    pub fn serialize<'a, S: Serializer, I: Iterator<Item = &'a Path>>(
        items: &RefCell<Option<I>>,
        sz: S,
    ) -> Result<S::Ok, S::Error> {
        sz.collect_seq(items.borrow_mut().take().unwrap())
    }
}

pub async fn cut(arg: JsValue) {
    invoke("cut", arg).await;
}

pub async fn copy(arg: JsValue) {
    invoke("copy", arg).await;
}

pub async fn delete(arg: JsValue) {
    invoke("delete", arg).await;
}

pub async fn paste(directory: &PathBuf) {
    let arg = to_value(&PathArg {
        path: directory.as_ref(),
    })
    .unwrap();
    invoke("paste", arg).await;
}

pub async fn rename(path: &PathBuf, name: &str) {
    let arg = to_value(&PathNameArgs {
        path: path.as_ref(),
        name,
    })
    .unwrap();
    invoke("rename", arg).await;
}

pub async fn createfile(path: &PathBuf) {
    let arg = to_value(&PathArg {
        path: path.as_ref(),
    })
    .unwrap();
    invoke("createfile", arg).await;
}

pub async fn createdir(path: &PathBuf) {
    let arg = to_value(&PathArg {
        path: path.as_ref(),
    })
    .unwrap();
    invoke("createdir", arg).await;
}

pub async fn default_directory() -> PathBuf {
    let arg = to_value(&NoArg).unwrap();
    let res = invoke("default_directory", arg).await;
    from_value::<Path>(res).unwrap().into_inner()
}

pub async fn clipboard() -> LinkedList<Path> {
    let arg = to_value(&NoArg).unwrap();
    let res = invoke("clipboard", arg).await;
    from_value(res).unwrap()
}

pub async fn children(path: &PathBuf, filter: Filter) -> Vec<File> {
    let arg = to_value(&RetrieveArgs {
        path: path.as_ref(),
        filter,
    })
    .unwrap();
    from_value(invoke("children", arg).await).unwrap()
}

pub async fn siblings(path: &PathBuf, filter: Filter) -> Vec<File> {
    let arg = to_value(&RetrieveArgs {
        path: path.as_ref(),
        filter,
    })
    .unwrap();
    from_value(invoke("siblings", arg).await).unwrap()
}

pub async fn children_dirs(path: &PathBuf, filter: Filter) -> Vec<File> {
    let arg = to_value(&RetrieveArgs {
        path: path.as_ref(),
        filter,
    })
    .unwrap();
    from_value(invoke("children_dirs", arg).await).unwrap()
}

pub async fn open(path: &PathBuf) {
    let arg = to_value(&PathArg {
        path: path.as_ref(),
    })
    .unwrap();
    invoke("open", arg).await;
}

pub async fn locals() -> Locals {
    let arg = to_value(&NoArg).unwrap();
    let jsval = invoke("locals", arg).await;
    from_value::<Locals>(jsval).unwrap()
}

pub async fn drives() -> Vec<Drive> {
    let arg = to_value(&NoArg).unwrap();
    let jsval = invoke("drives", arg).await;
    from_value(jsval).unwrap()
}

/// returns the file that this sumbolic link points to along with the directory and also return
pub async fn read_link(path: &PathBuf) -> Option<File> {
    let arg = to_value(&PathArg {
        path: path.as_ref(),
    })
    .unwrap();
    let js_res = invoke("read_link", arg).await;
    from_value(js_res).unwrap()
}

pub async fn retrieve(path: &PathBuf) -> Option<File> {
    let arg = to_value(&PathArg {
        path: path.as_ref(),
    })
    .unwrap();
    let js_res = invoke("retrieve", arg).await;
    from_value(js_res).unwrap()
}

pub async fn terminate_search_stream() {
    let arg = to_value(&NoArg).unwrap();
    invoke("reset_search_stream", arg).await;
}

pub async fn restart_stream_search(key: &String, path: &Path, options: SearchOptions) {
    let arg = to_value(&SearchArgs { key, path, options }).unwrap();
    invoke("restart_stream_search", arg).await;
}

pub async fn read_search_stream() -> Option<SearchMsg> {
    let arg = to_value(&NoArg).unwrap();
    let res = invoke("read_search_stream", arg).await;
    from_value(res).unwrap()
}
