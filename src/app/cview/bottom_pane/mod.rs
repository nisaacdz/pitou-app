use yew::prelude::*;

use crate::app::ApplicationContext;
mod loading;
mod search;
pub use loading::*;
use search::SearchComponent;

#[function_component]
pub fn BottomPane() -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context().unwrap();

    let background_color = theme.background1();
    let size = sizes.bottombar();

    let style = format! {"
    {size}
    background-color: {background_color};
    box-sizing: border-box;
    "};

    let inner_style = format! {"
    width: 100%;
    height: 100%;
    box-sizing: border-box;
    display: flex;
    align-items: center;
    justify-content: center;
    "};

    html! {
        <div {style}>
            <div style = {inner_style}>
                <SearchComponent />
            </div>
        </div>
    }
}
