use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SensorData {
    pub index: Option<i32>,
    pub label: Option<String>,
    pub value: Option<String>,
    pub value_raw: Option<String>,
}

pub fn get_sensor_settings() -> Vec<SensorData> {
    let mut table = vec![SensorData {
        index: Some(0),
        label: Some("Used RAM".to_string()),
        value: None,
        value_raw: None,
    }];

    table
}
