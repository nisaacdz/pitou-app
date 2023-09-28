use std::{cell::RefCell, rc::Rc};

use backend::File;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

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
    let tabs = use_state(|| Rc::new(RefCell::new(TabEntries::new())));
    let application_ctx = use_state_eq(|| {
        let sizes = {
            let window = web_sys::window().unwrap();

            let screen = window.screen().unwrap();

            let screen_height = screen.avail_height().unwrap() - 30;
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

    let removetab = {
        let tabs = tabs.clone();
        move |idx| {
            tabs.borrow_mut().remove(idx);
            tabs.set(Rc::clone(&*tabs))
        }
    };

    let changetab = {
        let tabs = tabs.clone();
        move |idx| {
            tabs.borrow_mut().change_tab(idx);
            tabs.set(Rc::clone(&*tabs))
        }
    };

    let newtab = {
        let tabs = tabs.clone();
        move |()| {
            tabs.borrow_mut().new_tab();
            tabs.set(Rc::clone(&*tabs))
        }
    };

    log(&format!("{}", tabs.borrow().current_tab().as_key()));

    let tab = tabs.borrow().current_tab();
    let key = tab.as_key();
    let data = tab.application_data();

    html! {
        <ContextProvider<ApplicationContext> context = { *application_ctx }>
            <TitleBar {changetab} {newtab} {removetab} tabs = {(*tabs).clone()}/>
            <ContentView {updatesettings} {updatetheme} {key} {data} />
        </ContextProvider<ApplicationContext>>
    }
}

type ID = Rc<String>;
const UNIQUE_LEN: usize = 10;

#[derive(Clone)]
pub struct Tab {
    pub id: ID,
    pub data: ApplicationData,
}

impl PartialEq for Tab {
    fn eq(&self, other: &Self) -> bool {
        &self.id == &other.id
    }
}

impl Tab {
    pub fn next() -> Self {
        let id = Rc::new(crate::app::data::generate_string(UNIQUE_LEN));
        let data = ApplicationData::new();
        Self { id, data }
    }

    pub fn as_key(&self) -> String {
        (*self.id).clone()
    }

    pub fn application_data(&self) -> ApplicationData {
        self.data.clone()
    }

    pub fn display(&self) -> Html {
        let dir_name = if let Some(d) = self.data.directory() {
            File::name_of(&*d).to_owned()
        } else {
            "".to_owned()
        };

        let cnt = match self.data.active_menu() {
            AppMenu::Explorer => html! {
            <>
                <img class = "title-bar-tab-icon" src="./public/icons/side/explorer.png"/>
                <span class = "title-bar-tab-name"> { dir_name } </span>
            </>
            },
            AppMenu::Home => html! {
            <>
                <img class = "title-bar-tab-icon" src="./public/icons/side/home.png"/>
                <span class = "title-bar-tab-name"> { dir_name } </span>
            </>
            },
            AppMenu::Settings => html! {
            <>
                <img class = "title-bar-tab-icon" src="./public/icons/side/settings.png"/>
                <span class = "title-bar-tab-name"> { "settings" } </span>
            </>
            },
            AppMenu::Search => html! {
            <>
                <img class = "title-bar-tab-icon" src="./public/icons/top/search.png"/>
                <span class = "title-bar-tab-name"> { "search results" } </span>
            </>
            },
            AppMenu::History => html! {
            <>
                <img class = "title-bar-tab-icon" src="./public/icons/side/history.png"/>
                <span class = "title-bar-tab-name"> { "history" } </span>
            </>
            },
            AppMenu::Bookmarks => html! {
            <>
                <img class = "title-bar-tab-icon" src="./public/icons/side/bookmark.png"/>
                <span class = "title-bar-tab-name"> { "bookmarks" } </span>
            </>
            },
            AppMenu::Locked => html! {
            <>
                <img class = "title-bar-tab-icon" src="./public/icons/side/locked.png"/>
                <span class = "title-bar-tab-name"> { "locked files" } </span>
            </>
            },
            AppMenu::Cloud => html! {
            <>
                <img class = "title-bar-tab-icon" src="./public/icons/side/cloud_dir.png"/>
                <span class = "title-bar-tab-name"> { "cloud" } </span>
            </>
            },
        };

        html! {
            <div class = "title-bar-tab-content">
                { cnt }
            </div>
        }
    }
}

#[derive(PartialEq)]
pub struct TabEntries {
    pub tabs: Vec<Tab>,
    pub current: usize,
}

impl TabEntries {
    fn new() -> Self {
        let tabs = vec![Tab::next()];
        let current = 0;
        Self { tabs, current }
    }

    fn remove(&mut self, idx: usize) {
        assert!(idx < self.tabs.len() && self.tabs.len() > 1);
        if self.current >= idx {
            self.tabs.remove(idx);
            self.current -= 1;
        } else {
            self.tabs.remove(idx);
        }
    }

    fn new_tab(&mut self) {
        self.tabs.push(Tab::next());
        self.current = self.tabs.len() - 1;
    }

    fn change_tab(&mut self, idx: usize) {
        self.current = idx;
    }

    fn current_tab(&self) -> Tab {
        self.tabs[self.current].clone()
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
