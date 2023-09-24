mod dsc;
mod rows;

use crate::app::{ApplicationContext, ApplicationData, LoadingDisplay};
use backend::{File, PitouType};
use dsc::*;
use rows::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use std::{path::PathBuf, rc::Rc};
#[derive(PartialEq, Properties)]
pub struct MainPaneProps {
    pub children: Option<Rc<Vec<File>>>,
    pub updatedirectory: Callback<File>,
}

#[function_component]
pub fn MainPane(prop: &MainPaneProps) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context().unwrap();

    let cdata = use_context::<ApplicationData>().unwrap();

    let selections = use_state(|| cdata.selected_files());

    {
        let selections = selections.clone();
        let children = prop.children.clone();
        use_effect_with_deps(
            move |_| {
                let mut borrow = selections.borrow_mut();
                if borrow.len() > 0 {
                    borrow.clear();
                    drop(borrow);
                    selections.set((&*selections).clone())
                }
            },
            children,
        );
    }

    let onclick = move |e: MouseEvent| e.prevent_default();

    let onselect = {
        let selections = selections.clone();
        let children = prop.children.clone();
        move |(idx, append): (usize, bool)| {
            if let Some(children) = &children {
                let newselections = (*selections).clone();
                let mut borrow = newselections.borrow_mut();
                match append {
                    true => {
                        if borrow.contains(&children[idx]) {
                            borrow.remove(&children[idx]);
                        } else {
                            borrow.insert(children[idx].clone());
                        }
                    }
                    false => {
                        if borrow.contains(&children[idx]) && borrow.len() == 1 {
                            borrow.clear()
                        } else {
                            borrow.clear();
                            borrow.insert(children[idx].clone());
                        }
                    }
                }

                std::mem::drop(borrow);
                selections.set(newselections);
            }
        }
    };

    let ondbclick = {
        let updatedirectory = prop.updatedirectory.clone();
        let children = prop.children.clone();

        move |idx: usize| {
            if let Some(file) = children.as_ref().map(|v| &v[idx]) {
                match file.metadata().file_type() {
                    PitouType::Directory => {
                        updatedirectory.emit(file.clone());
                    }
                    PitouType::File => {
                        let file = file.clone();
                        spawn_local(async move { crate::app::tasks::open(file.path()).await })
                    }
                    PitouType::Link => {
                        let file = file.clone();
                        let updatedirectory = updatedirectory.clone();
                        spawn_local(async move {
                            if let Some(file) = crate::app::tasks::read_link(file.path()).await {
                                let parent_dir = PathBuf::from(
                                    file.path().parent().unwrap_or(std::path::Path::new("")),
                                );
                                if let Some(parent_file) =
                                    crate::app::tasks::retrieve(&parent_dir).await
                                {
                                    // TODO crate::app::data::persist(file.clone());
                                    updatedirectory.emit(parent_file);
                                }
                            }
                        })
                    }
                }
            }
        }
    };

    let toggleselectall = {
        let selections = selections.clone();
        let children = prop.children.clone();
        move |_| {
            if let Some(children) = &children {
                let newselections = (&*selections).clone();
                let mut borrow = newselections.borrow_mut();
                if borrow.len() == children.len() {
                    borrow.clear();
                } else {
                    children.iter().for_each(|child| {
                        borrow.insert(child.clone());
                    });
                }
                std::mem::drop(borrow);
                selections.set(newselections);
            }
        }
    };

    let background_color = theme.background2();
    let spare_color = theme.spare();
    let size = sizes.mainpane();

    let style = format! {"
    display: flex;
    flex-direction: column;
    gap: 0;
    background-color: {background_color};
    position: relative;
    border: 1px solid {spare_color};
    box-sizing: border-box;
    {size}"};

    let top = sizes.dsc().height;
    let height = size.height - top;

    let inner_style = format! {"
    display: flex;
    flex-direction: column;
    gap: 0;
    
    align-items: center;
    overflow-y: auto;
    overflow-x: hidden;
    background-color: {background_color};

    height: {height}px;
    width: 100%;
    "};

    let mut free_area_size = sizes.row();
    free_area_size.value *= 3;
    let free_area_style = format! {"
    {free_area_size}
    width: 100px;
    "};

    // if let Some(_) = &*metadata {
    //     gloo::console::log!("metadata exists");
    // } else {
    //     gloo::console::log!("no metadata exists");
    // }

    let content = prop
        .children
        .as_ref()
        .map(|children| children.iter()
            .enumerate()
            .map(|(idx, file)| (idx, file.clone(), ondbclick.clone(), onselect.clone(), selections.borrow().contains(file)))
            .map(|(idx, file, ondbclick, onselect, selected)| html! { <Row {idx} {file} {onselect} {ondbclick} {selected}/> })
            .collect::<Html>())
        .map(|entries| html! {
            <div style = {inner_style}>
                { entries }
                <div style = {free_area_style}></div>
            </div>
        })
        .unwrap_or(html! { <LoadingScreen /> });

    let selected = prop
        .children
        .as_ref()
        .map(|c| c.len() <= selections.borrow().len())
        .unwrap_or(true);

    html! {
        <div {style} {onclick}>
            <RowDescriptor {toggleselectall} {selected}/>
            { content }
        </div>
    }
}

#[function_component]
fn LoadingScreen() -> Html {
    let style = format! {"
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    "};

    html! {
        <div {style}>
            <LoadingDisplay />
        </div>
    }
}
