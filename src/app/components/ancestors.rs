use crate::app::ApplicationContext;
use backend::File;
use std::{path::PathBuf, rc::Rc};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct FolderDir {
    pub folder: Option<File>,
    pub updatedirectory: Callback<PathBuf>,
}
#[derive(PartialEq, Properties)]
pub struct FolderDir2 {
    pub folder: File,
    pub updatedirectory: Callback<PathBuf>,
}

#[derive(PartialEq, Properties)]
pub struct MainAncestorTabsProps {
    pub folder: Option<Rc<PathBuf>>,
    pub updatedirectory: Callback<PathBuf>,
}

#[function_component]
pub fn AncestorsTabs(prop: &MainAncestorTabsProps) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context().unwrap();
    let isinput = use_state_eq(|| false);

    let onclick = {
        let isinput = isinput.clone();
        move |_| isinput.set(!*isinput)
    };

    let finished = {
        let isinput = isinput.clone();
        let updatedirectory = prop.updatedirectory.clone();
        move |ns| {
            isinput.set(false);
            updatedirectory.emit(PathBuf::from(ns))
        }
    };

    let foreground_color = theme.foreground1();
    let background_color_2 = theme.background2();
    let spare_color = theme.spare();
    let outer_size = sizes.ancestorspane();

    let inner_size = sizes.ancestorsbar();

    let outer_style = format! {"
    display: flex;
    align-items: center;
    border: 1px solid {spare_color};
    box-sizing: border-box;
    background-color: {background_color_2};
    {outer_size}"};

    let inner_style = format! {"
    color: {foreground_color};
    {inner_size}
    display: flex;
    flex-direction: row-reverse;
    align-items: center;
    column-gap: 2px;
    justify-content: left;
    overflow: hidden;
    margin-left: 1%;
    margin-right: 1%;
    "};

    let entries = if let Some(path) = prop.folder.as_ref() {
        if *isinput {
            html! { <EnterPath folder = {path.clone()} {finished}/> }
        } else {
            html! { <Ancestors folder = {path.clone()} updatedirectory = {prop.updatedirectory.clone()}/> }
        }
    } else {
        html! {}
    };

    html! {
        <div style = {outer_style}>
            <div style = {inner_style} {onclick}>
                {entries}
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct AncestorsProps {
    folder: Rc<PathBuf>,
    updatedirectory: Callback<PathBuf>,
}

#[function_component]
pub fn Ancestors(prop: &AncestorsProps) -> Html {
    prop.folder
        .ancestors()
        .map(|path| (Rc::new(PathBuf::from(path)), prop.updatedirectory.clone()))
        .map(|(folder, updatedirectory)| html! { <Ancestor {folder} {updatedirectory} /> })
        .collect::<Html>()
}

#[derive(PartialEq, Properties)]
struct EnterPathProps {
    folder: Rc<PathBuf>,
    finished: Callback<String>,
}

#[function_component]
fn EnterPath(prop: &EnterPathProps) -> Html {
    let ApplicationContext {
        theme: _,
        sizes: _,
        settings: _,
    } = use_context().unwrap();
    let input_ref = use_node_ref();
    let onclick = move |e: MouseEvent| e.stop_propagation();

    let style = format! {"
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    width: 100%;
    height: 80%
    box-sizing: border-box;
    "};

    let inner_style = format! {"
    box-sizing: border-box;
    width: 100%;
    height: 100%;
    "};

    let onsubmit = {
        let finished = prop.finished.clone();
        let input_ref = input_ref.clone();

        move |e: SubmitEvent| {
            e.prevent_default();
            let entered = input_ref.cast::<HtmlInputElement>().unwrap().value();
            finished.emit(entered)
        }
    };

    let value = prop.folder.display().to_string();

    html! {
        <form {style} {onsubmit}>
            <input type = "text" ref = {input_ref} {value} {onclick} style = {inner_style}/>
        </form>
    }
}

#[derive(PartialEq, Properties)]
struct AncestorProps {
    folder: Rc<PathBuf>,
    updatedirectory: Callback<PathBuf>,
}

#[function_component]
fn Ancestor(prop: &AncestorProps) -> Html {
    let ApplicationContext {
        theme,
        sizes: _,
        settings: _,
    } = use_context().unwrap();

    let onclick = move |e: MouseEvent| e.stop_propagation();

    let background_color = theme.background1();
    let border_color = theme.spare();

    let style = format! {"
    height: 80%;
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 0;
    width: auto;
    border: 1px solid {border_color};
    box-sizing: border-box;
    background-color: {background_color};
    font-size: 80%;"};

    html! {
        <div {style} {onclick}>
            <TabName folder = { prop.folder.clone() } updatedirectory = { prop.updatedirectory.clone() } />
            <ChooseDir folder = {prop.folder.clone()} updatedirectory = { prop.updatedirectory.clone() } />
        </div>
    }
}

#[function_component]
fn ChooseDir(prop: &AncestorProps) -> Html {
    let ApplicationContext {
        theme: _,
        sizes,
        settings,
    } = use_context().unwrap();
    let children = use_state(|| None);

    {
        let children = children.clone();
        let folder = prop.folder.clone();
        use_effect_with_deps(
            move |folder| {
                let folder = folder.clone();
                spawn_local(async move {
                    let res = crate::app::tasks::children_dirs(&folder, settings.filter).await;
                    children.set(Some(res));
                });
            },
            folder,
        );
    }

    let options = if let Some(children) = &*children {
        let msg = if children.len() == 0 {
            String::from("no folders present")
        } else if children.len() == 1 {
            String::from("click folder to open")
        } else {
            String::from("select folder to open")
        };

        Some(html! { <option selected = {true} disabled = {true}>{msg}</option> })
            .into_iter()
            .chain(
                children
                    .into_iter()
                    .map(|pitou| html! { <option selected = {false}>{pitou.name()}</option> }),
            )
            .collect::<Html>()
    } else {
        html! {}
    };

    let width = sizes.choosedir();

    let style = format! {"
    {width}
    height: 100%;
    display: flex;
    align-items: center;
    box-sizing: border-box;
    cursor: pointer;
    "};

    let select_style = format! {"
    width: 100%;
    height: 100%;
    box-sizing: border-box;
    "};

    let onchange = {
        let children = children.clone();
        let onselect = prop.updatedirectory.clone();
        move |e: Event| {
            e.prevent_default();
            let idx = e
                .target_dyn_into::<HtmlSelectElement>()
                .unwrap()
                .selected_index()
                - 1;
            if let Some(children) = &*children {
                if idx >= 0 && idx < children.len() as i32 {
                    onselect.emit(children[idx as usize].path().clone());
                }
            }
        }
    };

    html! {
        <div {style}>
            <select style = {select_style} {onchange}>
                {options}
            </select>
        </div>
    }
}

#[function_component]
fn TabName(prop: &AncestorProps) -> Html {
    let style = format! {"
    width: auto;
    height: 100%;
    padding-left: 4px;
    padding-right: 4px;
    min-width: 15px;
    display: flex;
    align-items: center;
    justify-content: center;
    "};

    let updatedirectory = {
        let folder = prop.folder.clone();
        let updatedirectory = prop.updatedirectory.clone();

        move |_| updatedirectory.emit((&*folder).clone())
    };

    let name = File::name_of(&prop.folder).to_owned();

    html! {
        <div {style} onclick = { updatedirectory }>
            <span>{ name }</span>
        </div>
    }
}
