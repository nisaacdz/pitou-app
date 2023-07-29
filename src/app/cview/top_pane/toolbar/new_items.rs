use crate::app::{new_file::NewFilePopUp, rename::RenamePopUp, AddFile, AddFolder, RenameIcon, new_dir::NewDirPopUp};
use yew::prelude::*;

use super::{NameField, TopButtonProps};

#[function_component]
pub fn RenameButton(prop: &TopButtonProps) -> Html {
    let item_to_rename = use_state(|| None);

    let finished = {
        let item_to_rename = item_to_rename.clone();
        let updateui = prop.updateui.clone();

        move |_| {
            item_to_rename.set(None);
            updateui.emit(());
        }
    };

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

    let style = format! {"
    width: 3%;
    height: 100%;
    display: flex;
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

    let theme = prop.theme;

    let rename_or_not = if let Some(file) = &*item_to_rename {
        html! { <RenamePopUp file = { file.clone() } {finished} {theme}/> }
    } else {
        html! {}
    };

    html! {
        <div {style} {onclick}>
            {rename_or_not}
            <div class = "card" style = {icon_style}>
                <RenameIcon />

            </div>
            <NameField name = { "rename" }  {theme} />
        </div>
    }
}

#[function_component]
pub fn NewFolderButton(prop: &TopButtonProps) -> Html {
    let create_dir_in = use_state(|| None);

    let finished = {
        let create_dir_in = create_dir_in.clone();
        let updateui = prop.updateui.clone();

        move |_| {
            create_dir_in.set(None);
            updateui.emit(());
        }
    };

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

    let style = format! {"
    width: 3%;
    height: 100%;
    display: flex;
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

    let theme = prop.theme;

    let create_or_not = if let Some(directory) = &*create_dir_in {
        html! { <NewDirPopUp directory = { directory.clone() } {finished} {theme}/> }
    } else {
        html! {}
    };

    html! {
        <div {style} {onclick}>
            {create_or_not}
            <div class = "card" style = {icon_style}>
                <AddFolder />

            </div>
            <NameField name = { "folder" }  {theme} />
        </div>
    }
}

#[function_component]
pub fn NewFileButton(prop: &TopButtonProps) -> Html {
    let create_file_in = use_state(|| None);

    let finished = {
        let create_file_in = create_file_in.clone();
        let updateui = prop.updateui.clone();

        move |_| {
            create_file_in.set(None);
            updateui.emit(());
        }
    };

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

    let theme = prop.theme;

    let style = format! {"
    width: 3%;
    height: 100%;
    display: flex;
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
        html! { <NewFilePopUp directory = { directory.clone() } {finished} {theme}/> }
    } else {
        html! {}
    };

    html! {
        <div {style} {onclick}>
            {create_or_not}
            <div class = "card" style = {icon_style}>
                <AddFile />
            </div>
            <NameField name = { "file" }  {theme} />
        </div>
    }
}
