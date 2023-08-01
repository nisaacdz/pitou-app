use std::cell::RefCell;

use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::app::{invoke, PitouArg, PitouNoArg};
use backend::Pitou;

use super::{cview::*, Theme};

#[derive(PartialEq, Properties)]
pub struct ContentViewProps {
    pub theme: Theme,
}

#[function_component]
pub fn ContentView(prop: &ContentViewProps) -> Html {
    let directory = use_state(|| None);
    let children = use_state(|| RefCell::new(None));

    {
        let directory = directory.clone();
        let children = children.clone();
        let arg = to_value(&PitouNoArg).unwrap();
        use_effect_with_deps(
            |_| {
                spawn_local(async move {
                    let js_val = invoke("get_debug_file", arg).await;
                    let res = from_value::<Pitou>(js_val).unwrap();
                    update_children(&res, &*children).await;
                    directory.set(Some(res))
                });
            },
            (),
        );
    }

    async fn update_children(pitou: &Pitou, children: &RefCell<Option<Vec<Pitou>>>) {
        crate::data::update_directory(Some(pitou.clone()));
        let arg = to_value(&PitouArg { pitou }).unwrap();
        let res = from_value::<Vec<Pitou>>(invoke("children", arg).await).unwrap();
        children.borrow_mut().replace(res);
    }

    let updatedirectory = Callback::from({
        let directory = directory.clone();
        let children = children.clone();

        move |new_dir: Pitou| {
            let children = children.clone();
            let directory = directory.clone();
            spawn_local(async move {
                update_children(&new_dir, &*children).await;
                directory.set(Some(new_dir))
            });
        }
    });

    let updateui = {
        let directory = directory.clone();
        let updatedirectory = updatedirectory.clone();
        move |_| {
            gloo::console::log!("ui updated or refreshed");
            if let Some(new_dir) = &*directory {
                updatedirectory.emit(new_dir.clone())
            }
        }
    };

    let theme = prop.theme;

    let background_color = theme.background1();

    let style = format! {"
    width: 100%;
    height: 100%;
    background-color: {background_color};
    margin: 0% 0% 0% 0%;
    padding: 0% 0% 0% 0%;
    position: absolute;" };

    html! {
        <div {style} >
            <TopPane {theme} updatedirectory = { updatedirectory.clone() } pitou = { (&*directory).clone() } {updateui}/>

            <BottomPane {theme} />

            <LeftPane {theme} />

            <SidePane pitou = { (&*directory).clone() } {theme} updatedirectory = { updatedirectory.clone() } />

            <MainPane {theme} {updatedirectory} children = {(*children.borrow()).clone()}/>
        </div>
    }
}

#[allow(unused)]
pub struct OpeningView {}
