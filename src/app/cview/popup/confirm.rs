use yew::prelude::*;

use crate::app::Theme;

#[derive(PartialEq, Properties)]
pub struct ConfirmProps {
    pub prompt: String,
    pub cancel: Callback<()>,
    pub confirm: Callback<()>,
}

#[function_component]
pub fn Confirm(prop: &ConfirmProps) -> Html {
    let theme = use_context::<Theme>().unwrap();

    let onclickcancel = {
        let cancel = prop.cancel.clone();
        move |_| cancel.emit(())
    };

    let onclick = move |e: MouseEvent| e.stop_immediate_propagation();

    let background_color = theme.background2();
    let border_color = theme.spare();

    let style = format! {"
    background-color: {background_color};
    border: 2px solid {border_color};
    "};

    let onclickconfirm = {
        let confirm = prop.confirm.clone();
        move |_| confirm.emit(())
    };

    let buttons_style = format! {"
    height: 15%;
    display: flex;
    gap: 0;
    "};

    let button1_style = format! {"
    width: 50%;
    "};

    let button2_style = format! {"
    width: 50%;
    "};

    html! {
        <div {style} class = {"popup"} {onclick}>
            <span>{ prop.prompt.clone() }</span>
            <div style = {buttons_style}>
                <button onclick={onclickcancel} style = {button2_style}>{ "Cancel" }</button>
                <button onclick={onclickconfirm} style = {button1_style}>{ "Confirm" }</button>
            </div>
        </div>
    }
}
