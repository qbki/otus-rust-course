use super::print::Print;

pub trait Device : Print {
    fn get_name(&self) -> &str;
    fn report(&self) -> String;
}
