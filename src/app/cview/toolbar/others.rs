use std::rc::Rc;

use crate::app::{confirm::ConfirmDelete, ApplicationContext, ApplicationData};

use super::NameField;
use backend::File;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

struct DeleteData {
    items: Rc<Vec<File>>,
    prompt: String,
}

use super::TopButtonProps;

pub fn generate_prompt(items: &Vec<File>) -> String {
    if items.len() > 0 {
        let first_item = items.iter().next().unwrap().name();
        let others = if items.len() == 2 {
            format! {" and {} other", items.len() - 1}
        } else if items.len() > 2 {
            format! {" and {} others", items.len() - 1}
        } else {
            "".into()
        };
        format! {"Are you sure you want to delete '{first_item}'{others}?"}
    } else {
        String::from("Nothing to delete")
    }
}

#[function_component]
pub fn DeleteButton(_prop: &TopButtonProps) -> Html {
    let ApplicationContext {
        sizes,
        theme: _,
        settings: _,
    } = use_context().unwrap();
    let cdata = use_context::<ApplicationData>().unwrap();
    let deletedata = use_state(|| None);

    let onclick = {
        let deletedata = deletedata.clone();
        let cdata = cdata.clone();
        move |_| {
            let items = cdata
                .selected_files()
                .borrow()
                .iter()
                .map(|v| v.clone())
                .collect::<Vec<_>>();
            let prompt = generate_prompt(&items);

            deletedata.set(Some(DeleteData {
                items: Rc::new(items),
                prompt,
            }));
        }
    };

    let cancel = {
        let deletedata = deletedata.clone();
        move |_| deletedata.set(None)
    };

    let delete = {
        let deletedata = deletedata.clone();
        move |()| {
            if let Some(dd) = &*deletedata {
                let items = dd.items.clone();
                spawn_local(async move { crate::app::tasks::delete(&*items).await });
                deletedata.set(None);
            }
        }
    };

    let prompt_or_not = if let Some(DeleteData { items: _, prompt }) = &*deletedata {
        html! {
            <ConfirmDelete  {delete} prompt = {prompt.clone()} {cancel}/>
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
