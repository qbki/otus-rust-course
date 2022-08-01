use crate::common::Report;

pub trait DeviceInterface: Report {
    fn get_name(&self) -> String;
}
