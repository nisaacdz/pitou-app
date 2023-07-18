use crate::app::{CopyIcon, CutIcon, PasteIcon, PitouProps};
use yew::prelude::*;

use super::HoverNameDisp;

#[function_component]
pub fn PasteButton(prop: &PitouProps) -> Html {
    let mouse_over = use_state(|| false);

    let onmouseover = {
        let mouse_over = mouse_over.clone();
        move |_| mouse_over.set(true)
    };

    let onmouseout = {
        let mouse_over = mouse_over.clone();
        move |_| mouse_over.set(false)
    };

    let style = format! {"
        position: relative;
        width: 3%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
    "};

    let icon_style = format! {"
        display: flex;
        height: 80%;
        width: 100%;
    "};

    let theme = prop.theme();

    html! {
        <div {style} {onmouseover} {onmouseout}>
            <div class = "card" style = {icon_style}>
                <PasteIcon {theme}/>

            </div>
            {
                if *mouse_over {
                    html! { <HoverNameDisp name = { "paste" }  {theme} /> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

#[function_component]
pub fn CopyButton(prop: &PitouProps) -> Html {
    let mouse_over = use_state(|| false);

    let onmouseover = {
        let mouse_over = mouse_over.clone();
        move |_| mouse_over.set(true)
    };

    let onmouseout = {
        let mouse_over = mouse_over.clone();
        move |_| mouse_over.set(false)
    };

    let style = format! {"
        position: relative;
        width: 3%;
        top: 0;
        bottom: 0;
        display: flex;
        align-items: center;
        justify-content: center;
    "};

    let icon_style = format! {"
        display: flex;
        height: 80%;
        width: 100%;
    "};

    let theme = prop.theme();

    html! {
        <div {style} {onmouseover} {onmouseout}>
            <div class = "card" style = {icon_style}>
                <CopyIcon { theme }/>

            </div>
            {
                if *mouse_over {
                    html! { <HoverNameDisp name = { "copy" }  { theme } /> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

#[function_component]
pub fn CutButton(prop: &PitouProps) -> Html {
    let mouse_over = use_state(|| false);

    let onmouseover = {
        let mouse_over = mouse_over.clone();
        move |_| mouse_over.set(true)
    };

    let onmouseout = {
        let mouse_over = mouse_over.clone();
        move |_| mouse_over.set(false)
    };

    let style = format! {"
        position: relative;
        width: 3%;
        top: 0;
        bottom: 0;
        display: flex;
        align-items: center;
        justify-content: center;
    "};

    let icon_style = format! {"
        display: flex;
        height: 80%;
        width: 100%;
    "};

    let theme = prop.theme();

    html! {
        <div {style} {onmouseover} {onmouseout}>
            <div class = "card" style = {icon_style}>
                <CutIcon { theme }/>

            </div>
            {
                if *mouse_over {
                    html! { <HoverNameDisp name = { "cut" }  { theme } /> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
