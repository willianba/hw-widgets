use crate::sensors::{get_sensor_data, SensorData};

#[derive(Clone, Default)]
pub struct AppState {
    pub always_on_top: bool,
    pub sensor_data: Vec<SensorData>,
}

impl AppState {
    pub fn refresh_sensors(&mut self) {
        self.sensor_data = get_sensor_data();
    }
}
