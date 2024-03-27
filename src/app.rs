use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::router::Router;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str) -> JsValue;
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Router />
    }
}
