mod rooms;

use crate::common::{Device, Print, Report, PRINT_OFFSET};
use rooms::Room;
use std::collections::HashMap;

pub struct SmartHome {
    name: String,
    rooms: HashMap<String, Box<Room>>,
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
            .insert(room_name.to_string(), Box::new(Room::new(room_name)));
    }

    pub fn add_device(&mut self, room_name: &str, device: Box<dyn Device>) {
        if !self.rooms.contains_key(room_name) {
            self.add_room(room_name);
        }

        if let Some(ref mut room) = self.rooms.get_mut(room_name) {
            room.add_device(device);
        }
    }

    pub fn get_rooms(&self) -> Vec<&Room> {
        self.rooms.values().map(|v| v.as_ref()).collect()
    }

    pub fn get_devices_from(&self, room_name: &str) -> Option<Vec<&dyn Device>> {
        self.rooms.get(room_name).map(|room| room.get_devices())
    }
}

impl Print for SmartHome {
    fn print(&self, depth: usize) {
        println!("{}Smart home: {}", PRINT_OFFSET.repeat(depth), self.name);

        let mut rooms = self.get_rooms();
        rooms.sort_by(|a, b| a.get_name().cmp(b.get_name()));
        for room in rooms {
            room.print(depth + 1);
        }
    }
}

impl Report for SmartHome {
    fn report(&self, room_name: &str, device_name: &str) -> Option<String> {
        self.rooms
            .get(room_name)
            .and_then(|room| room.get_device(device_name))
            .map(|device| device.report())
    }
}
