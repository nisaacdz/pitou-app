use backend::Pitou;
use yew::prelude::*;

use crate::app::{LoadingDisplay, Theme};
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
    let theme = use_context::<Theme>().unwrap();

    let background_color = theme.background2();
    let border_color = theme.spare();

    let style = format! {"
        position: absolute;
        display: flex;
        flex-direction: column;
        gap: 0;
        top: 10%;
        bottom: 4%;
        align-items: center;
        overflow: auto;
        background-color: {background_color};
        overflow-anchor: none;
        border: 1px solid {border_color};
        margin: 1px 1px 1px 1px;
        left: 4%;
        width: 20%;
    "};

    let inner_style = format! {"
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
    "};

    let content = if let Some(pitous) = prop.siblings.as_ref() {
        let entries = pitous
            .iter()
            .map(|pitou| (pitou.clone(), prop.updatedirectory.clone(), prop.selected.as_ref() == Some(pitou)))
            .map(|(pitou, updatedirectory, selected)| html! { <SidePaneRow  { pitou } {updatedirectory} {selected} /> })
            .collect::<Html>();

        html! {
            <div style = {inner_style}>
                <TopOfParentDir selected = { prop.selected.clone() }/>
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
