mod dsc;
mod rows;

use backend::Pitou;
use dsc::*;
use gloo::console::log;
use rows::*;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_futures::spawn_local;

use crate::app::{invoke, PitouArg, PitouProps};
use yew::prelude::*;

macro_rules! do_nothing {
    () => {
        ()
    };
}

#[derive(Clone)]
struct Selected {
    vec: Vec<bool>,
    selected_len: usize,
}

impl Selected {
    fn new(size: usize) -> Self {
        let vec = vec![false; size];
        let selected_len = 0;

        Self { vec, selected_len }
    }
    fn toggle(&mut self, idx: usize) {
        self.vec[idx] = !self.vec[idx];
        if self.vec[idx] {
            self.selected_len += 1;
        } else {
            self.selected_len -= 1;
        }
    }

    fn toggle_all(&mut self) {
        if self.selected_len == self.vec.len() {
            self.vec.iter_mut().for_each(|f| *f = false);
            self.selected_len = 0;
        } else {
            self.vec.iter_mut().for_each(|f| *f = true);
            self.selected_len = self.vec.len();
        }
    }

    fn checked(&self) -> bool {
        self.selected_len == self.vec.len()
    }
}

impl std::ops::Index<usize> for Selected {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

#[function_component]
pub fn MainPane(prop: &PitouProps) -> Html {
    let children = use_state(|| None);

    {
        let children = children.clone();
        let arg = to_value(&PitouArg {
            pitou: prop.pitou().clone(),
        })
        .unwrap();

        use_effect_with_deps(
            |_| {
                spawn_local(async move {
                    log!("spawning from main_pane");
                    let val = invoke("children", arg).await;
                    let values = from_value::<Vec<Pitou>>(val)
                        .expect("couldn't convert output to a vec of pitou's");

                    let selected = Selected::new(values.len());

                    children.set(Some((values, selected)));
                })
            },
            (),
        )
    }

    let toggle_select = {
        let children = children.clone();

        move |idx: usize| match &*children {
            Some(inner) => {
                let mut inner = inner.clone();
                inner.1.toggle(idx);

                children.set(Some(inner));
            }
            None => do_nothing!(),
        }
    };

    //TODO
    let onclick = { move |_| do_nothing!() };

    let toggle_select_all = {
        let children = children.clone();

        move |_| match &*children {
            Some(inner) => {
                let mut inner = inner.clone();
                inner.1.toggle_all();
                children.set(Some(inner));
            }
            None => do_nothing!(),
        }
    };

    let theme = prop.theme();
    let background_color = theme.background2();
    let spare_color = theme.spare();

    let style = format! {"
        position: absolute;
        display: flex;
        flex-direction: column;
        gap: 0;
        top: 10%;
        bottom: 4%;
        overflow: auto;
        background-color: {background_color};
        border: 1px solid {spare_color};
        margin: 1px 1px 1px 1px;
        left: 25%;
        right: 0%;"};

    let entries = if let Some(values) = &*children {
        values
            .0
            .iter()
            .enumerate()
            .map(|(idx, pitou)| (idx, pitou.clone(), values.1[idx], toggle_select.clone()))
            .map(|(idx, pitou, selected, toggle_select)| {
                html! { <Row {idx} {pitou} {theme} {selected} {toggle_select}/> }
            })
            .collect::<Html>()
    } else {
        html! {}
    };

    let checked = (&*children)
        .as_ref()
        .map(|val| val.1.checked())
        .unwrap_or_default();

    html! {
        <div {style} {onclick}>
            <RowDescriptor {toggle_select_all} {checked} />
            {
                entries
            }
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct CheckBoxProps {
    pub is_checked: bool,
    pub on_toggle: Callback<()>,
}

#[function_component]
pub fn CheckBox(prop: &CheckBoxProps) -> Html {
    let onclick = {
        let on_toggle = prop.on_toggle.clone();

        move |_| {
            on_toggle.emit(());
        }
    };

    let style = format! {"
        display: flex;
        flex-direction: row;
        gap: 0;
        width: 5%;
        height: 100%;
        justify-content: center;
        align-items: center;
    "};

    html! {
        <div {style}>
            <input type = "checkbox" {onclick} checked = { prop.is_checked }/>
        </div>
    }
}
