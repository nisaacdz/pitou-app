use yew::prelude::*;

use super::{cview::*, PitouProps};

#[function_component]
pub fn ContentView(prop: &PitouProps) -> Html {
    let pitou = prop.pitou().clone();
    let theme = prop.theme();

    html! {
        <div style = { format!{
            "
            width: 100%;
            height: 100%;
            background-color: {};
            margin: 0% 0% 0% 0%;
            padding: 0% 0% 0% 0%;
            position: absolute;", prop.theme().background1() }} >
            <TopPane pitou = { pitou.clone() } {theme} />

            <BottomPane pitou = { pitou.clone() } {theme} />

            <LeftPane pitou = { pitou.clone() } {theme} />

            <SidePane pitou = { pitou.clone() } {theme} />

            <MainPane {pitou} {theme} />
        </div>
    }
}

#[allow(unused)]
pub struct OpeningView {}
