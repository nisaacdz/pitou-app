use crate::app::ApplicationContext;
use backend::Drive;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

mod locals;

use locals::{LocalCmp, LocalFolderKind};

#[function_component]
pub fn HomeView() -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context().unwrap();
    let background = theme.background2();
    let foreground = theme.spare();
    let size = sizes.pane();

    let style = format! {"
    padding: 20px;
    display: flex;
    flex-direction: column;
    align-items: center;
    {size}
    background-color: {background};
    border: 1px solid {foreground};
    box-sizing: border-box;
    "};
    html! {
        <div {style}>
            <LocalFolders />
            <DrivesPane />
        </div>
    }
}

#[function_component]
fn LocalFolders() -> Html {
    let ApplicationContext {
        theme: _,
        sizes: _,
        settings: _,
    } = use_context().unwrap();
    use backend::Locals;

    let locals = use_state(|| None);

    {
        let locals = locals.clone();
        use_effect_with_deps(
            |_| {
                spawn_local(async move {
                    let res = crate::app::tasks::locals().await;
                    locals.set(Some(res));
                });
            },
            (),
        )
    }

    let style = format! {"
    display: flex;
    flex-wrap: wrap;
    width: 80%;
    height: 300px;
    gap: 45px;
    "};

    let entries = locals
        .as_ref()
        .map(
            |Locals {
                 documents,
                 videos,
                 audios,
                 downloads,
                 desktop,
                 pictures,
             }| {
                let documents =
                    html! { <LocalCmp kind = {LocalFolderKind::Documents(documents.clone())}/> };
                let videos = html! { <LocalCmp kind = {LocalFolderKind::Videos(videos.clone())}/> };
                let audios = html! { <LocalCmp kind = {LocalFolderKind::Audios(audios.clone())}/> };
                let downloads =
                    html! { <LocalCmp kind = {LocalFolderKind::Downloads(downloads.clone())}/> };
                let desktop =
                    html! { <LocalCmp kind = {LocalFolderKind::Desktop(desktop.clone())}/> };
                let pictures =
                    html! { <LocalCmp kind = {LocalFolderKind::Pictures(pictures.clone())}/> };

                Some(documents)
                    .into_iter()
                    .chain(Some(videos))
                    .chain(Some(audios))
                    .chain(Some(downloads))
                    .chain(Some(desktop))
                    .chain(Some(pictures))
                    .collect::<Html>()
            },
        )
        .unwrap_or_default();

    html! {
        <div {style}>
        {
            entries
        }
        </div>
    }
}

#[function_component]
fn DrivesPane() -> Html {
    let ApplicationContext {
        theme: _,
        sizes: _,
        settings: _,
    } = use_context().unwrap();
    let drives = use_state(|| None);

    {
        let drives = drives.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let newdrives = crate::app::tasks::drives().await;
                    drives.set(Some(newdrives))
                })
            },
            (),
        )
    }

    let style = format! {"
    width: 100%;
    height: 300px;
    "};

    let inner_style = format! {"
    display: flex;
    gap: 30px;
    flex-wrap: wrap;
    margin: 20px;
    align-items: center;
    width: 100%;
    "};

    let entries = drives
        .as_ref()
        .map(|drives| {
            drives
                .into_iter()
                .map(|drive| html! { <DriveCmp drive = {drive.clone()}/>})
                .collect::<Html>()
        })
        .unwrap_or_default();

    html! {
        <details {style} open = {true}>
            <summary>{"Devices and drives"}</summary>
            <div style = {inner_style} class = "test_me_rm">
                {entries}
            </div>
        </details>
    }
}

#[derive(PartialEq, Properties)]
struct DriveProp {
    drive: Drive,
}

#[function_component]
fn DriveCmp(prop: &DriveProp) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context().unwrap();
    let totalsize = sizes.diskcmp();

    let background = theme.background1();
    let foreground = theme.foreground1();

    let style = format! {"
    background-color: {background};
    color: {foreground};
    display: flex;
    flex-direction: column;
    gap: 0;
    {totalsize}
    "};

    let inner_style = format! {"
    display: flex;
    gap: 0;
    "};

    let meter_len = sizes.diskmeter();
    let span_style = format! {"
    padding-left: 5px;
    padding-right: 5px;
    "};

    let meter_style = format! {"
    {meter_len}
    "};

    let inner_inner_style = format! {"
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: 0;
    "};

    use backend::DriveKind;

    let img = match prop.drive.kind() {
        DriveKind::HDD => html! { <img src="./public/icons/home/hdd.png"/> },
        DriveKind::SSD => html! { <img src="./public/icons/home/ssd.png"/> },
        DriveKind::Unkown => html! { <img src="./public/icons/home/flash.png"/> },
    };

    let dsc = format! {"{} {}", prop.drive.mount_point().display(), prop.drive.name() };

    let progress_txt = {
        let free = prop.drive.free_space() as f64 / f64::powi(1024f64, 3);
        let total = prop.drive.total_space() as f64 / f64::powi(1024f64, 3);
        format! {"{:.0} GB free of {:.0} GB", free, total}
    };

    let max = format! {"{}", prop.drive.total_space()};
    let value = format! {"{}", prop.drive.total_space() - prop.drive.free_space()};

    html! {
        <div {style}>
            <span style = {span_style}>{dsc}</span>
            <div style = {inner_style}>
                {img}
                <div style = {inner_inner_style}>
                    <meter style = {meter_style} min = "0" {max} {value}></meter>
                    <span>{progress_txt}</span>
                </div>
            </div>
        </div>
    }
}
