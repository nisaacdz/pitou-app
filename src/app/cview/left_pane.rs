use yew::prelude::*;

use crate::app::{
    BackIcon, BookmarksIcon, CloudStorageIcon, HistoryIcon, HomeIcon, LockedIcon, SettingsIcon,
    Theme,
};
use backend::Pitou;

#[derive(PartialEq, Properties)]
struct LeftPaneMembersProps {
    pitou: Pitou,
    onclick: Callback<()>,
    onhover: Callback<()>,
}

#[derive(PartialEq, Properties)]
struct HoverNameProp {
    name: String,
}

impl HoverNameProp {
    fn name(&self) -> &String {
        &self.name
    }
}

#[function_component]
fn HoverNameDisp(prop: &HoverNameProp) -> Html {
    let theme = use_context::<Theme>().unwrap();

    let style = format! {"
        background-color: {};
        position: relative;
        z-index: 1;
        color: {};
        left: 101%;
        padding-left: 5%;
        padding-right: 5%;
        right: auto;
        top: 35%;
        height: 50%;
        ", theme.background1(), theme.foreground1()
    };

    html! {
        <div {style}> { prop.name() } </div>
    }
}

#[function_component]
pub fn LeftPane() -> Html {
    let theme = use_context::<Theme>().unwrap();
    let background_color = theme.background1();

    let style = format! {"
    position: absolute;
    display: flex;
    flex-direction: column;
    gap: 3%;
    width: 4%;
    bottom: 4%;
    top: 10%;
    padding-top: 3%;
    background-color: {background_color};
    left: 0%;
    margin-bottom: 1px;"};

    html! {
        <div {style}>
            <BackButton />
            <HomeButton />
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
    let style = format! {"
        position: relative;
        overflow-x: visible;
        width: 100%;
        height: 9%;
    "};

    let icon_style = format! {"
        width: 100%;
        height: 100%;
        position: absolute;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
    "};

    html! {
        <div {style}>
            <div class = "card" style = {icon_style}>
                <BackIcon />
            </div>
        </div>
    }
}

#[function_component]
pub fn HomeButton() -> Html {
    let style = format! {"
        position: relative;
        overflow-x: visible;
        width: 100%;
        height: 9%;
    "};

    let icon_style = format! {"
        width: 100%;
        height: 100%;
        position: absolute;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;    
    "};

    html! {
        <div {style}>
            <div class = "card" style = {icon_style}>
                <HomeIcon />
            </div>
        </div>
    }
}

#[function_component]
pub fn SettingsButton() -> Html {
    let style = format! {"
        position: relative;
        overflow-x: visible;
        width: 100%;
        height: 9%;
    "};

    let icon_style = format! {"
        width: 100%;
        height: 100%;
        position: absolute;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;    
    "};

    html! {
        <div {style}>
            <div class = "card" style = {icon_style}>
                <SettingsIcon />
            </div>
        </div>
    }
}

#[function_component]
pub fn HistoryButton() -> Html {
    let style = format! {"
        position: relative;
        overflow-x: visible;
        width: 100%;
        height: 9%;
    "};

    let icon_style = format! {"
        width: 100%;
        height: 100%;
        position: absolute;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;    
    "};

    html! {
        <div {style}>
            <div class = "card" style = {icon_style}>
                <HistoryIcon />
            </div>
        </div>
    }
}

#[function_component]
pub fn BookmarksButton() -> Html {
    let style = format! {"
        position: relative;
        overflow-x: visible;
        width: 100%;
        height: 9%;
    "};

    let icon_style = format! {"
        width: 100%;
        height: 100%;
        position: absolute;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;    
    "};

    html! {
        <div {style}>
            <div class = "card" style = {icon_style}>
                <BookmarksIcon />
            </div>
        </div>
    }
}

#[function_component]
pub fn CloudButton() -> Html {
    let style = format! {"
        position: relative;
        overflow-x: visible;
        width: 100%;
        height: 9%;
    "};

    let icon_style = format! {"
        width: 100%;
        height: 100%;
        position: absolute;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;    
    "};

    html! {
        <div {style}>
            <div class = "card" style = {icon_style}>
                <CloudStorageIcon />
            </div>
        </div>
    }
}

#[function_component]
pub fn LockedButton() -> Html {
    let style = format! {"
        position: relative;
        overflow-x: visible;
        width: 100%;
        height: 9%;
    "};

    let icon_style = format! {"
        width: 100%;
        height: 100%;
        position: absolute;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;    
    "};

    html! {
        <div {style}>
            <div class = "card" style = {icon_style}>
                <LockedIcon />
            </div>
        </div>
    }
}
