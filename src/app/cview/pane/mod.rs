mod side_pane;

use crate::app::{
    data::SharedBorrow, tasks::SpawnHandle, AncestorsTabs, ApplicationContext, ApplicationData,
    MainPane,
};
use wasm_bindgen_futures::spawn_local;
use yew::{platform::time::sleep, prelude::*};

use backend::{File, Filter};
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

    let cdata = use_context::<ApplicationData>().unwrap();

    let directory = use_state(|| cdata.directory());

    let entries = use_state(|| Entries::new());

    let aborthandle = use_state(|| SharedBorrow::new(None));

    {
        let entries = entries.clone();
        let directory = directory.clone();
        let aborthandle = aborthandle.clone();
        let cdata = cdata.clone();

        use_effect(move || {
            let newhandle = SpawnHandle::new(async move {
                if let Some(cur) = &*directory {
                    if entries.is_none() {
                        let newentries = Entries::init(&**cur, settings.filter).await;
                        cdata.update_dir_siblings(newentries.siblings());
                        cdata.update_dir_children(newentries.children());
                        entries.set(newentries);
                    } else {
                        sleep(settings.refresh_wait()).await;
                        let newentries = entries.refresh(&**cur, settings.filter).await;
                        cdata.update_dir_siblings(newentries.siblings());
                        cdata.update_dir_children(newentries.children());
                        entries.set(newentries)
                    }
                } else {
                    let newdirectory = Rc::new(crate::app::tasks::default_directory().await);
                    cdata.update_directory(newdirectory.clone());
                    directory.set(Some(newdirectory));
                }
            });

            spawn_local(async move {
                if let Some(oldhandle) = aborthandle.get_mut() {
                    SpawnHandle::abort(oldhandle).await;
                }

                aborthandle.get_mut().insert(newhandle).await;
            });
        })
    }

    let updatedir_with = Callback::from({
        let entries = entries.clone();
        let directory = directory.clone();
        let cdata = cdata.clone();
        move |dir: PathBuf| {
            let newdir = Rc::new(dir);
            cdata.update_directory(newdir.clone());
            directory.set(Some(newdir));
            entries.set(Entries::new())
        }
    });

    let updatedirectory = Callback::from({
        let directory = directory.clone();
        let entries = entries.clone();
        let cdata = cdata.clone();
        move |file: File| {
            let newdir = Rc::new(file.path().clone());
            cdata.update_directory(newdir.clone());
            directory.set(Some(newdir));
            entries.set(Entries::new())
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
            <AncestorsTabs updatedirectory = { updatedir_with.clone() } folder = {(*directory).clone()}/>
            <div style = {split_pane_style}>
                <SidePane siblings = { entries.siblings() } directory = {(*directory).clone()} updatedirectory = { updatedir_with } />
                <MainPane {updatedirectory} children = { entries.children() }/>
            </div>
        </div>
    }
}

#[derive(Clone)]
struct Entries {
    children: Option<Rc<Vec<File>>>,
    siblings: Option<Rc<Vec<File>>>,
}

impl std::fmt::Debug for Entries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbs = f.debug_struct("Entries");
        dbs.field("children", &self.children.as_ref().map(|v| &**v));
        dbs.field("siblings", &self.siblings.as_ref().map(|v| &**v));
        dbs.finish()
    }
}

impl Default for Entries {
    fn default() -> Self {
        Entries {
            children: None,
            siblings: None,
        }
    }
}

impl Entries {
    fn is_none(&self) -> bool {
        matches!(&self.children, None)
    }

    async fn init(directory: &PathBuf, filter: Filter) -> Self {
        let children = crate::app::tasks::children(directory, filter).await;
        let siblings = crate::app::tasks::siblings(directory, filter).await;

        let children = Some(Rc::new(children));
        let siblings = Some(Rc::new(siblings));

        Self { children, siblings }
    }

    fn children(&self) -> Option<Rc<Vec<File>>> {
        self.children.clone()
    }

    fn siblings(&self) -> Option<Rc<Vec<File>>> {
        self.siblings.clone()
    }

    async fn refresh(&self, dir: &PathBuf, filter: Filter) -> Self {
        let children = crate::app::tasks::children(dir, filter).await;
        let siblings = crate::app::tasks::siblings(dir, filter).await;

        let children = Some(Rc::new(children));
        let siblings = Some(Rc::new(siblings));
        Self { children, siblings }
    }

    fn new() -> Self {
        Self {
            children: None,
            siblings: None,
        }
    }
}
