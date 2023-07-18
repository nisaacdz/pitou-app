use yew::prelude::*;

use super::{cview::*, Pitou, Theme};
use gloo::console::log;

#[derive(PartialEq, Properties)]
pub struct ContentViewProps {
    pub pitou: Pitou,
    pub theme: Theme,
    pub updatedirectory: Callback<Pitou>,
}

#[function_component]
pub fn ContentView(prop: &ContentViewProps) -> Html {
    let pitou = prop.pitou.clone();
    let theme = prop.theme;

    log!("rerendered! hurrah!");
    html! {
        <div style = { format!{
            "
            width: 100%;
            height: 100%;
            background-color: {};
            margin: 0% 0% 0% 0%;
            padding: 0% 0% 0% 0%;
            position: absolute;", prop.theme.background1() }} >
            <TopPane pitou = { pitou.clone() } {theme} />

            <BottomPane pitou = { pitou.clone() } {theme} />

            <LeftPane pitou = { pitou.clone() } {theme} />

            <SidePane pitou = { pitou.clone() } {theme} updatedirectory = { prop.updatedirectory.clone() } />

            <MainPane {pitou} {theme} updatedirectory = { prop.updatedirectory.clone() } />
        </div>
    }
}

#[allow(unused)]
pub struct OpeningView {}
