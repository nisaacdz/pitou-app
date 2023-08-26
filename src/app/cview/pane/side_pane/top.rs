use backend::Pitou;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::app::ApplicationContext;

#[derive(PartialEq, Properties)]
pub struct TopOfParentDirProps {
    pub selected: Option<Pitou>,
    pub onfilter: Callback<String>,
    pub onenter: Callback<String>,
}

#[function_component]
pub fn TopOfParentDir(prop: &TopOfParentDirProps) -> Html {
    let ApplicationContext {
        theme: _,
        sizes,
        settings: _,
    } = use_context::<ApplicationContext>().unwrap();
    let input = use_state(|| String::new());

    {
        let onfilter = prop.onfilter.clone();
        use_effect_with_deps(
            move |input| {
                onfilter.emit((**input).clone());
            },
            input.clone(),
        );
    }

    let height = sizes.dsc().height();

    let style = format! {"
    {height}
    box-sizing: border-box;
    display: flex;
    align-items: center;
    justify-content: center;
    "};

    let placeholder = prop
        .selected
        .as_ref()
        .map(|item| item.name())
        .unwrap_or_default();

    let onsubmit = {
        let onenter = prop.onenter.clone();
        let input = input.clone();
        move |e: SubmitEvent| {
            e.prevent_default();
            let newstr = (*input).clone();
            input.set(String::new());
            onenter.emit(newstr);
        }
    };

    let oninput = {
        let input = input.clone();
        move |e: InputEvent| {
            let strval = e.target_dyn_into::<HtmlInputElement>().unwrap().value();
            input.set(strval);
        }
    };

    let value = (&*input).clone();

    html! {
        <form {style} {onsubmit}>
            <input {value} {oninput} {placeholder}/>
        </form>
    }
}
