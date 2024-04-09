#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod router;
mod sensors;
mod windows;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
