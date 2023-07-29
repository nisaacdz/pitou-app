use yew::prelude::*;

use crate::app::{
    BackIcon, BookmarksIcon, CloudStorageIcon, HistoryIcon, HomeIcon, LockedIcon, SettingsIcon,
    Theme,
};
use backend::Pitou;

#[derive(PartialEq, Properties)]
struct LeftPaneMembersProps {
    pitou: Pitou,
    theme: Theme,
    onclick: Callback<()>,
    onhover: Callback<()>,
}

#[derive(PartialEq, Properties)]
struct HoverNameProp {
    name: String,
    theme: Theme,
}

impl HoverNameProp {
    fn name(&self) -> &String {
        &self.name
    }
    fn theme(&self) -> &Theme {
        &self.theme
    }
}

#[function_component]
fn HoverNameDisp(prop: &HoverNameProp) -> Html {
    let style = format! {"
        background-color: {};
        position: relative;
        z-index: 1;
        color: {};
        left: 101%;
        padding-left: 5%;
        padding-right: 5%;
        right: auto;
        top: 35%;
        height: 50%;
        ", prop.theme().background1(), prop.theme().foreground1()
    };

    html! {
        <div {style}> { prop.name() } </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct LeftPaneProps {
    pub theme: Theme,
}

#[function_component]
pub fn LeftPane(prop: &LeftPaneProps) -> Html {
    let theme = prop.theme;
    let background_color = theme.background1();

    let style = format! {"
    position: absolute;
    display: flex;
    flex-direction: column;
    gap: 3%;
    width: 4%;
    bottom: 4%;
    top: 10%;
    padding-top: 3%;
    background-color: {background_color};
    left: 0%;
    margin-bottom: 1px;"};

    html! {
        <div {style}>

            <BackButton {theme}/>
            <HomeButton {theme}/>
            <HistoryButton {theme}/>
            <BookmarksButton {theme}/>
            <LockedButton  {theme}/>
            <CloudButton {theme}/>
            <SettingsButton {theme}/>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub theme: Theme,
}

impl ButtonProps {
    fn theme(&self) -> Theme {
        self.theme
    }
}

#[function_component]
pub fn BackButton(prop: &ButtonProps) -> Html {
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
        overflow-x: visible;
        width: 100%;
        height: 9%;
    "};

    let icon_style = format! {"
        width: 100%;
        height: 100%;
        position: absolute;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
    "};

    html! {
        <div {style} {onmouseover} {onmouseout}>
            <div class = "card" style = {icon_style}>
                <BackIcon theme = { prop.theme() }/>

            </div>
            {
                if *mouse_over {
                    html! { <HoverNameDisp name = { "back" }  theme = { prop.theme() } /> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

#[function_component]
pub fn HomeButton(prop: &ButtonProps) -> Html {
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
        overflow-x: visible;
        width: 100%;
        height: 9%;
    "};

    let icon_style = format! {"
        width: 100%;
        height: 100%;
        position: absolute;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;    
    "};

    html! {
        <div {style} {onmouseover} {onmouseout}>
            <div class = "card" style = {icon_style}>
                <HomeIcon theme = { prop.theme() }/>

            </div>
            {
                if *mouse_over {
                    html! { <HoverNameDisp name = { "home" }  theme = { prop.theme() } /> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

#[function_component]
pub fn SettingsButton(prop: &ButtonProps) -> Html {
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
        overflow-x: visible;
        width: 100%;
        height: 9%;
    "};

    let icon_style = format! {"
        width: 100%;
        height: 100%;
        position: absolute;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;    
    "};

    html! {
        <div {style} {onmouseover} {onmouseout}>
            <div class = "card" style = {icon_style}>
                <SettingsIcon theme = { prop.theme() }/>
            </div>
            {
                if *mouse_over {
                    html! { <HoverNameDisp name = { "settings" }  theme = { prop.theme() } /> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

#[function_component]
pub fn HistoryButton(prop: &ButtonProps) -> Html {
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
        overflow-x: visible;
        width: 100%;
        height: 9%;
    "};

    let icon_style = format! {"
        width: 100%;
        height: 100%;
        position: absolute;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;    
    "};

    html! {
        <div {style} {onmouseover} {onmouseout}>
            <div class = "card" style = {icon_style}>
                <HistoryIcon theme = { prop.theme() }/>

            </div>
            {
                if *mouse_over {
                    html! { <HoverNameDisp name = { "history" }  theme = { prop.theme() } /> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

#[function_component]
pub fn BookmarksButton(prop: &ButtonProps) -> Html {
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
        overflow-x: visible;
        width: 100%;
        height: 9%;
    "};

    let icon_style = format! {"
        width: 100%;
        height: 100%;
        position: absolute;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;    
    "};

    html! {
        <div {style} {onmouseover} {onmouseout}>
            <div class = "card" style = {icon_style}>
                <BookmarksIcon />

            </div>
            {
                if *mouse_over {
                    html! { <HoverNameDisp name = { "bookmarks" }  theme = { prop.theme() } /> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

#[function_component]
pub fn CloudButton(prop: &ButtonProps) -> Html {
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
        overflow-x: visible;
        width: 100%;
        height: 9%;
    "};

    let icon_style = format! {"
        width: 100%;
        height: 100%;
        position: absolute;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;    
    "};

    html! {
        <div {style} {onmouseover} {onmouseout}>
            <div class = "card" style = {icon_style}>
                <CloudStorageIcon />

            </div>
            {
                if *mouse_over {
                    html! { <HoverNameDisp name = { "cloud" }  theme = { prop.theme() } /> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

#[function_component]
pub fn LockedButton(prop: &ButtonProps) -> Html {
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
        overflow-x: visible;
        width: 100%;
        height: 9%;
    "};

    let icon_style = format! {"
        width: 100%;
        height: 100%;
        position: absolute;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;    
    "};

    html! {
        <div {style} {onmouseover} {onmouseout}>
            <div class = "card" style = {icon_style}>
                <LockedIcon />

            </div>
            {
                if *mouse_over {
                    html! { <HoverNameDisp name = { "locked" }  theme = { prop.theme() } /> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
