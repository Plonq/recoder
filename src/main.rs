mod app;
mod components;
mod engine;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
