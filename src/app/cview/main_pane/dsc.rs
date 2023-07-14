
//use gloo::console::log;
use yew::prelude::*;

use super::CheckBox;


#[derive(Properties, PartialEq)]
pub struct RowDescriptorProps {
    pub(super) toggle_select_all: Callback<()>,
    pub(super) checked: bool,
}

#[function_component]
pub fn RowDescriptor(prop: &RowDescriptorProps) -> Html {

    let on_toggle = {
        let toggle_select_all = prop.toggle_select_all.clone();
        move |()| {
            toggle_select_all.emit(());
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
            <CheckBox {on_toggle} is_checked = { prop.checked }/>
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
    left: 60%;
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
    width: 50%;
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
    left: 80%;
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
        <div {style}>{ "Last Modified" }</div>
    }
}