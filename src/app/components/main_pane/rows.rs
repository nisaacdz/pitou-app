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
    pub(super) onselect: Callback<(usize, bool)>,
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

    let ondblclick = {
        let ondbclick = prop.ondbclick.clone();
        let idx = prop.idx;

        move |_| {
            ondbclick.emit(idx);
        }
    };

    let appendselect = {
        let func = prop.onselect.clone();
        let idx = prop.idx;
        move |_| {
            func.emit((idx, true));
        }
    };

    let height = sizes.row();
    let background = theme.background1();

    let style = format! {"
    display: flex;
    gap: 0;
    font-size: 90%;
    {height}
    width: auto;
    {}", background_color!(prop.selected, background)};

    let onclick = {
        let onselect = prop.onselect.clone();
        let idx = prop.idx;
        move |_| onselect.emit((idx, false))
    };

    let lastmodified = prop.file.metadata().modified();
    let filetype = prop.file.metadata().file_type();

    html! {
        <div class = "row" {style} {onclick} {ondblclick}>
            <CheckBox {appendselect} ischecked = { prop.selected } />
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
    pub appendselect: Callback<()>,
}

#[function_component]
pub fn CheckBox(prop: &CheckBoxProps) -> Html {
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context().unwrap();

    let onclick = {
        let ontoggle = prop.appendselect.clone();
        move |e: MouseEvent| {
            e.stop_propagation();
            ontoggle.emit(());
        }
    };

    let width = sizes.row_checkbox();

    let show_on_checked = if prop.ischecked {
        format! {"
            visibility: visible;
        "}
    } else {
        "".to_owned()
    };

    let style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    {width}
    height: 100%;
    "};

    html! {
        <div {style} class = "checkbox-container" {onclick}>
            <input type = "checkbox" class = "row-checkbox" checked = {prop.ischecked} style = {show_on_checked}/>
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
