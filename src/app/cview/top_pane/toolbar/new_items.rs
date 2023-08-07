use std::{cell::RefCell, rc::Rc};

use crate::app::{
    invoke, new_dir::NewDirPopUp, new_file::NewFilePopUp, rename::RenamePopUp, AddFile, AddFolder,
    PitouAndNameArgs, PitouArg, RenameIcon,
};
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use super::{NameField, TopButtonProps};

#[function_component]
pub fn RenameButton(prop: &TopButtonProps) -> Html {
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
                if let Some(file) = crate::data::selected()
                    .map(|items| items.first())
                    .unwrap_or(None)
                {
                    item_to_rename.set(Some(file.clone()))
                }
            }
        }
    };

    let onclickok = {
        let finished = finished.clone();
        let item_to_rename = item_to_rename.clone();

        move |name: Rc<RefCell<String>>| {
            let name = name.clone();
            let finished = finished.clone();
            let item_to_rename = item_to_rename.clone();
            spawn_local(async move {
                if let Some(file) = (*item_to_rename).as_ref() {
                    let args = to_value(&PitouAndNameArgs {
                        pitou: &file,
                        name: &name.borrow(),
                    })
                    .unwrap();
                    invoke("rename", args).await;
                    finished.emit(());
                }
            });
        }
    };

    let onclickcancel = {
        let item_to_rename = item_to_rename.clone();

        move |_| item_to_rename.set(None)
    };

    let style = format! {"
    width: 50px;
    height: 100%;
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    align-items: center;
    "};

    let icon_style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    height: 70%;
    width: 100%;
    "};

    let rename_or_not = if let Some(file) = &*item_to_rename {
        html! { <RenamePopUp file = { file.clone() } {onclickok} {onclickcancel}/> }
    } else {
        html! {}
    };

    html! {
        <div {style} {onclick}>
            {rename_or_not}
            <div class = "card" style = {icon_style}>
                <RenameIcon />

            </div>
            <NameField name = { "rename" }/>
        </div>
    }
}

#[function_component]
pub fn NewFolderButton(prop: &TopButtonProps) -> Html {
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
                if let Some(directory) = crate::data::directory() {
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

        move |name: Rc<RefCell<String>>| {
            if let Some(directory) = &*directory {
                let name = name.clone();
                let finished = finished.clone();
                let directory = directory.clone();
                spawn_local(async move {
                    let createme = directory.path().join(&*name.borrow()).into();
                    let args = to_value(&PitouArg { pitou: &createme }).unwrap();
                    invoke("createdir", args).await;
                    finished.emit(());
                });
            }
        }
    };

    let style = format! {"
    width: 50px;
    height: 100%;
    display: flex;
    flex-shrink: 0;
    flex-direction: column;
    align-items: center;
    "};

    let icon_style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    height: 70%;
    width: 100%;
    "};

    let create_or_not = if let Some(directory) = &*create_dir_in {
        html! { <NewDirPopUp directory = { directory.clone() } {onclickok} {onclickcancel} /> }
    } else {
        html! {}
    };

    html! {
        <div {style} {onclick}>
            {create_or_not}
            <div class = "card" style = {icon_style}>
                <AddFolder />

            </div>
            <NameField name = { "folder" }/>
        </div>
    }
}

#[function_component]
pub fn NewFileButton(prop: &TopButtonProps) -> Html {
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
                if let Some(directory) = crate::data::directory() {
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

        move |name: Rc<RefCell<String>>| {
            if let Some(directory) = &*directory {
                let name = name.clone();
                let finished = finished.clone();
                let directory = directory.clone();
                spawn_local(async move {
                    let createme = directory.path().join(&*name.borrow()).into();
                    let args = to_value(&PitouArg { pitou: &createme }).unwrap();
                    invoke("createfile", args).await;
                    finished.emit(());
                });
            }
        }
    };

    let style = format! {"
    width: 50px;
    height: 100%;
    display: flex;
    flex-shrink: 0;
    flex-direction: column;
    align-items: center;
    "};

    let icon_style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    height: 70%;
    width: 100%;
    "};

    let create_or_not = if let Some(directory) = &*create_file_in {
        html! { <NewFilePopUp directory = { directory.clone() } {onclickok} {onclickcancel}/> }
    } else {
        html! {}
    };

    html! {
        <div {style} {onclick}>
            {create_or_not}
            <div class = "card" style = {icon_style}>
                <AddFile />
            </div>
            <NameField name = { "file" }/>
        </div>
    }
}
