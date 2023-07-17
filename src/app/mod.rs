use backend::Pitou;
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
//use serde_wasm_bindgen::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
//use wasm_bindgen_futures::*;
use gloo::console::log;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize)]
pub(crate) struct PitouArg {
    pub pitou: Pitou,
}

#[derive(Serialize)]
pub(crate) struct PitouNoArg;

mod components;
mod cview;
mod home;
mod pitou;
mod settings;
mod view;

pub use components::*;
pub use cview::*;
pub use home::*;
pub use pitou::*;
pub use settings::*;
pub use view::*;

#[function_component]
pub fn App() -> Html {
    let settings = use_state(|| Settings::DEFAULT);
    let theme = use_state(|| Theme::DEFAULT);

    let selected_directory = use_state(|| None);

    {
        let selected_directory = selected_directory.clone();
        let theme = theme.clone();
        let pitou = to_value(&PitouNoArg).unwrap();
        use_effect_with_deps(
            |_| {
                spawn_local(async move {
                    log!("spawning from app");
                    let js_val = invoke("get_debug_file", pitou).await;
                    let res = from_value::<backend::Pitou>(js_val).unwrap();
                    let theme = *theme;
                    selected_directory.set(Some(PitouProps::new(res, theme)))
                })
            },
            (),
        );
    }

    html! {
        match settings.view() {
            AppView::Content =>
            if let Some(PitouProps { pitou, theme}) = &*selected_directory {
                html! { <ContentView pitou = { pitou.clone() } theme = {*theme}/> }
            } else {
                html! { <h3>{ "Waiting" }</h3> }
            },
            AppView::Opening => html! { <h1>{"Hello Opening View"}</h1> },
            AppView::Settings => html! { <h1>{"Hello Settings"}</h1> },
        }
    }
}

#[macro_export]
macro_rules! insert_or_remove {
    ($con:expr, $val:expr) => {
        if $con {
            $val
        } else {
            String::new()
        }
    };
}

#[macro_export]
macro_rules! background_color {
    ($con:expr, $val:expr) => {
        crate::insert_or_remove!($con, format!("background-color: {};", $val))
    };
}

#[macro_export]
macro_rules! color {
    ($con:expr, $val:expr) => {
        crate::insert_or_remove!($con, format!("color: {};", $val))
    };
}

#[macro_export]
macro_rules! brightness {
    ($con:expr, $val:expr) => {
        crate::insert_or_remove!($con, format!("filter: brightness({}%);", $val))
    };
}

#[macro_export]
macro_rules! enlarge {
    ($con:expr, $val:expr) => {
        crate::insert_or_remove!($con, format!("transform: scale({});", $val))
    };
}
