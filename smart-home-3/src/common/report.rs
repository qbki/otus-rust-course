pub trait Report {
    fn report(&self) -> Vec<String>;
}

pub enum ReportType<'a> {
    Home,
    /// (Room name)
    Room(&'a str),
    /// (Room name, Device name)
    Device(&'a str, &'a str),
}
