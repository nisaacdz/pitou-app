use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct LoadingComponentProps {
    pub task: String,
}

#[function_component]
pub fn LoadingComponent(prop: &LoadingComponentProps) -> Html {
    let style = format! {"
    width: 50px;
    height: 10px;
    "};

    let glow_style = format! {"
    background-color: orange;
    animation: glowing 2s linear infinite;
    "};

    html! {
        <div {style}>
            <span>{ prop.task.clone() }</span>
            <div style = {glow_style}></div>
        </div>
    }
}

#[function_component]
pub fn EndedLoading() -> Html {
    html! {}
}
