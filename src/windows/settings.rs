use yew::prelude::*;

use crate::sensors::SensorData;

#[function_component(Settings)]
pub fn settings() -> Html {
    let sensor_data = use_context::<Vec<SensorData>>().expect("Sensor data context not found");

    html! {
        <>
            <h1>{"Settings"}</h1>
            { for sensor_data.clone().iter().map(|item| html!{
                <div>
                    { item.label.as_ref().unwrap() }
                    {": "}
                    { item.value.as_ref().unwrap() }
                </div>
            })}
        </>
    }
}
