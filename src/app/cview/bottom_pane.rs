use yew::prelude::*;

use crate::app::Theme;

#[derive(PartialEq, Properties)]
pub struct BottomPaneProps {
    pub theme: Theme,
}

#[function_component]
pub fn BottomPane(prop: &BottomPaneProps) -> Html {
    let background_color = prop.theme.background1();
    let style = format! {
    "right: 0%;
    height: 4%;
    background-color: {background_color};
    position: absolute;
    bottom: 0%;
    left: 0%;"
    };

    html! {
        <div {style}>
        </div>
    }
}
