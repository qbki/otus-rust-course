pub trait Report {
    fn report(&self) -> Vec<String>;

    fn report_to_string(&self) -> String {
        self.report().join("\n")
    }
}
