use std::collections::HashMap;
use crate::common::{Device, Print, PRINT_OFFSET};

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
        self.devices.insert(
            device.get_name().to_string(),
            device,
        );
    }

    pub fn get_device(&self, device_name: &str) -> Option<&Box<dyn Device>> {
        self.devices.get(device_name)
    }

    pub fn get_devices(&self) -> Vec<&Box<dyn Device>> {
        self.devices.values().collect()
    }
}

impl Print for Room {
    fn print(&self, depth: usize) {
        println!("{}Room: {}", PRINT_OFFSET.repeat(depth), self.name);

        let mut devices = self.get_devices();
        devices.sort_by(|a, b| a.get_name().cmp(&b.get_name()));
        for device in devices {
            device.print(depth + 1);
        }
    }
}
