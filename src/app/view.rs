use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::app::{invoke, PitouArg, PitouNoArg};
use backend::Pitou;

use super::{cview::*, Theme};

#[derive(PartialEq)]
struct ContentViewState {
    directory: Option<Pitou>,
    children: Option<Vec<Pitou>>,
    siblings: Option<Vec<Pitou>>,
}

impl Default for ContentViewState {
    fn default() -> Self {
        ContentViewState {
            directory: None,
            children: None,
            siblings: None,
        }
    }
}

impl ContentViewState {
    fn directory(&self) -> Option<Pitou> {
        self.directory.clone()
    }

    fn children(&self) -> Option<Vec<Pitou>> {
        self.children.clone()
    }

    fn siblings(&self) -> Option<Vec<Pitou>> {
        self.siblings.clone()
    }
}

#[function_component]
pub fn ContentView() -> Html {
    let state = use_state(|| ContentViewState::default());
    let theme = use_context::<Theme>().unwrap();

    {
        let state = state.clone();
        use_effect_with_deps(
            move |state| {
                let state = state.clone();

                match &*state {
                    ContentViewState {
                        directory: None,
                        children: _,
                        siblings: _,
                    } => {
                        let state = state.clone();
                        spawn_local(async move {
                            let js_val =
                                invoke("last_history_or_default", to_value(&PitouNoArg).unwrap())
                                    .await;
                            let directory = from_value::<Pitou>(js_val).unwrap();
                            state.set(ContentViewState {
                                directory: Some(directory),
                                children: None,
                                siblings: None,
                            })
                        })
                    }
                    ContentViewState {
                        directory: Some(directory),
                        children: None,
                        siblings: None,
                    } => {
                        let state = state.clone();
                        let directory = directory.clone();

                        spawn_local(async move {
                            let children = from_value::<Vec<Pitou>>(
                                invoke(
                                    "children",
                                    to_value(&PitouArg { pitou: &directory }).unwrap(),
                                )
                                .await,
                            )
                            .unwrap();
                            let siblings = from_value::<Vec<Pitou>>(
                                invoke(
                                    "siblings",
                                    to_value(&PitouArg { pitou: &directory }).unwrap(),
                                )
                                .await,
                            )
                            .unwrap();
                            state.set(ContentViewState {
                                directory: Some(directory.clone()),
                                children: Some(children),
                                siblings: Some(siblings),
                            });
                        });
                    }

                    ContentViewState {
                        directory: Some(_),
                        children: _,
                        siblings: _,
                    } => (),
                }
            },
            state.clone(),
        );
    }

    fn jot_dir_history(pitou: &Pitou) {
        let arg = to_value(&PitouArg { pitou }).unwrap();
        spawn_local(async move {
            invoke("append_history", arg).await;
        });
    }

    let updatedirectory = Callback::from({
        let state = state.clone();

        move |directory: Pitou| {
            crate::data::update_directory(Some(directory.clone()));
            crate::data::clear_selected();
            jot_dir_history(&directory);
            let new_state = ContentViewState {
                directory: Some(directory),
                children: None,
                siblings: None,
            };
            state.set(new_state);
        }
    });

    let updateui = {
        let state = state.clone();
        move |_| state.set(ContentViewState::default())
    };

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
            <TopPane updatedirectory = { updatedirectory.clone() } pitou = { state.directory() } {updateui}/>

            <BottomPane/>

            <LeftPane/>

            <SidePane siblings = { state.siblings() } selected = { state.directory() } updatedirectory = { updatedirectory.clone() } />

            <MainPane {updatedirectory} children = { state.children() }/>
        </div>
    }
}

#[allow(unused)]
pub struct OpeningView {}
