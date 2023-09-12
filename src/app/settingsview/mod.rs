use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use crate::app::ApplicationContext;

use super::{Settings, Theme};

#[derive(PartialEq, Properties)]
pub struct SettingsViewProps {
    pub updatesettings: Callback<Settings>,
    pub updatetheme: Callback<Theme>,
}

#[function_component]
pub fn SettingsView(prop: &SettingsViewProps) -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings,
    } = use_context().unwrap();

    let onchangetheme = {
        let update = prop.updatetheme.clone();
        move |e: Event| {
            let idx = e
                .target_dyn_into::<HtmlSelectElement>()
                .unwrap()
                .selected_index();
            match idx {
                0 => update.emit(Theme::DEFAULT),
                1 => update.emit(Theme::LIGHTDEFAULT),
                2 => update.emit(Theme::LIGHTDEFAULT2),
                3 => update.emit(Theme::MAINGPT),
                4 => update.emit(Theme::MAINGPTDARK),
                _ => update.emit(Theme::MAINLIGHTDEFAULT),
            }
        }
    };

    let newfilter = Callback::from({
        let updatesettings = prop.updatesettings.clone();
        move |newfilter| {
            let mut newsettings = settings;
            newsettings.filter = newfilter;
            updatesettings.emit(newsettings)
        }
    });

    let sys_hidden = {
        let filter = newfilter.clone();
        move |_| {
            let mut newfilter = settings.filter;
            newfilter.sys_hidden = !newfilter.sys_hidden;
            filter.emit(newfilter)
        }
    };

    let dot_hidden = {
        let filter = newfilter.clone();
        move |_| {
            let mut newfilter = settings.filter;
            newfilter.dot_hidden = !newfilter.dot_hidden;
            filter.emit(newfilter)
        }
    };

    let link_hidden = {
        let filter = newfilter.clone();
        move |_| {
            let mut newfilter = settings.filter;
            newfilter.link = !newfilter.link;
            filter.emit(newfilter)
        }
    };

    let dir_hidden = {
        let filter = newfilter.clone();
        move |_| {
            let mut newfilter = settings.filter;
            newfilter.dir = !newfilter.dir;
            filter.emit(newfilter)
        }
    };

    let file_hidden = {
        let filter = newfilter.clone();
        move |_| {
            let mut newfilter = settings.filter;
            newfilter.file = !newfilter.file;
            filter.emit(newfilter)
        }
    };

    let change_refresh = {
        let updatesettings = prop.updatesettings.clone();
        move |e: Event| {
            let val = e
                .target_dyn_into::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse()
                .unwrap_or(settings.refresh_rate);
            let mut newsettings = settings;
            newsettings.refresh_rate = val;
            updatesettings.emit(newsettings)
        }
    };

    let background = theme.background2();
    let foreground = theme.spare();
    let size = sizes.pane();

    let style = format! {"
    padding: 20px;
    display: flex;
    flex-direction: column;
    align-items: center;
    {size}
    background-color: {background};
    border: 1px solid {foreground};
    box-sizing: border-box;
    "};
    html! {
        <div {style}>
            <div class="form-item">
                <label>
                    {"Change Theme:"}
                    <select class="select-box" onchange = {onchangetheme}>
                        <option selected = {theme == Theme::DEFAULT}>{"DEFAULT"}</option>
                        <option selected = {theme == Theme::LIGHTDEFAULT}>{"LIGHTDEFAULT"}</option>
                        <option selected = {theme == Theme::LIGHTDEFAULT2}>{"LIGHTDEFAULT2"}</option>
                        <option selected = {theme == Theme::MAINGPT}>{"MAINGPT"}</option>
                        <option selected = {theme == Theme::MAINGPTDARK}>{"MAINGPTDARK"}</option>
                        <option selected = {theme == Theme::MAINLIGHTDEFAULT}>{"MAINLIGHTDEFAULT"}</option>
                    </select>
                </label>
            </div>

            <div class="form-item">
                <label>
                    {"Refresh Rate"}
                    <input class="number-input" type="number" value={settings.refresh_rate.to_string()} onchange={change_refresh} min={1} max={255} />
                </label>
            </div>

            <div class="form-item">
                <span>
                    {"Hidden Files"}
                </span>
                <label>
                    {"System files"}
                    <input type = "checkbox" checked = {settings.filter.sys_hidden} onchange = {sys_hidden}/>
                </label>
                <label>
                    {"Dot Files"}
                    <input type = "checkbox" checked = {settings.filter.dot_hidden} onchange = {dot_hidden}/>
                </label>
                <label>
                    {"Symlink Files"}
                    <input type = "checkbox" checked = {settings.filter.link} onchange = {link_hidden}/>
                </label>
                <label>
                    {"Directories"}
                    <input type = "checkbox" checked = {settings.filter.dir} onchange = {dir_hidden}/>
                </label>
                <label>
                    {"Files"}
                    <input type = "checkbox" checked = {settings.filter.file} onchange = {file_hidden}/>
                </label>
            </div>
        </div>
    }
}
