use std::time::Duration;

use backend::{Pitou, SearchMsg};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::app::{
    invoke, AncestorsTabs, ApplicationContext, MainPane, PitouNoArg, PitouSearchArgs,
};

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
                        let jsval =
                            invoke("default_directory", to_value(&PitouNoArg).unwrap()).await;
                        let newdir = from_value::<Pitou>(jsval).unwrap();
                        dir.set(Some(newdir))
                    });
                }
            },
            (),
        );
    }

    let updatedirectory = {
        let dir = dir.clone();
        move |newdir| dir.set(Some(newdir))
    };

    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context::<ApplicationContext>().unwrap();

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
            <AncestorsTabs pitou = {(&*dir).clone()} updatedirectory = {updatedirectory.clone()} />
            <SearchView dir = {(&*dir).clone()} {updatedirectory}/>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct SearchViewProps {
    dir: Option<Pitou>,
    updatedirectory: Callback<Pitou>,
}

#[derive(Clone, Copy)]
pub enum SearchState {
    Idle,
    Searching,
}

#[derive(Clone)]
pub struct SearchResultState {
    results: std::rc::Rc<Vec<Pitou>>,
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
    fn results(&self) -> std::rc::Rc<Vec<Pitou>> {
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
    } = use_context::<ApplicationContext>().unwrap();
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
                                        println!("search stream terminated!");
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
            if let Some(dir) = &dir {
                let arg = to_value(&PitouSearchArgs {
                    key: &input,
                    pitou: &dir,
                    options,
                })
                .unwrap();

                let search_state = search_state.clone();

                spawn_local(async move {
                    invoke("restart_stream_search", arg).await;
                    let results = std::rc::Rc::new(Vec::new());
                    let state = SearchState::Searching;
                    search_state.set(SearchResultState { results, state })
                });
            }
        }
    };

    html! {
        <div {style}>
            <SearchOptionsCmp {onsubmit}/>
            <MainPane children = {(*state.results()).clone()} updatedirectory = {prop.updatedirectory.clone()}/>
        </div>
    }
}
