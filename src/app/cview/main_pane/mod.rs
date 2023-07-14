mod rows;
mod dsc;

use rows::*;
use dsc::*;

use crate::app::PitouProps;
//use gloo::console::log;
use yew::prelude::*;
use std::sync::{Arc, Mutex};


#[function_component]
pub fn MainPane(prop: &PitouProps) -> Html {
    let sub_files = use_state(|| prop.entries());
    let selected = use_state(|| Arc::new(Mutex::new(vec![false; sub_files.len()])));
    let selected_len = use_state(|| Arc::new(Mutex::new(0)));

    let toggle_select = {
        let selected = selected.clone();
        let selected_len = selected_len.clone();

        move |idx: usize| {
            let replacement = Arc::clone(&*selected);
            let mut mg = replacement.lock().unwrap();
            mg[idx] = !mg[idx];
            *selected_len.lock().unwrap() += if mg[idx] { 1 } else { -1 };
            std::mem::drop(mg);
            selected.set(replacement)
        }
    };

    //TODO
    let onclick = { move |_| () };

    let toggle_select_all = {
        let selected_items = selected.clone();
        let selected_len = selected_len.clone();
        let n = sub_files.len();

        move |_| {
            if *selected_len.lock().unwrap() == n as i32 {
                selected_items.set(Arc::new(Mutex::new(vec![false; n])));
                selected_len.set(Arc::new(Mutex::new(0)));
            } else {
                selected_items.set(Arc::new(Mutex::new(vec![true; n])));
                selected_len.set(Arc::new(Mutex::new(n as i32)));
            }
        }
    };

    let style = format! {"
        position: absolute;
        display: flex;
        flex-direction: column;
        gap: 0;
        top: 10%;
        bottom: 4%;
        overflow: auto;
        background-color: {};
        border: 1px solid {};
        margin: 1px;
        left: 25%;
        right: 0%;",
        prop.theme().background2(),
        prop.theme().spare(),
    };

    let entries = sub_files
        .iter()
        .enumerate()
        .map(|(idx, pitou)| {
            (
                idx,
                pitou.clone(),
                selected.lock().unwrap()[idx],
                toggle_select.clone(),
            )
        })
        .map(|(idx, pitou, selected, toggle_select)| {
            html! { <Row {idx} {pitou} {selected} {toggle_select}/> }
        })
        .collect::<Html>();

    html! {
        <div {style} {onclick}>
            <RowDescriptor {toggle_select_all} checked = { *selected_len.lock().unwrap() == sub_files.len() as i32 } />
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