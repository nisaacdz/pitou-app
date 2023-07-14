use yew::prelude::*;

use crate::{
    app::{PitouProps, Props},
    brightness, color,
};

#[function_component]
pub(super) fn AncestorsTabs(props: &PitouProps) -> Html {
    let foreground_color = props.theme().foreground1();
    let background_color_2 = props.theme().background2();
    let spare_color = props.theme().spare();

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

    let entries = props
        .pitou_file()
        .ancestors()
        .map(|p| html! { <Ancestor pitou = { Into::<Props>::into((p, props.theme())) } /> })
        .collect::<Html>();

    html! {
        <div style = {outer_style}>
            <div style = {inner_style}>
                {entries}
            </div>
        </div>
    }
}

#[function_component]
pub(super) fn Ancestor(prop: &PitouProps) -> Html {
    let mouse_is_overed = use_state(|| false);

    let onmouseover = {
        let mouse_is_overed = mouse_is_overed.clone();
        move |_| mouse_is_overed.set(true)
    };
    let onmouseout = {
        let mouse_is_overed = mouse_is_overed.clone();
        move |_| mouse_is_overed.set(false)
    };

    let background_color = prop.theme().background2();
    let border_color = prop.theme().spare();

    let style = format! {"
        height: calc(100% - 2px);
        position: relative;
        top: 0;
        width: auto;
        border: 1px solid {border_color};
        background-color: {background_color};
        font-size: 80%;
        {}
        {}
    ", color!(*mouse_is_overed, prop.theme().spare()), brightness!(*mouse_is_overed, 300)};

    html! {
        <div class = "tab-item" {style} {onmouseover} {onmouseout}>
            <TabName pitou = { prop.pitou().clone() } />
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

#[function_component]
pub(super) fn TabName(prop: &PitouProps) -> Html {
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
    html! {
        <span {style}>{ std::path::PathBuf::from(prop.pitou().file_name()).display() }</span>
    }
}
