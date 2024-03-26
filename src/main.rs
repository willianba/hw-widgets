mod app;
mod sensors;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
