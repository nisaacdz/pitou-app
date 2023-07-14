//use serde_wasm_bindgen::*;
use wasm_bindgen::prelude::*;
//use wasm_bindgen_futures::*;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

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

    let pitou = use_state(|| PitouProps::debug());

    html! {
        match settings.view() {
            AppView::Content => html! { <ContentView pitou = { pitou.pitou().clone() } /> },
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
