use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

mod components;
mod cview;
mod home;
mod search;
mod settings;
mod settingsview;
mod view;

pub mod data;
pub mod tasks;

pub use components::*;
pub use cview::*;
pub use home::*;
pub use search::*;
pub use settings::*;
pub use settingsview::*;
pub use view::*;

#[function_component]
pub fn App() -> Html {
    let application_ctx = use_state_eq(|| {
        let sizes = {
            let window = web_sys::window().unwrap();

            let screen = window.screen().unwrap();

            let screen_height = screen.avail_height().unwrap() - 23;
            let screen_width = screen.avail_width().unwrap();

            Sizes {
                screen_height,
                screen_width,
            }
        };

        let settings = Settings::DEFAULT;
        let theme = Theme::MAINGPTDARK;

        ApplicationContext {
            theme,
            settings,
            sizes,
        }
    });

    let updatesettings = {
        let applicationctx = application_ctx.clone();
        move |newsettings| {
            let mut newval = *applicationctx;
            newval.settings = newsettings;
            applicationctx.set(newval)
        }
    };

    let updatetheme = {
        let applicationctx = application_ctx.clone();
        move |newtheme| {
            let mut newval = *applicationctx;
            newval.theme = newtheme;
            applicationctx.set(newval)
        }
    };

    html! {
        <ContextProvider<ApplicationContext> context = { *application_ctx }>
            <ContentView {updatesettings} {updatetheme} />
        </ContextProvider<ApplicationContext>>
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
