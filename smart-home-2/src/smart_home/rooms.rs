use crate::common::Device;
use std::collections::HashMap;

pub struct Room {
    name: String,
    devices: HashMap<String, Box<dyn Device>>,
}

impl Room {
    pub fn new(description: &str) -> Room {
        Room {
            name: description.to_string(),
            devices: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_device(&mut self, device: Box<dyn Device>) {
        self.devices.insert(device.get_name().to_string(), device);
    }

    pub fn get_device(&self, device_name: &str) -> Option<&dyn Device> {
        self.devices.get(device_name).map(|device| device.as_ref())
    }

    pub fn get_devices(&self) -> Vec<&dyn Device> {
        self.devices
            .values()
            .map(|device| device.as_ref())
            .collect()
    }
}
