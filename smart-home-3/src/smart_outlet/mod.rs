use crate::common::{Device, SwitchStatusEnum, Report, PRINT_OFFSET};
use crate::sensors::Sensor;
#[cfg(test)]
mod smart_outlet_tests;

pub struct SmartOutlet {
    name: String,
    amperage_sensor: Box<dyn Sensor<f64>>,
    voltage_sensor: Box<dyn Sensor<f64>>,
    switch_state_sensor: Box<dyn Sensor<SwitchStatusEnum>>,
}

impl SmartOutlet {
    pub fn new(
        name: String,
        amperage_sensor: Box<dyn Sensor<f64>>,
        voltage_sensor: Box<dyn Sensor<f64>>,
        switch_state_sensor: Box<dyn Sensor<SwitchStatusEnum>>,
    ) -> Self {
        Self {
            name,
            amperage_sensor,
            voltage_sensor,
            switch_state_sensor,
        }
    }

    pub fn get_power_state(&self) -> SwitchStatusEnum {
        self.switch_state_sensor.sample()
    }

    pub fn get_power_units(&self) -> f64 {
        self.amperage_sensor.sample() * self.voltage_sensor.sample()
    }
}

impl Device for SmartOutlet {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl Report for SmartOutlet {
    fn report(&self) -> Vec<String> {
        vec![
            format!("Outlet: {}", self.name),
            format!("{}power: {}", PRINT_OFFSET, self.get_power_state()),
            format!(
                "{}consumption: {:.1}kW",
                PRINT_OFFSET,
                self.get_power_units() * 0.001
            ),
        ]
    }
}
