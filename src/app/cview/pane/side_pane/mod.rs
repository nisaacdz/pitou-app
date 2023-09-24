use backend::{File, PitouType};
use std::path::PathBuf;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::app::{ApplicationContext, LoadingDisplay};
mod rows;
mod top;
use rows::*;
use top::*;

use std::rc::Rc;

#[derive(PartialEq, Properties)]
pub struct SidePaneProps {
    pub directory: Option<Rc<PathBuf>>,
    pub siblings: Option<Rc<Vec<File>>>,
    pub updatedirectory: Callback<PathBuf>,
}

#[function_component]
pub fn SidePane(prop: &SidePaneProps) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context().unwrap();

    let filter = use_state(|| None);

    let background_color = theme.background2();
    let spare_color = theme.spare();
    let size = sizes.sidepane();

    let style = format! {"
    background-color: {background_color};
    display: flex;
    flex-direction: column;
    gap: 0;
    position: relative;
    border: 1px solid {spare_color};
    box-sizing: border-box;
    {size}"};

    let top = sizes.dsc().height;
    let height = size.height - top;

    let inner_style = format! {"
    display: flex;
    flex-direction: column;
    gap: 0;
    
    align-items: center;
    overflow-y: auto;
    overflow-x: hidden;
    background-color: {background_color};

    height: {height}px;
    width: 100%;
    "};

    let onfilter = {
        let filter = filter.clone();
        move |newval: String| filter.set(Some(newval))
    };

    let onenter = {
        let updatedirectory = prop.updatedirectory.clone();
        let selected = prop.directory.clone();
        move |newstr| {
            use std::path::PathBuf;

            if let Some(val) = &selected {
                let pitou = val
                    .parent()
                    .map(|path| PathBuf::from(path))
                    .unwrap_or_default()
                    .join(newstr);
                updatedirectory.emit(pitou)
            }
        }
    };

    let onclick = Callback::from({
        let updatedirectory = prop.updatedirectory.clone();

        move |file: File| match file.metadata().file_type() {
            PitouType::Directory => {
                updatedirectory.emit(file.path().clone());
            }
            PitouType::File => {
                let file = file.clone();
                spawn_local(async move { crate::app::tasks::open(file.path()).await })
            }
            PitouType::Link => {
                let symlink = file.clone();
                let updatedirectory = updatedirectory.clone();
                spawn_local(async move {
                    if let Some(file) = crate::app::tasks::read_link(symlink.path()).await {
                        let parent_dir =
                            PathBuf::from(file.path().parent().unwrap_or(std::path::Path::new("")));
                        // TODO crate::app::data::persist(file.clone());
                        updatedirectory.emit(parent_dir);
                    }
                })
            }
        }
    });

    let is_selected = {
        let directory = prop.directory.clone();

        move |path| matches!(directory.as_ref(), Some(x) if &**x == path)
    };

    let content = if let Some(files) = prop.siblings.as_ref() {
        let entries = files
            .iter()
            .filter(|item| {
                use backend::StrOps;
                filter.as_ref().map(|pat| item.name().starts_with_ignore_case(pat)).unwrap_or(true)
            })
            .map(|file| (file.clone(), onclick.clone(), is_selected(file.path())))
            .map(|(file, onclick, selected)| html! { <SidePaneRow  { file } {onclick} {selected} /> })
            .collect::<Html>();

        html! {
            <div style = {inner_style}>
                {
                    entries
                }
            </div>
        }
    } else {
        html! {
            <LoadingScreen />
        }
    };

    html! {
        <div {style}>
            <TopOfParentDir {onfilter} {onenter} selected = { prop.directory.clone() }/>
            { content }
        </div>
    }
}

#[function_component]
fn LoadingScreen() -> Html {
    let style = format! {"
    width: 100%;
    height: 90%;
    display: flex;
    justify-content: center;
    align-items: center;
    "};
    html! {
        <div {style}>
            <LoadingDisplay />
        </div>
    }
}
