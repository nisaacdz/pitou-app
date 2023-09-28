use crate::app::{data::SharedBorrow, tasks::SpawnHandle};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component]
pub fn SearchPreloader() -> Html {
    let searching = use_state_eq(|| Rc::new(RefCell::new(HashSet::<i32>::new())));

    let suscriber = use_state(|| SharedBorrow::new(None::<SpawnHandle<_>>));
    let unsuscriber = use_state(|| SharedBorrow::new(None::<SpawnHandle<_>>));

    {
        let searching = searching.clone();

        use_effect(move || {
            let searching1 = searching.clone();
            let newsuscriber = SpawnHandle::new(async move {
                crate::app::tasks::listen_to_began_search(move |v| {
                    searching1.borrow_mut().insert(v.id);
                    searching1.set((*searching1).clone())
                })
                .await;
            });

            let newunsuscriber = SpawnHandle::new(async move {
                crate::app::tasks::listen_to_ended_search(move |v| {
                    searching.borrow_mut().remove(&v.id);
                    searching.set((*searching).clone());
                })
                .await;
            });

            spawn_local(async move {
                if let Some(v) = suscriber.get_mut() {
                    v.abort().await;
                };
                suscriber.get_mut().insert(newsuscriber).await;
            });

            spawn_local(async move {
                if let Some(v) = unsuscriber.get_mut() {
                    v.abort().await;
                };
                unsuscriber.get_mut().insert(newunsuscriber).await;
            });
        })
    }

    let brv = searching.borrow();

    brv.iter()
        .map(|_| {
            html! {
                <div class = "preloader search">
                    <img class = "preloader-anim" src="./public/anims/searching.gif"/>
                    <span class = "preloader-dsc">{"searching"}</span>
                </div>
            }
        })
        .collect()
}

#[function_component]
pub fn DeletePreloader() -> Html {
    let deleting = use_state_eq(|| Rc::new(RefCell::new(HashMap::<i32, usize>::new())));
    let suscriber = use_state(|| SharedBorrow::new(None::<SpawnHandle<_>>));
    let unsuscriber = use_state(|| SharedBorrow::new(None::<SpawnHandle<_>>));

    {
        let deleting = deleting.clone();

        use_effect(move || {
            let deleting1 = deleting.clone();
            let newsuscriber = SpawnHandle::new(async move {
                crate::app::tasks::listen_to_began_delete(move |v| {
                    deleting1.borrow_mut().insert(v.id, v.arg);
                    deleting1.set((*deleting1).clone())
                })
                .await;
            });

            let newunsuscriber = SpawnHandle::new(async move {
                crate::app::tasks::listen_to_ended_delete(move |v| {
                    deleting.borrow_mut().remove(&v.id);
                    deleting.set((*deleting).clone());
                })
                .await;
            });

            spawn_local(async move {
                if let Some(v) = suscriber.get_mut() {
                    v.abort().await;
                };
                suscriber.get_mut().insert(newsuscriber).await;
            });

            spawn_local(async move {
                if let Some(v) = unsuscriber.get_mut() {
                    v.abort().await;
                };
                unsuscriber.get_mut().insert(newunsuscriber).await;
            });
        })
    }

    let brv = deleting.borrow();

    brv.iter()
        .map(|(_, &len)| {
            let msg = if len == 1 {
                format! {"Deleting 1 item"}
            } else {
                format! {"Deleting {} items", len}
            };
            html! {
                <div class = "preloader search">
                    <img class = "preloader-anim" src="./public/anims/spinner.gif"/>
                    <span class = "preloader-dsc">{msg}</span>
                </div>
            }
        })
        .collect()
}

#[function_component]
pub fn RenamePreloader() -> Html {
    let renaming = use_state_eq(|| Rc::new(RefCell::new(HashSet::<i32>::new())));

    let suscriber = use_state(|| SharedBorrow::new(None::<SpawnHandle<_>>));
    let unsuscriber = use_state(|| SharedBorrow::new(None::<SpawnHandle<_>>));

    {
        let renaming = renaming.clone();

        use_effect(move || {
            let renaming1 = renaming.clone();
            let newsuscriber = SpawnHandle::new(async move {
                crate::app::tasks::listen_to_began_rename(move |v| {
                    renaming1.borrow_mut().insert(v.id);
                    renaming1.set((*renaming1).clone())
                })
                .await;
            });

            let newunsuscriber = SpawnHandle::new(async move {
                crate::app::tasks::listen_to_ended_rename(move |v| {
                    renaming.borrow_mut().remove(&v.id);
                    renaming.set((*renaming).clone());
                })
                .await;
            });

            spawn_local(async move {
                if let Some(v) = suscriber.get_mut() {
                    v.abort().await;
                };
                suscriber.get_mut().insert(newsuscriber).await;
            });

            spawn_local(async move {
                if let Some(v) = unsuscriber.get_mut() {
                    v.abort().await;
                };
                unsuscriber.get_mut().insert(newunsuscriber).await;
            });
        })
    }

    let brv = renaming.borrow();

    brv.iter()
        .map(|_| {
            html! {
                <div class = "preloader search">
                    <img class = "preloader-anim" src="./public/anims/cracking.gif"/>
                    <span class = "preloader-dsc">{"Renaming"}</span>
                </div>
            }
        })
        .collect()
}

#[function_component]
pub fn AddFolderPreloader() -> Html {
    let adding_folder = use_state_eq(|| Rc::new(RefCell::new(HashSet::<i32>::new())));

    let suscriber = use_state(|| SharedBorrow::new(None::<SpawnHandle<_>>));
    let unsuscriber = use_state(|| SharedBorrow::new(None::<SpawnHandle<_>>));

    {
        let adding_folder = adding_folder.clone();

        use_effect(move || {
            let adding_folder1 = adding_folder.clone();
            let newsuscriber = SpawnHandle::new(async move {
                crate::app::tasks::listen_to_began_addfolder(move |v| {
                    adding_folder1.borrow_mut().insert(v.id);
                    adding_folder1.set((*adding_folder1).clone())
                })
                .await;
            });

            let newunsuscriber = SpawnHandle::new(async move {
                crate::app::tasks::listen_to_ended_addfolder(move |v| {
                    adding_folder.borrow_mut().remove(&v.id);
                    adding_folder.set((*adding_folder).clone());
                })
                .await;
            });

            spawn_local(async move {
                if let Some(v) = suscriber.get_mut() {
                    v.abort().await;
                };
                suscriber.get_mut().insert(newsuscriber).await;
            });

            spawn_local(async move {
                if let Some(v) = unsuscriber.get_mut() {
                    v.abort().await;
                };
                unsuscriber.get_mut().insert(newunsuscriber).await;
            });
        })
    }

    let brv = adding_folder.borrow();

    brv.iter()
        .map(|_| {
            html! {
                <div class = "preloader search">
                    <img class = "preloader-anim" src="./public/anims/cracking.gif"/>
                    <span class = "preloader-dsc">{"Creating new file"}</span>
                </div>
            }
        })
        .collect()
}

#[function_component]
pub fn AddFilePreloader() -> Html {
    let cutting = use_state_eq(|| Rc::new(RefCell::new(HashSet::<i32>::new())));

    let suscriber = use_state(|| SharedBorrow::new(None::<SpawnHandle<_>>));
    let unsuscriber = use_state(|| SharedBorrow::new(None::<SpawnHandle<_>>));

    {
        let cutting = cutting.clone();

        use_effect(move || {
            let cutting1 = cutting.clone();
            let newsuscriber = SpawnHandle::new(async move {
                crate::app::tasks::listen_to_began_addfile(move |v| {
                    cutting1.borrow_mut().insert(v.id);
                    cutting1.set((*cutting1).clone())
                })
                .await;
            });

            let newunsuscriber = SpawnHandle::new(async move {
                crate::app::tasks::listen_to_ended_addfile(move |v| {
                    cutting.borrow_mut().remove(&v.id);
                    cutting.set((*cutting).clone());
                })
                .await;
            });

            spawn_local(async move {
                if let Some(v) = suscriber.get_mut() {
                    v.abort().await;
                };
                suscriber.get_mut().insert(newsuscriber).await;
            });

            spawn_local(async move {
                if let Some(v) = unsuscriber.get_mut() {
                    v.abort().await;
                };
                unsuscriber.get_mut().insert(newunsuscriber).await;
            });
        })
    }

    let brv = cutting.borrow();

    brv.iter()
        .map(|_| {
            html! {
                <div class = "preloader search">
                    <img class = "preloader-anim" src="./public/anims/cracking.gif"/>
                    <span class = "preloader-dsc">{"Creating new file"}</span>
                </div>
            }
        })
        .collect()
}

#[function_component]
pub fn CutPreloader() -> Html {
    let cutting = use_state_eq(|| Rc::new(RefCell::new(HashMap::<i32, usize>::new())));
    let suscriber = use_state(|| SharedBorrow::new(None::<SpawnHandle<_>>));
    let unsuscriber = use_state(|| SharedBorrow::new(None::<SpawnHandle<_>>));

    {
        let cutting = cutting.clone();

        use_effect(move || {
            let cutting1 = cutting.clone();
            let newsuscriber = SpawnHandle::new(async move {
                crate::app::tasks::listen_to_began_cut(move |v| {
                    cutting1.borrow_mut().insert(v.id, v.arg);
                    cutting1.set((*cutting1).clone())
                })
                .await;
            });

            let newunsuscriber = SpawnHandle::new(async move {
                crate::app::tasks::listen_to_ended_cut(move |v| {
                    cutting.borrow_mut().remove(&v.id);
                    cutting.set((*cutting).clone());
                })
                .await;
            });

            spawn_local(async move {
                if let Some(v) = suscriber.get_mut() {
                    v.abort().await;
                };
                suscriber.get_mut().insert(newsuscriber).await;
            });

            spawn_local(async move {
                if let Some(v) = unsuscriber.get_mut() {
                    v.abort().await;
                };
                unsuscriber.get_mut().insert(newunsuscriber).await;
            });
        })
    }

    let brv = cutting.borrow();

    brv.iter()
        .map(|(_, &len)| {
            let msg = if len == 1 {
                format! {"Cutting 1 item"}
            } else {
                format! {"Cutting {} items", len}
            };
            html! {
                <div class = "preloader search">
                    <img class = "preloader-anim" src="./public/anims/loading.gif"/>
                    <span class = "preloader-dsc">{msg}</span>
                </div>
            }
        })
        .collect()
}

#[function_component]
pub fn CopyPreloader() -> Html {
    let copying = use_state_eq(|| Rc::new(RefCell::new(HashMap::<i32, usize>::new())));
    let suscriber = use_state(|| SharedBorrow::new(None::<SpawnHandle<_>>));
    let unsuscriber = use_state(|| SharedBorrow::new(None::<SpawnHandle<_>>));

    {
        let copying = copying.clone();

        use_effect(move || {
            let copying1 = copying.clone();
            let newsuscriber = SpawnHandle::new(async move {
                crate::app::tasks::listen_to_began_copy(move |v| {
                    copying1.borrow_mut().insert(v.id, v.arg);
                    copying1.set((*copying1).clone())
                })
                .await;
            });

            let newunsuscriber = SpawnHandle::new(async move {
                crate::app::tasks::listen_to_ended_copy(move |v| {
                    copying.borrow_mut().remove(&v.id);
                    copying.set((*copying).clone());
                })
                .await;
            });

            spawn_local(async move {
                if let Some(v) = suscriber.get_mut() {
                    v.abort().await;
                };
                suscriber.get_mut().insert(newsuscriber).await;
            });

            spawn_local(async move {
                if let Some(v) = unsuscriber.get_mut() {
                    v.abort().await;
                };
                unsuscriber.get_mut().insert(newunsuscriber).await;
            });
        })
    }

    let brv = copying.borrow();

    brv.iter()
        .map(|(_, &len)| {
            let msg = if len == 1 {
                format! {"Copying 1 item"}
            } else {
                format! {"Copying {} items", len}
            };
            html! {
                <div class = "preloader search">
                    <img class = "preloader-anim" src="./public/anims/loading.gif"/>
                    <span class = "preloader-dsc">{msg}</span>
                </div>
            }
        })
        .collect()
}

#[function_component]
pub fn PastePreloader() -> Html {
    let pasting = use_state_eq(|| Rc::new(RefCell::new(HashSet::<i32>::new())));

    let suscriber = use_state(|| SharedBorrow::new(None::<SpawnHandle<_>>));
    let unsuscriber = use_state(|| SharedBorrow::new(None::<SpawnHandle<_>>));

    {
        let pasting = pasting.clone();

        use_effect(move || {
            let pasting1 = pasting.clone();
            let newsuscriber = SpawnHandle::new(async move {
                crate::app::tasks::listen_to_began_paste(move |v| {
                    pasting1.borrow_mut().insert(v.id);
                    pasting1.set((*pasting1).clone())
                })
                .await;
            });

            let newunsuscriber = SpawnHandle::new(async move {
                crate::app::tasks::listen_to_ended_paste(move |v| {
                    pasting.borrow_mut().remove(&v.id);
                    pasting.set((*pasting).clone());
                })
                .await;
            });

            spawn_local(async move {
                if let Some(v) = suscriber.get_mut() {
                    v.abort().await;
                };
                suscriber.get_mut().insert(newsuscriber).await;
            });

            spawn_local(async move {
                if let Some(v) = unsuscriber.get_mut() {
                    v.abort().await;
                };
                unsuscriber.get_mut().insert(newunsuscriber).await;
            });
        })
    }

    let brv = pasting.borrow();

    brv.iter()
        .map(|_| {
            html! {
                <div class = "preloader search">
                    <img class = "preloader-anim" src="./public/anims/infinite.gif"/>
                    <span class = "preloader-dsc">{"Pasting files"}</span>
                </div>
            }
        })
        .collect()
}
