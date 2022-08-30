use crate::common::{
    Device, DeviceInterface, Report, RequestType, SmartHomeErrorEnum, PRINT_OFFSET,
};
use crate::smart_outlet::SmartOutlet;
use crate::smart_room::SmartRoom;
use crate::smart_thermometer::SmartThermometer;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

pub struct SmartHome {
    name: String,
    rooms: HashMap<String, Arc<Mutex<SmartRoom>>>,
}

pub enum ResponseData {
    Room(Arc<Mutex<SmartRoom>>),
    Device(Arc<Device>),
}

pub struct Response(pub Result<ResponseData, SmartHomeErrorEnum>);

impl SmartHome {
    pub fn new(name: &str) -> SmartHome {
        SmartHome {
            name: name.to_string(),
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, room_name: &str) {
        self.rooms
            .insert(room_name.to_string(), Arc::new(Mutex::new(SmartRoom::new(room_name))));
    }

    pub fn remove_room(&mut self, room_name: &str) {
        self.rooms.remove(room_name);
    }

    pub fn add_device(&mut self, room_name: &str, device: Device) {
        if !self.rooms.contains_key(room_name) {
            self.add_room(room_name);
        }

        if let Some(room) = self.rooms.get_mut(room_name) {
            room.lock().unwrap().add_device(device);
        }
    }

    pub fn get(&mut self, request_type: &RequestType) -> Response {
        let result = match request_type {
            RequestType::Room(room_name) => self
                .rooms
                .get_mut(*room_name)
                .map(|v| ResponseData::Room(Arc::clone(v)))
                .ok_or_else(|| SmartHomeErrorEnum::NotFoundRoomError(room_name.to_string())),
            RequestType::Device(room_name, device_name) => {
                let room = self.rooms.get(*room_name);
                match room {
                    Some(room) => room
                        .lock()
                        .unwrap()
                        .get_device(device_name)
                        .map(|v| ResponseData::Device(v))
                        .ok_or_else(|| {
                            SmartHomeErrorEnum::NotFoundDeviceError(device_name.to_string())
                        }),
                    None => Err(SmartHomeErrorEnum::NotFoundRoomError(room_name.to_string())),
                }
            }
        };
        Response(result)
    }

    pub fn get_rooms(&self) -> Vec<Arc<Mutex<SmartRoom>>> {
        self.rooms.values().map(|v| Arc::clone(v)).collect()
    }
}

impl Report for SmartHome {
    fn report(&self) -> Vec<String> {
        let mut result = Vec::new();
        result.push(format!("Home: {}", self.name));

        let mut rooms = Vec::from_iter(self.rooms.values());
        rooms.sort_by(|a, b| a.lock().unwrap().get_name().cmp(b.lock().unwrap().get_name()));
        let rooms_report = rooms.into_iter().flat_map(|room| room.lock().unwrap().report());
        for line in rooms_report {
            result.push(format!("{}{}", PRINT_OFFSET, line));
        }

        result
    }
}

impl From<Response> for Result<Arc<Mutex<SmartRoom>>, SmartHomeErrorEnum> {
    fn from(value: Response) -> Self {
        match value {
            Response(Ok(ResponseData::Room(payload))) => Ok(payload.clone()),
            _ => Err(SmartHomeErrorEnum::NotFoundRoomError("Unknown".to_string())),
        }
    }
}

impl From<Response> for Result<Arc<SmartOutlet>, SmartHomeErrorEnum> {
    fn from(value: Response) -> Self {
        let make_error = || {
            Err(SmartHomeErrorEnum::NotFoundDeviceError("Unknown outlet".to_string()))
        };
        match value {
            Response(Ok(ResponseData::Device(device_wrapper))) => {
                match &*device_wrapper {
                    Device::Outlet(device) => Ok(device.clone()),
                    _ => make_error(),
                }
            }
            _ => make_error(),
        }
    }
}

impl From<Response> for Result<Arc<SmartThermometer>, SmartHomeErrorEnum> {
    fn from(value: Response) -> Self {
        let make_error = || {
            Err(SmartHomeErrorEnum::NotFoundDeviceError("Unknown thermometer".to_string()))
        };
        match value {
            Response(Ok(ResponseData::Device(device_wrapper))) => {
                match &*device_wrapper {
                    Device::Thermometer(device) => Ok(device.clone()),
                    _ => make_error(),
                }
            }
            _ => make_error(),
        }
    }
}

impl From<Response> for Result<Arc<dyn DeviceInterface>, SmartHomeErrorEnum> {
    fn from(value: Response) -> Self {
        match value {
            Response(Ok(ResponseData::Device(device))) => match &*device {
                Device::Outlet(outlet) => Ok(outlet.clone()),
                Device::Thermometer(thermometer) => Ok(thermometer.clone()),
            },
            _ => Err(SmartHomeErrorEnum::NotFoundDeviceError(
                "Unknown".to_string(),
            )),
        }
    }
}

impl From<Response> for String {
    fn from(value: Response) -> Self {
        match value.0 {
            Ok(data) => match data {
                ResponseData::Room(room) => room.lock().unwrap().report_to_string(),
                ResponseData::Device(unknown_device) => match &*unknown_device {
                    Device::Outlet(device) => device.report_to_string(),
                    Device::Thermometer(device) => device.report_to_string(),
                },
            },
            Err(error) => error.to_string(),
        }
    }
}

impl Deref for Response {
    type Target = Result<ResponseData, SmartHomeErrorEnum>;

    fn deref(&self) -> &Self::Target {
        &(self.0)
    }
}
