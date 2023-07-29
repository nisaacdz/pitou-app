use crate::app::PitouProps;
use yew::prelude::*;

#[function_component]
pub fn RightClickMenu(_prop: &PitouProps) -> Html {
    html! {
        <div>
            <Cut />
            <Copy />
            <Delete />
            <Rename />
            <Open />
            <Share />
            <Properties />
            <CreateShortcut />
            <OpenInSeparateWindow />
        </div>
    }
}

#[function_component]
fn Cut() -> Html {
    html! {}
}

#[function_component]
fn Copy() -> Html {
    html! {}
}

#[function_component]
fn Delete() -> Html {
    html! {}
}

#[function_component]
fn Rename() -> Html {
    html! {}
}

#[function_component]
fn Share() -> Html {
    html! {}
}

#[function_component]
fn Properties() -> Html {
    html! {}
}

#[function_component]
fn Open() -> Html {
    html! {}
}

#[function_component]
fn CreateShortcut() -> Html {
    html! {}
}

#[function_component]
fn OpenInSeparateWindow() -> Html {
    html! {}
}
