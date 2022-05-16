#[derive(Debug)]
pub enum SwitchStateEnum {
    On,
    Off,
}

pub struct SmartOutlet {
    description: String,
    power_state: SwitchStateEnum,
    last_power_consumption_mesurement: f64,
}

impl SmartOutlet {
    pub fn new(description: String) -> Self {
        Self {
            description,
            power_state: SwitchStateEnum::Off,
            last_power_consumption_mesurement: 0.0
        }
    }

    pub fn power_state(&self) -> &SwitchStateEnum {
        &self.power_state
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn get_power_units(&self) -> f64 {
        self.last_power_consumption_mesurement * 0.001
    }

    pub fn enable(&mut self) {
        self.power_state = SwitchStateEnum::On;
    }

    pub fn disable(&mut self) {
        self.power_state = SwitchStateEnum::Off;
    }

    pub fn set_power_consumption(&mut self, volts: f64, ampers: f64) {
        self.last_power_consumption_mesurement = volts * ampers;
    }
}
