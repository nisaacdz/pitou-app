//use gloo::console::log;
use yew::prelude::*;

use crate::app::Theme;

use super::Selected;

#[derive(Properties, PartialEq)]
pub struct RowDescriptorProps {
    pub(super) theme: Theme,
    pub(super) toggleselectall: Callback<()>,
    pub(super) selected: Option<Selected>,
}

#[function_component]
pub fn RowDescriptor(prop: &RowDescriptorProps) -> Html {
    let ontoggle = {
        let toggleselectall = prop.toggleselectall.clone();
        move |()| {
            toggleselectall.emit(());
        }
    };

    let background_color = prop.theme.background2();

    let style = format! {"
    display: flex;
    gap: 0;
    height: 5%;
    width: 100%;
    top: 0;
    left: 0;
    background-color: {background_color};
    "};

    html! {
        <div {style}>
            <CheckBox {ontoggle} ischecked = { prop.selected.as_ref().map(|s| s.all_checked()).unwrap_or(false) }/>
            <Ico />
            <NameDescriptor />
            <FileTypeDescriptor />
            <LastModifiedDescriptor />
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct CheckBoxProps {
    pub ischecked: bool,
    pub ontoggle: Callback<()>,
}

#[function_component]
pub fn CheckBox(prop: &CheckBoxProps) -> Html {
    let onclick = {
        let ontoggle = prop.ontoggle.clone();

        move |_| {
            ontoggle.emit(());
        }
    };

    let style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    width: 5%;
    height: 100%;
    "};

    html! {
        <div {style}>
            <input type = "checkbox" {onclick} checked = { prop.ischecked }/>
        </div>
    }
}

#[function_component]
fn FileTypeDescriptor() -> Html {
    let style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20%;
    height: 100%;"};

    html! {
        <div {style}>
            { "FILE TYPE" }
        </div>
    }
}

#[function_component]
fn NameDescriptor() -> Html {
    let style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    width: 45%;
    height: 100%;"};

    html! {
        <div {style}>
            <span>{ "FILENAME" }</span>
        </div>
    }
}

#[function_component]
fn Ico() -> Html {
    let style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    width: 5%;
    height: 100%;
    "};
    html! {
        <div {style}>
            <span> { "ico" } </span>
        </div>
    }
}

#[function_component]
fn LastModifiedDescriptor() -> Html {
    let style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    width: 25%;
    height: 100%;
    "};

    html! {
        <div {style}>{ "Last Modified" }</div>
    }
}
