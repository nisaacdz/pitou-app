use std::{cell::RefCell, rc::Rc, time::Duration};

use wasm_bindgen_futures::spawn_local;
use yew::{platform::time::sleep, prelude::*};

use crate::app::{
    data::SharedBorrow, tasks::SpawnHandle, ApplicationContext, ApplicationData, SearchPage,
};

use super::{
    cview::*, AppMenu, BottomPane, HomeView, Pane, Settings, SettingsView, Tab, TabEntries, Theme,
    ToolBar,
};

#[derive(PartialEq, Properties)]
pub struct ContentViewProp {
    pub updatesettings: Callback<Settings>,
    pub updatetheme: Callback<Theme>,
    pub data: ApplicationData,
}

#[function_component]
pub fn ContentView(prop: &ContentViewProp) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context().unwrap();

    let force_update = use_force_update();

    let updateui = { move |_| {} };

    let updatetheme = prop.updatetheme.clone();
    let updatesettings = prop.updatesettings.clone();

    let updateview = {
        let force_update = force_update.clone();
        let data = prop.data.clone();
        move |newmenu| {
            data.update_app_menu(newmenu);
            force_update.force_update();
        }
    };

    let background_color = theme.background1();
    let size = sizes.screen();

    let style = format! {"
    --row-hover-color: {background_color};
    --scrollbar-thumb: {background_color};
    background-color: {background_color};
    display: flex;
    flex-direction: column;
    {size}
    gap: 0;" };

    let middle_size = sizes.middle_portion();

    let middle_style = format! {"
    display: flex;
    flex-direction: row;
    {middle_size}
    gap: 0;
    "};

    let entries = match prop.data.active_menu() {
        AppMenu::Explorer => html! {
            <div {style} class = "app-body">
                <ToolBar {updateui}/>
                <div style = {middle_style}>
                    <LeftPane {updateview}/>
                    <Pane/>
                </div>
                <BottomPane/>
            </div>
        },
        AppMenu::Home => html! {
            <div {style} class = "app-body">
                <ToolBar {updateui}/>
                <div style = {middle_style}>
                    <LeftPane updateview = {updateview.clone()}/>
                    <HomeView {updateview}/>
                </div>
                <BottomPane/>
            </div>
        },
        AppMenu::Settings => html! {
            <div {style} class = "app-body">
                <ToolBar {updateui}/>
                <div style = {middle_style}>
                    <LeftPane {updateview}/>
                    <SettingsView updatetheme = {updatetheme.clone()} updatesettings = {updatesettings.clone()}/>
                </div>
                <BottomPane/>
            </div>
        },
        AppMenu::Search => html! {
            <div {style} class = "app-body">
                <ToolBar {updateui}/>
                <div style = {middle_style}>
                    <LeftPane updateview = {updateview.clone()}/>
                    <SearchPage {updateview}/>
                </div>
                <BottomPane/>
            </div>
        },
        _ => html! { <h1>{"Unimplemented"}</h1> },
    };

    html! {
        <ContextProvider<ApplicationData> context = { prop.data.clone() }>
            { entries }
        </ContextProvider<ApplicationData>>
    }
}

#[derive(PartialEq, Properties)]
pub struct TitleBarProps {
    pub tabs: Rc<RefCell<TabEntries>>,
    pub newtab: Callback<()>,
    pub changetab: Callback<usize>,
    pub removetab: Callback<usize>,
}

#[function_component]
pub fn TitleBar(prop: &TitleBarProps) -> Html {
    let ApplicationContext {
        theme,
        sizes: _,
        settings: _,
    } = use_context().unwrap();

    let background_color = theme.background2();

    let style = format! {"
    background-color: {background_color};
    "};

    let tab_entries = {
        let borrow = prop.tabs.borrow();
        borrow
            .tabs
            .iter()
            .enumerate()
            .map(|(idx, tab)| {
                let changetab = prop.changetab.clone();
                let removetab = prop.removetab.clone();
                let tab = tab.clone();
                let isactive = tab == borrow.current_tab();
                let open = move |()| changetab.emit(idx);
                let close = move |()| removetab.emit(idx);
                html! { <TitleBarTab {close} {open} {isactive} {tab}/> }
            })
            .chain(Some(
                html! { <TitleBarTabAdder newtab = {prop.newtab.clone()} /> },
            ))
            .collect::<Html>()
    };

    let onclose = { move |_| spawn_local(crate::app::tasks::close_window()) };

    let onmini = { move |_| spawn_local(crate::app::tasks::minimize_window()) };

    let onresize = { move |_| spawn_local(crate::app::tasks::toggle_maximize()) };

    // TODO
    // Investigate how to add `data-tauri-drag-region` to the titlebar

    html! {
        <div class = "title-bar" {style} data-tauri-drag-region = "true">
            <div class = "logo"></div>
            <div class = "tabs">
                { tab_entries }
            </div>
            <div class = "ctrl">
                <div class = "ctrl-button mini" onclick = {onmini}>
                    <img class = "ctrl-button-img card" src="./public/icons/title/mini.png"/>
                </div>
                <div class = "ctrl-button resize" onclick = {onresize}>
                    <img class = "ctrl-button-img card" src="./public/icons/title/resize.png"/>
                </div>
                <div class = "ctrl-button close" onclick = {onclose}>
                    <img class = "ctrl-button-img card" src="./public/icons/title/close.png"/>
                </div>
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct TitleBarTabAdderProps {
    newtab: Callback<()>,
}

#[function_component]
fn TitleBarTabAdder(prop: &TitleBarTabAdderProps) -> Html {
    let onclick = {
        let newtab = prop.newtab.clone();
        move |_| newtab.emit(())
    };

    html! {
        <div class = "title-bar-add-tab">
            <img class = "title-bar-add-tab-btn card" {onclick} src="./public/icons/title/add_tab.svg"/>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct TitleBarTabProps {
    tab: Tab,
    isactive: bool,
    close: Callback<()>,
    open: Callback<()>,
}

#[function_component]
fn TitleBarTab(prop: &TitleBarTabProps) -> Html {
    let ApplicationContext {
        theme,
        settings: _,
        sizes: _,
    } = use_context().unwrap();
    let aborthandle = use_state(|| SharedBorrow::new(None));
    let force_update = use_force_update();

    {
        let isactive = prop.isactive;
        use_effect(move || {
            if isactive {
                let newhandle = SpawnHandle::new(async move {
                    sleep(Duration::from_millis(250)).await;
                    force_update.force_update();
                });
                spawn_local(async move {
                    if let Some(oldhandle) = aborthandle.get_mut() {
                        SpawnHandle::abort(oldhandle).await;
                    }

                    aborthandle.get_mut().insert(newhandle).await;
                });
            }
        });
    }

    let value = prop.tab.display();

    let background_color = if prop.isactive {
        theme.background1()
    } else {
        theme.background2()
    };
    let foreground_color = theme.foreground1();
    let border_color = theme.spare();

    let class = if prop.isactive {
        "tab-item active"
    } else {
        "tab-item inactive"
    };

    let style = format! {"
    background-color: {background_color};
    border-color: {border_color};
    color: {foreground_color};
    "};

    let onclick = {
        let open = prop.open.clone();
        move |_| open.emit(())
    };

    let onclose = {
        let close = prop.close.clone();
        move |e: MouseEvent| {
            e.stop_propagation();
            close.emit(())
        }
    };

    html! {
        <div {class} {onclick} {style}>
            { value }
            <img class = "title-bar-tab-close card" onclick = {onclose} src="./public/icons/title/close.png"/>
        </div>
    }
}
