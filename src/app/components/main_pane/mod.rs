mod dsc;
mod rows;
mod space;

use crate::app::{ApplicationContext, LoadingDisplay};
use backend::{File, PitouType};
use dsc::*;
use rows::*;
use space::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use std::{cell::RefCell, collections::HashSet, rc::Rc, path::PathBuf};
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
        settings,
    } = use_context().unwrap();

    let selections = use_state(|| {
        let vals = Rc::new(RefCell::new(HashSet::new()));
        crate::app::data::init_selections(vals.clone());
        vals
    });

    {
        let selections = selections.clone();
        let children = prop.children.clone();
        use_effect_with_deps(
            move |_| {
                let newselections = (&*selections).clone();
                let mut borrow = newselections.borrow_mut();
                borrow.clear();
                borrow.extend(crate::app::data::get_persistent());
                std::mem::drop(borrow);
                selections.set(newselections);
            },
            children,
        );
    }

    let onclick = move |e: MouseEvent| e.prevent_default();

    let toggleselect = {
        let selections = selections.clone();
        let children = prop.children.clone();
        move |idx: usize| {
            if let Some(children) = &children {
                let newselections = (&*selections).clone();
                let mut borrow = newselections.borrow_mut();
                if !borrow.remove(&children[idx]) {
                    borrow.insert(children[idx].clone());
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
                        spawn_local(async move {
                            crate::app::tasks::open(file.path()).await
                        })
                    },
                    PitouType::Link => {
                        let file = file.clone();
                        let updatedirectory = updatedirectory.clone();
                        spawn_local(async move {
                            if let Some(file) = crate::app::tasks::read_link(file.path()).await {
                                let parent_dir = PathBuf::from(file.path().parent().unwrap_or(std::path::Path::new("")));
                                if let Some(parent_file) = crate::app::tasks::retrieve(&parent_dir).await {
                                    crate::app::data::persist(file.clone());
                                    updatedirectory.emit(parent_file);
                                }
                            }
                        })
                    },
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
    background-color: {background_color};
    position: relative;
    border: 1px solid {spare_color};
    box-sizing: border-box;
    {size}"};

    let top = sizes.dsc().height;
    let height = size.height - top;

    let inner_style = format! {"
    position: absolute;
    display: flex;
    flex-direction: column;
    gap: 0;
    
    align-items: center;
    overflow-y: auto;
    overflow-x: hidden;
    background-color: {background_color};

    top: {top}px;
    height: {height}px;
    width: 100%;
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
            .filter(|(_, file)| settings.filter.include(file))
            .map(|(idx, file)| (idx, file.clone(), ondbclick.clone(), toggleselect.clone(), selections.borrow().contains(file)))
            .map(|(idx, file, ondbclick, toggleselect, selected)| html! { <Row {idx} {file} {toggleselect} {ondbclick} {selected}/> })
            .collect::<Html>())
        .map(|entries| html! {
            <div style = {inner_style}>
                { entries }
                <FreeArea />
            </div>
        })
        .unwrap_or(html! { <LoadingScreen /> });

    let selected = prop
        .children
        .as_ref()
        .map(|c| c.len() == selections.borrow().len())
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
