use yew::prelude::*;

use crate::app::ApplicationContext;

mod preloaders;
mod statics;

use preloaders::*;
use statics::*;

#[function_component]
pub fn BottomPane() -> Html {
    let ApplicationContext {
        theme,
        sizes,
        settings: _,
    } = use_context().unwrap();

    let background_color = theme.background1();
    let preloader_foreground = theme.foreground1();
    let size = sizes.bottombar();

    let style = format! {"
    {size}
    background-color: {background_color};
    box-sizing: border-box;
    --preloader-foreground: {preloader_foreground};
    "};

    let inner_style = format! {"
    width: 100%;
    height: 100%;
    box-sizing: border-box;
    display: flex;
    align-items: center;
    justify-content: center;
    "};

    html! {
        <div {style}>
            <div style = {inner_style}>
                <FolderInfo />
                <SearchPreloader />
                <DeletePreloader />
                <CutPreloader />
                <CopyPreloader />
                <RenamePreloader />
                <AddFolderPreloader />
                <AddFilePreloader />
                <PastePreloader />
            </div>
        </div>
    }
}
