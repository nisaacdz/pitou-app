use yew::prelude::*;

#[function_component]
pub fn HomeView() -> Html {
    let style = format!{"
    display: flex;
    gap: 1%;
    "};
    html! {
        <div {style}></div>
    }
}
