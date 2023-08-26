mod dsc;
mod rows;
mod space;

use crate::app::{ApplicationContext, LoadingDisplay};
use backend::Pitou;
use dsc::*;
use rows::*;
use space::*;
use yew::prelude::*;

#[derive(PartialEq)]
struct Inner {
    allitems: Vec<Pitou>,
    selected: Vec<bool>,
    len_selt: usize,
}

impl Inner {
    fn allitems(&self) -> &Vec<Pitou> {
        &self.allitems
    }

    fn selected(&self) -> &Vec<bool> {
        &self.selected
    }

    fn len_selt(&self) -> usize {
        self.len_selt
    }

    fn allitems_mut(&mut self) -> &mut Vec<Pitou> {
        &mut self.allitems
    }

    fn selected_mut(&mut self) -> &mut Vec<bool> {
        &mut self.selected
    }

    fn len_selt_mut(&mut self) -> &mut usize {
        &mut self.len_selt
    }
}

#[derive(PartialEq)]
struct Selections {
    inner: std::rc::Rc<std::cell::RefCell<Inner>>,
}

impl Selections {
    fn init(items: &Vec<Pitou>) -> Self {
        let allitems = items.clone();
        let selected = vec![false; items.len()];
        let len_selt = 0;

        Self {
            inner: std::rc::Rc::new(std::cell::RefCell::new(Inner {
                allitems,
                selected,
                len_selt,
            })),
        }
    }

    fn fully_selected(&self) -> bool {
        let inner = self.inner.borrow();
        inner.len_selt() == inner.allitems.len()
    }

    fn toggle(&self, idx: usize) {
        let mut inner = self.inner.borrow_mut();

        #[cfg(debug_assertions)]
        if idx > inner.allitems().len() {
            panic!("unexpected occurrence, index of selections is out of bounds")
        }

        inner.selected_mut()[idx] = !inner.selected_mut()[idx];

        if inner.selected_mut()[idx] {
            *inner.len_selt_mut() += 1;
        } else {
            *inner.len_selt_mut() -= 1;
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct MainPaneProps {
    pub children: Option<Vec<Pitou>>,
    pub updatedirectory: Callback<Pitou>,
}

#[function_component]
pub fn MainPane(prop: &MainPaneProps) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context::<ApplicationContext>().unwrap();
    let force_update = use_force_update();

    //let selections = std::rc::Rc::new(prop.children.as_ref().map(|c| vec![false; children]));
    
    let onclick = move |e: MouseEvent| e.prevent_default();

    let toggleselect = {
        move |_idx: usize| ()
    };

    let toggleselectall = {
        move |_| ()
    };

    let background_color = theme.background2();
    let spare_color = theme.spare();
    let size = sizes.mainpane();

    let style = format! {"
    position: relative;
    border: 1px solid {spare_color};
    box-sizing: border-box;
    {size}"};

    let top = sizes.dsc().height;
    let height = sizes.mainpane().height - top;

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

    let content = prop
        .children
        .as_ref()
        .map(|children| children.iter()
            .enumerate()
            .map(|(idx, pitou)| (idx, pitou.clone(), prop.updatedirectory.clone(), toggleselect.clone(), false))
            .map(|(idx, pitou, updatedirectory, toggleselect, selected)| html! { <Row {idx} {pitou} {toggleselect} {updatedirectory} {selected} /> })
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
            <RowDescriptor {toggleselectall} selected = { false }/>
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
