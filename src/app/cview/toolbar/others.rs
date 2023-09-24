use std::{cell::RefCell, rc::Rc};

use crate::app::{confirm::ConfirmDelete, ApplicationContext, ApplicationData};

use super::NameField;
use wasm_bindgen::JsValue;
use yew::prelude::*;

struct StateData {
    deletedata: JsValue,
    prompt: String,
}

use super::TopButtonProps;

#[function_component]
pub fn DeleteButton(prop: &TopButtonProps) -> Html {
    let ApplicationContext {
        sizes,
        theme: _,
        settings: _,
    } = use_context().unwrap();
    let cdata = use_context::<ApplicationData>().unwrap();
    let deletedata = use_state(|| RefCell::new(None));

    let onclick = {
        let deletedata = deletedata.clone();
        let cdata = cdata.clone();
        move |_| {
            let items = cdata.selected_files();
            let items = items.borrow();
            if items.len() > 0 {
                let prompt = {
                    let first_item = items.iter().next().unwrap().name();
                    let others = if items.len() == 2 {
                        format! {" and {} other", items.len() - 1}
                    } else if items.len() > 2 {
                        format! {" and {} others", items.len() - 1}
                    } else {
                        "".into()
                    };
                    format! {"Are you sure you want to delete '{first_item}'{others}?"}
                };
                let res = crate::app::tasks::to_js_items(items.iter());

                deletedata.set(RefCell::new(Some(StateData {
                    deletedata: res,
                    prompt,
                })));
            }
        }
    };

    let cancel = {
        let deletedata = deletedata.clone();
        move |_| deletedata.set(RefCell::new(None))
    };

    let prompt_or_not =
        if let Some(StateData { deletedata, prompt }) = deletedata.borrow_mut().take() {
            let deletedata = Rc::new(RefCell::new(Some(deletedata)));
            html! {
                <ConfirmDelete {deletedata}  {prompt} {cancel} updateui = {prop.updateui.clone()}/>
            }
        } else {
            html! {}
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

    html! {
        <div {style} {onclick}>
            {prompt_or_not}
            <div style = {icon_style}>
                <img class = "card" style = {img_style} src="./public/icons/top/delete.png" alt="delete" />
            </div>
            <NameField name = { "delete" }/>
        </div>
    }
}

#[function_component]
pub fn RefreshButton(_prop: &TopButtonProps) -> Html {
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context().unwrap();

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

    html! {
        <div {style}>
            <div style = {icon_style}>
                <img class = "card" style = {img_style} src="./public/icons/top/refresh.png" alt="refresh" />
            </div>
            <NameField name = { "refresh" }/>
        </div>
    }
}
