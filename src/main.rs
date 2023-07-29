mod app;
pub mod data;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
