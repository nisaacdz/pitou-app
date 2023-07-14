use yew::prelude::*;

use crate::app::PitouProps;

mod ancestors;
mod toolbar;

use ancestors::*;
use toolbar::*;

#[function_component]
pub fn TopPane(props: &PitouProps) -> Html {
    let pitou = props.pitou();
    let background_color = props.theme().background1();
    let style = format! {"
        background-color: {background_color};
        top: 0%;
        height: 10%;
        left: 0%;
        right: 0%;
        position: absolute;
        display: flex;
        flex-direction: column;
        gap: 0;
",  };
    html! {
        <div {style}>
            <ToolBar pitou = { pitou.clone() }/>
            <AncestorsTabs pitou = { pitou.clone() }/>
        </div>
    }
}