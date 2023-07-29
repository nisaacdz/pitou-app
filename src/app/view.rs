use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::app::{invoke, PitouNoArg};

use super::{cview::*, Theme};

#[derive(PartialEq, Properties)]
pub struct ContentViewProps {
    pub theme: Theme,
}

#[function_component]
pub fn ContentView(prop: &ContentViewProps) -> Html {
    let toggle = use_state(|| false);
    let directory = use_state(|| None);

    use_effect_with_deps(|directory| {
        crate::data::update_directory((**directory).clone());
    }, directory.clone());

    let updateui = move |_| toggle.set(false);

    {
        let directory = directory.clone();
        let arg = to_value(&PitouNoArg).unwrap();
        use_effect_with_deps(
            |_| {
                spawn_local(async move {
                    let js_val = invoke("get_debug_file", arg).await;
                    let res = from_value::<backend::Pitou>(js_val).unwrap();
                    directory.set(Some(res))
                });
            },
            (),
        );
    }

    let updatedirectory = {
        let directory = directory.clone();
        move |new_dir| directory.set(Some(new_dir))
    };

    let theme = prop.theme;

    let background_color = theme.background1();

    let style = format! {"
    width: 100%;
    height: 100%;
    background-color: {background_color};
    margin: 0% 0% 0% 0%;
    padding: 0% 0% 0% 0%;
    position: absolute;" };

    html! {
        <div {style} >
            <TopPane {theme} updatedirectory = { updatedirectory.clone() } pitou = { (&*directory).clone() } {updateui}/>

            <BottomPane {theme} />

            <LeftPane {theme} />

            <SidePane pitou = { (&*directory).clone() } {theme} updatedirectory = { updatedirectory.clone() } />

            <MainPane pitou = { (&*directory).clone() } {theme} {updatedirectory} />
        </div>
    }
}

#[allow(unused)]
pub struct OpeningView {}
