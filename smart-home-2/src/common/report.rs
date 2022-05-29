pub trait Report {
    fn report(&self, room_name: &str, device_name: &str) -> Option<String>;
}

pub fn report<T: Report>(reporter: &T, room_name: &str, device_name: &str) -> String {
    match reporter.report(room_name, device_name) {
        Some(message) => message,
        None => "Device was not found".to_string(),
    }
}
