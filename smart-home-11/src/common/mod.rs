mod accessor;
mod report;
mod switch_status_enum;

pub const PRINT_OFFSET: &str = "    ";
pub const POLLING_TIMEOUT: u64 = 1000; // ms

pub use report::Report;
pub use switch_status_enum::SwitchStatusEnum;
