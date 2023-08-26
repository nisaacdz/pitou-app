use backend::Pitou;
use yew::prelude::*;

use crate::app::{ApplicationContext, LoadingDisplay};
mod rows;
mod top;
use rows::*;
use top::*;

#[derive(PartialEq, Properties)]
pub struct SidePaneProps {
    pub selected: Option<Pitou>,
    pub siblings: Option<Vec<Pitou>>,
    pub updatedirectory: Callback<Pitou>,
}

#[function_component]
pub fn SidePane(prop: &SidePaneProps) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context::<ApplicationContext>().unwrap();

    let filter = use_state(|| None);

    let background_color = theme.background2();
    let spare_color = theme.spare();
    let size = sizes.sidepane();

    let style = format! {"
    background-color: {background_color};
    position: relative;
    border: 1px solid {spare_color};
    box-sizing: border-box;
    {size}"};

    let top = sizes.dsc().height;
    let height = size.height - top;

    let inner_style = format! {"
    position: absolute;
    display: flex;
    flex-direction: column;
    gap: 0;
    
    align-items: center;
    overflow-y: auto;
    overflow-x: hidden;

    top: {top}px;
    height: {height}px;
    width: 100%;
    "};

    let onfilter = {
        let filter = filter.clone();
        move |newval: String| filter.set(Some(newval))
    };

    let onenter = {
        let updatedirectory = prop.updatedirectory.clone();
        let selected = prop.selected.clone();
        move |newstr| {
            use std::path::PathBuf;

            if let Some(val) = &selected {
                let pitou: Pitou = val
                    .path()
                    .parent()
                    .map(|path| PathBuf::from(path))
                    .unwrap_or_default()
                    .join(newstr)
                    .into();
                updatedirectory.emit(pitou)
            }
        }
    };

    let content = if let Some(pitous) = prop.siblings.as_ref() {
        let entries = pitous
            .iter()
            .filter(|item| {
                use backend::StrOps;
                filter.as_ref().map(|pat| item.name().starts_with_ignore_case(pat)).unwrap_or(true)
            })
            .map(|pitou| (pitou.clone(), prop.updatedirectory.clone(), prop.selected.as_ref() == Some(pitou)))
            .map(|(pitou, updatedirectory, selected)| html! { <SidePaneRow  { pitou } {updatedirectory} {selected} /> })
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
            <TopOfParentDir {onfilter} {onenter} selected = { prop.selected.clone() }/>
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
