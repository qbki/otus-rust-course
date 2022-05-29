use crate::common::{Device, SwitchStatusEnum, Print, PRINT_OFFSET};
use crate::sensors::Sensor;

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

impl Print for SmartOutlet {
    fn print(&self, depth: usize) {
        let offset = PRINT_OFFSET.repeat(depth);
        let sub_offset = PRINT_OFFSET.repeat(depth + 1);

        println!("{}Outlet: {}", offset, self.name);
        println!("{}power: {}", sub_offset, self.get_power_state());
        println!("{}consumption: {:.1}kW", sub_offset, self.get_power_units() * 0.001);
    }
}

impl Device for SmartOutlet {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn report(&self) -> String {
        let mut result = String::new();
        result.push_str(format!("Outlet name: {}\n", self.name).as_str());
        result.push_str(format!("power: {}\n", self.get_power_state()).as_str());
        result.push_str(format!("consumption: {}", self.get_power_units()).as_str());
        result
    }
}
