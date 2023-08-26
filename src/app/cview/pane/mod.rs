mod side_pane;

use crate::app::{invoke, AncestorsTabs, ApplicationContext, MainPane, PitouArg, PitouNoArg};
use yew::prelude::*;

use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_futures::spawn_local;

use backend::Pitou;
use side_pane::*;

#[function_component]
pub fn Pane() -> Html {
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context::<ApplicationContext>().unwrap();

    let state = use_state(|| Contents::default());

    {
        let state = state.clone();
        use_effect_with_deps(
            move |state| {
                let state = state.clone();

                match &*state {
                    Contents {
                        directory: None,
                        children: _,
                        siblings: _,
                    } => {
                        if let Some(directory) = crate::app::data::directory() {
                            state.set(Contents {
                                directory: Some(directory.clone()),
                                children: None,
                                siblings: None,
                            })
                        } else {
                            let state = state.clone();
                            spawn_local(async move {
                                let js_val =
                                    invoke("default_directory", to_value(&PitouNoArg).unwrap())
                                        .await;
                                let directory = from_value::<Pitou>(js_val).unwrap();
                                state.set(Contents {
                                    directory: Some(directory),
                                    children: None,
                                    siblings: None,
                                })
                            })
                        }
                    }
                    Contents {
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
                            state.set(Contents {
                                directory: Some(directory.clone()),
                                children: Some(children),
                                siblings: Some(siblings),
                            });
                        });
                    }

                    Contents {
                        directory: Some(_),
                        children: Some(v),
                        siblings: _,
                    } => (),

                    _ => (),
                }
            },
            state.clone(),
        );
    }

    //TODO
    fn jot_dir_history(pitou: &Pitou) {
        let arg = to_value(&PitouArg { pitou }).unwrap();
        spawn_local(async move {
            invoke("append_history", arg).await;
        });
    }

    let updatedirectory = Callback::from({
        let state = state.clone();

        move |directory: Pitou| {
            crate::app::data::update_directory(Some(directory.clone()));
            let new_state = Contents {
                directory: Some(directory),
                children: None,
                siblings: None,
            };
            state.set(new_state);
        }
    });

    let size = sizes.pane();
    let split_pane_size = sizes.split_pane();

    let style = format! {"
    display: flex;
    flex-direction: column;
    gap: 0;
    {size}
    "};

    let split_pane_style = format! {"
    display: flex;
    flex-direction: row;
    gap: 0;
    {split_pane_size}
    "};

    html! {
        <div {style}>
            <AncestorsTabs updatedirectory = {updatedirectory.clone()} pitou = {state.directory()} />
            <div style = {split_pane_style}>
                <SidePane siblings = { state.siblings() } selected = { state.directory() } updatedirectory = { updatedirectory.clone() } />
                <MainPane {updatedirectory} children = { state.children() }/>
            </div>
        </div>
    }
}

#[derive(PartialEq)]
struct Contents {
    directory: Option<Pitou>,
    children: Option<Vec<Pitou>>,
    siblings: Option<Vec<Pitou>>,
}

impl Contents {
    fn default() -> Self {
        Contents {
            directory: None,
            children: None,
            siblings: None,
        }
    }

    fn children(&self) -> Option<Vec<Pitou>> {
        self.children.clone()
    }

    fn siblings(&self) -> Option<Vec<Pitou>> {
        self.siblings.clone()
    }

    fn directory(&self) -> Option<Pitou> {
        self.directory.clone()
    }
}
