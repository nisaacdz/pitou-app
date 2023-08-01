use crate::{
    app::{DirIcon, FileIcon, LoadingIcon, SymLinkIcon, Theme, invoke, PitouArg},
    background_color,
};
use backend::{DateTime, Metadata, Pitou, PitouType};
use serde_wasm_bindgen::{to_value, from_value};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RowProps {
    pub(super) pitou: Pitou,
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

    {
        let metadata = metadata.clone();
        let pitou = prop.pitou.clone();

        spawn_local(async move {
            let arg = to_value(&PitouArg { pitou: &pitou }).unwrap();
            let val = from_value::<Option<Metadata>>(invoke("metadata", arg).await).unwrap();
            metadata.set(val)        
        });
    }
    
    let is_hovered = use_state_eq(|| false);
    let is_selected = use_state_eq(|| false);

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
        let is_selected = is_selected.clone();
        let func = prop.toggleselect.clone();
        let idx = prop.idx;

        move |_| {
            func.emit(idx);
            is_selected.set(!*is_selected);
        }
    };

    let hover_background = prop.theme().background1();

    let style = format! {"
    display: flex;
    flex-direction: row;
    gap: 0;
    font-size: 90%;
    height: 10%;
    width: 100%;
    flex-shrink: 0;
    {}", background_color!(*is_selected || *is_hovered, hover_background) };

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
            <CheckBox ontoggle = {toggleselect} ischecked = { *is_selected } />
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
    flex-shrink: 0;
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
    flex-shrink: 0;
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
    flex-shrink: 0;
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
    flex-shrink: 0;
    color: {foreground};" };

    let date = prop.lastmodified.map(|v| v.format()).unwrap_or_default();

    html! {
        <div {style}>
            {date}
        </div>
    }
}
