use crate::app::ApplicationContext;
use yew::prelude::*;

use super::{NameField, TopButtonProps};

#[function_component]
pub fn PropertiesButton(_prop: &TopButtonProps) -> Html {
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context::<ApplicationContext>().unwrap();

    let onclick = { move |_| () };

    let tool_size = sizes.toolbar_item();
    let icon_size = sizes.toolbar_icon();
    let img_height = sizes.toolbar_icon_img().height();

    let style = format! {"
    {tool_size}
    display: flex;
    flex-direction: column;
    align-items: center;
    "};

    let icon_style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    {icon_size}
    "};

    let img_style = format! {"
    {img_height}
    "};

    html! {
        <div {style} {onclick}>
            <div style = {icon_style}>
                <img class = "card" style = {img_style} src="./public/icons/top/details.png" alt="info" />
            </div>
            <NameField name = { "info" }/>
        </div>
    }
}

#[function_component]
pub fn SearchButton(_prop: &TopButtonProps) -> Html {
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context::<ApplicationContext>().unwrap();

    let onclick = { move |_| () };

    let tool_size = sizes.toolbar_item();
    let icon_size = sizes.toolbar_icon();
    let img_height = sizes.toolbar_icon_img().height();

    let style = format! {"
    {tool_size}
    display: flex;
    flex-direction: column;
    align-items: center;
    "};

    let icon_style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    {icon_size}
    "};

    let img_style = format! {"
    {img_height}
    "};

    html! {
        <div {style} {onclick}>
            <div style = {icon_style}>
                <img class = "card" style = {img_style} src="./public/icons/top/search.png" alt="search" />
            </div>
            <NameField name = { "search" }/>
        </div>
    }
}
