use crate::accessors;
use crate::common::{DeviceInterface, Report, SwitchStatusEnum, PRINT_OFFSET};
use std::sync::{Arc, Mutex};

pub struct SmartOutlet {
    name: String,
    power: Arc<Mutex<f64>>, // Power units (Watt)
    switch: Arc<Mutex<SwitchStatusEnum>>,
}

impl SmartOutlet {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            power: Arc::new(Mutex::new(f64::default())),
            switch: Arc::new(Mutex::new(SwitchStatusEnum::default())),
        }
    }

    accessors!(get_power, set_power, power, f64);

    accessors!(get_switch, set_switch, switch, SwitchStatusEnum);
}

impl DeviceInterface for SmartOutlet {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Report for SmartOutlet {
    fn report(&self) -> Vec<String> {
        vec![
            format!("Outlet: {}", self.get_name()),
            format!("{}switch: {}", PRINT_OFFSET, self.get_switch()),
            format!(
                "{}consumption: {:.1}kW",
                PRINT_OFFSET,
                self.get_power() * 0.001
            ),
        ]
    }
}
