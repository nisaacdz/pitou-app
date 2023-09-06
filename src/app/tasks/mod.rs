use std::{path::PathBuf, cell::RefCell, collections::LinkedList};

use backend::{File, SearchMsg, Path, SearchOptions, Locals, Drive};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::JsValue;
use yew::Callback;
use super::invoke;

use serde::Serialize;

#[derive(Serialize)]
struct SearchArgs<'a> {
    key: &'a String,
    path: &'a Path,
    options: SearchOptions,
}


#[derive(Serialize)]
struct PathArg<'a> {
    path: &'a Path,
}

#[derive(Serialize)]
struct NoArg;


#[derive(Serialize)]
struct PitouSearchArgs<'a> {
    key: &'a String,
    path: &'a Path,
    options: SearchOptions,
}

#[derive(Serialize)]
struct PathNameArgs<'a> {
    path: &'a Path,
    name: &'a str,
}

#[derive(Serialize)]
struct ItemsArg<'a, I: Iterator<Item = &'a Path>> {
    #[serde(with = "items_serde")]
    pub items: RefCell<Option<I>>
}


impl<'a, I: Iterator<Item = &'a Path>> ItemsArg<'a, I> {
    fn new(items: I) -> Self {
        Self { items: RefCell::new(Some(items)) }
    }
}

pub fn to_js_items<'a, I: Iterator<Item = &'a File>>(iter: I) -> JsValue {
    to_value(&ItemsArg::new(iter.map(|f| f.path().as_ref()))).unwrap()
}

mod items_serde {
    use serde::Serializer;
    use backend::Path;
    use std::cell::RefCell;

    pub fn serialize<'a, S: Serializer, I: Iterator<Item = &'a Path>>(items: &RefCell<Option<I>>, sz: S) -> Result<S::Ok, S::Error> {
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
    let arg = to_value(&PathArg { path: directory.as_ref() }).unwrap();
    invoke("paste", arg).await;
}

pub async fn rename(path: &PathBuf, name: &str) {
    let arg = to_value(&PathNameArgs { path: path.as_ref(), name }).unwrap();
    invoke("rename", arg).await;
}

pub async fn createfile(path: &PathBuf) {
    let arg = to_value(&PathArg { path: path.as_ref() }).unwrap();
    invoke("createfile", arg).await;
}

pub async fn createdir(path: &PathBuf) {
    let arg = to_value(&PathArg { path: path.as_ref() }).unwrap();
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

pub async fn children(dir: &PathBuf) -> Vec<File> {
    let arg = to_value(&PathArg { path: dir.as_ref() }).unwrap();
    from_value(invoke("children", arg).await).unwrap()
}

pub async fn siblings(path: &PathBuf) -> Vec<File> {
    let arg = to_value(&PathArg { path: path.as_ref() }).unwrap();
    from_value(invoke("siblings", arg).await).unwrap()
}

pub async fn children_dirs(dir: &PathBuf) -> Vec<File> {
    let arg = to_value(&PathArg { path: dir.as_ref() }).unwrap();
    from_value(invoke("children_dirs", arg).await).unwrap()
}

pub async fn open(path: &PathBuf) {
    let arg = to_value(&PathArg { path: path.as_ref() }).unwrap();
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

pub fn updatedirectory_with_symlink(link: &PathBuf, updatedirectory: Callback<File>) {
    // let arg = to_value(&PitouArg { pitou: link }).unwrap();
    // spawn_local(async move {
    //     let res = from_value::<Option<Pitou>>(invoke("read_link", arg).await).unwrap();
    //     if let Some(pitou) = &res {
    //         let metadata = from_value::<Option<Metadata>>(
    //             invoke("metadata", to_value(&PitouArg { pitou }).unwrap()).await,
    //         )
    //         .unwrap();
    //         if let Some(metadata) = &metadata {
    //             match metadata.file_type() {
    //                 backend::PitouType::Directory => updatedirectory.emit(pitou.clone()),
    //                 backend::PitouType::File => open_local(pitou),
    //                 backend::PitouType::Link => {
    //                     updatedirectory_with_symlink(pitou, updatedirectory)
    //                 }
    //             }
    //         }
    //     }
    // });
    todo!()
}

pub async fn terminate_search_stream() {
    let arg = to_value(&NoArg).unwrap();
    invoke("reset_search_stream", arg).await;
}

pub async fn restart_stream_search(key: &String, path: &Path, options: SearchOptions) {
    let arg = to_value(&SearchArgs {key, path, options}).unwrap();
    invoke("restart_stream_search", arg).await;
}

pub async fn read_search_stream() -> Option<SearchMsg> {
    let arg = to_value(&NoArg).unwrap();
    let res = invoke("read_search_stream", arg).await;
    from_value(res).unwrap()
}
