use backend::File;
use yew::prelude::*;

use crate::{
    app::{ApplicationContext, DirIcon, FileIcon, SymLinkIcon},
    background_color,
};

#[derive(PartialEq, Properties)]
pub struct SidePaneRowProps {
    pub file: File,
    pub onclick: Callback<File>,
    pub selected: bool,
}

#[function_component]
pub fn SidePaneRow(prop: &SidePaneRowProps) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context().unwrap();
    let hovered = use_state_eq(|| false);

    let onmouseover = {
        let hovered = hovered.clone();
        move |_| {
            hovered.set(true);
        }
    };

    let onclick = {
        let onclick = prop.onclick.clone();
        let file = prop.file.clone();
        move |_| onclick.emit(file.clone())
    };

    let onmouseout = {
        let hovered = hovered.clone();
        move |_| {
            hovered.set(false);
        }
    };

    let foreground_color = theme.foreground1();

    let height = sizes.row();

    let style = format! {"
    display: flex;
    flex-shrink: 0;
    {height}
    gap: 0;
    font-size: 90%;
    color: {foreground_color};
    font-family: monospace;
    width: 100%;
    {}", background_color!(*hovered || prop.selected, theme.background1()) };

    let filetype = prop.file.metadata().file_type();

    html! {
        <div {style} {onmouseover} {onmouseout} {onclick}>
            <FileIconCmp {filetype} />
            <SidePaneFileName file = { prop.file.clone() } />
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct FileTypeProps {
    pub filetype: backend::PitouType,
}

#[function_component]
fn FileIconCmp(prop: &FileTypeProps) -> Html {
    let ApplicationContext {
        sizes,
        theme: _,
        settings: _,
    } = use_context().unwrap();
    let width = sizes.sidepane_icon();

    let style = format! {"
        display: flex;
        align-items: center;
        {width}
        height: 100%;
        padding-left: 3%;
        justify-content: center;
    "};

    let icon = match prop.filetype {
        backend::PitouType::File => html! { <FileIcon /> },
        backend::PitouType::Directory => html! { <DirIcon /> },
        backend::PitouType::Link => html! { <SymLinkIcon /> },
    };

    html! {
        <div {style}> { icon } </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct SidePaneFileNameProps {
    file: File,
}

#[function_component]
pub(super) fn UseLess() -> Html {
    let _style = format! {"
        left: 15%;
        width: 75%;
        height: 100%;
        align-items: center;
        overflow: hidden;
        white-space: nowrap;
        padding-left: 3%;
        text-overflow: ellipsis;
    "};

    html! {}
}

#[function_component]
pub(super) fn SidePaneFileName(prop: &SidePaneFileNameProps) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context().unwrap();

    let width = sizes.sidepane_filename();
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

    let name = prop.file.name().to_owned();

    html! {
        <div {style}>
            <div class = "filenamewrap" style = {inner_style}>
                <span class = "filename"> { name } </span>
            </div>
        </div>
    }
}
