use crate::app::{confirm::Confirm, data::selections, invoke, ApplicationContext, ItemsArg};

use super::NameField;
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use super::TopButtonProps;

#[function_component]
pub fn DeleteButton(prop: &TopButtonProps) -> Html {
    let ApplicationContext {
        sizes,
        theme: _,
        settings: _,
    } = use_context::<ApplicationContext>().unwrap();
    let items_to_delete = use_state(|| None);

    let onclick = {
        let items_to_delete = items_to_delete.clone();
        move |_| {
            if selections::len() > 0 {
                items_to_delete.set(selections::all().map(|items| items.collect::<Vec<_>>()))
            }
        }
    };

    let confirm = {
        let updateui = prop.updateui.clone();
        let items_to_delete = items_to_delete.clone();
        move |_| {
            if let Some(items) = &*items_to_delete {
                let arg = to_value(&ItemsArg { items: items }).unwrap();
                let items_to_delete = items_to_delete.clone();
                let updateui = updateui.clone();
                spawn_local(async move {
                    invoke("delete", arg).await;
                    items_to_delete.set(None);
                    updateui.emit(());
                });
            }
        }
    };

    let cancel = {
        let items_to_delete = items_to_delete.clone();
        move |_| items_to_delete.set(None)
    };

    let prompt_or_not = if let Some(items) = &*items_to_delete {
        let first_item = items.first().map(|first| first.name()).unwrap_or_default();
        let others = if items.len() == 2 {
            format! {" and {} other", items.len() - 1}
        } else if items.len() > 2 {
            format! {" and {} others", items.len() - 1}
        } else {
            "".into()
        };

        let prompt = format! {"Are you sure you want to delete '{first_item}'{others}?"};

        html! {
            <Confirm {confirm} {cancel} {prompt}/>
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
pub fn RefreshButton(prop: &TopButtonProps) -> Html {
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context::<ApplicationContext>().unwrap();
    let updateui = prop.updateui.clone();

    let onclick = move |_| updateui.emit(());

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
            <div style = {icon_style}>
                <img class = "card" style = {img_style} src="./public/icons/top/refresh.png" alt="refresh" />
            </div>
            <NameField name = { "refresh" }/>
        </div>
    }
}
