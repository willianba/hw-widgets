use std::sync::{Arc, Mutex};

use crate::sensors::{get_sensor_data, SensorData};

#[derive(Clone, Default)]
pub struct SensorState {
    pub data: Arc<Mutex<Vec<SensorData>>>,
}

impl SensorState {
    pub async fn update(&self) {
        let new_data = get_sensor_data();
        let mut data = self.data.lock().expect("Failed to lock data");
        *data = new_data;
    }
}
