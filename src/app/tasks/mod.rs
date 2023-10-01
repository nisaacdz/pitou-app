use super::invoke;
use backend::{Drive, File, Filter, Locals, Path, SearchMsg, SearchOptions};
use serde_wasm_bindgen::{from_value, to_value};
use std::{
    cell::RefCell,
    collections::{HashSet, LinkedList},
    path::PathBuf,
    rc::Rc,
};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;

macro_rules! clamp {
    ($begin:ident, $end:ident) => {
        paste::paste! {
            stringify!([<$begin $end>])
        }
    }
}

macro_rules! register_event {
    ($eventname:ident, $typename:ty) => {
        paste::paste! {
            pub async fn [<emit_began_ $eventname>](payload: $typename) {
                tauri_sys::event::emit(clamp!(began_, $eventname), &payload).await.unwrap();
            }

            pub async fn [<emit_ended_ $eventname>](payload: PayloadById) {
                tauri_sys::event::emit(clamp!(ended_, $eventname), &payload).await.unwrap();
            }

            pub async fn [<listen_to_began_ $eventname>]<F: FnOnce(&$typename)>(callback: F) {
                let event = tauri_sys::event::once::<$typename>(clamp!(began_, $eventname)).await.unwrap();
                callback(&event.payload)
            }

            pub async fn [<listen_to_ended_ $eventname>]<F: FnOnce(&PayloadById)>(callback: F) {
                let event = tauri_sys::event::once::<PayloadById>(clamp!(ended_, $eventname)).await.unwrap();
                callback(&event.payload)
            }
        }
    };
}


#[test]
fn test_me() {
    
}

use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
pub struct NoArg;

#[derive(Serialize)]
struct PathNameArgs<'a> {
    path: &'a Path,
    name: &'a str,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Payload<T> {
    pub value: T,
    pub id: i32,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct PayloadById {
    pub id: i32,
}

impl PayloadById {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}


impl<T> Payload<T> {
    pub fn new(value: T, id: i32) -> Self {
        Self { value, id }
    }
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

pub async fn cut(items: Rc<RefCell<HashSet<File>>>) {
    let items = items.borrow();
    let len = items.len();
    if len == 0 {
        return;
    }
    let id = id_gen::generate();
    let arg = to_js_items(items.iter());
    std::mem::drop(items);
    emit_began_cut(Payload::new(len, id)).await;
    invoke("cut", arg).await;
    spawn_local(emit_ended_cut(PayloadById::new(id)));
}

pub async fn copy(items: Rc<RefCell<HashSet<File>>>) {
    let items = items.borrow();
    let len = items.len();
    if len == 0 {
        return;
    }
    let id = id_gen::generate();
    let arg = to_js_items(items.iter());
    std::mem::drop(items);
    emit_began_copy(Payload::new(len, id)).await;
    invoke("copy", arg).await;
    spawn_local(emit_ended_copy(PayloadById::new(id)));
}

pub async fn delete(items: &Vec<File>) {
    if items.len() == 0 {
        return;
    }
    let id = id_gen::generate();
    let arg = to_js_items(items.iter());
    emit_began_delete(Payload::new(items.len(), id)).await;
    invoke("delete", arg).await;
    spawn_local(emit_ended_delete(PayloadById::new(id)));
}

pub async fn paste(directory: &PathBuf) {
    let id = id_gen::generate();
    let arg = to_value(&PathArg {
        path: directory.as_ref(),
    })
    .unwrap();
    emit_began_paste(PayloadById::new(id)).await;
    invoke("paste", arg).await;
    spawn_local(emit_ended_paste(PayloadById::new(id)));
}

pub async fn rename(path: &PathBuf, name: &str) {
    let id = id_gen::generate();
    let arg = to_value(&PathNameArgs {
        path: path.as_ref(),
        name,
    })
    .unwrap();
    emit_began_rename(PayloadById::new(id)).await;
    invoke("rename", arg).await;
    spawn_local(emit_ended_rename(PayloadById::new(id)));
}

pub async fn createfile(path: &PathBuf) {
    let id = id_gen::generate();
    let arg = to_value(&PathArg {
        path: path.as_ref(),
    })
    .unwrap();
    emit_began_addfile(PayloadById::new(id)).await;
    invoke("createfile", arg).await;
    spawn_local(emit_ended_addfile(PayloadById::new(id)));
}

pub async fn createdir(path: &PathBuf) {
    let id = id_gen::generate();
    let arg = to_value(&PathArg {
        path: path.as_ref(),
    })
    .unwrap();
    emit_began_addfolder(PayloadById::new(id)).await;
    invoke("createdir", arg).await;
    spawn_local(emit_ended_addfolder(PayloadById::new(id)));
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
    from_value(jsval).unwrap()
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

pub async fn terminate_search_stream(id: i32) {
    let arg = to_value(&NoArg).unwrap();
    emit_ended_search(PayloadById::new(id)).await;
    invoke("reset_search_stream", arg).await;
}

pub async fn restart_stream_search(key: &String, path: &PathBuf, options: SearchOptions, id: i32) {
    let arg = to_value(&SearchArgs {
        key,
        path: path.as_ref(),
        options,
    })
    .unwrap();
    emit_began_search(PayloadById::new(id)).await;
    invoke("restart_stream_search", arg).await;
}

pub async fn read_search_stream() -> Option<SearchMsg> {
    let arg = to_value(&NoArg).unwrap();
    let res = invoke("read_search_stream", arg).await;
    from_value(res).unwrap()
}

pub async fn close_window() {
    let wd = tauri_sys::window::current_window();
    wd.close().await.expect("failed to close window")
}

pub async fn minimize_window() {
    let wd = tauri_sys::window::current_window();
    wd.minimize().await.expect("failed to minimize window")
}

pub async fn toggle_maximize() {
    let wd = tauri_sys::window::current_window();
    wd.toggle_maximize()
        .await
        .expect("failed to toggle maximized")
}

pub use spawner::SpawnHandle;

pub use event_registry::*;

mod event_registry {
    use super::{Payload, PayloadById};
    register_event!(delete, Payload<usize>);
    register_event!(search, PayloadById);
    register_event!(copy, Payload<usize>);
    register_event!(cut, Payload<usize>);
    register_event!(addfile, PayloadById);
    register_event!(rename, PayloadById);
    register_event!(paste, PayloadById);
    register_event!(addfolder, PayloadById);
}

mod spawner {
    use std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll, Waker},
    };

    pub struct SpawnHandle<F> {
        future: Option<Pin<Box<F>>>,
        wake_sp: Option<Waker>,
        wake_ab: Option<Waker>,
    }

    struct AbortHandle<'a, F> {
        handle: Option<&'a mut SpawnHandle<F>>,
    }

    impl<'a, F: Future<Output = ()>> Future for AbortHandle<'a, F> {
        type Output = ();
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            match self.handle.take() {
                None => Poll::Ready(()),
                Some(v) => {
                    if v.is_completed() {
                        return Poll::Ready(());
                    }
                    let _future = v.future.take();
                    v.wake_ab.replace(cx.waker().clone());
                    // TODO replace below to return Poll::Ready when wake_sp is not ready
                    v.wake_sp
                        .as_ref()
                        .expect("waking a future that has not been polled even once")
                        .wake_by_ref();
                    Poll::Pending
                }
            }
        }
    }

    impl<F> SpawnHandle<F> {
        pub fn new(value: F) -> Self {
            let future = Some(Box::pin(value));
            Self {
                future,
                wake_sp: None,
                wake_ab: None,
            }
        }

        pub fn is_completed(&self) -> bool {
            self.future.is_none()
        }
    }

    impl<F: Future<Output = ()>> SpawnHandle<F> {
        pub async fn abort(&mut self) {
            AbortHandle { handle: Some(self) }.await;
        }
    }

    impl<F: Future<Output = ()>> Future for SpawnHandle<F> {
        type Output = ();
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if let Some(pin) = &mut self.future {
                match pin.as_mut().poll(cx) {
                    Poll::Pending => {
                        self.wake_sp.replace(cx.waker().clone());
                        Poll::Pending
                    }
                    Poll::Ready(_) => {
                        self.future.take();
                        Poll::Ready(())
                    }
                }
            } else {
                if let Some(ab) = &self.wake_ab {
                    ab.wake_by_ref();
                }
                Poll::Ready(())
            }
        }
    }
}

mod id_gen {
    static mut ID: i32 = 0;

    pub fn generate() -> i32 {
        unsafe {
            ID = ID.wrapping_add(1);
            ID
        }
    }
}
