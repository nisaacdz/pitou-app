use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::app::LoadingComponent;

#[function_component]
pub fn SearchComponent() -> Html {
    let is_searching = use_state_eq(|| false);

    {
        let is_searching = is_searching.clone();
        use_effect(move || {
            spawn_local(async move {
                if *is_searching {
                    crate::app::tasks::listen_to_ended_search(move || is_searching.set(false))
                        .await;
                } else {
                    crate::app::tasks::listen_to_began_search(move || is_searching.set(true)).await;
                }
            });
        })
    }
    match *is_searching {
        true => html! { <LoadingComponent task = {"searching"}/> },
        false => html! {},
    }
}
