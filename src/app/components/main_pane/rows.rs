use crate::{
    app::{AppView, ApplicationContext, DirIcon, FileIcon, SymLinkIcon},
    background_color,
};

use backend::{DateTime, File, PitouType};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RowProps {
    pub(super) file: File,
    pub(super) idx: usize,
    pub(super) toggleselect: Callback<usize>,
    pub(super) ondbclick: Callback<usize>,
    pub(super) selected: bool,
}

#[function_component]
pub fn Row(prop: &RowProps) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context().unwrap();

    let is_hovered = use_state_eq(|| false);

    let onmouseover = {
        let is_hovered = is_hovered.clone();
        move |_| is_hovered.set(true)
    };

    let onmouseout = {
        let is_hovered = is_hovered.clone();
        move |_| is_hovered.set(false)
    };

    let ondblclick = {
        let ondbclick = prop.ondbclick.clone();
        let idx = prop.idx;

        move |_| {
            ondbclick.emit(idx);
        }
    };

    let toggleselect = {
        let func = prop.toggleselect.clone();
        let idx = prop.idx;

        move |_| {
            func.emit(idx);
        }
    };

    let hover_background = theme.background1();
    let height = sizes.row();

    let style = format! {"
    display: flex;
    gap: 0;
    font-size: 90%;
    {height}
    width: auto;
    {}", background_color!(prop.selected || *is_hovered, hover_background) };

    let onclick = {
        let toggleselect = toggleselect.clone();
        move |_| toggleselect(())
    };

    let lastmodified = prop.file.metadata().modified();
    let filetype = prop.file.metadata().file_type();

    html! {
        <div {style} {onmouseover} {onmouseout} {onclick} {ondblclick}>
            <CheckBox ontoggle = {toggleselect} ischecked = { prop.selected } />
            <FileIconCmp {filetype} />
            <FileName file = { prop.file.clone() }/>
            <FileTypeCmp {filetype} />
            <LastModifiedCmp {lastmodified} />
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
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context().unwrap();

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

    let width = sizes.row_checkbox();

    let style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    {width}
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
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context().unwrap();

    let width = sizes.row_icon();

    let style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    {width}
    height: 100%;"};

    let icon = match prop.filetype {
        Some(v) => match v {
            backend::PitouType::File => html! { <FileIcon /> },
            backend::PitouType::Directory => html! { <DirIcon /> },
            backend::PitouType::Link => html! { <SymLinkIcon /> },
        },
        None => html! {},
    };

    html! {
        <div {style}> { icon } </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct FileNameProps {
    pub file: File,
}

#[function_component]
fn FileName(prop: &FileNameProps) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings,
    } = use_context().unwrap();

    let width = sizes.row_namefield();

    let foreground = theme.foreground1();

    let style = format! {"
    {width}
    color: {foreground};
    height: 100%;
    "};

    let inner_style = format! {"
    left: 2%;
    right: 2%;
    height: 100%;"};

    let value = match settings.view {
        AppView::Search => prop.file.path().display().to_string(),
        _ => prop.file.name().to_owned(),
    };

    html! {
        <div {style}>
            <div class = "filenamewrap" style = {inner_style}>
                <span class = "filename"> { value } </span>
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct FileTypeProps {
    pub filetype: Option<PitouType>,
}

#[function_component]
fn FileTypeCmp(prop: &FileTypeProps) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context().unwrap();

    let width = sizes.row_typefield();

    let foreground = theme.foreground1();
    let style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    {width}
    height: 100%;
    color: {foreground};" };

    let res = prop.filetype.map(|v| v.to_string()).unwrap_or_default();

    html! {
        <div {style}>{ res }</div>
    }
}

#[derive(PartialEq, Properties)]
pub struct LastModifiedProps {
    lastmodified: Option<DateTime>,
}

#[function_component]
fn LastModifiedCmp(prop: &LastModifiedProps) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context().unwrap();

    let width = sizes.row_sparefield();

    let foreground = theme.foreground1();
    let style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    {width}
    height: 100%;
    color: {foreground};" };

    let date = prop.lastmodified.map(|v| v.format()).unwrap_or_default();

    html! {
        <div {style}>
            {date}
        </div>
    }
}
