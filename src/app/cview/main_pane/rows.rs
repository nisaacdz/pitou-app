use crate::{
    app::{invoke, DirIcon, FileIcon, LoadingIcon, PitouArg, SymLinkIcon, Theme},
    background_color,
};
use backend::{DateTime, Metadata, Pitou, PitouType};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_futures::spawn_local;
// use gloo::console::log;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RowProps {
    pub(super) pitou: Pitou,
    pub(super) selected: bool,
    pub(super) theme: Theme,
    pub(super) idx: usize,
    pub(super) toggleselect: Callback<usize>,
    pub(super) updatedirectory: Callback<Pitou>,
}

impl RowProps {
    fn pitou(&self) -> &Pitou {
        &self.pitou
    }

    fn theme(&self) -> Theme {
        self.theme
    }
}

#[function_component]
pub fn Row(prop: &RowProps) -> Html {
    let metadata = use_state(|| None);
    let is_hovered = use_state_eq(|| false);

    let onmouseover = {
        let is_hovered = is_hovered.clone();
        move |_| is_hovered.set(true)
    };

    let onmouseout = {
        let is_hovered = is_hovered.clone();
        move |_| is_hovered.set(false)
    };

    let updatedirectory = {
        let updatedirectory = prop.updatedirectory.clone();
        let pitou = prop.pitou.clone();

        move |_| updatedirectory.emit(pitou.clone())
    };

    let toggleselect = {
        let func = prop.toggleselect.clone();
        let idx = prop.idx;

        move |_| func.emit(idx)
    };

    {
        let pitou = prop.pitou().clone();
        let metadata = metadata.clone();

        use_effect_with_deps(
            |_| {
                spawn_local(async move {
                    let arg = PitouArg { pitou: &pitou };
                    let arg = to_value(&arg).unwrap();
                    let res = invoke("metadata", arg).await;
                    let res = from_value::<Metadata>(res).unwrap();
                    metadata.set(Some(res));
                });
            },
            (),
        );
    }

    let hover_background = prop.theme().background1();

    let style = format! {"
        display: flex;
        flex-direction: row;
        gap: 0;
        height: 10%;
        width: 100%;
        flex-shrink: 0;
        {}", background_color!(prop.selected || *is_hovered, hover_background) };

    let pitou = prop.pitou();
    let theme = prop.theme();

    let filetype = {
        if let Some(m) = &*metadata {
            Some(m.file_type())
        } else {
            None
        }
    };

    let onclick = {
        let toggleselect = toggleselect.clone();
        move |_| toggleselect(())
    };

    let lastmodified = {
        if let Some(m) = &*metadata {
            match m.modified() {
                None => None,
                Some(v) => Some(v),
            }
        } else {
            None
        }
    };

    html! {
        <div {style} {onmouseover} {onmouseout} {onclick}>
            <CheckBox ontoggle = {toggleselect} ischecked = { prop.selected } />
            <FileIconCmp {filetype} {theme} />
            <FileName pitou = { pitou.clone() } {theme} {updatedirectory} />
            <FileTypeCmp {filetype} {theme} />
            <LastModifiedCmp {lastmodified} {theme}/>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct CheckBoxProps {
    pub ischecked: bool,
    pub ontoggle: Callback<()>,
}

#[function_component]
pub fn CheckBox(prop: &CheckBoxProps) -> Html {
    let onclick = {
        let ontoggle = prop.ontoggle.clone();
        move |_| {
            ontoggle.emit(());
        }
    };

    let checked = prop.ischecked;

    let checkbox_elem = if prop.ischecked {
        html! { <input type = "checkbox" {onclick} {checked}/> }
    } else {
        html! {}
    };

    let style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    width: 5%;
    height: 100%;
    "};

    html! {
        <div {style}>
            { checkbox_elem }
        </div>
    }
}

#[function_component]
fn FileIconCmp(prop: &FileTypeProps) -> Html {
    let style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    width: 5%;
    height: 100%;"};

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
pub struct FileNameProps {
    pub pitou: Pitou,
    pub theme: Theme,
    pub updatedirectory: Callback<()>,
}

#[function_component]
fn FileName(prop: &FileNameProps) -> Html {
    let foreground = prop.theme.foreground1();
    let style = format! {"
    display: flex;
    align-items: center;
    justify-content: left;
    padding-left: 2%;
    width: 45%;
    height: 100%;
    color: {foreground};"};

    let ondblclick = {
        let update_directory = prop.updatedirectory.clone();
        move |me: MouseEvent| {
            me.cancel_bubble();
            update_directory.emit(())
        }
    };

    html! {
        <div {style} {ondblclick}>
            { prop.pitou.name() }
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct FileTypeProps {
    pub theme: Theme,
    pub filetype: Option<PitouType>,
}

#[function_component]
fn FileTypeCmp(prop: &FileTypeProps) -> Html {
    let foreground = prop.theme.foreground1();
    let style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20%;
    height: 100%;
    color: {foreground};" };

    let res = format!(
        "{}",
        prop.filetype.map(|v| v.to_string()).unwrap_or_default()
    );

    //TODO
    html! {
        <div {style}>{ res }</div>
    }
}

#[derive(PartialEq, Properties)]
pub struct LastModifiedProps {
    lastmodified: Option<DateTime>,
    theme: Theme,
}

#[function_component]
fn LastModifiedCmp(prop: &LastModifiedProps) -> Html {
    let foreground = prop.theme.foreground1();
    let style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    width: 25%;
    height: 100%;
    color: {foreground};" };

    let date = prop.lastmodified.map(|v| v.format()).unwrap_or_default();

    html! {
        <div {style}>
            {date}
        </div>
    }
}
