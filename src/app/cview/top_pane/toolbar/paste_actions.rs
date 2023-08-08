use std::collections::LinkedList;

use crate::app::{
    invoke, ClipboardIcon, CopyIcon, CutIcon, ItemsArg, PasteIcon, PitouArg, PitouNoArg,
};
use backend::Pitou;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use super::{NameField, TopButtonProps};

#[derive(PartialEq, Properties)]
pub struct PasteButtonProps {
    pub updateui: Callback<()>,
}

#[function_component]
pub fn PasteButton(prop: &PasteButtonProps) -> Html {
    let updateui = prop.updateui.clone();

    let onclick = move |_| {
        let updateui = updateui.clone();
        crate::data::directory()
            .map(|pitou| {
                let args = to_value(&PitouArg { pitou }).unwrap();
                spawn_local(async move {
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

    html! {
        <div {style} {onclick}>
            <div class = "card" style = {icon_style}>
                <PasteIcon />
            </div>
            <NameField name = { "paste" }/>
        </div>
    }
}

#[function_component]
pub fn CopyButton(_prop: &TopButtonProps) -> Html {
    let onclick = move |_| {
        if crate::data::selected_len() > 0 {
            let arg = to_value(&ItemsArg {
                items: &crate::data::selected().collect::<Vec<_>>(),
            })
            .unwrap();
            spawn_local(async move {
                invoke("copy", arg).await;
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

    html! {
        <div {style} {onclick}>
            <div class = "card" style = {icon_style}>
                <CopyIcon />

            </div>
            <NameField name = { "copy" }/>
        </div>
    }
}

#[function_component]
pub fn CutButton(_prop: &TopButtonProps) -> Html {
    let onclick = move |_| {
        if crate::data::selected_len() > 0 {
            let arg = to_value(&ItemsArg {
                items: &crate::data::selected().collect::<Vec<_>>(),
            })
            .unwrap();
            spawn_local(async move {
                invoke("cut", arg).await;
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

    html! {
        <div {style} {onclick}>
            <div class = "card" style = {icon_style}>
                <CutIcon />

            </div>
            <NameField name = { "cut" }/>
        </div>
    }
}

#[function_component]
pub fn ClipboardButton(_prop: &TopButtonProps) -> Html {
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

    html! {
        <div {style} {onclick}>
            <div class = "card" style = {icon_style}>
                <ClipboardIcon />

            </div>
            <NameField name = { "clipboard" }/>
        </div>
    }
}
