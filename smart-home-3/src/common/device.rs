use crate::common::Report;

pub trait Device : Report {
    fn get_name(&self) -> &str;
}
