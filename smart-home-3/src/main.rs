pub extern crate self as smart;

pub mod common;
mod smart_home;
mod smart_outlet;
mod smart_room;
mod smart_thermometer;

use common::{Report, ReportType::*};
use smart_home::SmartHome;
use smart_outlet::SmartOutlet;
use smart_thermometer::SmartThermometer;

const KITCHEN: &str = "Kitchen";
const LIVING_ROOM: &str = "Living room";
const BASEMENT: &str = "Deep scary basement";

fn main() {
    let fridge_outlet = SmartOutlet::new("Fridge");
    let unknown_outlet = SmartOutlet::new("Unknown outlet");
    let unknown_thermometer = SmartThermometer::new("Unknown thermometer");
    let outside_thermometer = SmartThermometer::new("Outside");
    let inside_thermometer = SmartThermometer::new("Inside");

    let mut home = SmartHome::new("Home, sweet home");
    home.add_device(KITCHEN, Box::new(fridge_outlet));
    home.add_device(LIVING_ROOM, Box::new(inside_thermometer));
    home.add_device(LIVING_ROOM, Box::new(outside_thermometer));
    home.add_device(BASEMENT, Box::new(unknown_outlet));
    home.add_device(BASEMENT, Box::new(unknown_thermometer));

    let mut saved_for_report = String::new();
    saved_for_report.push_str(&home.report_by(
        &Device(BASEMENT, "Unknown thermometer")
    ));
    saved_for_report.push_str("\n\n");
    saved_for_report.push_str(&home.report_by(
        &Device(BASEMENT, "WRONG_DEVICE_NAME")
    ));

    {
        print!("\x1B[2J"); // clear screen
        print!("\x1B[H"); // move cursor to (0, 0)

        println!("*** Report ***");
        println!("{}", home.report().join("\n"));
        println!();

        println!("*** List of Rooms ***");
        for room in home.get_rooms() {
            println!("{}", room.get_name());
        }
        println!();

        println!("*** List of devices from \"{}\"***", BASEMENT);
        if let Some(devices) = home.get_devices_from(BASEMENT) {
            for device in devices {
                println!("{}", device.get_name());
            }
        }
        println!();

        println!("*** Please copy and paste it into a weakly report ***");
        println!("{}", saved_for_report);
    }
}
