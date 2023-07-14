use yew::prelude::*;

use crate::{
    app::{DirIcon, PitouProps, RefreshIcon},
    background_color,
};

#[derive(Default)]
struct RowState {
    is_hovered: bool,
}

impl RowState {
    fn is_hovered(&self) -> bool {
        self.is_hovered
    }
}

#[function_component]
pub fn SidePane(prop: &PitouProps) -> Html {
    let entries = prop
        .entries()
        .into_iter()
        .map(|pitou| html! { <Row {pitou}/> })
        .collect::<Html>();

    let background_color = prop.theme().background2();
    let border_color = prop.theme().spare();

    let style = format! {"
        position: absolute;
        display: flex;
        flex-direction: column;
        gap: 0;
        top: 10%;
        bottom: 4%;
        align-items: center;
        overflow: auto;
        background-color: {background_color};
        overflow-anchor: none;
        border: 1px solid {border_color};
        margin: 1px 1px 1px 1px;
        left: 4%;
        width: 20%;
    "};

    let pitou = prop.pitou();

    html! {
        <div {style}>
            <TopOfParentDir pitou = { pitou.clone() }/>
            {entries}
        </div>
    }
}

#[function_component]
fn ParentDirName(prop: &PitouProps) -> Html {
    let spare_color = prop.theme().spare();
    let background_color = prop.theme().background2();

    let style = format! {"
        display: flex;
        flex-direction: row;
        gap: 0;
        height: calc(100% - 4px);
        width: 75%;
        padding-left: 5%;
        padding-right: 5%;
        border: 1px solid {spare_color};
        background-color: {background_color};
    "};

    let a = prop.pitou().path().parent().map(|p| p.file_name().unwrap_or_default());

    let parent_name = std::path::PathBuf::from(a.unwrap_or_default());


    html! {
        <div {style}>
        { parent_name.display() }
        </div>
        
    }
}

#[function_component]
fn TopOfParentDir(prop: &PitouProps) -> Html {
    let style = format! {"
        width: calc(100% - 4px);
        height: 5%;
        top: 0;
        display: flex;
        flex-direction: row;
        height: 5%;
        gap: 3px;
        position: relative;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
    "};

    let pitou = prop.pitou();
    html! {
        <div {style}>
            <ParentDirName pitou = { pitou.clone() }/>
            <RefreshButton pitou = { pitou.clone() }/>
        </div>
    }
}

#[function_component]
fn RefreshButton(prop: &PitouProps) -> Html {
    let onclick = |_| println!("");

    let spare_color = prop.theme().spare();
    let background_color = prop.theme().background1();

    let style = format! {"
        width: 10%;
        border: 1px solid {};
        border-radius: 10%;
        height: 70%;
        background-color: {background_color};
        ", spare_color};
    html! {
        <div class = "card" {style} {onclick}> <RefreshIcon theme = { *prop.theme() } /> </div>
    }
}

#[function_component]
pub fn Row(prop: &PitouProps) -> Html {
    let state: UseStateHandle<RowState> = use_state(RowState::default);

    let onmouseover = {
        let state: UseStateHandle<RowState> = state.clone();
        move |_| {
            state.set(RowState { is_hovered: true });
        }
    };

    let onmouseout = {
        let state = state.clone();
        move |_| {
            state.set(RowState { is_hovered: false });
        }
    };

    let style = format! {"
        display: flex;
        flex-direction: row;
        gap: 0;
        color: {};
        font-family: monospace;
        height: 10%;
        width: 100%;
        font-size: 100%;
        {}
        text-align: left;",
    prop.theme().foreground1(), background_color!(state.is_hovered(), prop.theme().background1()) };

    let pitou = prop.pitou();

    html! {
        <div {style} {onmouseover} {onmouseout}>
            <FileIcon pitou = { pitou.clone() } />
            <FileName pitou = { pitou.clone() } />
        </div>
    }
}

#[function_component]
fn FileIcon(_prop: &PitouProps) -> Html {
    let style = format! {"
        display: flex;
        align-items: center;
        left: 0%;
        width: 15%;
        height: 100%;
        justify-content: center;
    "};

    html! {
        <div {style}> <DirIcon/></div>
    }
}

#[function_component]
fn FileName(prop: &PitouProps) -> Html {
    let style = format! {"
        left: 15%;
        width: 80%;
        height: 100%;
        align-items: center;
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
    "};
    html! {
        <p {style}>
            { std::path::PathBuf::from(prop.pitou_file().name()).display() }
        </p>
    }
}
