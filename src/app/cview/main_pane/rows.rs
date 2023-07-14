use crate::{
    app::{StackedDirIcon, PitouProps, Props, Theme},
    background_color,
};
//use gloo::console::log;
use yew::prelude::*;

use super::CheckBox;


#[derive(Properties, PartialEq)]
pub struct RowProps {
    pub(super) pitou: Props,
    pub(super) idx: usize,
    pub(super) selected: bool,
    pub(super) toggle_select: Callback<usize>,
}

impl RowProps {
    fn pitou(&self) -> &Props {
        &self.pitou
    }

    fn theme(&self) -> &Theme {
        &self.pitou.theme()
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

    let on_toggle = {
        let toggle_select = prop.toggle_select.clone();
        let idx = prop.idx;

        move |()| {
            toggle_select.emit(idx);
        }
    };

    let hover_background = prop.theme().background1();

    let style = format! {"
        display: flex;
        flex-direction: row;
        gap: 0;
        height: 10%;
        width: 100%;
        {}
        text-align: left;", background_color!(prop.selected || *is_hovered, hover_background) };

    let pitou = prop.pitou();

    html! {
        <div {style} {onmouseover} {onmouseout}>
            <CheckBox {on_toggle} is_checked = { prop.selected } />
            <FileIcon pitou = { pitou.clone() } />
            <FileName pitou = { pitou.clone() } />
            <FileType pitou = { pitou.clone() } />
            <LastModified pitou = { pitou.clone() }/>
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
        <StackedDirIcon/>
        </div>
    }
}

#[function_component]
fn FileName(prop: &PitouProps) -> Html {
    let style = format! {"
    display: flex;
    flex-direction: row;
    gap: 0;
    left: 10%;
    width: 50%;
    height: 100%;
    color: {};
    font-family: monospace;
    padding-left: 1%;
    font-size: 100%;
    align-items: center;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;", prop.theme().foreground1() };
    html! {
        <div {style}>{ std::path::PathBuf::from(prop.pitou_file().name()).display() }</div>
    }
}

#[function_component]
fn FileType(prop: &PitouProps) -> Html {
    let style = format! {"
    display: flex;
    flex-direction: row;
    gap: 0;
    left: 60%;
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
    left: 80%;
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
        <div {style}>
            {"Last Modified Date"}
        </div>
    }
}
