use crate::common::{Device, PRINT_OFFSET};
use crate::sensors::Sensor;

pub struct SmartThermometer {
    name: String,
    temperature_sensor: Box<dyn Sensor<f64>>,
}

impl SmartThermometer {
    pub fn new(name: String, temperature_sensor: Box<dyn Sensor<f64>>) -> Self {
        Self {
            name,
            temperature_sensor,
        }
    }

    pub fn get_temperature(&self) -> f64 {
        self.temperature_sensor.sample()
    }
}

impl Device for SmartThermometer {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn report(&self) -> Vec<String> {
        vec![
            format!("Thermometer: {}", self.name),
            format!("{}temperature: {:.1}°C", PRINT_OFFSET, self.get_temperature()),
        ]
    }
}
