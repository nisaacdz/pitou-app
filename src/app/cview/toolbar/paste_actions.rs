use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::app::ApplicationContext;

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
    } = use_context().unwrap();
    let updateui = prop.updateui.clone();

    let onclick = move |_| {
        let updateui = updateui.clone();
        crate::app::data::directory()
            .map(|dir| {
                spawn_local(async move {
                    crate::app::tasks::paste(&*dir).await;
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
    } = use_context().unwrap();

    let onclick = move |_| {
        if let Some(items) = crate::app::data::all() {
            let borrow = items.borrow();
            if borrow.len() > 0 {
                let arg = crate::app::tasks::to_js_items(borrow.iter());

                spawn_local(async move {
                    crate::app::tasks::copy(arg).await;
                });
            }
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
    } = use_context().unwrap();

    let onclick = move |_| {
        if let Some(items) = crate::app::data::all() {
            let borrow = items.borrow();
            if borrow.len() > 0 {
                let arg = crate::app::tasks::to_js_items(borrow.iter());

                spawn_local(async move {
                    crate::app::tasks::cut(arg).await;
                });
            }
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
    } = use_context().unwrap();
    let clipboard = use_state(|| None);

    let onclick = {
        let clipboard = clipboard.clone();

        move |_| {
            let clipboard = clipboard.clone();
            spawn_local(async move {
                if let None = &*clipboard {
                    let items = crate::app::tasks::clipboard().await;
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
