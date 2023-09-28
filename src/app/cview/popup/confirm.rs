use yew::prelude::*;

use crate::app::ApplicationContext;

#[derive(PartialEq, Properties)]
pub struct ConfirmDeleteProps {
    pub delete: Callback<()>,
    pub prompt: String,
    pub cancel: Callback<()>,
}

#[function_component]
pub fn ConfirmDelete(prop: &ConfirmDeleteProps) -> Html {
    let ApplicationContext {
        theme,
        sizes: _,
        settings: _,
    } = use_context().unwrap();

    let onclickcancel = {
        let cancel = prop.cancel.clone();
        move |_| cancel.emit(())
    };

    let onsubmit = {
        let delete = prop.delete.clone();
        move |e: SubmitEvent| {
            e.prevent_default();
            delete.emit(());
        }
    };

    let onclick = move |e: MouseEvent| e.stop_propagation();

    let background_color = theme.background2();
    let border_color = theme.spare();

    let style = format! {"
    background-color: {background_color};
    border: 2px solid {border_color};
    "};

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
            <form {onsubmit} style = {buttons_style}>
                <button onclick={onclickcancel} style = {button2_style}>{ "Cancel" }</button>
                <button type="submit" style = {button1_style}>{ "Confirm" }</button>
            </form>
        </div>
    }
}
