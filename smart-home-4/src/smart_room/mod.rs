use std::rc::Rc;
use std::collections::HashMap;
use crate::common::{Device, DeviceInterface, Report, PRINT_OFFSET};

pub struct SmartRoom {
    name: String,
    devices: HashMap<String, Rc<Device>>,
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
        self.devices.insert(device.get_name().to_string(), Rc::new(device));
    }

    pub fn get_device(&self, device_name: &str) -> Option<&Device> {
        self.devices.get(device_name).map(|device| device.as_ref())
    }

    pub fn get_devices(&self) -> Vec<&Device> {
        self.devices
            .values()
            .map(|device| device.as_ref())
            .collect()
    }
}

impl Report for SmartRoom {
    fn report(&self) -> Vec<String> {
        let mut result = Vec::new();
        result.push(format!("Room: {}", self.get_name()));

        let mut devices = self.get_devices();
        devices.sort_by(|a, b| a.get_name().cmp(b.get_name()));
        let devices_report = devices.into_iter().flat_map(|device| device.report());
        for line in devices_report {
            result.push(format!("{}{}", PRINT_OFFSET, line));
        }

        result
    }
}
