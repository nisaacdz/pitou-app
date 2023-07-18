//use gloo::console::log;
use yew::prelude::*;

use super::CheckBox;

#[derive(Properties, PartialEq)]
pub struct RowDescriptorProps {
    pub(super) toggleselectall: Callback<()>,
    pub(super) checked: bool,
}

#[function_component]
pub fn RowDescriptor(prop: &RowDescriptorProps) -> Html {
    let ontoggle = {
        let toggleselectall = prop.toggleselectall.clone();
        move |()| {
            toggleselectall.emit(());
        }
    };

    let style = format! {"
        display: flex;
        flex-direction: row;
        gap: 0;
        height: 5%;
        width: 100%;
    "};

    html! {
        <div {style}>
            <CheckBox {ontoggle} ischecked = { prop.checked }/>
            <Ico />
            <NameDescriptor />
            <FileTypeDescriptor />
            <LastModifiedDescriptor />
        </div>
    }
}

#[function_component]
fn FileTypeDescriptor() -> Html {
    let style = format! {"
    display: flex;
    flex-direction: row;
    gap: 0;
    left: 55%;
    width: 20%;
    height: 100%;
    font-family: monospace;
    padding-left: 1%;
    font-size: 100%;
    align-items: center;
    overflow: hidden;
    justify-content: center;
    white-space: nowrap;
    text-overflow: ellipsis;"};

    html! {
        <div {style}> { "FILE TYPE" } </div>
    }
}

#[function_component]
fn NameDescriptor() -> Html {
    let style = format! {"
    display: flex;
    flex-direction: row;
    gap: 0;
    left: 10%;
    width: 45%;
    height: 100%;
    font-family: monospace;
    padding-left: 1%;
    font-size: 100%;
    align-items: center;
    overflow: hidden;
    justify-content: center;
    white-space: nowrap;
    text-overflow: ellipsis;"};

    html! {
        <div {style}>{ "FILENAME" }</div>
    }
}

#[function_component]
fn Ico() -> Html {
    let style = format! {"
        display: flex;
        flex-direction: row;
        align-items: center;
        left: 5%;
        width: 5%;
        height: 100%;
        justify-content: center;
    "};
    html! {
        <div {style}>{ "ico" }</div>
    }
}

#[function_component]
fn LastModifiedDescriptor() -> Html {
    let style = format! {"
    display: flex;
    flex-direction: row;
    gap: 0;
    left: 75%;
    width: 25%;
    height: 100%;
    font-family: monospace;
    padding-left: 1%;
    font-size: 100%;
    align-items: center;
    overflow: hidden;
    justify-content: center;
    white-space: nowrap;
    text-overflow: ellipsis;"};

    html! {
        <div {style}>{ "Last Modified" }</div>
    }
}
