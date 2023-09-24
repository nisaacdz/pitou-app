use yew::prelude::*;

use crate::app::ApplicationContext;

#[derive(Properties, PartialEq)]
pub struct RowDescriptorProps {
    pub(super) selected: bool,
    pub(super) toggleselectall: Callback<()>,
}

#[function_component]
pub fn RowDescriptor(prop: &RowDescriptorProps) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context().unwrap();

    let ontoggle = {
        let toggleselectall = prop.toggleselectall.clone();
        move |_| {
            toggleselectall.emit(());
        }
    };

    let background_color = theme.background2();
    let height = sizes.dsc().height();

    let style = format! {"
    display: flex;
    gap: 0;
    {height}
    width: 100%;
    background-color: {background_color};
    "};

    html! {
        <div {style}>
            <CheckBox {ontoggle} ischecked = { prop.selected }/>
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
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context().unwrap();

    let onclick = {
        let ontoggle = prop.ontoggle.clone();
        move |_| {
            ontoggle.emit(());
        }
    };

    let width = sizes.row_checkbox();

    let style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    {width}
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
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context().unwrap();

    let width = sizes.row_typefield();

    let style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    {width}
    height: 100%;"};

    html! {
        <div {style}>
            { "FILE TYPE" }
        </div>
    }
}

#[function_component]
fn NameDescriptor() -> Html {
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context().unwrap();

    let width = sizes.row_namefield();

    let style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    {width}
    height: 100%;"};

    html! {
        <div {style}>
            <span>{ "FILENAME" }</span>
        </div>
    }
}

#[function_component]
fn Ico() -> Html {
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context().unwrap();

    let width = sizes.row_icon();

    let style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    {width}
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
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context().unwrap();

    let width = sizes.row_sparefield();

    let style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    {width}
    height: 100%;
    "};

    html! {
        <div {style}>{ "Last Modified" }</div>
    }
}
