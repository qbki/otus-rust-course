mod accessor;
mod device;
mod device_interface;
mod report;
mod request_type;
mod smart_home_error_enum;
mod switch_status_enum;

pub const PRINT_OFFSET: &str = "    ";
pub const POLLING_TIMEOUT: u64 = 1000; // ms

pub use device::Device;
pub use device_interface::DeviceInterface;
pub use report::Report;
pub use request_type::RequestType;
pub use smart_home_error_enum::*;
pub use switch_status_enum::SwitchStatusEnum;
