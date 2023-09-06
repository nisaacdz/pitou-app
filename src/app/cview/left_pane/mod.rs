use yew::prelude::*;

use crate::{
    app::{AppView, ApplicationContext},
    background_color,
};

#[derive(PartialEq, Properties)]
pub struct LeftPaneProps {
    pub updateview: Callback<AppView>,
}

#[function_component]
pub fn LeftPane(prop: &LeftPaneProps) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context().unwrap();
    let background_color = theme.background1();
    let size = sizes.menubar();
    let border_color = theme.spare();
    let foreground_color = theme.foreground1();

    let style = format! {"
    --tooltip-background: {background_color};
    --tooltip-foreground: {foreground_color};
    display: flex;
    flex-direction: column;
    gap: 10px;
    {size}
    background-color: {background_color};
    box-sizing: border-box;
    border: 1px solid {border_color};
    "};

    html! {
        <div {style}>
            <BackButton />
            <ExploreButton updateview = {prop.updateview.clone()}/>
            <HomeButton updateview = {prop.updateview.clone()}/>
            <SearchButton updateview = {prop.updateview.clone()}/>
            <HistoryButton />
            <BookmarksButton />
            <LockedButton />
            <CloudButton />
            <SettingsButton />
        </div>
    }
}

#[function_component]
fn BackButton() -> Html {
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context().unwrap();

    let mut outer_size = sizes.menu_item();
    outer_size.width -= 2;
    let icon_style = sizes.menu_item_icon();

    let style = format! {"
    {outer_size}
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    position: relative;
    "};

    let icon_style = format! {"
    {icon_style}
    "};

    html! {
        <div {style} class = "menu_icon" tooltip = "back">
            <img class = "card" style = {icon_style} src="./public/icons/side/back_arrow.png"/>
        </div>
    }
}

#[function_component]
fn ExploreButton(prop: &ButtonProp) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings,
    } = use_context().unwrap();

    let mut outer_size = sizes.menu_item();
    outer_size.width -= 2;
    let icon_style = sizes.menu_item_icon();

    let background_color = background_color!(
        matches!(settings.view, AppView::Explorer),
        theme.background2()
    );

    let style = format! {"
    {outer_size}
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    position: relative;
    {background_color}
    "};

    let icon_style = format! {"
    {icon_style}
    "};

    let onclick = {
        let updateview = prop.updateview.clone();
        move |_| updateview.emit(AppView::Explorer)
    };

    html! {
        <div {style} class = "menu_icon" tooltip = "explorer">
            <img {onclick} class = "card" style = {icon_style} src="./public/icons/side/explorer.png"/>
        </div>
    }
}

#[function_component]
fn HomeButton(prop: &ButtonProp) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings,
    } = use_context().unwrap();

    let background_color =
        background_color!(matches!(settings.view, AppView::Home), theme.background2());

    let onclick = {
        let updateview = prop.updateview.clone();
        move |_| updateview.emit(AppView::Home)
    };

    let mut outer_size = sizes.menu_item();
    outer_size.width -= 2;

    let icon_style = sizes.menu_item_icon();

    let style = format! {"
    {outer_size}
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    position: relative;
    {background_color}
    "};

    let icon_style = format! {"
    {icon_style}
    "};

    html! {
        <div {style} class = "menu_icon" tooltip = "home">
            <img {onclick} class = "card" style = {icon_style} src="./public/icons/side/home.png"/>
        </div>
    }
}

#[function_component]
fn SettingsButton() -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings,
    } = use_context().unwrap();

    let background_color = background_color!(
        matches!(settings.view, AppView::Settings),
        theme.background2()
    );

    let mut outer_size = sizes.menu_item();
    outer_size.width -= 2;
    let icon_style = sizes.menu_item_icon();

    let style = format! {"
    {outer_size}
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    position: relative;
    {background_color}
    "};

    let icon_style = format! {"
    {icon_style}
    "};

    html! {
        <div {style} class = "menu_icon" tooltip = "settings">
            <img class = "card" style = {icon_style} src="./public/icons/side/settings.png"/>
        </div>
    }
}

#[function_component]
fn HistoryButton() -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings,
    } = use_context().unwrap();

    let background_color = background_color!(
        matches!(settings.view, AppView::History),
        theme.background2()
    );

    let mut outer_size = sizes.menu_item();
    outer_size.width -= 2;
    let icon_style = sizes.menu_item_icon();

    let style = format! {"
    {outer_size}
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    position: relative;
    {background_color}
    "};

    let icon_style = format! {"
    {icon_style}
    "};

    html! {
        <div {style} class = "menu_icon" tooltip = "history">
            <img class = "card" style = {icon_style} src="./public/icons/side/history.png"/>
        </div>
    }
}

#[function_component]
fn BookmarksButton() -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings,
    } = use_context().unwrap();

    let background_color = background_color!(
        matches!(settings.view, AppView::Bookmarks),
        theme.background2()
    );

    let mut outer_size = sizes.menu_item();
    outer_size.width -= 2;
    let icon_style = sizes.menu_item_icon();

    let style = format! {"
    {outer_size}
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    position: relative;
    {background_color}
    "};

    let icon_style = format! {"
    {icon_style}
    "};

    html! {
        <div {style} class = "menu_icon" tooltip = "bookmarks">
            <img class = "card" style = {icon_style} src="./public/icons/side/bookmark.png"/>
        </div>
    }
}

#[function_component]
fn CloudButton() -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings,
    } = use_context().unwrap();

    let background_color =
        background_color!(matches!(settings.view, AppView::Cloud), theme.background2());

    let mut outer_size = sizes.menu_item();
    outer_size.width -= 2;
    let icon_style = sizes.menu_item_icon();

    let style = format! {"
    {outer_size}
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    position: relative;
    {background_color}
    "};

    let icon_style = format! {"
    {icon_style}
    "};

    html! {
        <div {style} class = "menu_icon" tooltip = "cloud">
            <img class = "card" style = {icon_style} src="./public/icons/side/cloud_dir.png"/>
        </div>
    }
}

#[function_component]
fn LockedButton() -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings,
    } = use_context().unwrap();

    let background_color = background_color!(
        matches!(settings.view, AppView::Locked),
        theme.background2()
    );

    let mut outer_size = sizes.menu_item();
    outer_size.width -= 2;
    let icon_style = sizes.menu_item_icon();

    let style = format! {"
    {outer_size}
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    position: relative;
    {background_color}
    "};

    let icon_style = format! {"
    {icon_style}
    "};

    html! {
        <div {style} class = "menu_icon" tooltip = "locked">
            <img class = "card" style = {icon_style} src="./public/icons/side/locked.png"/>
        </div>
    }
}

#[function_component]
fn SearchButton(prop: &ButtonProp) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings,
    } = use_context().unwrap();

    let background_color = background_color!(
        matches!(settings.view, AppView::Search),
        theme.background2()
    );

    let mut outer_size = sizes.menu_item();
    outer_size.width -= 2;
    let icon_style = sizes.menu_item_icon();

    let style = format! {"
    {outer_size}
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    position: relative;
    {background_color}
    "};

    let icon_style = format! {"
    {icon_style}
    "};

    let onclick = {
        let updateview = prop.updateview.clone();

        move |_| updateview.emit(AppView::Search)
    };

    html! {
        <div {style} class = "menu_icon" tooltip = "search">
            <img class = "card" {onclick} style = {icon_style} src="./public/icons/top/search.png"/>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct ButtonProp {
    updateview: Callback<AppView>,
}
