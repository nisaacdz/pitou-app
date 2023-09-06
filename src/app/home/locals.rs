use crate::{app::ApplicationContext, background_color};
use backend::File;
use yew::prelude::*;

#[derive(PartialEq)]
pub enum LocalFolderKind {
    Desktop(File),
    Downloads(File),
    Documents(File),
    Audios(File),
    Videos(File),
    Pictures(File),
}

impl LocalFolderKind {
    fn pitou(&self) -> &File {
        match self {
            LocalFolderKind::Desktop(pitou) => pitou,
            LocalFolderKind::Downloads(pitou) => pitou,
            LocalFolderKind::Documents(pitou) => pitou,
            LocalFolderKind::Audios(pitou) => pitou,
            LocalFolderKind::Videos(pitou) => pitou,
            LocalFolderKind::Pictures(pitou) => pitou,
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct LocalFolderProps {
    pub kind: LocalFolderKind,
}

#[function_component]
pub fn LocalCmp(prop: &LocalFolderProps) -> Html {
    let ApplicationContext {
        theme,
        settings: _,
        sizes,
    } = use_context().unwrap();
    let ishovered = use_state_eq(|| false);

    let onmouseover = {
        let ishovered = ishovered.clone();
        move |_| ishovered.set(true)
    };

    let onmouseout = {
        let ishovered = ishovered.clone();
        move |_| ishovered.set(false)
    };

    let size = sizes.home_local();
    let text_color = theme.foreground1();

    let style = format! {"
    align-items: center;
    display: flex;
    gap: 5px;
    cursor: pointer;
    padding-left: 10px;
    color: {text_color};
    {size}
    {}
    ", background_color!(*ishovered, theme.background1())};

    let img_style = format! {"
    display: flex;
    align-items: center;
    height: 100%;
    "};

    let img = match prop.kind {
        LocalFolderKind::Audios(_) => {
            html! { <img width = "60px" height = "60px" src="./public/icons/home/audios.png"/> }
        }
        LocalFolderKind::Videos(_) => {
            html! { <img width = "60px" height = "60px" src="./public/icons/home/videos.png"/> }
        }
        LocalFolderKind::Downloads(_) => {
            html! { <img width = "60px" height = "60px" src="./public/icons/home/downloads.png"/> }
        }
        LocalFolderKind::Documents(_) => {
            html! { <img width = "60px" height = "60px" src="./public/icons/home/documents.png"/> }
        }
        LocalFolderKind::Pictures(_) => {
            html! { <img width = "60px" height = "60px" src="./public/icons/home/pictures.png"/> }
        }
        LocalFolderKind::Desktop(_) => {
            html! { <img width = "60px" height = "60px" src="./public/icons/home/desktop.png"/> }
        }
    };

    let name = html! { <span>{ prop.kind.pitou().name() }</span> };

    html! {
        <div {style} {onmouseover} {onmouseout}>
            <div style = {img_style}> {img} </div>
            {name}
        </div>
    }
}
