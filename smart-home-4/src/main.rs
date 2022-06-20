mod common;
mod mocks;
mod smart_home;
mod smart_outlet;
mod smart_room;
mod smart_thermometer;

use common::{DeviceInterface, Report, RequestType, SmartHomeErrorEnum, SwitchStatusEnum};
use mocks::{make_home, BASEMENT, UNKNOWN_OUTLET};
use smart_home::ResponseData;
use smart_outlet::SmartOutlet;
use std::error::Error;

fn main() {
    let home = make_home();

    let mut saved_for_report = String::new();

    let device_report: String = home
        .get(&RequestType::Device(BASEMENT, "Unknown thermometer"))
        .into();
    saved_for_report.push_str(&device_report);

    saved_for_report.push_str("\n\n");

    let wrong_device_report: Result<&SmartOutlet, SmartHomeErrorEnum> = home
        .get(&RequestType::Device(BASEMENT, "WRONG_DEVICE_NAME"))
        .into();

    if let Err(error) = wrong_device_report {
        saved_for_report.push_str(error.to_string().as_ref());
        saved_for_report.push('\n');
        saved_for_report.push_str(format!("{:?}", error.source()).as_ref());
    }

    println!("*** List of Rooms ***");
    for room in home.get_rooms() {
        println!("{}", room.get_name());
    }
    println!();

    println!("*** List of devices from \"{}\"***", BASEMENT);
    let response = home.get(&RequestType::Room(BASEMENT));
    let devices_result = response.as_ref().map(|response| {
        if let ResponseData::Room(room) = response {
            room.get_devices()
        } else {
            vec![]
        }
    });
    if let Ok(devices) = devices_result {
        for device in devices {
            println!("{}", device.get_name());
        }
    }
    println!();

    println!("*** Get outlet from \"{}\"***", BASEMENT);
    let response: Result<&SmartOutlet, SmartHomeErrorEnum> = home
        .get(&RequestType::Device(BASEMENT, UNKNOWN_OUTLET))
        .into();
    if let Ok(outlet) = response {
        println!("{}", outlet.report_to_string());
        outlet.set_switch(SwitchStatusEnum::On);
    }
    let generic_response: Result<&dyn DeviceInterface, SmartHomeErrorEnum> = home
        .get(&RequestType::Device(BASEMENT, UNKNOWN_OUTLET))
        .into();
    if let Ok(generic) = generic_response {
        println!("{}", generic.report_to_string());
    }
    println!();

    println!("*** Please copy and paste it into a weakly report ***");
    println!("{}", saved_for_report);
}
