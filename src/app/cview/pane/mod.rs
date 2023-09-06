mod side_pane;

use crate::app::{AncestorsTabs, ApplicationContext, MainPane};
use yew::prelude::*;

use wasm_bindgen_futures::spawn_local;

use backend::File;
use std::path::PathBuf;

use side_pane::*;

use std::rc::Rc;

#[function_component]
pub fn Pane() -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context().unwrap();

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
                                let directory = Rc::new(crate::app::tasks::default_directory().await);
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
                            let children = crate::app::tasks::children(&*directory).await;
                            let siblings = crate::app::tasks::siblings(&*directory).await;
                            let children = Rc::new(children);
                            let siblings = Rc::new(siblings);
                            state.set(Contents {
                                directory: Some(directory.clone()),
                                children: Some(children),
                                siblings: Some(siblings),
                            });
                        });
                    }

                    Contents {
                        directory: Some(_),
                        children: Some(_),
                        siblings: _,
                    } => (),

                    _ => (),
                }
            },
            state.clone(),
        );
    }

    let updatedirectory_with_path = {
        let state = state.clone();
        move |dir: PathBuf| {
            let directory = Rc::new(dir);
            crate::app::data::update_directory(Some(directory.clone()));
            let new_state = Contents {
                directory: Some(directory),
                children: None,
                siblings: None,
            };
            state.set(new_state);
        }
    };

    let updatedirectory = Callback::from({
        let state = state.clone();

        move |dir: File| {
            let directory = Rc::new(dir.path().clone());
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
    let border_color = theme.spare();

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
    border: 1px solid {border_color};
    box-sizing: border-box;
    "};

    html! {
        <div {style}>
            <AncestorsTabs updatedirectory = {updatedirectory_with_path.clone()} folder = {state.directory()} />
            <div style = {split_pane_style}>
                <SidePane siblings = { state.siblings() } directory = { state.directory() } updatedirectory = { updatedirectory_with_path } />
                <MainPane {updatedirectory} children = { state.children() }/>
            </div>
        </div>
    }
}

#[derive(PartialEq)]
struct Contents {
    directory: Option<Rc<PathBuf>>,
    children: Option<Rc<Vec<File>>>,
    siblings: Option<Rc<Vec<File>>>,
}

impl Contents {
    fn default() -> Self {
        Contents {
            directory: None,
            children: None,
            siblings: None,
        }
    }

    fn children(&self) -> Option<Rc<Vec<File>>> {
        self.children.clone()
    }

    fn siblings(&self) -> Option<Rc<Vec<File>>> {
        self.siblings.clone()
    }

    fn directory(&self) -> Option<Rc<PathBuf>> {
        self.directory.clone()
    }
}
