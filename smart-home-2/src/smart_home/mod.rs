mod rooms;

use std::collections::HashMap;
use rooms::Room;
use crate::common::{Device, Print, Report, PRINT_OFFSET};

pub struct SmartHome {
    name: String,
    rooms: HashMap<String, Box<Room>>,
}

impl SmartHome {
    pub fn new(name: &str) -> SmartHome {
        SmartHome {
            name: name.to_string(),
            rooms: HashMap::new()
        }
    }

    pub fn add_room(&mut self, room_name: &str) {
        self.rooms.insert(
            room_name.to_string(),
            Box::new(Room::new(room_name),
        ));
    }

    pub fn add_device(&mut self, room_name: &str, device: Box<dyn Device>) {
        if !self.rooms.contains_key(room_name) {
            self.add_room(room_name);
        }

        if let Some(ref mut room) = self.rooms.get_mut(room_name) {
            room.add_device(device);
        }
    }

    pub fn get_rooms(&self) -> Vec<&Box<Room>> {
        self.rooms.values().collect()
    }

    pub fn get_devices_from(&self, room_name: &str) -> Option<Vec<&Box<dyn Device>>> {
        match self.rooms.get(room_name) {
            Some(room) => Some(room.get_devices()),
            None => None,
        }
    }
}

impl Print for SmartHome {
    fn print(&self, depth: usize) {
        println!("{}Smart home: {}", PRINT_OFFSET.repeat(depth), self.name);

        let mut rooms = self.get_rooms();
        rooms.sort_by(|a, b| a.get_name().cmp(&b.get_name()));
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
            .and_then(|device| Some(device.report()))
    }
}
