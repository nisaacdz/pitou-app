use yew::prelude::*;

use crate::app::{ApplicationContext, SearchPage};

use super::{cview::*, AppView, BottomPane, HomeView, Pane, ToolBar};

#[derive(PartialEq, Properties)]
pub struct ContentViewProp {
    pub updateview: Callback<AppView>,
}

#[function_component]
pub fn ContentView(prop: &ContentViewProp) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings,
    } = use_context().unwrap();
    let force_update = use_force_update();

    let updateui = { move |_| force_update.force_update() };

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

    match settings.view() {
        AppView::Explorer => html! {
            <div {style} >
                <ToolBar {updateui}/>
                <div style = {middle_style}>
                    <LeftPane updateview = {prop.updateview.clone()}/>
                    <Pane/>
                </div>
                <BottomPane/>
            </div>
        },
        AppView::Home => html! {
            <div {style} >
                <ToolBar {updateui}/>
                <div style = {middle_style}>
                    <LeftPane updateview = {prop.updateview.clone()}/>
                    <HomeView/>
                </div>
                <BottomPane/>
            </div>
        },
        AppView::Settings => html! { <h1>{"Hello Settings"}</h1> },
        AppView::Search => html! {
            <div {style} >
                <ToolBar {updateui}/>
                <div style = {middle_style}>
                    <LeftPane updateview = {prop.updateview.clone()}/>
                    <SearchPage/>
                </div>
                <BottomPane/>
            </div>
        },
        _ => html! { <h1>{"Unimplemented"}</h1> },
    }
}

#[allow(unused)]
pub struct OpeningView {}
