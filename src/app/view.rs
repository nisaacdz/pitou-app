use yew::prelude::*;

use super::{cview::*, PitouProps};

#[function_component]
pub fn ContentView(prop: &PitouProps) -> Html {
    let pitou = prop.pitou().clone();
    html! {
        <div style = { format!{
            "
            width: 100%;
            height: 100%;
            background-color: {};
            margin: 0% 0% 0% 0%;
            padding: 0% 0% 0% 0%;
            position: absolute;", prop.theme().background1() }} >
            <TopPane pitou = { pitou.clone() } />

            <BottomPane pitou = { pitou.clone() } />

            <LeftPane pitou = { pitou.clone() } />

            <SidePane pitou = { pitou.clone() } />

            <MainPane {pitou} />
        </div>
    }
}

#[allow(unused)]
pub struct OpeningView {}
