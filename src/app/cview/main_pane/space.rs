use yew::prelude::*;

#[function_component]
pub fn FreeArea() -> Html {
    let style = format! {"
        height: 30%;
        width: 100%;
        flex-shrink: 0;
    "};

    html! {
        <div {style}></div>
    }
}
