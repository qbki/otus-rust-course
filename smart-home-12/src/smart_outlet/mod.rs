use crate::common::{Report, SwitchStatusEnum, PRINT_OFFSET};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[repr(C)]
#[derive(Clone)]
pub struct SmartOutlet {
    switch: SwitchStatusEnum,
}

impl SmartOutlet {
    pub fn new() -> Self {
        Self {
            switch: SwitchStatusEnum::default(),
        }
    }

    pub fn get_power(&self) -> f64 {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::new(0, 0))
            .as_micros();
        (time % 10000) as f64
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
            format!("Outlet report"),
            format!("{}switch: {}", PRINT_OFFSET, self.get_switch()),
            format!(
                "{}consumption: {:.1}kW",
                PRINT_OFFSET,
                self.get_power() * 0.001
            ),
        ]
    }
}
