use backend::Pitou;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    app::{invoke, PitouArg, Theme},
    /*brightness*/ color,
};

#[derive(PartialEq, Properties)]
pub struct AncestorsTabsProps {
    pub pitou: Option<Pitou>,
    pub theme: Theme,
    pub updatedirectory: Callback<Pitou>,
}

#[function_component]
pub(super) fn AncestorsTabs(prop: &AncestorsTabsProps) -> Html {
    let directory = use_state(|| prop.pitou.clone());
    let ancestors = use_state(|| None);
    let clicked = use_state(|| false);

    {
        let ancestors = ancestors.clone();

        use_effect_with_deps(
            |directory| {
                let directory = directory.clone();

                spawn_local(async move {
                    if let Some(directory) = &*directory {
                        let args = to_value(&PitouArg { pitou: &*directory }).unwrap();
                        let js_res = invoke("ancestors", args).await;
                        let res = from_value::<Vec<Pitou>>(js_res).unwrap();
                        ancestors.set(Some(res));
                        clicked.set(false);
                    }
                });
            },
            directory.clone(),
        );
    }

    if &prop.pitou != &*directory {
        directory.set(prop.pitou.clone())
    }

    let theme = prop.theme;

    let foreground_color = theme.foreground1();
    let background_color_2 = theme.background2();
    let spare_color = theme.spare();

    let outer_style = format! {"
    border: 1px solid {spare_color};
    background-color: {background_color_2};
    top: 60%;
    height: 30%;
    position: absolute;
    left: 0%;
    right: 0%;
    "};

    let inner_style = format! {"
        top: 2px;
        bottom: 2px;
        padding-left: 4%;
        color: {foreground_color};
        position: absolute;

        display: flex;
        flex-direction: row-reverse;
        align-items: center;
        column-gap: 2px;
        justify-content: left;
        overflow: hidden;
    "};

    let entries =
        if let Some(v) = &*ancestors {
            v.iter()
        .map(|p| (p.clone(), prop.updatedirectory.clone()))
        .map(|(pitou, updatedirectory)| html! { <Ancestor {pitou} {theme} {updatedirectory} /> })
        .collect::<Html>()
        } else {
            html! {}
        };

    html! {
        <div style = {outer_style}>
            <div style = {inner_style}>
                {entries}
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct AncestorProps {
    pitou: Pitou,
    theme: Theme,
    updatedirectory: Callback<Pitou>,
}

#[function_component]
pub(super) fn Ancestor(prop: &AncestorProps) -> Html {
    let mouse_is_overed = use_state_eq(|| false);

    let onmouseover = {
        let mouse_is_overed = mouse_is_overed.clone();
        move |_| mouse_is_overed.set(true)
    };
    let onmouseout = {
        let mouse_is_overed = mouse_is_overed.clone();
        move |_| mouse_is_overed.set(false)
    };

    let pitou = prop.pitou.clone();
    let theme = prop.theme;

    let background_color = theme.background2();
    let border_color = theme.spare();

    let style = format! {"
        height: calc(100% - 2px);
        position: relative;
        top: 0;
        display: flex;
        flex-direction: row;
        gap: 0;
        width: auto;
        border: 1px solid {border_color};
        background-color: {background_color};
        font-size: 80%;
        {}
    ", color!(*mouse_is_overed, prop.theme.spare())/*, brightness!(*mouse_is_overed, 300) */};

    html! {
        <div class = "card" {style} {onmouseover} {onmouseout}>
            <TabName {pitou} {theme} updatedirectory = { prop.updatedirectory.clone() } />
            <Side/>
        </div>
    }
}

#[function_component]
pub(super) fn Side() -> Html {
    let style = format! {"
        width: 15px;
        height: calc(100% - 2px);
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
    "};
    html! {
        <div {style}>
            { ">" }
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct TabNameProps {
    pitou: Pitou,
    theme: Theme,
    updatedirectory: Callback<Pitou>,
}

#[function_component]
pub(super) fn TabName(prop: &TabNameProps) -> Html {
    let style = format! {"
        width: auto;
        height: 100%;
        display: flex;
        gap: 0;
        flex-direction: row;
        align-items: center;
        justify-content: center;
        padding-left: 10%;
        padding-right: 10%;
        overflow: hidden;
        position: relative;
        min-width: 10%;
        white-space: nowrap;
        flex-wrap: nowrap;
    "};

    let updatedirectory = {
        let pitou = prop.pitou.clone();
        let updatedirectory = prop.updatedirectory.clone();

        move |_| updatedirectory.emit(pitou.clone())
    };

    html! {
        <span {style} onclick = { updatedirectory }>{ prop.pitou.name() }</span>
    }
}
