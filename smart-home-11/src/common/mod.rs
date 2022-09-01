mod accessor;
mod dsc_error;
mod report;
mod request_type;
mod smart_home_error_enum;
mod switch_status_enum;

pub const PRINT_OFFSET: &str = "    ";
pub const POLLING_TIMEOUT: u64 = 1000; // ms
pub const UI_UPDATE_TIMEOUT: u64 = 1000; // ms

pub use dsc_error::*;
pub use report::Report;
pub use request_type::RequestType;
pub use smart_home_error_enum::*;
pub use switch_status_enum::SwitchStatusEnum;
