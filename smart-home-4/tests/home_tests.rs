extern crate smart_home_4 as smart;

use smart::common::{Report, Device, DeviceInterface, RequestType, SmartHomeErrorEnum};
use smart::smart_home::SmartHome;
use smart::smart_room::SmartRoom;
use smart::smart_thermometer::SmartThermometer;

const HOME_NAME: &str = "HOME";
const ROOM_NAME: &str = "ROOM";
const DEVICE_NAME: &str = "DEVICE";

#[test]
fn should_add_a_room_into_a_home() {
    let mut home = SmartHome::new(HOME_NAME);
    home.add_room(ROOM_NAME);

    let rooms = home.get_rooms();

    assert_eq!(rooms[0].get_name(), ROOM_NAME);
}

#[test]
fn should_add_a_device_into_a_home() {
    let mut home = SmartHome::new(HOME_NAME);
    let thermometer = SmartThermometer::new(DEVICE_NAME);
    home.add_device(ROOM_NAME, Device::Thermometer(thermometer));

    let device: Result<&dyn DeviceInterface, SmartHomeErrorEnum> = home.get(&RequestType::Device(ROOM_NAME, DEVICE_NAME)).into();

    assert_eq!(device.unwrap().get_name(), DEVICE_NAME);
}

#[test]
fn should_get_report_by_an_entity_type() {
    let mut home = SmartHome::new(HOME_NAME);
    let thermometer = SmartThermometer::new(DEVICE_NAME);
    home.add_device(ROOM_NAME, Device::Generic(Box::new(thermometer)));

    let room: Result<&SmartRoom, SmartHomeErrorEnum> = home.get(&RequestType::Room(ROOM_NAME)).into();
    let device: Result<&dyn DeviceInterface, SmartHomeErrorEnum> = home.get(&RequestType::Device(ROOM_NAME, DEVICE_NAME)).into();

    assert_eq!(
        home.report_to_string(),
        "Home: HOME\n    Room: ROOM\n        Thermometer: DEVICE\n            temperature: 0.0°C",
        "snapshot was made by ReportType::Home"
    );
    assert_eq!(
        room.unwrap().report_to_string(),
        "Room: ROOM\n    Thermometer: DEVICE\n        temperature: 0.0°C",
        "snapshot was made by ReportType::Room"
    );
    assert_eq!(
        device.unwrap().report_to_string(),
        "Thermometer: DEVICE\n    temperature: 0.0°C",
        "snapshot was made by ReportType::Device"
    );
}
