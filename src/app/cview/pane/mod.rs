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
        theme: _,
        sizes,
        settings,
    } = use_context().unwrap();

    let state = use_state(|| Contents::default());

    {
        let state = state.clone();
        use_effect_with_deps(
            move |state| {
                let state = state.clone();
                spawn_local(async move {
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
                                let directory =
                                    Rc::new(crate::app::tasks::default_directory().await);
                                crate::app::data::update_directory(Some(directory.clone()));
                                state.set(Contents {
                                    directory: Some(directory),
                                    children: None,
                                    siblings: None,
                                })
                            }
                        }
                        Contents {
                            directory: Some(directory),
                            children: Some(_),
                            siblings: Some(_),
                        } => {
                            let state = state.clone();
                            async_std::task::sleep(settings.refresh_wait()).await;
                            // check to see if the state was not changed during sleep
                            if matches!(crate::app::data::directory(), Some(sd) if &sd == directory)
                            {
                                let children =
                                    crate::app::tasks::children(&**directory, settings.filter)
                                        .await;
                                let siblings =
                                    crate::app::tasks::siblings(&**directory, settings.filter)
                                        .await;
                                let children = Rc::new(children);
                                let siblings = Rc::new(siblings);
                                state.set(Contents {
                                    directory: Some(directory.clone()),
                                    children: Some(children),
                                    siblings: Some(siblings),
                                });
                            } else {
                                let new_state = Contents {
                                    directory: None,
                                    children: None,
                                    siblings: None,
                                };
                                state.set(new_state);
                            }
                        }
                        Contents {
                            directory: Some(directory),
                            children: _,
                            siblings: _,
                        } => {
                            let state = state.clone();
                            let directory = directory.clone();
                            let children =
                                crate::app::tasks::children(&*directory, settings.filter).await;
                            let siblings =
                                crate::app::tasks::siblings(&*directory, settings.filter).await;
                            let children = Rc::new(children);
                            let siblings = Rc::new(siblings);
                            state.set(Contents {
                                directory: Some(directory.clone()),
                                children: Some(children),
                                siblings: Some(siblings),
                            });
                        }
                    }
                });
            },
            state.clone(),
        );
    }

    let updatedirectory_with_path = {
        move |dir: PathBuf| {
            let directory = Rc::new(dir);
            crate::app::data::update_directory(Some(directory.clone()));
        }
    };

    let updatedirectory = Callback::from({
        move |dir: File| {
            let directory = Rc::new(dir.path().clone());
            crate::app::data::update_directory(Some(directory.clone()));
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
    gap: 0;
    {split_pane_size}
    "};

    html! {
        <div {style}>
            <AncestorsTabs updatedirectory = {updatedirectory_with_path.clone()} folder = {state.directory()}/>
            <div style = {split_pane_style}>
                <SidePane siblings = { state.siblings() } directory = { state.directory() } updatedirectory = { updatedirectory_with_path } />
                <MainPane {updatedirectory} children = { state.children() }/>
            </div>
        </div>
    }
}

struct Contents {
    directory: Option<Rc<PathBuf>>,
    children: Option<Rc<Vec<File>>>,
    siblings: Option<Rc<Vec<File>>>,
}

impl PartialEq for Contents {
    fn eq(&self, _: &Self) -> bool {
        false
    }
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
// use web_time::{Duration, Instant};
// use std::{future::Future, task::{Poll, Context}};

// pub async fn sleep(directory: Rc<PathBuf> , duration: Duration) -> Option<Rc<PathBuf>> {
//     struct SleepHandle {
//         directory: Rc<PathBuf>,
//         timeout: Instant,
//     }

//     impl Future for SleepHandle {
//         type Output = Option<Rc<PathBuf>>;
//         fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

//         }
//     }

//     let timeout = Instant::now().checked_add(duration).unwrap();

//     SleepHandle { directory, timeout }.await
// }
