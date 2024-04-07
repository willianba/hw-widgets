#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod components;
mod router;
mod sensors;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
