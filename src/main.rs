mod app;
mod components;
mod keycodes;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
