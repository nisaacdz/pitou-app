use backend::Pitou;
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use serde::Serialize;

use crate::app::{Theme, invoke, PitouArg};

#[derive(PartialEq, Properties)]
pub struct NewFilePopUpProps {
    pub directory: Pitou,
    pub theme: Theme,
    pub finished: Callback<()>,
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

    let filename = std::rc::Rc::new(std::cell::RefCell::new(String::new()));

    let oninput = {
        let filename = filename.clone();
        move |e: InputEvent| {
            e.target_dyn_into::<HtmlInputElement>()
                .map(|elem| *filename.borrow_mut() = elem.value())
                .unwrap_or_default()
        }
    };

    let onclickok = {
        let finished = prop.finished.clone();
        let name = filename.clone();
        let directory = prop.directory.clone();

        move |_| {
            let name = name.clone();
            let finished = finished.clone();
            let directory = directory.clone();
            spawn_local(async move {
                let createme = directory.path().join(&*name.borrow()).into();
                let args = to_value(&PitouArg{ pitou: &createme }).unwrap();
                invoke("createfile", args).await;
                finished.emit(());
            });
        }
    };

    let onclickcancel = {
        let finished = prop.finished.clone();
        move |_| finished.emit(())
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