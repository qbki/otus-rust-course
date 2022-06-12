extern crate smart_home_3 as smart_home;

use smart_home::common::ReportType;
use smart_home::smart_home::SmartHome;
use smart_home::smart_thermometer::SmartThermometer;

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
    home.add_device(ROOM_NAME, Box::new(thermometer));

    let device = home
        .get_room(ROOM_NAME)
        .and_then(|room| room.get_device(DEVICE_NAME));

    assert_eq!(device.unwrap().get_name(), DEVICE_NAME);
}

#[test]
fn should_get_report_by_an_entity_type() {
    let mut home = SmartHome::new(HOME_NAME);
    let thermometer = SmartThermometer::new(DEVICE_NAME);
    home.add_device(ROOM_NAME, Box::new(thermometer));

    assert_eq!(
        home.report_by(&ReportType::Home),
        "Home: HOME\n    Room: ROOM\n        Thermometer: DEVICE\n            temperature: 0.0°C",
        "returns a report about a home"
    );
    assert_eq!(
        home.report_by(&ReportType::Room(ROOM_NAME)),
        "Room: ROOM\n    Thermometer: DEVICE\n        temperature: 0.0°C",
        "returns a report about a root"
    );
    assert_eq!(
        home.report_by(&ReportType::Device(ROOM_NAME, DEVICE_NAME)),
        "Thermometer: DEVICE\n    temperature: 0.0°C",
        "returns a report about a device"
    );
}
