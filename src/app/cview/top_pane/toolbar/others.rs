use crate::app::{invoke, DeleteIcon, ItemsArg, RefreshIcon, Theme, cview::top_pane::toolbar::NameField};
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component]
pub fn DeleteButton(prop: &RefreshButtonProps) -> Html {
    let onclick = {
        let updateui = prop.updateui.clone();
        move |_| {
            let updateui = updateui.clone();
            spawn_local(async move {
                if let Some(items) = &crate::data::get_selected() {
                    let args = to_value(&ItemsArg { items }).unwrap();
                    invoke("delete", args).await;
                    updateui.emit(());
                }
            });
        }
    };

    let theme = prop.theme;

    let style = format! {"
    width: 3%;
    height: 100%;
    display: flex;
    flex-direction: column;
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
    width: 3%;
    height: 100%;
    display: flex;
    flex-direction: column;
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
