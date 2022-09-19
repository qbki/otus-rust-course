use crate::common::{Report, SwitchStatusEnum, PRINT_OFFSET};
use std::cell::RefCell;

#[repr(C)]
#[derive(Clone)]
pub struct SmartOutlet {
    name: RefCell<String>,
    power: f64, // Power units (Watt)
    switch: SwitchStatusEnum,
}

impl SmartOutlet {
    pub fn new(name: &str) -> Self {
        Self {
            name: RefCell::new(name.to_string()),
            power: f64::default(),
            switch: SwitchStatusEnum::default(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.borrow().to_string()
    }

    pub fn set_name(&mut self, name: &str) {
        *self.name.get_mut() = name.to_string();
    }

    pub fn get_power(&self) -> f64 {
        self.power
    }

    pub fn get_switch(&self) -> SwitchStatusEnum {
        self.switch
    }

    pub fn set_switch(&mut self, value: SwitchStatusEnum) {
        self.switch = value;
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
