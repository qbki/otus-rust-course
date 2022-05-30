mod common;
mod sensors;
mod smart_home;
mod smart_outlet;
mod smart_thermometer;

use common::{report, SwitchStatusEnum};
use sensors::MockedSensor;
use smart_home::SmartHome;
use smart_outlet::SmartOutlet;
use smart_thermometer::SmartThermometer;
use std::thread::sleep;
use std::time::Duration;

const KITCHEN: &str = "Kitchen";
const LIVING_ROOM: &str = "Living room";
const BASEMENT: &str = "Deep scary basement";

fn main() {
    let fridge_outlet = SmartOutlet::new(
        String::from("Fridge"),
        Box::new(MockedSensor::new(vec![1000.0, 0.0, 3000.0])),
        Box::new(MockedSensor::new(vec![4000.0, 0.0, 6000.0])),
        Box::new(MockedSensor::new(vec![
            SwitchStatusEnum::On,
            SwitchStatusEnum::Off,
            SwitchStatusEnum::On,
        ])),
    );
    let unknown_outlet = SmartOutlet::new(
        String::from("Unknown outlet"),
        Box::new(MockedSensor::new(vec![0.0, 0.0, 0.0, 300.0])),
        Box::new(MockedSensor::new(vec![0.0, 0.0, 0.0, 600.0])),
        Box::new(MockedSensor::new(vec![
            SwitchStatusEnum::Off,
            SwitchStatusEnum::Off,
            SwitchStatusEnum::Off,
            SwitchStatusEnum::On,
        ])),
    );
    let unknown_thermometer = SmartThermometer::new(
        String::from("Unknown thermometer"),
        Box::new(MockedSensor::new(vec![4.0, 5.0, 4.5, -1.0])),
    );
    let outside_thermometer = SmartThermometer::new(
        String::from("Outside"),
        Box::new(MockedSensor::new(vec![-5.0, -1.0, 0.0])),
    );
    let inside_thermometer = SmartThermometer::new(
        String::from("Inside"),
        Box::new(MockedSensor::new(vec![23.0, 24.0, 25.0])),
    );

    let mut home = SmartHome::new("Home, sweet home");
    home.add_device(KITCHEN, Box::new(fridge_outlet));
    home.add_device(LIVING_ROOM, Box::new(inside_thermometer));
    home.add_device(LIVING_ROOM, Box::new(outside_thermometer));
    home.add_device(BASEMENT, Box::new(unknown_outlet));
    home.add_device(BASEMENT, Box::new(unknown_thermometer));

    let mut saved_for_report = String::new();
    saved_for_report.push_str(report(&home, BASEMENT, "Unknown thermometer").as_str());
    saved_for_report.push_str("\n\n");
    saved_for_report.push_str(report(&home, BASEMENT, "WRONG_DEVICE_NAME").as_str());

    let sleep_duration = Duration::from_millis(1000);
    loop {
        print!("\x1B[2J"); // clear screen
        print!("\x1B[H"); // move cursor to (0, 0)

        println!("*** Report ***");
        for line in home.full_report() {
            println!("{}", line)
        }
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

        sleep(sleep_duration);
    }
}
