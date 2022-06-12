mod accessor;
mod device;
mod eq_floats;
mod report;
mod switch_status_enum;

pub const PRINT_OFFSET: &str = "    ";

pub use device::Device;
pub use eq_floats::eq_floats;
pub use report::{Report, ReportType};
pub use switch_status_enum::SwitchStatusEnum;
