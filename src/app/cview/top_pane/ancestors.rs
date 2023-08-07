use backend::Pitou;
use yew::prelude::*;

use crate::{app::Theme, color};

#[derive(PartialEq, Properties)]
pub struct AncestorsTabsProps {
    pub pitou: Option<Pitou>,
    pub updatedirectory: Callback<Pitou>,
}

#[function_component]
pub(super) fn AncestorsTabs(prop: &AncestorsTabsProps) -> Html {
    let theme = use_context::<Theme>().unwrap();

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

    let entries = prop
        .pitou
        .as_ref()
        .map(|pitou| {
            pitou
                .ancestors()
                .into_iter()
                .map(|pitou| (pitou, prop.updatedirectory.clone()))
                .map(|(pitou, updatedirectory)| html! { <Ancestor {pitou} {updatedirectory} /> })
                .collect::<Html>()
        })
        .unwrap_or(html! {});

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
    updatedirectory: Callback<Pitou>,
}

#[function_component]
pub(super) fn Ancestor(prop: &AncestorProps) -> Html {
    let theme = use_context::<Theme>().unwrap();
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
    ", color!(*mouse_is_overed, theme.spare())/*, brightness!(*mouse_is_overed, 300) */};

    html! {
        <div class = "card" {style} {onmouseover} {onmouseout}>
            <TabName {pitou} updatedirectory = { prop.updatedirectory.clone() } />
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
