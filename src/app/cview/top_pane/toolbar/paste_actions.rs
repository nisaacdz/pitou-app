use crate::app::{invoke, ClipboardIcon, CopyIcon, CutIcon, ItemsArg, PasteIcon, PitouArg, Theme};
use serde_wasm_bindgen::to_value;
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

    let theme = prop.theme;

    html! {
        <div {style}>
            <div class = "card" style = {icon_style}>
                <ClipboardIcon />

            </div>
            <NameField name = { "clipboard" }  { theme } />
        </div>
    }
}
