use std::collections::LinkedList;

use crate::app::{data::selections, invoke, ApplicationContext, ItemsArg, PitouArg, PitouNoArg};
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
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context::<ApplicationContext>().unwrap();
    let updateui = prop.updateui.clone();

    let onclick = move |_| {
        let updateui = updateui.clone();
        crate::app::data::directory()
            .map(|pitou| {
                let args = to_value(&PitouArg { pitou }).unwrap();
                spawn_local(async move {
                    invoke("paste", args).await;
                    updateui.emit(());
                });
            })
            .unwrap_or_default();
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
            <div style = {icon_style}>
                <img class = "card" style = {img_style} src="./public/icons/top/paste.png" alt="paste" />
            </div>
            <NameField name = { "paste" }/>
        </div>
    }
}

#[function_component]
pub fn CopyButton(_prop: &TopButtonProps) -> Html {
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context::<ApplicationContext>().unwrap();

    let onclick = move |_| {
        if selections::len() > 0 {
            let arg = to_value(&ItemsArg {
                items: &selections::all()
                    .map(|items| items.collect::<Vec<_>>())
                    .unwrap_or(Vec::new()),
            })
            .unwrap();
            spawn_local(async move {
                invoke("copy", arg).await;
            });
        }
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
            <div style = {icon_style}>
                <img class = "card" style = {img_style} src="./public/icons/top/copy.png" alt="copy" />
            </div>
            <NameField name = { "copy" }/>
        </div>
    }
}

#[function_component]
pub fn CutButton(_prop: &TopButtonProps) -> Html {
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context::<ApplicationContext>().unwrap();
    let onclick = move |_| {
        if selections::len() > 0 {
            let arg = to_value(&ItemsArg {
                items: &selections::all()
                    .map(|items| items.collect::<Vec<_>>())
                    .unwrap_or(Vec::new()),
            })
            .unwrap();
            spawn_local(async move {
                invoke("cut", arg).await;
            });
        }
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
            <div style = {icon_style}>
                <img class = "card" style = {img_style} src="./public/icons/top/cut.png" alt="cut" />
            </div>
            <NameField name = { "cut" }/>
        </div>
    }
}

#[function_component]
pub fn ClipboardButton(_prop: &TopButtonProps) -> Html {
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context::<ApplicationContext>().unwrap();
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
                <img class = "card" style = {img_style} src="./public/icons/top/clipboard.png" alt="clipboard" />
            </div>
            <NameField name = { "clipboard" }/>
        </div>
    }
}
