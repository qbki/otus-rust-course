pub struct SmartThermometer {
    description: String,
    last_temperature_mesurement: f64,
}

impl SmartThermometer {
    pub fn new(description: String) -> Self {
        Self {
            description,
            last_temperature_mesurement: 0.0,
        }
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn get_temperature(&self) -> f64 {
        self.last_temperature_mesurement
    }

    pub fn set_temperature(&mut self, temperature: f64) {
        self.last_temperature_mesurement = temperature;
    }
}
