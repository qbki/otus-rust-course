use crate::accessors;
use crate::common::{DeviceInterface, Report, SwitchStatusEnum, PRINT_OFFSET};
use std::cell::Cell;

#[cfg(test)]
mod smart_outlet_tests;

pub struct SmartOutlet {
    name: String,
    // Power units (Watt)
    power: Cell<f64>,
    switch: Cell<SwitchStatusEnum>,
}

impl SmartOutlet {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            power: Cell::new(f64::default()),
            switch: Cell::new(SwitchStatusEnum::default()),
        }
    }

    accessors!(get_power, set_power, power, f64);

    accessors!(get_switch, set_switch, switch, SwitchStatusEnum);
}

impl DeviceInterface for SmartOutlet {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl Report for SmartOutlet {
    fn report(&self) -> Vec<String> {
        vec![
            format!("Outlet: {}", self.name),
            format!("{}switch: {}", PRINT_OFFSET, self.get_switch()),
            format!(
                "{}consumption: {:.1}kW",
                PRINT_OFFSET,
                self.get_power() * 0.001
            ),
        ]
    }
}
