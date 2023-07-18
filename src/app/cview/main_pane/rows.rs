use crate::{
    app::{DirIcon, PitouProps, Theme},
    background_color,
};
use backend::Pitou;
//use gloo::console::log;
use yew::prelude::*;

use super::CheckBox;

#[derive(Properties, PartialEq)]
pub struct RowProps {
    pub(super) pitou: Pitou,
    pub(super) theme: Theme,
    pub(super) idx: usize,
    pub(super) selected: bool,
    pub(super) toggleselect: Callback<usize>,
    pub(super) onclick: Callback<Pitou>,
}

impl RowProps {
    fn pitou(&self) -> &Pitou {
        &self.pitou
    }

    fn theme(&self) -> Theme {
        self.theme
    }
}

#[function_component]
pub fn Row(prop: &RowProps) -> Html {
    let is_hovered = use_state_eq(|| false);

    let onmouseover = {
        let is_hovered = is_hovered.clone();

        move |_| is_hovered.set(true)
    };

    let onmouseout = {
        let is_hovered = is_hovered.clone();

        move |_| is_hovered.set(false)
    };

    let ontoggle = {
        let toggleselect = prop.toggleselect.clone();
        let idx = prop.idx;

        move |()| {
            toggleselect.emit(idx);
        }
    };

    let hover_background = prop.theme().background1();

    let updatedirectory = {
        let update_directory = prop.onclick.clone();
        let pitou = prop.pitou.clone();

        move |_| update_directory.emit(pitou.clone())
    };

    let style = format! {"
        display: flex;
        flex-direction: row;
        gap: 0;
        height: 10%;
        width: 100%;
        font-size: 100%;
        {}
        text-align: left;", background_color!(prop.selected || *is_hovered, hover_background) };

    let pitou = prop.pitou();
    let theme = prop.theme();

    html! {
        <div {style} {onmouseover} {onmouseout}>
            <CheckBox {ontoggle} ischecked = { prop.selected } />
            <FileIcon pitou = { pitou.clone() } {theme} />
            <FileName pitou = { pitou.clone() } {theme} {updatedirectory} />
            <FileType pitou = { pitou.clone() } {theme} />
            <LastModified pitou = { pitou.clone() } {theme}/>
        </div>
    }
}

#[function_component]
fn FileIcon(_prop: &PitouProps) -> Html {
    let style = format! {"
        display: flex;
        flex-direction: row;
        align-items: center;
        left: 5%;
        width: 5%;
        height: 100%;
        justify-content: center;
    "};

    html! {
        <div {style}>
        <DirIcon/>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct FileNameProps {
    pub pitou: Pitou,
    pub theme: Theme,
    pub updatedirectory: Callback<()>,
}

#[function_component]
fn FileName(prop: &FileNameProps) -> Html {
    let foreground = prop.theme.foreground1();
    let style = format! {"
    display: flex;
    flex-direction: row;
    gap: 0;
    left: 10%;
    width: 45%;
    height: 100%;
    color: {foreground};
    font-family: monospace;
    padding-left: 1%;
    font-size: 100%;
    align-items: center;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;"};

    let onclick = {
        let update_directory = prop.updatedirectory.clone();
        move |_| update_directory.emit(())
    };

    let name = prop.pitou.name().unwrap_or_default();

    html! {
        <div {style} {onclick}>{ std::path::PathBuf::from(name).display() }</div>
    }
}

#[function_component]
fn FileType(prop: &PitouProps) -> Html {
    let style = format! {"
    display: flex;
    flex-direction: row;
    gap: 0;
    left: 55%;
    width: 20%;
    height: 100%;
    color: {};
    font-family: monospace;
    padding-left: 1%;
    font-size: 100%;
    align-items: center;
    overflow: hidden;
    justify-content: center;
    
    white-space: nowrap;
    text-overflow: ellipsis;", prop.theme().foreground1() };

    //TODO
    html! {
        <div {style}>{"Directory"}</div>
    }
}

#[function_component]
fn LastModified(prop: &PitouProps) -> Html {
    let style = format! {"
    position: static;
    display: flex;
    flex-direction: row;
    gap: 0;
    left: 75%;
    width: 25%;
    height: 100%;
    color: {};
    font-family: monospace;
    padding-left: 1%;
    font-size: 100%;
    align-items: center;
    overflow: hidden;
    justify-content: center;
    white-space: nowrap;
    text-overflow: ellipsis;", prop.theme().foreground1() };

    //TODO
    html! {
        <div {style}>
            {"Last Modified Date"}
        </div>
    }
}
