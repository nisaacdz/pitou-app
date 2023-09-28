use crate::app::{AppMenu, ApplicationContext, ApplicationData};
use backend::File;
use std::rc::Rc;
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
    fn file(&self) -> &File {
        match self {
            LocalFolderKind::Desktop(f) => f,
            LocalFolderKind::Downloads(f) => f,
            LocalFolderKind::Documents(f) => f,
            LocalFolderKind::Audios(f) => f,
            LocalFolderKind::Videos(f) => f,
            LocalFolderKind::Pictures(f) => f,
        }
    }
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
    pub updateview: Callback<AppMenu>,
}

#[function_component]
pub fn LocalCmp(prop: &LocalFolderProps) -> Html {
    let ApplicationContext {
        theme,
        settings: _,
        sizes,
    } = use_context().unwrap();

    let cdata = use_context::<ApplicationData>().unwrap();

    let size = sizes.home_local();
    let text_color = theme.foreground1();

    let onclick = {
        let updateview = prop.updateview.clone();
        let dir = Rc::new(prop.kind.file().path().clone());
        let cdata = cdata.clone();
        move |e: MouseEvent| {
            e.stop_propagation();
            cdata.update_directory(dir.clone());
            updateview.emit(AppMenu::Explorer)
        }
    };

    let style = format! {"
    align-items: center;
    display: flex;
    gap: 5px;
    cursor: pointer;
    padding-left: 10px;
    color: {text_color};
    {size}
    "};

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
        <div {style} {onclick} class = "local">
            <div style = {img_style}> {img} </div>
            {name}
        </div>
    }
}
