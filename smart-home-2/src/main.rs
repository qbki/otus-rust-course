mod common;
mod sensors;
mod smart_home;
mod smart_outlet;
mod smart_thermometer;

use common::{Print, SwitchStatusEnum};
use sensors::MockedSensor;
use smart_outlet::SmartOutlet;
use smart_home::SmartHome;
use smart_thermometer::SmartThermometer;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let fridge_outlet = SmartOutlet::new(
        String::from("Fridge"),
        Box::new(MockedSensor::new(vec![1000.0, 0.0, 3000.0])),
        Box::new(MockedSensor::new(vec![4000.0, 0.0, 6000.0])),
        Box::new(MockedSensor::new(vec![SwitchStatusEnum::On, SwitchStatusEnum::Off, SwitchStatusEnum::On])),
    );
    let unknown_outlet = SmartOutlet::new(
        String::from("Unknown outlet"),
        Box::new(MockedSensor::new(vec![0.0, 0.0, 0.0, 300.0])),
        Box::new(MockedSensor::new(vec![0.0, 0.0, 0.0, 600.0])),
        Box::new(MockedSensor::new(vec![
                SwitchStatusEnum::Off, SwitchStatusEnum::Off,
                SwitchStatusEnum::Off, SwitchStatusEnum::On,
        ])),
    );
    let unknown_thermometer = SmartThermometer::new(
        String::from("Unknown thermometer"),
        Box::new(MockedSensor::new(vec![4.0, 5.0, 4.5, -1.0]))
    );
    let outside_thermometer = SmartThermometer::new(
        String::from("Outside"),
        Box::new(MockedSensor::new(vec![-5.0, -1.0, 0.0]))
    );
    let inside_thermometer = SmartThermometer::new(
        String::from("Inside"),
        Box::new(MockedSensor::new(vec![23.0, 24.0, 25.0]))
    );

    let mut home = SmartHome::new("Home, sweet home");
    home.add_device("Kitchen", Box::new(fridge_outlet));
    home.add_device("Living room", Box::new(inside_thermometer));
    home.add_device("Living room", Box::new(outside_thermometer));
    home.add_device("Deep scary basement", Box::new(unknown_outlet));
    home.add_device("Deep scary basement", Box::new(unknown_thermometer));

    let sleep_duration = Duration::from_millis(1000);
    loop {
        print!("\x1B[2J"); // clear screen
        print!("\x1B[H"); // move cursor to (0, 0)
        home.print(0);
        sleep(sleep_duration);
    }
}
