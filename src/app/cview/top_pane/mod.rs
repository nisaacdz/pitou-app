use backend::Pitou;
use yew::prelude::*;

mod toolbar;

use toolbar::*;

use crate::app::ApplicationContext;

#[derive(PartialEq, Properties)]
pub struct TopPaneProps {
    pub updatedirectory: Callback<Pitou>,
    pub pitou: Option<Pitou>,
    pub updateui: Callback<()>,
}

#[function_component]
pub fn TopPane(prop: &TopPaneProps) -> Html {
    let ApplicationContext { theme, sizes, settings: _ } = use_context::<ApplicationContext>().unwrap();
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
