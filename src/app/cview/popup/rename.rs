use backend::Pitou;
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::app::{invoke, Theme};
use serde::Serialize;

#[derive(PartialEq, Properties)]
pub struct RenamePopUpProps {
    pub file: Pitou,
    pub theme: Theme,
    pub finished: Callback<()>,
}

#[derive(Serialize)]
struct RenameArg<'a> {
    pitou: &'a Pitou,
    name: &'a String,
}

#[function_component]
pub fn RenamePopUp(prop: &RenamePopUpProps) -> Html {
    let border_color = prop.theme.spare();
    let background_color = prop.theme.background2();

    let style = format! {"
    background-color: {background_color};
    border: 2px solid {border_color};
    "};

    let current_name = prop.file.name();
    let oldname = format! {"current name: {current_name}"};

    let newname = std::rc::Rc::new(std::cell::RefCell::new(String::new()));

    let oninput = {
        let newname = newname.clone();
        move |e: InputEvent| {
            e.target_dyn_into::<HtmlInputElement>()
                .map(|elem| *newname.borrow_mut() = elem.value())
                .unwrap_or_default()
        }
    };

    let onclickok = {
        let finished = prop.finished.clone();
        let name = newname.clone();
        let file = prop.file.clone();

        move |_| {
            let name = name.clone();
            let finished = finished.clone();
            let file = file.clone();
            spawn_local(async move {
                let args = to_value(&RenameArg {
                    pitou: &file,
                    name: &name.borrow_mut(),
                })
                .unwrap();
                invoke("rename", args).await;
                finished.emit(());
            });
        }
    };

    let onclickcancel = {
        let finished = prop.finished.clone();
        move |_| finished.emit(())
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
        <div {style} class = {"popup"}>
            <p>{ oldname }</p>
            <input type="text" {oninput} {placeholder}/>
            <div>
                <input type="checkbox"/>
                <span>{ "Override Existing" }</span>
            </div>
            <div style = {buttons_style}>
                <button onclick={onclickok} style = {button1_style}>{ "OK" }</button>
                <button onclick={onclickcancel} style = {button2_style}>{ "Cancel" }</button>
            </div>
        </div>
    }
}
