use yew::prelude::*;
use crate::app::PitouProps;


#[function_component]
pub fn RightClickMenu(prop: &PitouProps) -> Html {
    let pitou = prop.pitou();
    html! {
        <div>
            <Cut pitou = { pitou.clone() } />
            <Copy pitou = { pitou.clone() } />
            <Delete pitou = { pitou.clone() } />
            <Rename pitou = { pitou.clone() } />
            <Open pitou = { pitou.clone() } />
            <Share pitou = { pitou.clone() } />
            <Properties pitou = { pitou.clone() } />
            <CreateShortcut pitou = { pitou.clone() } />
            <OpenInSeparateWindow pitou = { pitou.clone() } />
        </div>
    }
}

#[function_component]
fn Cut(_prop: &PitouProps) -> Html {
    html! {

    }
}

#[function_component]
fn Copy(_prop: &PitouProps) -> Html {
    html! {

    }
}

#[function_component]
pub fn Delete(_prop: &PitouProps) -> Html {
    html! {

    }
}

#[function_component]
fn Rename(_prop: &PitouProps) -> Html {
    html! {

    }
}

#[function_component]
fn Share(_prop: &PitouProps) -> Html {
    html! {

    }
}

#[function_component]
fn Properties(_prop: &PitouProps) -> Html {
    html! {

    }
}

#[function_component]
fn Open(_prop: &PitouProps) -> Html {
    html! {

    }
}

#[function_component]
fn CreateShortcut(_prop: &PitouProps) -> Html {
    html! {

    }
}

#[function_component]
fn OpenInSeparateWindow(_prop: &PitouProps) -> Html {
    html! {

    }
}