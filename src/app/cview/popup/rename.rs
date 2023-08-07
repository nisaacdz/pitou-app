use std::{cell::RefCell, rc::Rc};

use backend::Pitou;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::app::Theme;

#[derive(PartialEq, Properties)]
pub struct RenamePopUpProps {
    pub file: Pitou,
    pub onclickok: Callback<Rc<RefCell<String>>>,
    pub onclickcancel: Callback<()>,
}

#[function_component]
pub fn RenamePopUp(prop: &RenamePopUpProps) -> Html {
    let theme = use_context::<Theme>().unwrap();

    let border_color = theme.spare();
    let background_color = theme.background2();

    let style = format! {"
    background-color: {background_color};
    border: 2px solid {border_color};
    "};

    let current_name = prop.file.name();
    let oldname = format! {"current name: {current_name}"};

    let onclick = |e: MouseEvent| e.stop_immediate_propagation();

    let newname = std::rc::Rc::new(std::cell::RefCell::new(String::new()));

    let onclickok = {
        let onclickok = prop.onclickok.clone();
        let newname = newname.clone();
        move |_| onclickok.emit(newname.clone())
    };

    let onclickcancel = {
        let onclickcancel = prop.onclickcancel.clone();
        move |_| onclickcancel.emit(())
    };

    let oninput = {
        let newname = newname.clone();
        move |e: InputEvent| {
            e.target_dyn_into::<HtmlInputElement>()
                .map(|elem| *newname.borrow_mut() = elem.value())
                .unwrap_or_default()
        }
    };

    let onkeypress = {
        let onclickok = prop.onclickok.clone();
        let newname = newname.clone();
        move |e: KeyboardEvent| {
            if e.key_code() == 13 {
                onclickok.emit(newname.clone())
            }
        }
    };

    let placeholder = "Enter new name...".to_owned();

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
