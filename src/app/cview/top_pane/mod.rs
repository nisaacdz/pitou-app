use backend::Pitou;
use yew::prelude::*;

mod ancestors;
mod toolbar;

use ancestors::*;
use toolbar::*;

use crate::app::Theme;

#[derive(PartialEq, Properties)]
pub struct TopPaneProps {
    pub updatedirectory: Callback<Pitou>,
    pub pitou: Option<Pitou>,
    pub updateui: Callback<()>,
}

#[function_component]
pub fn TopPane(prop: &TopPaneProps) -> Html {
    let theme = use_context::<Theme>().unwrap();
    let pitou = prop.pitou.clone();

    let background_color = theme.background1();
    let style = format! {"
    background-color: {background_color};
    top: 0%;
    left: 0%;
    position: relative;
    height: 10%;
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 0;"};

    html! {
        <div {style}>
            <ToolBar updateui = {prop.updateui.clone()} />
            <AncestorsTabs {pitou} updatedirectory = { prop.updatedirectory.clone() } />
        </div>
    }
}
