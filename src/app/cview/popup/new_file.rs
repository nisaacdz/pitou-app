use std::{rc::Rc, cell::RefCell};

use backend::Pitou;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use serde::Serialize;

use crate::app::Theme;

#[derive(PartialEq, Properties)]
pub struct NewFilePopUpProps {
    pub directory: Pitou,
    pub theme: Theme,
    pub onclickok: Callback<Rc<RefCell<String>>>,
    pub onclickcancel: Callback<()>,
}

#[derive(Serialize)]
struct NewFileArgs<'a> {
    pitou: &'a Pitou,
    name: &'a String,
}

#[function_component]
pub fn NewFilePopUp(prop: &NewFilePopUpProps) -> Html {
    let border_color = prop.theme.spare();
    let background_color = prop.theme.background2();

    let style = format! {"
    background-color: {background_color};
    border: 2px solid {border_color};
    "};

    let folder_name = prop.directory.name();
    let oldname = format! {"Create new file in: {folder_name}"};

    let filename = Rc::new(RefCell::new(String::new()));

    let oninput = {
        let filename = filename.clone();
        move |e: InputEvent| e.target_dyn_into::<HtmlInputElement>()
                .map(|elem| *filename.borrow_mut() = elem.value())
                .unwrap_or_default()
    };

    let onclick = |e: MouseEvent| e.stop_immediate_propagation();

    let onclickok = {
        let onclickok = prop.onclickok.clone();
        let filename = filename.clone();
        move |_| onclickok.emit(filename.clone())
    };

    let onclickcancel = {
        let onclickcancel = prop.onclickcancel.clone();
        move |_| onclickcancel.emit(())
    };

    let onkeypress = {
        let onclickok = prop.onclickok.clone();
        let filename = filename.clone();
        move |e: KeyboardEvent| if e.key_code() == 13 { onclickok.emit(filename.clone()) }
    };

    let placeholder = "Enter file name...".to_owned();

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
            <input type="text" {oninput} {placeholder} {onkeypress} />
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