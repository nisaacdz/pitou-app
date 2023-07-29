use backend::Pitou;
// use gloo::console::log;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::app::{invoke, LoadingDisplay, PitouArg, Theme};
mod rows;
mod top;
use rows::*;
use top::*;

#[derive(PartialEq, Properties)]
pub struct SidePaneProps {
    pub pitou: Option<Pitou>,
    pub theme: Theme,
    pub updatedirectory: Callback<Pitou>,
}

#[function_component]
pub fn SidePane(prop: &SidePaneProps) -> Html {
    let theme = prop.theme;
    let directory = use_state(|| prop.pitou.clone());
    let siblings = use_state(|| None);

    {
        let siblings = siblings.clone();

        use_effect_with_deps(
            |directory| {
                let directory = directory.clone();
                spawn_local(async move {
                    if let Some(directory) = &*directory {
                        let arg = to_value(&PitouArg { pitou: directory }).unwrap();
                        let val = invoke("siblings", arg).await;
                        let res = from_value::<Vec<Pitou>>(val)
                            .expect("couldn't convert output to a vec of pitou's");
                        siblings.set(Some(res))
                    }
                })
            },
            directory.clone(),
        );
    }

    if &prop.pitou != &*directory {
        siblings.set(None);
        directory.set(prop.pitou.clone());
    }

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

    let pitou = prop.pitou.clone();

    let content = if let Some(pitous) = &*siblings {
        let entries = pitous
            .iter()
            .map(|pitou| (pitou.clone(), prop.updatedirectory.clone(), prop.pitou.as_ref() == Some(pitou)))
            .map(|(pitou, updatedirectory, selected)| html! { <SidePaneRow  { pitou } {theme} {updatedirectory} {selected} /> })
            .collect::<Html>();

        html! {
            <div style = {inner_style}>
                <TopOfParentDir {pitou} {theme} />
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
