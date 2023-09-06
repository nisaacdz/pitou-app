use crate::app::{new_dir::NewDirPopUp, new_file::NewFilePopUp, rename::RenamePopUp, ApplicationContext};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use super::{NameField, TopButtonProps};

#[function_component]
pub fn RenameButton(prop: &TopButtonProps) -> Html {
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context().unwrap();
    let item_to_rename = use_state(|| None);

    let finished = Callback::from({
        let item_to_rename = item_to_rename.clone();
        let updateui = prop.updateui.clone();

        move |_| {
            item_to_rename.set(None);
            updateui.emit(());
        }
    });

    let onclick = {
        let item_to_rename = item_to_rename.clone();
        move |_| {
            if let None = &*item_to_rename {
                if let Some(items) = crate::app::data::all() {
                    if let Some(file) = items.borrow().iter().next() {
                        item_to_rename.set(Some(file.clone()))
                    }
                }
            }
        }
    };

    let onclickok = {
        let finished = finished.clone();
        let item_to_rename = item_to_rename.clone();

        move |name: String| {
            let name = name.clone();
            let finished = finished.clone();
            let item_to_rename = item_to_rename.clone();
            if let Some(file) = item_to_rename.as_ref() {
                let file = file.clone();
                spawn_local(async move {
                    crate::app::tasks::rename(file.path(), &name).await;
                    finished.emit(());
                });
            }
        }
    };

    let onclickcancel = {
        let item_to_rename = item_to_rename.clone();

        move |_| item_to_rename.set(None)
    };

    let tool_size = sizes.toolbar_item();
    let icon_size = sizes.toolbar_icon();
    let img_height = sizes.toolbar_icon_img().height();

    let style = format! {"
    {tool_size}
    display: flex;
    flex-direction: column;
    align-items: center;
    "};

    let icon_style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    {icon_size}
    "};

    let img_style = format! {"
    {img_height}
    "};

    let rename_or_not = if let Some(file) = &*item_to_rename {
        html! { <RenamePopUp file = { file.clone() } {onclickok} {onclickcancel}/> }
    } else {
        html! {}
    };

    html! {
        <div {style} {onclick}>
            {rename_or_not}
            <div style = {icon_style}>
                <img class = "card" style = {img_style} src="./public/icons/top/rename.png" alt="rename" />
            </div>
            <NameField name = { "rename" }/>
        </div>
    }
}

#[function_component]
pub fn NewFolderButton(prop: &TopButtonProps) -> Html {
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context().unwrap();
    let create_dir_in = use_state(|| None);

    let finished = Callback::from({
        let create_dir_in = create_dir_in.clone();
        let updateui = prop.updateui.clone();

        move |_| {
            create_dir_in.set(None);
            updateui.emit(());
        }
    });

    let onclick = {
        let create_dir_in = create_dir_in.clone();
        move |_| {
            if let None = &*create_dir_in {
                if let Some(directory) = crate::app::data::directory() {
                    create_dir_in.set(Some(directory.clone()))
                }
            }
        }
    };

    let onclickcancel = {
        let create_dir_in = create_dir_in.clone();
        move |_| create_dir_in.set(None)
    };

    let onclickok = {
        let finished = finished.clone();
        let directory = create_dir_in.clone();

        move |name: String| {
            let name = name.clone();
            let finished = finished.clone();
            let directory = directory.clone();
            if let Some(path) = &*directory {
                let createme = path.join(name).into();
                spawn_local(async move {
                    crate::app::tasks::createdir(&createme).await;
                    finished.emit(());
                });
            }
        }
    };

    let tool_size = sizes.toolbar_item();
    let icon_size = sizes.toolbar_icon();
    let img_height = sizes.toolbar_icon_img().height();

    let style = format! {"
    {tool_size}
    display: flex;
    flex-direction: column;
    align-items: center;
    "};

    let icon_style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    {icon_size}
    "};

    let img_style = format! {"
    {img_height}
    "};

    let create_or_not = if let Some(directory) = &*create_dir_in {
        html! { <NewDirPopUp directory = { directory.clone() } {onclickok} {onclickcancel} /> }
    } else {
        html! {}
    };

    html! {
        <div {style} {onclick}>
            {create_or_not}
            <div style = {icon_style}>
                <img class = "card" style = {img_style} src="./public/icons/top/add_folder.png" alt="new folder" />
            </div>
            <NameField name = { "folder" }/>
        </div>
    }
}

#[function_component]
pub fn NewFileButton(prop: &TopButtonProps) -> Html {
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context().unwrap();
    let create_file_in = use_state(|| None);

    let finished = Callback::from({
        let create_file_in = create_file_in.clone();
        let updateui = prop.updateui.clone();

        move |_| {
            create_file_in.set(None);
            updateui.emit(());
        }
    });

    let onclick = {
        let create_file_in = create_file_in.clone();
        move |_| {
            if let None = &*create_file_in {
                if let Some(directory) = crate::app::data::directory() {
                    create_file_in.set(Some(directory.clone()))
                }
            }
        }
    };

    let onclickcancel = {
        let create_file_in = create_file_in.clone();
        move |_| create_file_in.set(None)
    };

    let onclickok = {
        let finished = finished.clone();
        let directory = create_file_in.clone();

        move |name: String| {
            if let Some(directory) = &*directory {
                let finished = finished.clone();
                let createme = directory.join(&name).into();
                spawn_local(async move {
                    crate::app::tasks::createfile(&createme).await;
                    finished.emit(());
                });
            }
        }
    };

    let tool_size = sizes.toolbar_item();
    let icon_size = sizes.toolbar_icon();
    let img_height = sizes.toolbar_icon_img().height();

    let style = format! {"
    {tool_size}
    display: flex;
    flex-direction: column;
    align-items: center;
    "};

    let icon_style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    {icon_size}
    "};

    let img_style = format! {"
    {img_height}
    "};

    let create_or_not = if let Some(directory) = &*create_file_in {
        html! { <NewFilePopUp directory = { directory.clone() } {onclickok} {onclickcancel}/> }
    } else {
        html! {}
    };

    html! {
        <div {style} {onclick}>
            {create_or_not}
            <div style = {icon_style}>
                <img class = "card" style = {img_style} src="./public/icons/top/add_file.png" alt="new file" />
            </div>
            <NameField name = { "file" }/>
        </div>
    }
}
