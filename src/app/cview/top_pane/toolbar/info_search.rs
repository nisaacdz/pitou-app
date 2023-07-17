use crate::app::{InfoIcon, PitouProps, SearchIcon};
use yew::prelude::*;

use super::HoverNameDisp;

#[function_component]
pub fn InfoButton(prop: &PitouProps) -> Html {
    let mouse_over = use_state(|| false);

    let onmouseover = {
        let mouse_over = mouse_over.clone();
        move |_| mouse_over.set(true)
    };

    let onmouseout = {
        let mouse_over = mouse_over.clone();
        move |_| mouse_over.set(false)
    };

    let style = format! {"
        position: relative;
        width: 3%;
        top: 0;
        bottom: 0;
        display: flex;
        align-items: center;
        justify-content: center;
    "};

    let icon_style = format! {"
        display: flex;
        height: 80%;
        width: 100%;
    "};

    html! {
        <div {style} {onmouseover} {onmouseout}>
            <div class = "card" style = {icon_style}>
                <InfoIcon theme = { prop.theme() }/>

            </div>
            {
                if *mouse_over {
                    html! { <HoverNameDisp name = { "info" }  theme = { prop.theme() } /> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

#[function_component]
pub fn SearchButton(prop: &PitouProps) -> Html {
    let mouse_over = use_state(|| false);

    let onmouseover = {
        let mouse_over = mouse_over.clone();
        move |_| mouse_over.set(true)
    };

    let onmouseout = {
        let mouse_over = mouse_over.clone();
        move |_| mouse_over.set(false)
    };

    let style = format! {"
        position: relative;
        width: 3%;
        top: 0;
        bottom: 0;
        display: flex;
        align-items: center;
        justify-content: center;
    "};

    let icon_style = format! {"
        display: flex;
        height: 80%;
        width: 100%;
    "};

    html! {
        <div {style} {onmouseover} {onmouseout}>
            <div class = "card" style = {icon_style}>
                <SearchIcon theme = { prop.theme() }/>

            </div>
            {
                if *mouse_over {
                    html! { <HoverNameDisp name = { "search" }  theme = { prop.theme() } /> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
