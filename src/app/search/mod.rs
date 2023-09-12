use std::{path::PathBuf, rc::Rc};

use backend::{File, SearchMsg, SearchOptions};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::app::{AncestorsTabs, ApplicationContext, MainPane, AppView};

mod options;

use options::SearchOptionsCmp;

#[derive(PartialEq, Properties)]
pub struct SearchPageProps {
    pub updateview: Callback<AppView>,
}

#[function_component]
pub fn SearchPage(prop: &SearchPageProps) -> Html {
    let dir = use_state(|| crate::app::data::directory().map(|p| p.clone()));

    {
        let dir: UseStateHandle<Option<Rc<PathBuf>>> = dir.clone();
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
        let updateview = prop.updateview.clone();
        move |file: File| {
            let path = Rc::new(file.path().clone());
            crate::app::data::update_directory(Some(path));
            updateview.emit(AppView::Explorer)
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

impl PartialEq for SearchState {
    fn eq(&self, other: &Self) -> bool {
        matches!(self, Self::Idle) && matches!(other, Self::Idle)
    }
}

#[derive(PartialEq)]
struct SearchData {
    input: Rc<String>,
    options: SearchOptions,
}

impl Default for SearchData {
    fn default() -> Self {
        SearchData {
            input: Rc::new(String::new()),
            options: SearchOptions::new(),
        }
    }
}

struct SearchResult {
    results: Rc<Vec<File>>,
    state: SearchState,
}

impl Default for SearchResult {
    fn default() -> Self {
        SearchResult {
            results: Rc::new(Vec::new()),
            state: SearchState::Idle,
        }
    }
}

impl PartialEq for SearchResult {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

#[function_component]
pub fn SearchView(prop: &SearchViewProps) -> Html {
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context().unwrap();
    let results = use_state(|| {
        if let Some(results) = crate::app::data::search_results() {
            SearchResult {
                results,
                state: SearchState::Idle,
            }
        } else {
            SearchResult::default()
        }
    });
    let search_data = use_state(|| None::<SearchData>);

    {
        let results = results.clone();
        let search_data = search_data.clone();

        use_effect_with_deps(
            move |search_data| {
                if let Some(search_data) = &**search_data {
                    if (*search_data).input.len() != 0 {
                        results.set(SearchResult {
                            results: Rc::new(Vec::new()),
                            state: SearchState::Searching,
                        });
                    }
                }
            },
            search_data,
        );
    }
    {
        let results = results.clone();
        use_effect_with_deps(
            |results| {
                let results = results.clone();
                spawn_local(async move {
                    match crate::app::tasks::read_search_stream().await {
                        Some(msg) => match msg {
                            SearchMsg::Searching(values) => {
                                let values = if values.len() > 0 {
                                    let mut clone = (*results.results).clone();
                                    clone.extend(values);
                                    Rc::new(clone)
                                } else {
                                    results.results.clone()
                                };

                                let state = SearchState::Searching;
                                results.set(SearchResult {
                                    results: values,
                                    state,
                                })
                            }
                            SearchMsg::Terminated(values) => {
                                let values = if values.len() > 0 {
                                    let mut clone = (*results.results).clone();
                                    clone.extend(values);
                                    Rc::new(clone)
                                } else {
                                    results.results.clone()
                                };
                                crate::app::data::update_search_results(values.clone());
                                spawn_local(async move {
                                    crate::app::tasks::terminate_search_stream().await;
                                });

                                let state = SearchState::Idle;
                                results.set(SearchResult {
                                    results: values,
                                    state,
                                })
                            }
                        },
                        None => {
                            let newresult = SearchResult {
                                results: results.results.clone(),
                                state: SearchState::Idle,
                            };
                            results.set(newresult);
                        }
                    }
                });
            },
            results,
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
        let search_data = search_data.clone();
        let dir = prop.dir.clone();
        move |(input, options): (Rc<String>, SearchOptions)| {
            let dir = dir.clone();
            let search_data = search_data.clone();
            spawn_local(async move {
                if let Some(dir) = &dir {
                    crate::app::tasks::restart_stream_search(&*input, &**dir, options).await;
                    search_data.set(Some(SearchData { input, options }));
                }
            });
        }
    };

    let children = Some((*results).results.clone());

    html! {
        <div {style}>
            <SearchOptionsCmp {onsubmit}/>
            <MainPane {children} updatedirectory = {prop.updatedirectory.clone()}/>
        </div>
    }
}
