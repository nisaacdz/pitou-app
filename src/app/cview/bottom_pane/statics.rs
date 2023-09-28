use crate::app::{data::SharedBorrow, tasks::SpawnHandle, ApplicationData};
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use yew::{platform::time::sleep, prelude::*};

#[function_component]
pub fn SelectedInfo() -> Html {
    html! {}
}

#[function_component]
pub fn FolderInfo() -> Html {
    let cdata = use_context::<ApplicationData>().unwrap();
    let file = use_state(|| None);
    let aborthandle = use_state(|| SharedBorrow::new(None));

    {
        let file = file.clone();
        use_effect(move || {
            let newhandle = SpawnHandle::new(async move {
                sleep(Duration::from_millis(100)).await;
                if let Some(p) = cdata.directory() {
                    let newfile = crate::app::tasks::retrieve(&*p).await;
                    file.set(newfile);
                } else {
                    file.set(None);
                }
            });

            spawn_local(async move {
                if let Some(oldhandle) = aborthandle.get_mut() {
                    SpawnHandle::abort(oldhandle).await;
                }

                aborthandle.get_mut().insert(newhandle).await;
            });
        });
    }

    html! {}
}
