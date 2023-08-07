mod dsc;
mod rows;
mod space;

use backend::Pitou;
use dsc::*;
// use gloo::console::log;
use crate::app::{LoadingDisplay, Theme};
use rows::*;
use space::*;
use yew::prelude::*;

use std::{cell::RefCell, rc::Rc};

macro_rules! do_nothing {
    () => {
        ()
    };
}

#[derive(PartialEq, Clone)]
pub struct Selected {
    selected: Rc<RefCell<(Vec<bool>, usize)>>,
}

impl Selected {
    fn new(len: usize) -> Self {
        let selected = Rc::new(RefCell::new((vec![false; len], 0)));

        Self { selected }
    }

    fn toggle(&self, idx: usize) {
        let mut borrow = self.selected.borrow_mut();
        borrow.0[idx] = !borrow.0[idx];
        if borrow.0[idx] {
            borrow.1 += 1
        } else {
            borrow.1 -= 1
        }
    }

    pub fn all_checked(&self) -> bool {
        let borrow = self.selected.borrow();
        borrow.1 == borrow.0.len()
    }

    fn toggle_all(&self) {
        let mut borrow = self.selected.borrow_mut();
        if borrow.1 == borrow.0.len() {
            borrow.0.iter_mut().for_each(|v| *v = false);
            borrow.1 = 0;
        } else {
            borrow.0.iter_mut().for_each(|v| *v = true);
            borrow.1 = borrow.0.len();
        }
    }

    fn idx(&self, idx: usize) -> bool {
        self.selected.borrow().0[idx]
    }
}

#[derive(PartialEq, Properties)]
pub struct MainPaneProps {
    pub children: Option<Vec<Pitou>>,
    pub updatedirectory: Callback<Pitou>,
}

#[function_component]
pub fn MainPane(prop: &MainPaneProps) -> Html {
    let theme = use_context::<Theme>().unwrap();
    let selected = prop
        .children
        .as_ref()
        .map(|children| Selected::new(children.len()));

    let onclick = { move |_| do_nothing!() };

    let toggleselect = {
        let selected = selected.clone();
        let children = prop.children.clone();
        move |idx| {
            selected
                .as_ref()
                .map(|selected| selected.toggle(idx))
                .unwrap_or_default();
            crate::data::update_selected(children.as_ref().map(|children| {
                children
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| selected.as_ref().map(|s| s.idx(*idx)).unwrap_or(false))
                    .map(|(_, v)| v.clone())
            }));
        }
    };

    let toggleselectall = {
        let selected = selected.clone();
        move |_| {
            selected
                .as_ref()
                .map(|selected| selected.toggle_all())
                .unwrap_or_default()
        }
    };

    let background_color = theme.background2();
    let spare_color = theme.spare();

    let style = format! {"
    position: absolute;
    border: 1px solid {spare_color};
    margin: 1px 1px 1px 1px;
    top: 10%;
    bottom: 4%;
    left: 25%;
    right: 0%;"};

    let inner_style = format! {"
    position: absolute;
    display: flex;
    flex-direction: column;
    gap: 0;
    
    align-items: center;
    overflow: auto;
    background-color: {background_color};

    top: 5%;
    height: 95%;
    width: 100%;
    "};

    let content = prop
        .children
        .as_ref()
        .map(|children| children.iter()
            .enumerate()
            .map(|(idx, pitou)| (idx, pitou.clone(), prop.updatedirectory.clone(), toggleselect.clone()))
            .map(|(idx, pitou, updatedirectory, toggleselect)| html! { <Row {idx} {pitou} {toggleselect} {updatedirectory} /> })
            .collect::<Html>())
        .map(|entries| html! {
                <div style = {inner_style}>
                    { entries }
                    <FreeArea />
                </div>
            })
        .unwrap_or(html! { <LoadingScreen /> });

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
