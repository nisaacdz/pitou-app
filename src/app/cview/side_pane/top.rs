use backend::Pitou;
use yew::prelude::*;

use crate::app::{RefreshIcon, Theme};

#[derive(PartialEq, Properties)]
pub struct TopOfParentDirProps {
    pub pitou: Option<Pitou>,
    pub theme: Theme,
}

#[function_component]
pub fn TopOfParentDir(prop: &TopOfParentDirProps) -> Html {
    let style = format! {"
        width: calc(100% - 4px);
        height: 5%;
        top: 0;
        display: flex;
        flex-direction: row;
        height: 5%;
        gap: 3px;
        position: relative;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
    "};

    let pitou = prop.pitou.clone();
    let theme = prop.theme;

    html! {
        <div {style}>
            <ParentDirName pitou = { pitou.clone() } {theme} />
            <RefreshButton { pitou } {theme} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct RefreshButtonProps {
    pitou: Option<Pitou>,
    theme: Theme,
}

#[function_component]
fn RefreshButton(prop: &RefreshButtonProps) -> Html {
    let onclick = |_| println!("");

    let spare_color = prop.theme.spare();
    let background_color = prop.theme.background1();

    let style = format! {"
        width: 10%;
        border: 1px solid {};
        border-radius: 10%;
        height: 70%;
        background-color: {background_color};
        ", spare_color};
    html! {
        <div class = "card" {style} {onclick}> <RefreshIcon /> </div>
    }
}

#[derive(PartialEq, Properties)]
struct ParentDirNameProps {
    pitou: Option<Pitou>,
    theme: Theme,
}

#[function_component]
fn ParentDirName(prop: &ParentDirNameProps) -> Html {
    let spare_color = prop.theme.spare();
    let background_color = prop.theme.background2();

    let style = format! {"
        display: flex;
        flex-direction: row;
        gap: 0;
        height: calc(100% - 4px);
        width: 75%;
        padding-left: 5%;
        padding-right: 5%;
        border: 1px solid {spare_color};
        background-color: {background_color};
    "};

    let name = prop.pitou.as_ref().map(|p| p.name()).unwrap_or_default();

    html! {
        <div {style}>
        { name }
        </div>

    }
}
