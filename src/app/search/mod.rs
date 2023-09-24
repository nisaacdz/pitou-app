use std::{cell::RefCell, path::PathBuf, rc::Rc, time::Duration};

use backend::{File, SearchMsg, SearchOptions};
use wasm_bindgen_futures::spawn_local;
use yew::{platform::time::sleep, prelude::*};

use crate::app::{
    data::SharedBorrow, tasks::SpawnHandle, AncestorsTabs, AppMenu, ApplicationContext, MainPane,
};

mod options;

use options::SearchOptionsCmp;

use super::ApplicationData;

#[derive(PartialEq, Properties)]
pub struct SearchPageProps {
    pub updateview: Callback<AppMenu>,
}

#[function_component]
pub fn SearchPage(prop: &SearchPageProps) -> Html {
    let cdata = use_context::<ApplicationData>().unwrap();
    let dir = use_state(|| cdata.directory());

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
        let cdata = cdata.clone();
        move |file: File| {
            let path = Rc::new(file.path().clone());
            cdata.update_directory(path);
            updateview.emit(AppMenu::Explorer)
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
    results: Option<Rc<Vec<File>>>,
    state: SearchState,
}

impl SearchResult {
    fn is_searching(&self) -> bool {
        matches!(self.state, SearchState::Searching)
    }

    fn is_none(&self) -> bool {
        self.results.is_none()
    }
}

impl Default for SearchResult {
    fn default() -> Self {
        SearchResult {
            results: None,
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
    let cdata = use_context::<ApplicationData>().unwrap();
    let search_results = use_state(|| SearchResult {
        results: cdata.search_results(),
        state: SearchState::Idle,
    });
    let search_data = use_state(|| RefCell::new(None));
    let aborthandle = use_state(|| SharedBorrow::new(None));

    {
        let search_data = search_data.clone();
        let aborthandle = aborthandle.clone();
        let search_results = search_results.clone();
        let cdata = cdata.clone();
        let directory = prop.dir.clone();

        use_effect(move || {
            let newhandle = SpawnHandle::new(async move {
                if let Some(SearchData { input, options }) = search_data.borrow_mut().take() {
                    let results = None;
                    let state = SearchState::Searching;
                    cdata.update_search_results(results.clone());
                    if let Some(path) = directory {
                        crate::app::tasks::restart_stream_search(&*input, path.as_ref(), options)
                            .await;
                    }
                    search_results.set(SearchResult { results, state });
                } else if search_results.is_searching() {
                    if search_results.is_none() {
                        match_search_stream(search_results, &cdata).await;
                    } else {
                        sleep(Duration::from_millis(100)).await;
                        match_search_stream(search_results, &cdata).await;
                    }
                }
            });

            if let Some(mut oldhandle) = aborthandle.as_mut().replace(newhandle) {
                oldhandle.cancel()
            }

            spawn_local(async move {
                if let Some(handle) = aborthandle.as_mut() {
                    handle.await;
                }
            });
        });
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
        move |(input, options): (Rc<String>, SearchOptions)| {
            search_data.set(RefCell::new(Some(SearchData { input, options })));
        }
    };

    let children = (*search_results).results.clone();

    html! {
        <div {style}>
            <SearchOptionsCmp {onsubmit}/>
            <MainPane {children} updatedirectory = {prop.updatedirectory.clone()}/>
        </div>
    }
}

async fn match_search_stream(
    search_results: UseStateHandle<SearchResult>,
    cdata: &ApplicationData,
) {
    match crate::app::tasks::read_search_stream().await {
        Some(msg) => match msg {
            SearchMsg::Searching(values) => {
                let results = if values.len() > 0 {
                    let mut clone = search_results
                        .results
                        .as_ref()
                        .map(|v| (**v).clone())
                        .unwrap_or(Vec::new());
                    clone.extend(values);
                    Some(Rc::new(clone))
                } else {
                    let res = search_results
                        .results
                        .as_ref()
                        .map(|v| v.clone())
                        .unwrap_or(Rc::new(Vec::new()));
                    Some(res)
                };

                let state = SearchState::Searching;
                search_results.set(SearchResult { results, state })
            }
            SearchMsg::Terminated(values) => {
                let values = if values.len() > 0 {
                    let mut clone = search_results
                        .results
                        .as_ref()
                        .map(|v| (**v).clone())
                        .unwrap_or(Vec::new());
                    clone.extend(values);
                    Some(Rc::new(clone))
                } else {
                    let res = search_results
                        .results
                        .as_ref()
                        .map(|v| v.clone())
                        .unwrap_or(Rc::new(Vec::new()));
                    Some(res)
                };
                cdata.update_search_results(values.clone());
                crate::app::tasks::terminate_search_stream().await;

                let state = SearchState::Idle;
                search_results.set(SearchResult {
                    results: values,
                    state,
                })
            }
        },
        None => {
            let newresult = SearchResult {
                results: search_results.results.clone(),
                state: SearchState::Idle,
            };
            search_results.set(newresult);
        }
    }
}
