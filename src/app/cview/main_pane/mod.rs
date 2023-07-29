mod dsc;
mod rows;
mod space;

use backend::Pitou;
use dsc::*;
// use gloo::console::log;
use crate::app::{invoke, LoadingDisplay, PitouArg, Theme};
use rows::*;
use serde_wasm_bindgen::{from_value, to_value};
use space::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

macro_rules! do_nothing {
    () => {
        ()
    };
}

#[derive(PartialEq, Properties)]
pub struct MainPaneProps {
    pub pitou: Option<Pitou>,
    pub theme: Theme,
    pub updatedirectory: Callback<Pitou>,
}

#[derive(Clone)]
struct MainPaneState {
    children: Vec<Pitou>,
    selected: Vec<bool>,
    len_selected: usize,
}

impl MainPaneState {
    fn new(children: Vec<Pitou>) -> Self {
        let selected = vec![false; children.len()];
        let len_selected = 0;

        Self {
            children,
            selected,
            len_selected,
        }
    }

    fn len(&self) -> usize {
        self.children.len()
    }

    fn children(&self) -> &Vec<Pitou> {
        &self.children
    }

    fn pitou(&self, idx: usize) -> Pitou {
        self.children()[idx].clone()
    }

    fn toggle(&mut self, idx: usize) {
        self.selected[idx] = !self.selected[idx];
        if self.selected[idx] {
            self.len_selected += 1;
        } else {
            self.len_selected -= 1;
        }
    }

    fn selected(&self, idx: usize) -> bool {
        self.selected[idx]
    }

    fn toggle_all(&mut self) {
        if self.len_selected == self.selected.len() {
            self.selected.iter_mut().for_each(|f| *f = false);
            self.len_selected = 0;
        } else {
            self.selected.iter_mut().for_each(|f| *f = true);
            self.len_selected = self.selected.len();
        }
    }

    fn all_checked(&self) -> bool {
        self.len_selected == self.selected.len()
    }
}

#[function_component]
pub fn MainPane(prop: &MainPaneProps) -> Html {
    let children: UseStateHandle<Option<MainPaneState>> = use_state(|| None);

    {
        let children = children.clone();
        let directory = prop.pitou.clone();

        spawn_local(async move {
            if let Some(directory) = &directory {
                let arg = to_value(&PitouArg { pitou: &directory }).unwrap();
                let val = invoke("children", arg).await;
                let values = from_value::<Vec<Pitou>>(val)
                    .expect("couldn't convert output to a vec of pitou's");

                let new_state = MainPaneState::new(values);

                match (*children).as_ref() {
                    Some(old_state) => {
                        if new_state.children() != old_state.children() {
                            crate::data::reset_selected();
                            children.set(Some(new_state));
                        }
                    }
                    None => children.set(Some(new_state)),
                }
            }
        })
    }

    let onclick = { move |_| do_nothing!() };

    let toggleselect = {
        let children = children.clone();

        move |idx| {
            let mut new_state = (&*children).clone();
            new_state.as_mut().map(|v| v.toggle(idx));
            crate::data::update_selected(new_state.as_ref().map(|v| {
                v.children()
                    .iter()
                    .enumerate()
                    .filter(|&(idx, _)| v.selected(idx))
                    .map(|(_, pitou)| pitou.clone())
            }));
            children.set(new_state)
        }
    };

    let toggleselectall = {
        let children = children.clone();

        move |_| match &*children {
            Some(inner) => {
                let mut inner = inner.clone();
                inner.toggle_all();
                children.set(Some(inner));
            }
            None => do_nothing!(),
        }
    };

    let theme = prop.theme;
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

    let checked = (&*children)
        .as_ref()
        .map(|val| val.all_checked())
        .unwrap_or_default();

    let content = if let Some(state) = &*children {
        let entries = (0..state.len()).map(|idx| (idx, state.pitou(idx), prop.updatedirectory.clone(), state.selected(idx), toggleselect.clone())).map(|(idx, pitou, updatedirectory, selected, toggleselect)| html! { <Row {idx} {selected} {pitou} {theme} {toggleselect} {updatedirectory} /> }).collect::<Html>();

        html! {
            <div style = {inner_style}>
                    { entries }
                <FreeArea />
            </div>
        }
    } else {
        html! { <LoadingScreen /> }
    };

    html! {
        <div {style} {onclick}>
        <RowDescriptor {toggleselectall} {checked} {theme}/>
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
