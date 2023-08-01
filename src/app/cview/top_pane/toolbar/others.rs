use crate::app::{
    confirm::Confirm, cview::top_pane::toolbar::NameField, invoke, DeleteIcon, ItemsArg,
    RefreshIcon, Theme,
};
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component]
pub fn DeleteButton(prop: &RefreshButtonProps) -> Html {
    let items_to_delete = use_state(|| None);

    let onclick = {
        let items_to_delete = items_to_delete.clone();
        move |_| {
            if let Some(items) = crate::data::selected() {
                if items.len() > 0 {
                    items_to_delete.set(Some(items.clone()))
                }
            }
        }
    };

    let confirm = {
        let updateui = prop.updateui.clone();
        let items_to_delete = items_to_delete.clone();
        move |_| {
            let updateui = updateui.clone();
            let items_to_delete = items_to_delete.clone();
            spawn_local(async move {
                if let Some(items) = &*items_to_delete {
                    let arg = to_value(&ItemsArg { items }).unwrap();
                    invoke("delete", arg).await;
                    items_to_delete.set(None);
                    updateui.emit(());
                }
            });
        }
    };

    let cancel = {
        let items_to_delete = items_to_delete.clone();
        move |_| items_to_delete.set(None)
    };

    let theme = prop.theme;

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
            <Confirm {theme} {confirm} {cancel} {prompt}/>
        }
    } else {
        html! {}
    };

    let style = format! {"
    width: 50px;
    height: 100%;
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    align-items: center;
    "};

    let icon_style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    height: 70%;
    width: 100%;
    "};

    html! {
        <div {style} {onclick}>
            {prompt_or_not}
            <div class = "card" style = {icon_style}>
                <DeleteIcon />

            </div>
            <NameField name = { "delete" }  { theme } />
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct RefreshButtonProps {
    pub theme: Theme,
    pub updateui: Callback<()>,
}

#[function_component]
pub fn RefreshButton(prop: &RefreshButtonProps) -> Html {
    let updateui = prop.updateui.clone();

    let onclick = move |_| updateui.emit(());

    let theme = prop.theme;

    let style = format! {"
    width: 50px;
    height: 100%;
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    align-items: center;
    "};

    let icon_style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    height: 70%;
    width: 100%;
    "};

    html! {
        <div {style} {onclick}>
            <div class = "card" style = {icon_style}>
                <RefreshIcon />
            </div>
            <NameField name = { "refresh" }  { theme } />
        </div>
    }
}
