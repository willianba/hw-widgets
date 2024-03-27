use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SensorData {
    pub index: Option<i32>,
    pub label: Option<String>,
    pub value: Option<String>,
    pub value_raw: Option<String>,
}
