mod rooms;

use crate::common::{Device, Report, PRINT_OFFSET};
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

    pub fn print(&self) {
        let room_offset = PRINT_OFFSET.repeat(1);
        let device_offset = PRINT_OFFSET.repeat(2);

        println!("Home: {}", self.name);

        let mut rooms = Vec::from_iter(self.rooms.values());
        rooms.sort_by(|a, b| a.get_name().cmp(b.get_name()));

        for room in rooms {
            println!("{}Room: {}", room_offset, room.get_name());

            let mut devices = room.get_devices();
            devices.sort_by(|a, b| a.get_name().cmp(b.get_name()));

            devices.into_iter()
                .map(|device| device.report())
                .flatten()
                .for_each(|message| println!("{}{}", device_offset, message));
        }
    }
}

impl Report for SmartHome {
    fn report(&self, room_name: &str, device_name: &str) -> Option<Vec<String>> {
        self.rooms
            .get(room_name)
            .and_then(|room| room.get_device(device_name))
            .map(|device| device.report())
    }
}
