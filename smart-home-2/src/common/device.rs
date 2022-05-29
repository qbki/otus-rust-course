pub trait Device {
    fn get_name(&self) -> &str;
    fn report(&self) -> Vec<String>;
}
