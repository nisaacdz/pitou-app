use std::{cell::RefCell, rc::Rc};

use yew::prelude::*;

use crate::app::{ApplicationContext, ApplicationData, SearchPage};

use super::{
    cview::*, AppMenu, BottomPane, HomeView, Pane, Settings, SettingsView, TabEntries, Theme,
    ToolBar, Tab,
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
        settings,
    } = use_context().unwrap();

    let updateui = { move |_| {} };

    let updatetheme = prop.updatetheme.clone();
    let updatesettings = prop.updatesettings.clone();

    let updateview = {
        let updatesettings = prop.updatesettings.clone();
        let data = prop.data.clone();
        move |newmenu| {
            data.update_app_menu(newmenu);
            updatesettings.emit(settings)
        }
    };

    let background_color = theme.background1();
    let size = sizes.screen();

    let style = format! {"
    --row-hover-color: {background_color};
    --scrollbar-thumb: {background_color};
    background-color: {background_color};
    margin: 0% 0% 0% 0%;
    padding: 0% 0% 0% 0%;
    position: relative;
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
            <div {style} >
                <ToolBar {updateui}/>
                <div style = {middle_style}>
                    <LeftPane {updateview}/>
                    <Pane/>
                </div>
                <BottomPane/>
            </div>
        },
        AppMenu::Home => html! {
            <div {style} >
                <ToolBar {updateui}/>
                <div style = {middle_style}>
                    <LeftPane updateview = {updateview.clone()}/>
                    <HomeView {updateview}/>
                </div>
                <BottomPane/>
            </div>
        },
        AppMenu::Settings => html! {
            <div {style} >
                <ToolBar {updateui}/>
                <div style = {middle_style}>
                    <LeftPane {updateview}/>
                    <SettingsView updatetheme = {updatetheme.clone()} updatesettings = {updatesettings.clone()}/>
                </div>
                <BottomPane/>
            </div>
        },
        AppMenu::Search => html! {
            <div {style} >
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
    let ApplicationContext { theme, sizes: _, settings: _} = use_context().unwrap();

    let background_color = theme.background2();

    let style = format! {"
    background-color: {background_color};
    "};

    let tab_entries = {
        let borrow = prop.tabs.borrow();
        borrow.tabs.iter().enumerate().map(|(idx, tab)| {
            let changetab = prop.changetab.clone();
            let removetab = prop.removetab.clone();
            let tab = tab.clone();
            let isactive = tab == borrow.current_tab();
            let open = move |()| changetab.emit(idx);
            let close = move |()| removetab.emit(idx);
            html! { <TitleBarTab {close} {open} {isactive} {tab}/> }
        }).chain(Some(html! { <TitleBarTabAdder newtab = {prop.newtab.clone()} /> })).collect::<Html>()
    };
    
    html! {
        <div class = "title-bar" {style}>
            <div class = "logo"></div>
            <div class = "tabs">
                { tab_entries }
            </div>
            <div class = "ctrl">
                <button class = "ctrl-button mini"></button>
                <button class = "ctrl-button maxi"></button>
                <button class = "ctrl-button close"></button>
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
            <button class = "title-bar-add-tab-btn" {onclick}>
            </button>
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
    let ApplicationContext { theme, settings: _, sizes: _} = use_context().unwrap();

    let dir = prop.tab.data.directory();

    let name = dir.as_ref().map(|dir| backend::File::name_of(&**dir)).unwrap_or_default();
    
    let border = if !prop.isactive { format! {"border: 1px solid {};", theme.spare()} } else { "".into() };
    let background_color = if prop.isactive { theme.background1() } else { theme.background2() };

    let style = format! {"
    {border}
    background-color: {background_color};
    "};

    let class = if prop.isactive { "tab-item active" } else { "tab-item" };

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
            <span>{ name }</span>
            <button class = "title-bar-tab-close" onclick = {onclose}></button>
        </div>
    }
}