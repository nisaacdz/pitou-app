use backend::Pitou;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use crate::app::{invoke, ApplicationContext, PitouArg};

#[derive(PartialEq, Properties)]
pub struct AncestorsTabsProps {
    pub pitou: Option<Pitou>,
    pub updatedirectory: Callback<Pitou>,
}

#[function_component]
pub fn AncestorsTabs(prop: &AncestorsTabsProps) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context::<ApplicationContext>().unwrap();
    let isinput = use_state_eq(|| false);

    let onclick = {
        let isinput = isinput.clone();
        move |_| isinput.set(!*isinput)
    };

    let finished = {
        let isinput = isinput.clone();
        let updatedirectory = prop.updatedirectory.clone();
        move |ns| {
            isinput.set(false);
            updatedirectory.emit(Pitou::from(std::path::PathBuf::from(ns)))
        }
    };

    let foreground_color = theme.foreground1();
    let background_color_2 = theme.background2();
    let spare_color = theme.spare();
    let outer_size = sizes.ancestorspane();

    let inner_size = sizes.ancestorsbar();

    let outer_style = format! {"
    display: flex;
    align-items: center;
    border: 1px solid {spare_color};
    box-sizing: border-box;
    background-color: {background_color_2};
    {outer_size}"};

    let inner_style = format! {"
    color: {foreground_color};
    {inner_size}
    display: flex;
    flex-direction: row-reverse;
    align-items: center;
    column-gap: 2px;
    justify-content: left;
    overflow: hidden;
    margin-left: 1%;
    margin-right: 1%;
    "};

    let entries = if let Some(pitou) = prop.pitou.as_ref() {
        if *isinput {
            html! { <EnterPath pitou = {pitou.clone()} {finished}/> }
        } else {
            pitou
                .ancestors()
                .into_iter()
                .map(|pitou| (pitou, prop.updatedirectory.clone()))
                .map(|(pitou, updatedirectory)| html! { <Ancestor {pitou} {updatedirectory} /> })
                .collect::<Html>()
        }
    } else {
        html! {}
    };

    html! {
        <div style = {outer_style}>
            <div style = {inner_style} {onclick}>
                {entries}
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct AncestorProps {
    pitou: Pitou,
    updatedirectory: Callback<Pitou>,
}

#[derive(PartialEq, Properties)]
pub struct EnterPathProps {
    pitou: Pitou,
    finished: Callback<String>,
}

#[function_component]
pub(super) fn EnterPath(prop: &EnterPathProps) -> Html {
    let ApplicationContext {
        theme: _,
        sizes: _,
        settings: _,
    } = use_context::<ApplicationContext>().unwrap();
    let input_ref = use_node_ref();
    let onclick = move |e: MouseEvent| e.stop_propagation();

    let style = format! {"
    width: auto;
    height: calc(100% - 2px);
    "};

    let onkeypress = {
        let finished = prop.finished.clone();
        let input_ref = input_ref.clone();

        move |e: KeyboardEvent| {
            if e.key_code() == 13 {
                let entered = input_ref.cast::<HtmlInputElement>().unwrap().value();
                finished.emit(entered.clone())
            }
        }
    };

    let value = prop.pitou.path().display().to_string();

    html! {
        <div {style}>
            <input type = "text" ref = {input_ref} {value} {onclick} {onkeypress}/>
        </div>
    }
}

#[function_component]
pub(super) fn Ancestor(prop: &AncestorProps) -> Html {
    let ApplicationContext {
        theme,
        sizes: _,
        settings: _,
    } = use_context::<ApplicationContext>().unwrap();

    let pitou = prop.pitou.clone();

    let onclick = move |e: MouseEvent| e.stop_propagation();

    let background_color = theme.background2();
    let border_color = theme.spare();

    let onselectitem = {
        let updatedirectory = prop.updatedirectory.clone();

        move |pitou| {
            gloo::console::log!("updating directory from choosedir");
            updatedirectory.emit(pitou)
        }
    };

    let style = format! {"
    height: calc(100% - 2px);
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 0;
    width: auto;
    border: 1px solid {border_color};
    box-sizing: border-box;
    background-color: {background_color};
    font-size: 80%;"};

    html! {
        <div {style} {onclick}>
            <TabName pitou = { pitou.clone() } updatedirectory = { prop.updatedirectory.clone() } />
            <ChooseDir {pitou} {onselectitem}/>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct ChooseDirProps {
    onselectitem: Callback<Pitou>,
    pitou: Pitou,
}

#[function_component]
fn ChooseDir(prop: &ChooseDirProps) -> Html {
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context::<ApplicationContext>().unwrap();
    let children = use_state(|| None);

    {
        let children = children.clone();
        let pitou = prop.pitou.clone();
        use_effect_with_deps(
            move |_| {
                let arg = to_value(&PitouArg { pitou: &pitou }).unwrap();
                let children = children.clone();
                spawn_local(async move {
                    let res = from_value::<Vec<Pitou>>(invoke("children_dirs", arg).await).unwrap();
                    children.set(Some(res));
                });
            },
            (),
        );
    }

    let options = children
        .as_ref()
        .map(|children| {
            children
                .into_iter()
                .map(|pitou| html! { <option selected = {false}>{pitou.name()}</option> })
                .collect::<Html>()
        })
        .unwrap_or_default();

    let width = sizes.choosedir();

    let style = format! {"
    {width}
    height: 100%;
    display: flex;
    align-items: center;
    box-sizing: border-box;
    "};

    let select_style = format! {"
    width: 100%;
    height: 100%;
    box-sizing: border-box;
    "};

    let onchange = {
        let children = children.clone();
        let onselect = prop.onselectitem.clone();
        move |e: Event| {
            let idx = e
                .target_dyn_into::<HtmlSelectElement>()
                .unwrap()
                .selected_index();
            if let Some(children) = &*children {
                if idx >= 0 || idx < children.len() as i32 {
                    onselect.emit(children[idx as usize].clone());
                }
            }
        }
    };

    html! {
        <div {style}>
            <select style = {select_style} {onchange}>
                {options}
            </select>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct TabNameProps {
    pitou: Pitou,
    updatedirectory: Callback<Pitou>,
}

#[function_component]
pub(super) fn TabName(prop: &TabNameProps) -> Html {
    let style = format! {"
    width: auto;
    height: 100%;
    padding-left: 4px;
    padding-right: 4px;
    min-width: 25px;
    display: flex;
    align-items: center;
    justify-content: center;
    "};

    let updatedirectory = {
        let pitou = prop.pitou.clone();
        let updatedirectory = prop.updatedirectory.clone();

        move |_| updatedirectory.emit(pitou.clone())
    };

    html! {
        <div {style}>
            <span onclick = { updatedirectory }>{ prop.pitou.name() }</span>
        </div>
    }
}
