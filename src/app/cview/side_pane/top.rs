use backend::Pitou;
use yew::prelude::*;

use crate::app::{RefreshIcon, Theme};

#[derive(PartialEq, Properties)]
pub struct TopOfParentDirProps {
    pub selected: Option<Pitou>,
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

    let name = prop.selected.as_ref().map(|directory| directory.name());

    html! {
        <div {style}>
            <ParentDirName {name} />
            <RefreshButton />
        </div>
    }
}


#[function_component]
fn RefreshButton() -> Html {
    let theme = use_context::<Theme>().unwrap();

    let onclick = |_| println!("");

    let spare_color = theme.spare();
    let background_color = theme.background1();

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
    name: Option<String>,
}

#[function_component]
fn ParentDirName(prop: &ParentDirNameProps) -> Html {
    let theme = use_context::<Theme>().unwrap();

    let spare_color = theme.spare();
    let background_color = theme.background2();

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

    let name = prop.name.clone().unwrap_or_default();

    html! {
        <div {style}>
        { name }
        </div>

    }
}
