use crate::common::{Device, DeviceInterface, Report, PRINT_OFFSET};
use std::collections::HashMap;
use std::sync::Arc;

pub struct SmartRoom {
    name: String,
    devices: HashMap<String, Arc<Device>>,
}

impl SmartRoom {
    pub fn new(description: &str) -> SmartRoom {
        SmartRoom {
            name: description.to_string(),
            devices: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_device(&mut self, device: Device) {
        self.devices.insert(device.get_name(), Arc::new(device));
    }

    pub fn remove_device(&mut self, name: &str) {
        self.devices.remove(name);
    }

    pub fn get_device(&self, device_name: &str) -> Option<&Device> {
        self.devices.get(device_name).map(|device| device.as_ref())
    }

    pub fn get_devices(&self) -> Vec<Arc<Device>> {
        self.devices
            .values()
            .map(|device| device.clone())
            .collect()
    }
}

impl Report for SmartRoom {
    fn report(&self) -> Vec<String> {
        let mut result = Vec::new();
        result.push(format!("Room: {}", self.get_name()));

        let mut devices = self.get_devices();
        devices.sort_by_key(|v| v.get_name());
        let devices_report = devices.into_iter().flat_map(|device| device.report());
        for line in devices_report {
            result.push(format!("{}{}", PRINT_OFFSET, line));
        }

        result
    }
}
