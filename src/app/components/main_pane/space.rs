use yew::prelude::*;

#[function_component]
pub fn FreeArea() -> Html {

    let style = format! {"
    height: 300px;
    width: 100%;
    background-color: green;
    "};

    html! {
        <div {style}></div>
    }
}
