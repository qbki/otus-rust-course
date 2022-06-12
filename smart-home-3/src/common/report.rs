pub trait Report {
    fn report(&self) -> Vec<String>;
}

pub enum ReportType<'a> {
    #[allow(dead_code)]
    Home,
    /// (Room name)
    #[allow(dead_code)]
    Room(&'a str),
    /// (Room name, Device name)
    Device(&'a str, &'a str),
}
