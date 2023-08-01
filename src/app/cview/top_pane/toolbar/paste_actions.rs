use std::collections::LinkedList;

use crate::app::{
    invoke, ClipboardIcon, CopyIcon, CutIcon, ItemsArg, PasteIcon, PitouArg, PitouNoArg, Theme,
};
use backend::Pitou;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use super::{NameField, TopButtonProps};

#[derive(PartialEq, Properties)]
pub struct PasteButtonProps {
    pub theme: Theme,
    pub updateui: Callback<()>,
}

#[function_component]
pub fn PasteButton(prop: &PasteButtonProps) -> Html {
    let updateui = prop.updateui.clone();

    let onclick = move |_| {
        let updateui = updateui.clone();
        crate::data::directory()
            .map(|pitou| {
                spawn_local(async move {
                    let args = to_value(&PitouArg { pitou }).unwrap();
                    invoke("paste", args).await;
                    updateui.emit(());
                });
            })
            .unwrap_or_default();
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

    let theme = prop.theme;

    html! {
        <div {style} {onclick}>
            <div class = "card" style = {icon_style}>
                <PasteIcon />
            </div>
            <NameField name = { "paste" }  { theme } />
        </div>
    }
}

#[function_component]
pub fn CopyButton(prop: &TopButtonProps) -> Html {
    let onclick = move |_| {
        crate::data::get_selected()
            .map(|items| {
                spawn_local(async move {
                    let args = to_value(&ItemsArg { items: &items }).unwrap();
                    invoke("copy", args).await;
                });
            })
            .unwrap_or_default()
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

    let theme = prop.theme;

    html! {
        <div {style} {onclick}>
            <div class = "card" style = {icon_style}>
                <CopyIcon />

            </div>
            <NameField name = { "copy" }  { theme } />
        </div>
    }
}

#[function_component]
pub fn CutButton(prop: &TopButtonProps) -> Html {
    let onclick = move |_| {
        crate::data::get_selected()
            .map(|items| {
                spawn_local(async move {
                    let args = to_value(&ItemsArg { items: &items }).unwrap();
                    invoke("cut", args).await;
                });
            })
            .unwrap_or_default()
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

    let theme = prop.theme;

    html! {
        <div {style} {onclick}>
            <div class = "card" style = {icon_style}>
                <CutIcon />

            </div>
            <NameField name = { "cut" }  { theme } />
        </div>
    }
}

#[function_component]
pub fn ClipboardButton(prop: &TopButtonProps) -> Html {
    let clipboard = use_state(|| None);

    let onclick = {
        let clipboard = clipboard.clone();

        move |_| {
            let clipboard = clipboard.clone();
            spawn_local(async move {
                if let None = &*clipboard {
                    let arg = to_value(&PitouNoArg {}).unwrap();
                    let res = invoke("clipboard", arg).await;
                    let items = from_value::<LinkedList<Vec<Pitou>>>(res).unwrap();
                    clipboard.set(Some(items));
                }
            });
        }
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

    let theme = prop.theme;

    html! {
        <div {style} {onclick}>
            <div class = "card" style = {icon_style}>
                <ClipboardIcon />

            </div>
            <NameField name = { "clipboard" }  { theme } />
        </div>
    }
}
