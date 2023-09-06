use yew::prelude::*;

#[function_component]
pub fn FreeArea() -> Html {
    let style = format! {"
    height: 200px;
    width: 100%;
    "};

    html! {
        <div {style}></div>
    }
}
