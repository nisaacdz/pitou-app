use yew::prelude::*;

use crate::app::ApplicationContext;
use backend::{KeyType, SearchArea, SearchOptions};
use web_sys::{HtmlInputElement, HtmlSelectElement};

#[derive(PartialEq, Properties)]
pub struct SearchOptionsCmpProp {
    pub onsubmit: Callback<(String, SearchOptions)>,
    //collapse: Callback<()>,
}

#[function_component]
pub fn SearchOptionsCmp(prop: &SearchOptionsCmpProp) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context().unwrap();
    let options = use_state_eq(|| SearchOptions::new());
    let input_ref = use_node_ref();

    let background_color = theme.background2();
    let border_color = theme.spare();

    let size = sizes.sidepane();
    let input_sz = sizes.search_input();

    let input_style = format! {"
    border: 3px solid {border_color};
    {input_sz}
    box-sizing: border-box;
    "};

    let base_box_style = format! {"
    border: 3px solid {border_color};
    "};

    let style = format! {"
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    {size}
    background-color: {background_color};
    border: 1px solid {border_color};
    box-sizing: border-box;
    "};

    let button_background_color = theme.background1();
    let button_size = sizes.sidepane_possible_button();
    let button_text_color = theme.foreground1();

    let submit_button_style = format! {"
    background-color: {button_background_color};
    color: {button_text_color};
    {button_size}
    "};

    let onsubmit = {
        let options = options.clone();
        let onsubmit = prop.onsubmit.clone();
        let input_ref = input_ref.clone();
        move |e: SubmitEvent| {
            e.prevent_default();
            let key = input_ref.cast::<HtmlInputElement>().unwrap().value();
            onsubmit.emit((key, *options))
        }
    };

    let onchange = {
        let options = options.clone();
        move |e: Event| {
            let idx = e
                .target_dyn_into::<HtmlSelectElement>()
                .unwrap()
                .selected_index();
            match idx {
                0 => options.set(options.match_beginning()),
                1 => options.set(options.match_regex()),
                _ => (),
            }
        }
    };

    let onchangedepth = {
        let options = options.clone();
        move |e: Event| {
            let val = e.target_dyn_into::<HtmlInputElement>().unwrap().value();
            options.set(options.depth(val.parse().unwrap_or(1)))
        }
    };

    let togglecasesensitive = {
        let options = options.clone();
        move |_| options.set(options.toggle_case_sensitive())
    };

    let toggleincludedirs = {
        let options = options.clone();
        move |_| options.set(options.toggle_include_dirs())
    };

    let toggleincludefiles = {
        let options = options.clone();
        move |_| options.set(options.toggle_include_files())
    };

    let toggleincludelinks = {
        let options = options.clone();
        move |_| options.set(options.toggle_include_links())
    };

    let entries = {
        let onchange = {
            let options = options.clone();
            move |e: Event| {
                let idx = e
                    .target_dyn_into::<HtmlSelectElement>()
                    .unwrap()
                    .selected_index();
                match idx {
                    0 => options.set(options.match_beginning()),
                    1 => options.set(options.match_anywhere()),
                    2 => options.set(options.match_ending()),
                    _ => (),
                }
            }
        };

        match (*options).keytype {
            KeyType::Regex => html! {},
            KeyType::RawSearch(rt) => html! {
                <div class="search-type">
                    <label>
                        {"Standard Search Type:"}
                        <select class="select-box" value={format!{"{}", rt}} {onchange} style = {base_box_style.clone()}>
                            <option selected = {matches!(rt, SearchArea::StartsWith )}>{"StartsWith"}</option>
                            <option selected = {matches!(rt, SearchArea::Contains )}>{"Contains"}</option>
                            <option selected = {matches!(rt, SearchArea::EndsWith )}>{"EndsWith"}</option>
                        </select>
                    </label>
                </div>
            },
        }
    };

    let placeholder = format! {"Enter search key"};

    html! {
        <div class="search-options" style={style}>
            <input style={input_style} class="search-input" type="text" ref={input_ref} {placeholder}/>
            <span class="title">{"Search Options"}</span>
            <form {onsubmit}>
                <div class="form-item">
                    <label>
                        {"Type:"}
                        <select class="select-box" style = {base_box_style} {onchange}>
                            <option selected = {matches!{(*options).keytype, KeyType::RawSearch(_)}}>{"Standard"}</option>
                            <option selected = {matches!{(*options).keytype, KeyType::Regex}}>{"Regex"}</option>
                        </select>
                    </label>
                </div>
                {entries}
                <div class="form-item">
                    <label>
                        {"Depth:"}
                        <input class="number-input" type="number" value={(*options).depth.to_string()} onchange={onchangedepth} min={1} max={28} />
                    </label>
                </div>
                <div class="form-item">
                    <label>
                        {"Case Sensitive:"}
                        <input class="checkbox-input" type="checkbox" checked={(&*options).case_sensitive} onchange={togglecasesensitive} />
                    </label>
                </div>
                <div class="form-item">
                    <label>
                        {"Include Files:"}
                        <input class="checkbox-input" type="checkbox" checked={(&*options).include_files} onchange={toggleincludefiles} />
                    </label>
                </div>
                <div class="form-item">
                    <label>
                        {"Include Folders:"}
                        <input class="checkbox-input" type="checkbox" checked={(&*options).include_dirs} onchange={toggleincludedirs} />
                    </label>
                </div>
                <div class="form-item">
                    <label>
                        {"Include Shortcuts:"}
                        <input class="checkbox-input" type="checkbox" checked={(&*options).include_links} onchange={toggleincludelinks} />
                    </label>
                </div>
                <button class="submit-button card" type="submit" style = {submit_button_style}>{"Search"}</button>
            </form>
        </div>
    }
}
