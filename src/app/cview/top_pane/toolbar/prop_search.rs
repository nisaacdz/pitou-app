use crate::app::{DetailsIcon, SearchIcon};
use yew::prelude::*;

use super::{NameField, TopButtonProps};

#[function_component]
pub fn PropertiesButton(prop: &TopButtonProps) -> Html {
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
    width: 50px;
    height: 100%;
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    align-items: center;
    "};

    let icon_style = format! {"
        display: flex;
        align-items: center;
        justify-content: center;
        height: 70%;
        width: 100%;
    "};

    html! {
        <div {style} {onmouseover} {onmouseout}>
            <div class = "card" style = {icon_style}>
                <DetailsIcon />

            </div>
            <NameField name = { "info" }  theme = { prop.theme } />
        </div>
    }
}

#[function_component]
pub fn SearchButton(prop: &TopButtonProps) -> Html {
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
    width: 50px;
    height: 100%;
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    align-items: center;
    "};

    let icon_style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    height: 70%;
    width: 100%;
    "};

    html! {
        <div {style} {onmouseover} {onmouseout}>
            <div class = "card" style = {icon_style}>
                <SearchIcon />

            </div>
            <NameField name = { "search" } theme = { prop.theme } />
        </div>
    }
}
