use crate::common::{Device, Print, PRINT_OFFSET};
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

impl Print for SmartThermometer {
    fn print(&self, depth: usize) {
        let offset = PRINT_OFFSET.repeat(depth);
        let sub_offset = PRINT_OFFSET.repeat(depth + 1);

        println!("{}Thermometer: {}", offset, self.name);
        println!("{}temperature: {:.1}°C", sub_offset, self.get_temperature());
    }
}

impl Device for SmartThermometer {
    fn get_name(&self) -> &str {
        &self.name
    }
}
