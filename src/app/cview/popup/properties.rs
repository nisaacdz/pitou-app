use crate::app::Theme;
use backend::Pitou;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct PropertiesProps {
    onclose: Callback<()>,
    path: Pitou,
}

#[function_component]
pub fn Properties(prop: &PropertiesProps) -> Html {
    let theme = use_context::<Theme>().expect("no theme context provided");

    let border_color = theme.spare();
    let background_color = theme.background2();

    let style = format! {"
    background-color: {background_color};
    border: 2px solid {border_color};
    "};

    let onclick = |e: MouseEvent| e.stop_immediate_propagation();
    let entries = {
        html! {
            <div>
                <Path pitou = { prop.path.clone() } />
            </div>
        }
    };

    html! {
        <div {style} class = {"popup"} {onclick}>
            <TitleBar onclose = { prop.onclose.clone() } />
            { entries }
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct PathProps {
    pitou: Pitou,
}

#[function_component]
fn Path(prop: &PathProps) -> Html {
    html! {
        <div>
            <span> { "full path: " } </span>
            <span> { prop.pitou.path().display() } </span>
            <button></button>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct TitleBarProps {
    onclose: Callback<()>,
}

#[function_component]
fn TitleBar(prop: &TitleBarProps) -> Html {
    let style = format! {"
    width: 100%;
    height: 15px;
    display: flex;
    gap: 0;
    flex-direction: row-reverse;
    "};

    let close_button_style = format![
        "
    width: 15px;
    height: 100px;
    backgound-color: red;
    "
    ];

    let onclick = {
        let onclose = prop.onclose.clone();
        move |_| onclose.emit(())
    };
    html! {
        <div {style}>
            <button {close_button_style} {onclick}></button>
        </div>
    }
}
