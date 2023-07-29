use backend::{Metadata, Pitou};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    app::{invoke, DirIcon, FileIcon, LoadingIcon, PitouArg, SymLinkIcon, Theme},
    background_color,
};

#[derive(PartialEq, Properties)]
pub struct SidePaneRowProps {
    pub pitou: Pitou,
    pub theme: Theme,
    pub updatedirectory: Callback<Pitou>,
    pub selected: bool,
}

#[function_component]
pub fn SidePaneRow(prop: &SidePaneRowProps) -> Html {
    let metadata = use_state(|| None);
    let hovered = use_state_eq(|| false);

    let onmouseover = {
        let hovered = hovered.clone();
        move |_| {
            hovered.set(true);
        }
    };

    let onmouseout = {
        let hovered = hovered.clone();
        move |_| {
            hovered.set(false);
        }
    };

    {
        let pitou = prop.pitou.clone();
        let metadata = metadata.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let arg = PitouArg { pitou: &pitou };
                    let arg = to_value(&arg).unwrap();
                    let res = invoke("metadata", arg).await;

                    let res = from_value::<Metadata>(res).unwrap();
                    metadata.set(Some(res));
                })
            },
            (),
        )
    }

    let theme = prop.theme;

    let foreground_color = theme.foreground1();

    let style = format! {"
        display: flex;
        flex-direction: row;
        gap: 0;
        color: {foreground_color};
        font-family: monospace;
        height: 10%;
        width: 100%;
        font-size: 100%;
        {}
        text-align: left;", background_color!(*hovered || prop.selected, theme.background1()) };

    let filetype = {
        if let Some(m) = &*metadata {
            Some(m.file_type())
        } else {
            None
        }
    };

    html! {
        <div {style} {onmouseover} {onmouseout}>
            <FileIconCmp {filetype} {theme} />
            <SidePaneFileName pitou = { prop.pitou.clone() } {theme} updatedirectory = { prop.updatedirectory.clone() } />
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct FileTypeProps {
    pub theme: Theme,
    pub filetype: Option<backend::PitouType>,
}

#[function_component]
fn FileIconCmp(prop: &FileTypeProps) -> Html {
    let style = format! {"
        display: flex;
        align-items: center;
        width: 15%;
        height: 100%;
        padding-left: 3%;
        justify-content: center;
    "};

    let icon = match prop.filetype {
        Some(v) => match v {
            backend::PitouType::File => html! { <FileIcon /> },
            backend::PitouType::Directory => html! { <DirIcon /> },
            backend::PitouType::Link => html! { <SymLinkIcon /> },
        },
        None => html! { <LoadingIcon /> },
    };

    html! {
        <div {style}> { icon } </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct SidePaneFileNameProps {
    pitou: Pitou,
    theme: Theme,
    updatedirectory: Callback<Pitou>,
}

#[function_component]
pub(super) fn SidePaneFileName(prop: &SidePaneFileNameProps) -> Html {
    let style = format! {"
        left: 15%;
        width: 75%;
        height: 100%;
        align-items: center;
        overflow: hidden;
        white-space: nowrap;
        padding-left: 3%;
        text-overflow: ellipsis;
    "};

    let onclick = {
        let pitou = prop.pitou.clone();
        let updatedirectory = prop.updatedirectory.clone();
        move |_| updatedirectory.emit(pitou.clone())
    };

    html! {
        <p {style} {onclick}>
            { prop.pitou.name() }
        </p>
    }
}
