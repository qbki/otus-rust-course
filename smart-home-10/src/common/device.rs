use crate::common::{DeviceInterface, Report};
use crate::smart_outlet::SmartOutlet;
use crate::smart_thermometer::SmartThermometer;
use std::sync::Arc;

pub enum Device {
    Outlet(Arc<SmartOutlet>),
    Thermometer(Arc<SmartThermometer>),
}

impl Report for Device {
    fn report(&self) -> Vec<String> {
        match self {
            Device::Outlet(device) => device.report(),
            Device::Thermometer(device) => device.report(),
        }
    }
}

impl DeviceInterface for Device {
    fn get_name(&self) -> String {
        match self {
            Device::Outlet(device) => device.get_name(),
            Device::Thermometer(device) => device.get_name(),
        }
    }
}
