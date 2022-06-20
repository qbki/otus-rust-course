use crate::common::{
    Device, DeviceInterface, Report, RequestType, SmartHomeErrorEnum, PRINT_OFFSET,
};
use crate::smart_outlet::SmartOutlet;
use crate::smart_room::SmartRoom;
use crate::smart_thermometer::SmartThermometer;
use std::collections::HashMap;
use std::ops::Deref;

pub struct SmartHome {
    name: String,
    rooms: HashMap<String, Box<SmartRoom>>,
}

pub enum ResponseData<'a> {
    Home(&'a SmartHome),
    Room(&'a SmartRoom),
    Device(&'a Device),
}

pub struct Response<'a>(Result<ResponseData<'a>, SmartHomeErrorEnum>);

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

    pub fn add_device(&mut self, room_name: &str, device: Device) {
        if !self.rooms.contains_key(room_name) {
            self.add_room(room_name);
        }

        if let Some(ref mut room) = self.rooms.get_mut(room_name) {
            room.add_device(device);
        }
    }

    pub fn get(&self, request_type: &RequestType) -> Response {
        let result = match request_type {
            RequestType::Home => Result::Ok(ResponseData::Home(self)),
            RequestType::Room(room_name) => self
                .rooms
                .get(*room_name)
                .map(|v| ResponseData::Room(v.as_ref()))
                .ok_or(SmartHomeErrorEnum::NotFoundRoomError),
            RequestType::Device(room_name, device_name) => {
                let room = self.rooms.get(*room_name);
                match room {
                    Some(room) => room
                        .get_device(device_name)
                        .map(ResponseData::Device)
                        .ok_or(SmartHomeErrorEnum::NotFoundDeviceError),
                    None => Err(SmartHomeErrorEnum::NotFoundRoomError),
                }
            }
        };
        Response(result)
    }

    pub fn get_rooms(&self) -> Vec<&SmartRoom> {
        self.rooms.values().map(|v| v.as_ref()).collect()
    }
}

impl Report for SmartHome {
    fn report(&self) -> Vec<String> {
        let mut result = Vec::new();
        result.push(format!("Home: {}", self.name));

        let mut rooms = Vec::from_iter(self.rooms.values());
        rooms.sort_by(|a, b| a.get_name().cmp(b.get_name()));
        let rooms_report = rooms.into_iter().flat_map(|room| room.report());
        for line in rooms_report {
            result.push(format!("{}{}", PRINT_OFFSET, line));
        }

        result
    }
}

impl<'a> From<Response<'a>> for Result<&'a SmartRoom, SmartHomeErrorEnum> {
    fn from(value: Response<'a>) -> Self {
        match value {
            Response(Ok(ResponseData::Room(payload))) => Ok(payload),
            _ => Err(SmartHomeErrorEnum::NotFoundRoomError),
        }
    }
}

impl<'a> From<Response<'a>> for Result<&'a SmartOutlet, SmartHomeErrorEnum> {
    fn from(value: Response<'a>) -> Self {
        match value {
            Response(Ok(ResponseData::Device(Device::Outlet(payload)))) => Ok(payload),
            _ => Err(SmartHomeErrorEnum::NotFoundDeviceError),
        }
    }
}

impl<'a> From<Response<'a>> for Result<&'a SmartThermometer, SmartHomeErrorEnum> {
    fn from(value: Response<'a>) -> Self {
        match value {
            Response(Ok(ResponseData::Device(Device::Thermometer(payload)))) => Ok(payload),
            _ => Err(SmartHomeErrorEnum::NotFoundDeviceError),
        }
    }
}

impl<'a> From<Response<'a>> for Result<&'a dyn DeviceInterface, SmartHomeErrorEnum> {
    fn from(value: Response<'a>) -> Self {
        match value {
            Response(Ok(ResponseData::Device(device))) => match device {
                Device::Outlet(outlet) => Ok(outlet),
                Device::Thermometer(thermometer) => Ok(thermometer),
                Device::Generic(generic) => Ok(generic.as_ref()),
            },
            _ => Err(SmartHomeErrorEnum::NotFoundDeviceError),
        }
    }
}

impl<'a> From<Response<'a>> for String {
    fn from(value: Response<'a>) -> Self {
        match value.0 {
            Ok(data) => match data {
                ResponseData::Home(home) => home.report_to_string(),
                ResponseData::Room(room) => room.report_to_string(),
                ResponseData::Device(unknown_device) => match unknown_device {
                    Device::Outlet(device) => device.report_to_string(),
                    Device::Thermometer(device) => device.report_to_string(),
                    Device::Generic(device) => device.report_to_string(),
                },
            },
            Err(error) => error.into(),
        }
    }
}

impl<'a> Deref for Response<'a> {
    type Target = Result<ResponseData<'a>, SmartHomeErrorEnum>;

    fn deref(&self) -> &Self::Target {
        &(self.0)
    }
}
