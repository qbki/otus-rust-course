mod common;
mod mocks;
mod smart_home;
mod smart_outlet;
mod smart_room;
mod smart_thermometer;

#[macro_use] extern crate rocket;
use mocks::make_home;
use rocket::{State, FromForm};
use rocket::form::Form;
use rocket::http::Status;
use common::RequestType;
use common::Device;
use crate::common::{Report, SmartHomeErrorEnum, DeviceInterface};
use crate::smart_home::SmartHome;
use crate::smart_room::SmartRoom;
use crate::smart_outlet::SmartOutlet;
use crate::smart_thermometer::SmartThermometer;
use std::sync::{Arc, Mutex};


struct HomeState(Mutex<SmartHome>);

#[derive(FromForm)]
struct RoomFormData {
    name: String,
}

#[derive(FromForm)]
struct DeviceFormData {
    name: String,
    device_type: String, // OUTLET | THERMOMETER
}

fn acquire_room(home: &State<HomeState>, room_name: &str) -> Result<Arc<Mutex<SmartRoom>>, SmartHomeErrorEnum> {
    home.0
        .lock()
        .unwrap()
        .get(&RequestType::Room(room_name))
        .into()
}

fn acquire_device(home: &State<HomeState>, room_name: &str, device_name: &str) -> Result<Arc<dyn DeviceInterface>, SmartHomeErrorEnum> {
    home.0
        .lock()
        .unwrap()
        .get(&RequestType::Device(room_name, device_name))
        .into()
}

#[get("/")]
fn get_home(home: &State<HomeState>) -> String {
    home.0.lock().unwrap().report_to_string()
}

#[get("/room/<room_name>")]
fn get_room(home: &State<HomeState>, room_name: &str) -> Option<String> {
    match acquire_room(home, room_name) {
        Ok(room) => Some(room.lock().unwrap().report_to_string()),
        _ => None,
    }
}

#[post("/room", data = "<form_data>")]
fn post_room(home: &State<HomeState>, form_data: Form<RoomFormData>) -> Status {
    let data = form_data.into_inner();
    if data.name.len() > 0 {
        home.0.lock().unwrap().add_room(data.name.as_ref());
        Status::Created
    } else {
        Status::BadRequest
    }
}

#[delete("/room/<room_name>")]
fn delete_room(home: &State<HomeState>, room_name: &str) -> Status {
    home.0
        .lock()
        .unwrap()
        .remove_room(room_name);
    Status::NoContent
}

#[get("/room/<room_name>/device/<device_name>")]
fn get_device(home: &State<HomeState>, room_name: &str, device_name: &str) -> Option<String> {
    match acquire_device(home, room_name, device_name) {
        Ok(room) => Some(room.report_to_string()),
        _ => None,
    }
}

#[post("/room/<room_name>/device", data = "<form_data>")]
fn post_device(home: &State<HomeState>, room_name: &str, form_data: Form<DeviceFormData>) -> Status {
    let data = form_data.into_inner();
    let room = match acquire_room(home, room_name) {
        Ok(room) => room,
        Err(_) => return Status::BadRequest,
    };

    if data.name.len() == 0 || room_name.len() == 0 {
        return Status::BadRequest;
    }

    if data.device_type == "OUTLET" {
        let outlet = Arc::new(SmartOutlet::new(data.name.as_ref()));
        room.lock().unwrap().add_device(Device::Outlet(outlet));
        return Status::Created;
    }

    if data.device_type == "THERMOMETER" {
        let thermometer = Arc::new(SmartThermometer::new(data.name.as_ref()));
        room.lock().unwrap().add_device(Device::Thermometer(thermometer));
        return Status::Created;
    }

    Status::BadRequest
}

#[delete("/room/<room_name>/device/<device_name>")]
fn delete_device(home: &State<HomeState>, room_name: &str, device_name: &str) -> Status {
    let room = acquire_room(home, room_name).unwrap();
    room.lock().unwrap().remove_device(device_name);
    Status::NoContent
}

#[launch]
fn rocket() -> _ {
    let home = HomeState(Mutex::new(make_home()));

    rocket::build()
        .mount("/", routes![
            delete_device,
            delete_room,
            get_device,
            get_home,
            get_room,
            post_device,
            post_room,
        ])
        .manage(home)
}
