use crate::app::{data::SharedBorrow, tasks::SpawnHandle, AppMenu, ApplicationData};
use backend::FileSize;
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use yew::{platform::time::sleep, prelude::*};

pub enum FolderSize {
    None,
    Computing,
    Some(FileSize),
}

impl From<Option<FileSize>> for FolderSize {
    fn from(value: Option<FileSize>) -> FolderSize {
        value
            .map(|fz| FolderSize::Some(fz))
            .unwrap_or(FolderSize::None)
    }
}

#[function_component]
pub fn FolderInfo() -> Html {
    let cdata = use_context::<ApplicationData>().unwrap();
    let file = use_state(|| None);
    let aborthandle = use_state(|| SharedBorrow::new(None));
    let folder_size = use_state(|| FolderSize::None);

    {
        let cdata = cdata.clone();
        let file = file.clone();
        use_effect(move || {
            let newhandle = SpawnHandle::new(async move {
                sleep(Duration::from_millis(100)).await;
                if let Some(p) = cdata.directory() {
                    let newfile = crate::app::tasks::retrieve(&*p).await;
                    file.set(newfile);
                } else {
                    file.set(None);
                }
            });

            spawn_local(async move {
                if let Some(oldhandle) = aborthandle.get_mut() {
                    SpawnHandle::abort(oldhandle).await;
                }

                aborthandle.get_mut().insert(newhandle).await;
            });
        });
    }

    if !matches!(cdata.active_menu(), AppMenu::Explorer) {
        return html! {};
    }

    match &*file {
        None => html! {},
        Some(f) => {
            let selected = match cdata.selected_files().borrow().len() {
                0 => "None".into(),
                1 => "1 item".into(),
                v => format! {"{v} items"},
            };

            let entries = cdata.dir_children().map(|v| v.len()).unwrap_or(0);

            let compute_sz = {
                let folder_size = folder_size.clone();
                let f = f.clone();
                move |_| {
                    let f = f.clone();
                    let folder_size = folder_size.clone();
                    folder_size.set(FolderSize::Computing);
                    spawn_local(async move {
                        let res = crate::app::tasks::folder_size(f.path()).await.into();
                        folder_size.set(res);
                    });
                }
            };

            let modified = f
                .metadata()
                .modified()
                .map(|date| date.format_slim())
                .unwrap_or("unavailable".to_owned());
            let accessed = f
                .metadata()
                .accessed()
                .map(|date| date.format_slim())
                .unwrap_or("unavailable".to_owned());

            let size_info = match *folder_size {
                FolderSize::Some(v) => html! {
                    <>
                        <span>{ v.format() }</span>
                        <button class = "task-elem-item-btn card" onclick = {compute_sz}>{"recompute"}</button>
                    </>
                },
                FolderSize::None => {
                    html! {<button class = "task-elem-item-btn card" onclick = {compute_sz}>{"compute"}</button>}
                }
                FolderSize::Computing => html! {
                    <>
                        <img class = "task-elem-item-anim" src="./public/anims/computing.gif"/>
                        <img class = "task-elem-item-close card" title = "stop computing" src="./public/icons/title/close.png"/>
                    </>
                },
            };

            html! {
                <div class = "task-elem">
                    <div class = "task-elem-item"> { format! {"Total Entries: {entries}"} } </div>
                    <div class = "task-elem-item"> { format! {"Selected Entries: {selected}"} } </div>
                    <div class = "task-elem-item"> { format! {"Last Modified: {modified}" } } </div>
                    <div class = "task-elem-item"> { format! {"Last Accessed: {accessed}"} } </div>
                    <div class = "task-elem-item">
                        <span>{"size:    "}</span>
                        {size_info}
                    </div>
                </div>
            }
        }
    }
}
