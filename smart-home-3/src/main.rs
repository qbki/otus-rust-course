mod common;
mod mocks;
mod smart_home;
mod smart_outlet;
mod smart_room;
mod smart_thermometer;

use common::ReportType::*;
use mocks::{make_home, BASEMENT};

fn main() {
    let home = make_home();

    let mut saved_for_report = String::new();
    saved_for_report.push_str(&home.report_by(&Device(BASEMENT, "Unknown thermometer")));
    saved_for_report.push_str("\n\n");
    saved_for_report.push_str(&home.report_by(&Device(BASEMENT, "WRONG_DEVICE_NAME")));

    println!("*** List of Rooms ***");
    for room in home.get_rooms() {
        println!("{}", room.get_name());
    }
    println!();

    println!("*** List of devices from \"{}\"***", BASEMENT);
    let devices = home.get_room(BASEMENT).map(|room| room.get_devices());
    if let Some(devices) = devices {
        for device in devices {
            println!("{}", device.get_name());
        }
    }
    println!();

    println!("*** Please copy and paste it into a weakly report ***");
    println!("{}", saved_for_report);
}
