use backend::SearchMsg;
use yew::prelude::*;

use crate::app::{EndedLoading, LoadingComponent};

#[derive(PartialEq, Properties)]
pub struct SearchProp {
    pub ended: Callback<()>,
    pub task: &'static Option<SearchMsg>,
}

#[function_component]
pub fn SearchComponent(prop: &SearchProp) -> Html {
    match prop.task.as_ref() {
        Some(task) => match task {
            SearchMsg::Terminated(_) => html! { <EndedLoading/> },
            _ => html! { <LoadingComponent task = {"searching"}/> },
        },
        None => html! {},
    }
}
