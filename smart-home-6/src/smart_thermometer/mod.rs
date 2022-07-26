use crate::common::{DeviceInterface, Report, PRINT_OFFSET};
#[cfg(test)]
mod smart_thermometer_tests;

use crate::accessors;
use std::cell::Cell;

#[derive(Clone)]
pub struct SmartThermometer {
    name: String,
    temperature: Cell<f64>,
}

impl SmartThermometer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            temperature: Cell::new(f64::default()),
        }
    }

    accessors!(get_temperature, set_temperature, temperature, f64);
}

impl DeviceInterface for SmartThermometer {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Report for SmartThermometer {
    fn report(&self) -> Vec<String> {
        vec![
            format!("Thermometer: {}", self.name),
            format!(
                "{}temperature: {:.1}°C",
                PRINT_OFFSET,
                self.get_temperature()
            ),
        ]
    }
}
