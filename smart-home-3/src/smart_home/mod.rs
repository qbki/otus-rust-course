use crate::common::{Device, Report, ReportType, PRINT_OFFSET};
use crate::smart_room::SmartRoom;
use std::collections::HashMap;

pub struct SmartHome {
    name: String,
    rooms: HashMap<String, Box<SmartRoom>>,
}

impl SmartHome {
    pub fn new(name: &str) -> SmartHome {
        SmartHome {
            name: name.to_string(),
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, room_name: &str) {
        self.rooms
            .insert(room_name.to_string(), Box::new(SmartRoom::new(room_name)));
    }

    pub fn add_device(&mut self, room_name: &str, device: Box<dyn Device>) {
        if !self.rooms.contains_key(room_name) {
            self.add_room(room_name);
        }

        if let Some(ref mut room) = self.rooms.get_mut(room_name) {
            room.add_device(device);
        }
    }

    pub fn get_rooms(&self) -> Vec<&SmartRoom> {
        self.rooms.values().map(|v| v.as_ref()).collect()
    }

    pub fn get_devices_from(&self, room_name: &str) -> Option<Vec<&dyn Device>> {
        self.rooms.get(room_name).map(|room| room.get_devices())
    }

    pub fn report_by(&self, report_type: &ReportType) -> String {
        let lines = match report_type {
            ReportType::Home => Some(self.report()),
            ReportType::Room(room_name) => {
                self.rooms
                    .get(*room_name)
                    .map(|room| room.report())
            }
            ReportType::Device(room_name, device_name) => {
                self.rooms
                    .get(*room_name)
                    .and_then(|room| {
                        room.get_device(&device_name).map(|v| v.report())
                    })
            }

        };

        match lines {
            Some(array) => array.join("\n"),
            None => "Device was not found".to_string(),
        }
    }
}

impl Report for SmartHome {
    fn report(&self) -> Vec<String> {
        let mut result = Vec::new();
        result.push(format!("Home: {}", self.name));

        let mut rooms = Vec::from_iter(self.rooms.values());
        rooms.sort_by(|a, b| a.get_name().cmp(b.get_name()));
        let rooms_report = rooms
            .into_iter()
            .flat_map(|room| room.report());
        for line in rooms_report {
            result.push(format!("{}{}", PRINT_OFFSET, line));
        }

        result
    }
}
