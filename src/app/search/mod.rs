use std::{path::PathBuf, rc::Rc, time::Duration};

use backend::{File, SearchMsg};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::app::{AncestorsTabs, ApplicationContext, MainPane};

mod options;

use options::SearchOptionsCmp;

#[function_component]
pub fn SearchPage() -> Html {
    let dir = use_state(|| crate::app::data::directory().map(|p| p.clone()));

    {
        let dir = dir.clone();
        use_effect_with_deps(
            move |_| {
                if let None = &*dir {
                    let dir = dir.clone();
                    spawn_local(async move {
                        let res = crate::app::tasks::default_directory().await;
                        let newdir = Rc::new(res);
                        dir.set(Some(newdir))
                    });
                }
            },
            (),
        );
    }

    let updatedirectory = {
        let dir = dir.clone();
        move |file: File| {
            let path = Rc::new(file.path().clone());
            dir.set(Some(path));
        }
    };

    let updatedirectory2 = {
        let dir = dir.clone();
        move |path| {
            let path = Rc::new(path);
            dir.set(Some(path))
        }
    };

    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context().unwrap();

    let outer_size = sizes.pane();
    let outer_background_color = theme.background2();

    let outer_style = format! {"
    {outer_size}
    box-sizing: border-box;
    background-color: {outer_background_color};
    display: flex;
    flex-direction: column;
    gap: 0;
    "};

    html! {
        <div style = {outer_style}>
            <AncestorsTabs folder = {(&*dir).clone()} updatedirectory = {updatedirectory2}/>
            <SearchView dir = {(&*dir).clone()} {updatedirectory}/>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct SearchViewProps {
    dir: Option<Rc<PathBuf>>,
    updatedirectory: Callback<File>,
}

#[derive(Clone, Copy)]
pub enum SearchState {
    Idle,
    Searching,
}

#[derive(Clone)]
pub struct SearchResultState {
    results: std::rc::Rc<Vec<File>>,
    state: SearchState,
}

impl PartialEq for SearchResultState {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl Default for SearchResultState {
    fn default() -> Self {
        Self {
            results: std::rc::Rc::new(Vec::new()),
            state: SearchState::Idle,
        }
    }
}

impl SearchResultState {
    fn results(&self) -> std::rc::Rc<Vec<File>> {
        self.results.clone()
    }
    fn state(&self) -> SearchState {
        self.state
    }
}

impl PartialEq for SearchState {
    fn eq(&self, other: &Self) -> bool {
        matches!(self, Self::Idle) && matches!(other, Self::Idle)
    }
}

#[function_component]
pub fn SearchView(prop: &SearchViewProps) -> Html {
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context().unwrap();
    let state = use_state_eq(|| SearchResultState::default());

    {
        let state = state.clone();

        use_effect_with_deps(
            move |search_state| {
                let search_state = search_state.clone();

                match search_state.state() {
                    SearchState::Idle => {}
                    SearchState::Searching => {
                        spawn_local(async move {
                            async_std::task::sleep(Duration::from_millis(100)).await;
                            match crate::app::tasks::read_search_stream().await {
                                Some(msg) => match msg {
                                    SearchMsg::Terminated(items) => {
                                        let results = if items.len() > 0 {
                                            let mut results = (&*search_state.results()).clone();
                                            results.extend(items);
                                            std::rc::Rc::new(results)
                                        } else {
                                            search_state.results()
                                        };

                                        let state = SearchState::Idle;

                                        crate::app::tasks::terminate_search_stream().await;

                                        search_state.set(SearchResultState { results, state })
                                    }
                                    SearchMsg::Searching(items) => {
                                        println!("search stream proceeding");
                                        let results = if items.len() > 0 {
                                            crate::app::tasks::terminate_search_stream().await;
                                            let mut results = (&*search_state.results()).clone();
                                            results.extend(items);
                                            std::rc::Rc::new(results)
                                        } else {
                                            search_state.results()
                                        };

                                        let state = SearchState::Searching;

                                        search_state.set(SearchResultState { results, state })
                                    }
                                },
                                None => {
                                    let results = search_state.results();
                                    let state = SearchState::Searching;

                                    search_state.set(SearchResultState { results, state })
                                }
                            }
                        });
                    }
                }
            },
            state,
        );
    }

    let size = sizes.split_pane();

    let style = format! {"
    {size}
    display: flex;
    flex-direction: row;
    gap: 0;
    "};

    let onsubmit = {
        let dir = prop.dir.clone();
        let search_state = state.clone();
        move |(input, options)| {
            let dir = dir.clone();
            let search_state = search_state.clone();

            spawn_local(async move {
                if let Some(dir) = dir.as_ref() {
                    crate::app::tasks::restart_stream_search(&input, (&**dir).as_ref(), options)
                        .await;
                    let results = std::rc::Rc::new(Vec::new());
                    let state = SearchState::Searching;
                    search_state.set(SearchResultState { results, state })
                }
            });
        }
    };

    html! {
        <div {style}>
            <SearchOptionsCmp {onsubmit}/>
            <MainPane children = {state.results()} updatedirectory = {prop.updatedirectory.clone()}/>
        </div>
    }
}
