use std::process::Command;

use serde::Serialize;

#[derive(Default, Debug, Serialize, Clone)]
pub struct SensorData {
    pub index: Option<u8>,
    pub label: Option<String>,
    pub value: Option<String>,
    pub value_raw: Option<String>,
}

pub struct Sensors {
    pub data: Vec<SensorData>,
}

impl Sensors {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn get_data(&mut self) -> Vec<SensorData> {
        self.update_data();
        self.data.clone()
    }

    fn update_data(&mut self) -> () {
        let reg_data =
            self.read_values_from_registry().expect("Failed to read values from registry");
        let parsed = self.parse_sensor_data(reg_data);
        self.data = parsed;
    }

    fn read_values_from_registry(&self) -> Result<String, String> {
        let output = Command::new("reg")
            .args(&["query", r"HKEY_CURRENT_USER\SOFTWARE\HWiNFO64\VSB"])
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    let result = String::from_utf8_lossy(&output.stdout).to_string();
                    Ok(result)
                } else {
                    // TODO handle gracefully. show something in the UI
                    Err("Ensure hwinfo is running".into())
                }
            }
            // TODO handle gracefully. show something in the UI
            Err(_) => Err("Failed accessing Windows registry".into()),
        }
    }

    fn parse_sensor_data(&self, data: String) -> Vec<SensorData> {
        let mut sensors: Vec<SensorData> = Vec::new();
        let mut current_sensor: SensorData = SensorData::default();

        for line in data.lines() {
            if let Some((key, value)) = line.split_once("REG_SZ") {
                let key = key.trim();
                let value = value.trim();

                if let Some((sensor, num)) = split_sensor_and_number(key) {
                    match sensor.trim() {
                        "Label" => {
                            if current_sensor.label.is_some() {
                                sensors.push(current_sensor);
                                current_sensor = SensorData::default();
                            }
                            current_sensor.label = Some(value.to_string());
                            current_sensor.index = Some(num.parse().unwrap());
                        }
                        "Value" => current_sensor.value = Some(value.to_string()),
                        "ValueRaw" => current_sensor.value_raw = Some(value.to_string()),
                        _ => {}
                    }
                }
            }
        }

        // Don't forget to add the last sensor entry to the list
        if current_sensor.label.is_some() {
            sensors.push(current_sensor);
        }

        sensors
    }
}

// moved outside of the impl block so i don't have to use self and deal with lifetimes
fn split_sensor_and_number(input: &str) -> Option<(&str, &str)> {
    let index = input.chars().position(|c| c.is_numeric());
    match index {
        Some(idx) => {
            let (sensor, num) = input.split_at(idx);
            Some((sensor, num))
        }
        None => None,
    }
}
