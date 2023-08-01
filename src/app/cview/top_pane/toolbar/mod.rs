use crate::app::Theme;
use backend::Pitou;
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
    pub theme: Theme,
    pub updatedirectory: Callback<Pitou>,
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

    let theme = prop.theme;

    html! {
        <div {style}>
        <ClipboardButton {theme} updateui = { prop.updateui.clone() }/>
        <CopyButton {theme} updateui = { prop.updateui.clone() }/>
        <CutButton  {theme} updateui = { prop.updateui.clone() }/>
        <PasteButton {theme} updateui = { prop.updateui.clone() } />
        <PropertiesButton {theme} updateui = { prop.updateui.clone() }/>
        <SearchButton {theme} updateui = { prop.updateui.clone() }/>
        <RenameButton {theme} updateui = { prop.updateui.clone() } />
        <NewFolderButton {theme} updateui = { prop.updateui.clone() }/>
        <NewFileButton {theme} updateui = { prop.updateui.clone() }/>
        <DeleteButton {theme} updateui = { prop.updateui.clone() } />
        <RefreshButton {theme} updateui = { prop.updateui.clone() }/>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct NameFieldProp {
    pub name: String,
    pub theme: Theme,
}

#[allow(unused)]
impl NameFieldProp {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn theme(&self) -> &Theme {
        &self.theme
    }
}

#[function_component]
pub fn NameField(prop: &NameFieldProp) -> Html {
    let foreground_color = prop.theme().foreground1();
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
    theme: Theme,
    updateui: Callback<()>,
}
