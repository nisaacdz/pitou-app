use crate::{
    app::{invoke, AppView, ApplicationContext, DirIcon, FileIcon, PitouArg, SymLinkIcon},
    background_color,
};
use backend::{DateTime, Metadata, Pitou, PitouType};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RowProps {
    pub(super) pitou: Pitou,
    pub(super) idx: usize,
    pub(super) toggleselect: Callback<usize>,
    pub(super) updatedirectory: Callback<Pitou>,
    pub(super) selected: bool,
}

impl RowProps {
    fn pitou(&self) -> &Pitou {
        &self.pitou
    }
}

#[function_component]
pub fn Row(prop: &RowProps) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context::<ApplicationContext>().unwrap();

    let metadata = use_state(|| None);

    let is_hovered = use_state_eq(|| false);

    {
        let metadata = metadata.clone();
        let pitou = prop.pitou.clone();

        use_effect_with_deps(
            move |_| {
                let metadata = metadata.clone();
                let pitou = pitou.clone();
                spawn_local(async move {
                    let arg = to_value(&PitouArg { pitou: &pitou }).unwrap();
                    let val =
                        from_value::<Option<Metadata>>(invoke("metadata", arg).await).unwrap();
                    metadata.set(val)
                })
            },
            (),
        );
    }

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

        move |_| {
            func.emit(idx);
        }
    };

    let hover_background = theme.background1();
    let height = sizes.row();

    let style = format! {"
    display: flex;
    flex-direction: row;
    gap: 0;
    font-size: 90%;
    {height}
    width: auto;
    {}", background_color!(prop.selected || *is_hovered, hover_background) };

    let pitou = prop.pitou();

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
            <FileIconCmp {filetype} />
            <FileName pitou = { pitou.clone() } {updatedirectory} />
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
    } = use_context::<ApplicationContext>().unwrap();

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
    } = use_context::<ApplicationContext>().unwrap();

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
    pub pitou: Pitou,
    pub updatedirectory: Callback<()>,
}

#[function_component]
fn FileName(prop: &FileNameProps) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings,
    } = use_context::<ApplicationContext>().unwrap();

    let width = sizes.row_namefield();

    let foreground = theme.foreground1();
    let style = format! {"
    display: flex;
    flex-direction: row-reverse;
    overflow: hidden;
    align-items: center;
    justify-content: left;
    padding-left: 2%;
    {width}
    height: 100%;
    color: {foreground};"};

    let ondblclick = {
        let update_directory = prop.updatedirectory.clone();
        move |me: MouseEvent| {
            me.cancel_bubble();
            update_directory.emit(())
        }
    };

    let value = match settings.view {
        AppView::Search => prop.pitou.path().display().to_string(),
        _ => prop.pitou.name(),
    };

    html! {
        <div {style} {ondblclick}> { value } </div>
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
    } = use_context::<ApplicationContext>().unwrap();

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
    } = use_context::<ApplicationContext>().unwrap();

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
