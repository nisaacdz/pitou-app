use yew::prelude::*;

use crate::{
    app::{AppView, ApplicationContext},
    background_color,
};
use backend::Pitou;

#[derive(PartialEq, Properties)]
struct LeftPaneMembersProps {
    pitou: Pitou,
    onclick: Callback<()>,
    onhover: Callback<()>,
}

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
    } = use_context::<ApplicationContext>().unwrap();
    let background_color = theme.background1();
    let size = sizes.menubar();
    let border_color = theme.spare();

    let style = format! {"
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
            <HomeButton />
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
pub fn BackButton() -> Html {
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context::<ApplicationContext>().unwrap();

    let outer_size = sizes.menu_item();
    let icon_style = sizes.menu_item_icon();

    let style = format! {"
    {outer_size}
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    "};

    let icon_style = format! {"
    {icon_style}
    "};

    html! {
        <div {style}>
            <img class = "card" style = {icon_style} src="./public/icons/side/back_arrow.png" alt="back"/>
        </div>
    }
}

#[function_component]
fn ExploreButton(prop: &ButtonProp) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings,
    } = use_context::<ApplicationContext>().unwrap();

    let outer_size = sizes.menu_item();
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
        <div {style}>
            <img {onclick} class = "card" style = {icon_style} src="./public/icons/side/explorer.png" alt="explorer"/>
        </div>
    }
}

#[function_component]
pub fn HomeButton() -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings,
    } = use_context::<ApplicationContext>().unwrap();

    let background_color =
        background_color!(matches!(settings.view, AppView::Home), theme.background2());

    let outer_size = sizes.menu_item();
    let icon_style = sizes.menu_item_icon();

    let style = format! {"
    {outer_size}
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    {background_color}
    "};

    let icon_style = format! {"
    {icon_style}
    "};

    html! {
        <div {style}>
            <img class = "card" style = {icon_style} src="./public/icons/side/home.png" alt="home"/>
        </div>
    }
}

#[function_component]
pub fn SettingsButton() -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings,
    } = use_context::<ApplicationContext>().unwrap();

    let background_color = background_color!(
        matches!(settings.view, AppView::Settings),
        theme.background2()
    );

    let outer_size = sizes.menu_item();
    let icon_style = sizes.menu_item_icon();

    let style = format! {"
    {outer_size}
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    {background_color}
    "};

    let icon_style = format! {"
    {icon_style}
    "};

    html! {
        <div {style}>
            <img class = "card" style = {icon_style} src="./public/icons/side/settings.png" alt="settings"/>
        </div>
    }
}

#[function_component]
pub fn HistoryButton() -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings,
    } = use_context::<ApplicationContext>().unwrap();

    let background_color = background_color!(
        matches!(settings.view, AppView::History),
        theme.background2()
    );

    let outer_size = sizes.menu_item();
    let icon_style = sizes.menu_item_icon();

    let style = format! {"
    {outer_size}
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    {background_color}
    "};

    let icon_style = format! {"
    {icon_style}
    "};

    html! {
        <div {style}>
            <img class = "card" style = {icon_style} src="./public/icons/side/history.png" alt="history"/>
        </div>
    }
}

#[function_component]
pub fn BookmarksButton() -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings,
    } = use_context::<ApplicationContext>().unwrap();

    let background_color = background_color!(
        matches!(settings.view, AppView::Bookmarks),
        theme.background2()
    );

    let outer_size = sizes.menu_item();
    let icon_style = sizes.menu_item_icon();

    let style = format! {"
    {outer_size}
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    {background_color}
    "};

    let icon_style = format! {"
    {icon_style}
    "};

    html! {
        <div {style}>
            <img class = "card" style = {icon_style} src="./public/icons/side/bookmark.png" alt="bookmarks"/>
        </div>
    }
}

#[function_component]
pub fn CloudButton() -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings,
    } = use_context::<ApplicationContext>().unwrap();

    let background_color =
        background_color!(matches!(settings.view, AppView::Cloud), theme.background2());

    let outer_size = sizes.menu_item();
    let icon_style = sizes.menu_item_icon();

    let style = format! {"
    {outer_size}
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    {background_color}
    "};

    let icon_style = format! {"
    {icon_style}
    "};

    html! {
        <div {style}>
            <img class = "card" style = {icon_style} src="./public/icons/side/cloud_dir.png" alt="cloud"/>
        </div>
    }
}

#[function_component]
pub fn LockedButton() -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings,
    } = use_context::<ApplicationContext>().unwrap();

    let background_color = background_color!(
        matches!(settings.view, AppView::Locked),
        theme.background2()
    );

    let outer_size = sizes.menu_item();
    let icon_style = sizes.menu_item_icon();

    let style = format! {"
    {outer_size}
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    {background_color}
    "};

    let icon_style = format! {"
    {icon_style}
    "};

    html! {
        <div {style}>
            <img class = "card" style = {icon_style} src="./public/icons/side/locked.png" alt="locked"/>
        </div>
    }
}

#[function_component]
fn SearchButton(prop: &ButtonProp) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings,
    } = use_context::<ApplicationContext>().unwrap();

    let background_color = background_color!(
        matches!(settings.view, AppView::Search),
        theme.background2()
    );

    let outer_size = sizes.menu_item();
    let icon_style = sizes.menu_item_icon();

    let style = format! {"
    {outer_size}
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
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
        <div {style}>
            <img  {onclick} style = {icon_style} class = "card" src="./public/icons/top/search.png" alt="search" />
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct ButtonProp {
    updateview: Callback<AppView>,
}
