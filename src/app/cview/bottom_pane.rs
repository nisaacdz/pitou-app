use yew::prelude::*;

use crate::app::Theme;

#[function_component]
pub fn BottomPane() -> Html {
    let theme = use_context::<Theme>().unwrap();

    let background_color = theme.background1();
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
