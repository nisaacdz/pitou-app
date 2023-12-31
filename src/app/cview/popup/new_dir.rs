use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::app::ApplicationContext;
use std::{path::PathBuf, rc::Rc};

#[derive(PartialEq, Properties)]
pub struct NewDirPopUpProps {
    pub directory: Rc<PathBuf>,
    pub onclickcancel: Callback<()>,
    pub onclickok: Callback<String>,
}

#[function_component]
pub fn NewDirPopUp(prop: &NewDirPopUpProps) -> Html {
    let ApplicationContext {
        theme,
        sizes: _,
        settings: _,
    } = use_context().unwrap();
    let input_ref = use_node_ref();

    let border_color = theme.spare();
    let background_color = theme.background2();

    let style = format! {"
    background-color: {background_color};
    border: 2px solid {border_color};
    "};

    let folder_name = prop
        .directory
        .file_name()
        .map(|v| v.to_str().unwrap_or_default())
        .unwrap_or_default();
    let oldname = format! {"Create new folder in: {folder_name}"};
    let onclick = |e: MouseEvent| e.stop_propagation();

    let onclickok = {
        let onclickok = prop.onclickok.clone();
        let input_ref = input_ref.clone();
        move |_| onclickok.emit(input_ref.cast::<HtmlInputElement>().unwrap().value())
    };

    let onclickcancel = {
        let onclickcancel = prop.onclickcancel.clone();
        move |_| onclickcancel.emit(())
    };

    let onkeypress = {
        let onclickok = prop.onclickok.clone();
        let input_ref = input_ref.clone();
        move |e: KeyboardEvent| {
            if e.key_code() == 13 {
                onclickok.emit(input_ref.cast::<HtmlInputElement>().unwrap().value())
            }
        }
    };

    let placeholder = "Enter Directory name...".to_owned();

    let buttons_style = format! {"
    height: 15%;
    display: flex;
    gap: 0;
    "};

    let button1_style = format! {"
    width: 50%;
    "};

    let button2_style = format! {"
    width: 50%;
    "};

    html! {
        <div {style} class = {"popup"} {onclick}>
            <p>{ oldname }</p>
            <input type="text" {placeholder} {onkeypress} ref = {input_ref}/>
            <div>
                <input type="checkbox"/>
                <span>{ "Override Existing" }</span>
            </div>
            <div style = {buttons_style}>
                <button onclick={onclickcancel} style = {button2_style}>{ "Cancel" }</button>
                <button onclick={onclickok} style = {button1_style}>{ "OK" }</button>
            </div>
        </div>
    }
}
