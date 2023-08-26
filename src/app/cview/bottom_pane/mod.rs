use yew::prelude::*;

use crate::app::ApplicationContext;
mod loading;
mod search;
pub use loading::*;

#[function_component]
pub fn BottomPane() -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context::<ApplicationContext>().unwrap();

    let background_color = theme.background1();
    let size = sizes.bottombar();

    let style = format! {"
    {size}
    background-color: {background_color};
    "};

    let inner_style = format! {"
    width: 100%;
    height: 100%;
    box-sizing: border-box;
    "};

    html! {
        <div {style}>
            <div style = {inner_style}></div>
        </div>
    }
}
