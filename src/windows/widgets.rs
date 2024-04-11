use yew::prelude::*;

use crate::sensors::SensorData;

#[function_component(Widgets)]
pub fn widgets() -> Html {
    let sensor_data = use_context::<Vec<SensorData>>().expect("Sensor data context not found");

    html! {
        <>
            <h1>{"System Monitor"}</h1>
            <ul>
                { for sensor_data.clone().iter().map(|item| html!{
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
