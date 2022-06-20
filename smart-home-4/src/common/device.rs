use crate::common::{DeviceInterface, Report};
use crate::smart_outlet::SmartOutlet;
use crate::smart_thermometer::SmartThermometer;

pub enum Device {
    Outlet(SmartOutlet),
    Thermometer(SmartThermometer),
    Generic(Box<dyn DeviceInterface>),
}

impl Report for Device {
    fn report(&self) -> Vec<String> {
        match self {
            Device::Outlet(device) => device.report(),
            Device::Thermometer(device) => device.report(),
            Device::Generic(device) => device.report(),
        }
    }
}

impl DeviceInterface for Device {
    fn get_name(&self) -> &str {
        match self {
            Device::Outlet(device) => device.get_name(),
            Device::Thermometer(device) => device.get_name(),
            Device::Generic(device) => device.get_name(),
        }
    }
}
