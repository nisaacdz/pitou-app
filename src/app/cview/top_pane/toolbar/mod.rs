use yew::prelude::*;
use crate::app::{Theme, PitouProps};

mod paste_actions;
mod info_search;

use paste_actions::*;
use info_search::*;

#[function_component]
pub(super) fn ToolBar(prop: &PitouProps) -> Html {
    let style = format!("
        display: flex;
        flex-direction: row;
        width: 100%;
        height: 60%;
        padding-left: 1%;
        padding-left: 1%;
        gap: 0;
    ");

    let pitou = prop.pitou();
    let theme = prop.theme();

    html! {
        <div {style}>
        <CopyButton pitou = { pitou.clone() } {theme} />
        <CutButton pitou = { pitou.clone() } {theme} />
        <PasteButton pitou = { pitou.clone() } {theme} />
        <InfoButton pitou = { pitou.clone() } {theme} />
        <SearchButton pitou = { pitou.clone() } {theme} />
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct HoverNameProp {
    pub name: String,
    pub theme: Theme,
}

#[allow(unused)]
impl HoverNameProp {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn theme(&self) -> &Theme {
        &self.theme
    }
}

#[function_component]
pub fn HoverNameDisp(_prop: &HoverNameProp) -> Html {
    html! {

    }
}