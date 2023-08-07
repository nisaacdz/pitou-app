use crate::app::Theme;
use yew::prelude::*;

mod new_items;
mod others;
mod paste_actions;
mod prop_search;

use new_items::*;
use others::*;
use paste_actions::*;
use prop_search::*;

#[derive(PartialEq, Properties)]
pub struct ToolBarProps {
    pub updateui: Callback<()>,
}

#[function_component]
pub(super) fn ToolBar(prop: &ToolBarProps) -> Html {
    let style = format! {"
    display: flex;
    flex-shrink: 0;
    flex-direction: row;
    width: 100%;
    height: 60%;
    padding-left: 1%;
    padding-left: 1%;
    overflow: hidden;
    column-gap: 5px;
    font-size: 90%;"
    };

    html! {
        <div {style}>
            <ClipboardButton updateui = { prop.updateui.clone() }/>
            <CopyButton updateui = { prop.updateui.clone() }/>
            <CutButton  updateui = { prop.updateui.clone() }/>
            <PasteButton updateui = { prop.updateui.clone() } />
            <PropertiesButton updateui = { prop.updateui.clone() }/>
            <SearchButton updateui = { prop.updateui.clone() }/>
            <RenameButton updateui = { prop.updateui.clone() } />
            <NewFolderButton updateui = { prop.updateui.clone() }/>
            <NewFileButton updateui = { prop.updateui.clone() }/>
            <DeleteButton updateui = { prop.updateui.clone() } />
            <RefreshButton updateui = { prop.updateui.clone() }/>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct NameFieldProp {
    pub name: String,
}

#[allow(unused)]
impl NameFieldProp {
    pub fn name(&self) -> &String {
        &self.name
    }
}

#[function_component]
pub fn NameField(prop: &NameFieldProp) -> Html {
    let theme = use_context::<Theme>().unwrap();

    let foreground_color = theme.foreground1();
    let style = format! {"
    height: 30%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: small;
    color: {foreground_color};
    "};

    html! {
        <div {style}>
            <span> { prop.name() } </span>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct TopButtonProps {
    updateui: Callback<()>,
}
