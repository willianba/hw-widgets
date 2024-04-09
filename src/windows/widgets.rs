use gloo_timers::future::TimeoutFuture;
use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{app::invoke, sensors::SensorData};

#[function_component(Widgets)]
pub fn widgets() -> Html {
    let data = use_state(|| Vec::<SensorData>::new());

    {
        let data = data.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                loop {
                    TimeoutFuture::new(2_000).await;
                    let stats_js = invoke("stats").await;
                    match JsValue::into_serde::<Vec<SensorData>>(&stats_js) {
                        Ok(stats) => {
                            data.set(stats);
                        }
                        Err(e) => {
                            eprintln!("Failed to parse stats: {:?}", e);
                            data.set(Vec::new());
                        }
                    }
                }
            });
            || ()
        });
    }

    html! {
        <>
            <h1>{"System Monitor"}</h1>
            <ul>
                { for (*data).clone().iter().map(|item| html!{
                    <li>
                        { item.label.as_ref().unwrap() }
                        {": "}
                        { item.value.as_ref().unwrap() }
                    </li>
                })}
            </ul>
        </>
    }
}
