use yew::prelude::*;

use crate::app::PitouProps;

#[function_component]
pub fn BottomPane(props: &PitouProps) -> Html {
    html! {
        <div style = { format!{"
            right: 0%;
            height: 4%;
            background-color: {};
            position: absolute;
            bottom: 0%;
            left: 0%;",
        props.theme().background1() } }>
        </div>
    }
}